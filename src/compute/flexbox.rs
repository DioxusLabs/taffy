//! Computes the [flexbox](https://css-tricks.com/snippets/css/a-guide-to-flexbox/) layout algorithm on [`Taffy`](crate::Taffy) according to the [spec](https://www.w3.org/TR/css-flexbox-1/)
//!
//! Note that some minor steps appear to be missing: see https://github.com/DioxusLabs/taffy/issues for more information.
use core::f32;

use crate::geometry::{Point, Rect, Size};
use crate::layout::{Cache, Layout, RunMode};
use crate::math::MaybeMath;
use crate::node::Node;
use crate::resolve::{MaybeResolve, ResolveOrDefault};
use crate::style::{AlignContent, AlignSelf, Dimension, Display, FlexWrap, JustifyContent, PositionType};
use crate::style::{FlexDirection, FlexboxLayout};
use crate::sys::{abs, round, Vec};
use crate::tree::LayoutTree;

/// The intermediate results of a flexbox calculation for a single item
struct FlexItem {
    /// The identifier for the associated [`Node`](crate::node::Node)
    node: Node,

    /// The base size of this item
    size: Size<Option<f32>>,
    /// The minimum allowable size of this item
    min_size: Size<Option<f32>>,
    /// The maximum allowable size of this item
    max_size: Size<Option<f32>>,

    /// The final offset of this item
    position: Rect<Option<f32>>,
    /// The margin of this item
    margin: Rect<f32>,
    /// The padding of this item
    padding: Rect<f32>,
    /// The border of this item
    border: Rect<f32>,

    /// The default size of this item
    flex_basis: f32,
    /// The default size of this item, minus padding and border
    inner_flex_basis: f32,
    /// The amount by which this item has deviated from its target size
    violation: f32,
    /// Is the size of this item locked
    frozen: bool,

    /// The proposed inner size of this item
    hypothetical_inner_size: Size<f32>,
    /// The proposed outer size of this item
    hypothetical_outer_size: Size<f32>,
    /// The size that this item wants to be
    target_size: Size<f32>,
    /// The size that this item wants to be, plus any padding and border
    outer_target_size: Size<f32>,

    /// The position of the bottom edge of this item
    baseline: f32,

    /// A temporary value for the main offset
    ///
    /// Offset is the relative position from the item's natural flow position based on
    /// relative position values, alignment, and justification. Does not include margin/padding/border.
    offset_main: f32,
    /// A temporary value for the cross offset
    ///
    /// Offset is the relative position from the item's natural flow position based on
    /// relative position values, alignment, and justification. Does not include margin/padding/border.
    offset_cross: f32,
}

/// A line of [`FlexItem`] used for intermediate computation
struct FlexLine<'a> {
    /// The slice of items to iterate over during computation of this line
    items: &'a mut [FlexItem],
    /// The dimensions of the cross-axis
    cross_size: f32,
    /// The relative offset of the cross-axis
    offset_cross: f32,
}

/// Values that can be cached during the flexbox algorithm
struct AlgoConstants {
    /// The direction of the current segment being laid out
    dir: FlexDirection,
    /// Is this segment a row
    is_row: bool,
    /// Is this segment a column
    is_column: bool,
    /// Is the wrap direction inverted
    is_wrap_reverse: bool,

    /// The margin of this section
    margin: Rect<f32>,
    /// The border of this section
    border: Rect<f32>,
    /// The padding of this section
    padding_border: Rect<f32>,

    /// The size of the internal node
    node_inner_size: Size<Option<f32>>,
    /// The size of the surrounding container
    container_size: Size<f32>,
    /// The size of the internal container
    inner_container_size: Size<f32>,
}

/// Computes the layout of [`LayoutTree`] according to the flexbox algorithm
pub fn compute(tree: &mut impl LayoutTree, root: Node, size: Size<Option<f32>>) {
    let style = tree.style(root);
    let has_root_min_max = style.min_size.width.is_defined()
        || style.min_size.height.is_defined()
        || style.max_size.width.is_defined()
        || style.max_size.height.is_defined();

    // Pull these out earlier to avoid borrowing issues
    let width_maybe_max = style.min_size.width.maybe_resolve(size.width);
    let width_maybe_min = style.max_size.width.maybe_resolve(size.width);
    let height_maybe_max = style.min_size.height.maybe_resolve(size.height);
    let height_maybe_min = style.max_size.height.maybe_resolve(size.height);

    let preliminary_size = if has_root_min_max {
        let first_pass = compute_preliminary(tree, root, style.size.maybe_resolve(size), size, RunMode::ComputeSize, true);

        compute_preliminary(
            tree,
            root,
            Size {
                width: first_pass.width.maybe_max(width_maybe_max).maybe_min(width_maybe_min).into(),
                height: first_pass.height.maybe_max(height_maybe_max).maybe_min(height_maybe_min).into(),
            },
            size,
            RunMode::PeformLayout,
            true,
        )
    } else {
        compute_preliminary(tree, root, style.size.maybe_resolve(size), size, RunMode::PeformLayout, true)
    };

    *tree.layout_mut(root) = Layout { order: 0, size: preliminary_size, location: Point::ZERO };

    round_layout(tree, root, 0.0, 0.0);
}

/// Compute a preliminary size for an item
fn compute_preliminary(
    tree: &mut impl LayoutTree,
    node: Node,
    node_size: Size<Option<f32>>,
    parent_size: Size<Option<f32>>,
    run_mode: RunMode,
    main_size: bool,
) -> Size<f32> {
    // clear the dirtiness of the node now that we've computed it
    tree.mark_dirty(node, false);

    // First we check if we have a result for the given input
    if let Some(cached_size) = compute_from_cache(tree, node, node_size, parent_size, run_mode, main_size) {
        return cached_size;
    }

    // Define some general constants we will need for the remainder of the algorithm.
    let mut constants = compute_constants(tree.style(node), node_size, parent_size);

    // If this is a leaf node we can skip a lot of this function in some cases
    if tree.children(node).is_empty() {
        if node_size.width.is_some() && node_size.height.is_some() {
            return node_size.map(|s| s.unwrap_or(0.0));
        }

        if tree.needs_measure(node) {
            let converted_size = tree.measure_node(node, node_size);
            *cache(tree, node, main_size) =
                Some(Cache { node_size, parent_size, run_mode, size: converted_size });
            return converted_size;
        }

        return Size {
            width: node_size.width.unwrap_or(0.0) + constants.padding_border.horizontal_axis_sum(),
            height: node_size.height.unwrap_or(0.0) + constants.padding_border.vertical_axis_sum(),
        };
    }

    // 9. Flex Layout Algorithm

    // 9.1. Initial Setup

    // 1. Generate anonymous flex items as described in §4 Flex Items.
    let mut flex_items = generate_anonymous_flex_items(tree, node, &constants);

    // 9.2. Line Length Determination

    // 2. Determine the available main and cross space for the flex items.
    let available_space = determine_available_space(node_size, parent_size, &constants);

    let has_baseline_child =
        flex_items.iter().any(|child| tree.style(child.node).align_self(tree.style(node)) == AlignSelf::Baseline);

    // 3. Determine the flex base size and hypothetical main size of each item.
    determine_flex_base_size(tree, node, node_size, &constants, available_space, &mut flex_items);

    // TODO: Add step 4 according to spec: https://www.w3.org/TR/css-flexbox-1/#algo-main-container
    // 9.3. Main Size Determination

    // 5. Collect flex items into flex lines.
    let mut flex_lines = collect_flex_lines(tree, node, &constants, available_space, &mut flex_items);

    // 6. Resolve the flexible lengths of all the flex items to find their used main size.
    for line in &mut flex_lines {
        resolve_flexible_lengths(tree, line, &constants, available_space);
    }

    // TODO: Cleanup and make according to spec
    // Not part of the spec from what i can see but seems correct
    constants.container_size.set_main(
        constants.dir,
        node_size.main(constants.dir).unwrap_or({
            let longest_line = flex_lines.iter().fold(f32::MIN, |acc, line| {
                let length: f32 = line.items.iter().map(|item| item.outer_target_size.main(constants.dir)).sum();
                acc.max(length)
            });

            let size = longest_line + constants.padding_border.main_axis_sum(constants.dir);
            match available_space.main(constants.dir) {
                Some(val) if flex_lines.len() > 1 && size < val => val,
                _ => size,
            }
        }),
    );

    constants.inner_container_size.set_main(
        constants.dir,
        constants.container_size.main(constants.dir) - constants.padding_border.main_axis_sum(constants.dir),
    );

    // 9.4. Cross Size Determination

    // 7. Determine the hypothetical cross size of each item.
    for line in &mut flex_lines {
        determine_hypothetical_cross_size(tree, line, &constants, available_space);
    }

    // TODO - probably should move this somewhere else as it doesn't make a ton of sense here but we need it below
    // TODO - This is expensive and should only be done if we really require a baseline. aka, make it lazy
    if has_baseline_child {
        calculate_children_base_lines(tree, node, node_size, &mut flex_lines, &constants);
    }

    // 8. Calculate the cross size of each flex line.
    calculate_cross_size(tree, &mut flex_lines, node, node_size, &constants);

    // 9. Handle 'align-content: stretch'.
    handle_align_content_stretch(tree, &mut flex_lines, node, node_size, &constants);

    // 10. Collapse visibility:collapse items. If any flex items have visibility: collapse,
    //     note the cross size of the line they’re in as the item’s strut size, and restart
    //     layout from the beginning.
    //
    //     In this second layout round, when collecting items into lines, treat the collapsed
    //     items as having zero main size. For the rest of the algorithm following that step,
    //     ignore the collapsed items entirely (as if they were display:none) except that after
    //     calculating the cross size of the lines, if any line’s cross size is less than the
    //     largest strut size among all the collapsed items in the line, set its cross size to
    //     that strut size.
    //
    //     Skip this step in the second layout round.

    // TODO implement once (if ever) we support visibility:collapse

    // 11. Determine the used cross size of each flex item.
    determine_used_cross_size(tree, &mut flex_lines, node, &constants);

    // 9.5. Main-Axis Alignment

    // 12. Distribute any remaining free space.
    distribute_remaining_free_space(tree, &mut flex_lines, node, &constants);

    // 9.6. Cross-Axis Alignment

    // 13. Resolve cross-axis auto margins (also includes 14).
    resolve_cross_axis_auto_margins(tree, &mut flex_lines, node, &constants);

    // 15. Determine the flex container’s used cross size.
    let total_cross_size = determine_container_cross_size(&mut flex_lines, node_size, &mut constants);

    // We have the container size.
    // If our caller does not care about performing layout we are done now.
    if run_mode == RunMode::ComputeSize {
        let container_size = constants.container_size;
        *cache(tree, node, main_size) = Some(Cache { node_size, parent_size, run_mode, size: container_size });
        return container_size;
    }

    // 16. Align all flex lines per align-content.
    align_flex_lines_per_align_content(tree, &mut flex_lines, node, &constants, total_cross_size);

    // Do a final layout pass and gather the resulting layouts
    final_layout_pass(tree, node, &mut flex_lines, &constants);

    // Before returning we perform absolute layout on all absolutely positioned children
    perform_absolute_layout_on_absolute_children(tree, node, &constants);

    let len = tree.children(node).len();
    for order in 0..len {
        let child = tree.child(node, order);
        if tree.style(child).display == Display::None {
            // if self.nodes[*child].style.display == Display::None {
            hidden_layout(tree, child, order as _);
        }
    }

    let container_size = constants.container_size;
    *cache(tree, node, main_size) = Some(Cache { node_size, parent_size, run_mode, size: container_size });

    container_size
}

/// Rounds the calculated [`NodeData`] according to the spec
fn round_layout(tree: &mut impl LayoutTree, root: Node, abs_x: f32, abs_y: f32) {
    let layout = tree.layout_mut(root);
    let abs_x = abs_x + layout.location.x;
    let abs_y = abs_y + layout.location.y;

    layout.location.x = round(layout.location.x);
    layout.location.y = round(layout.location.y);

    layout.size.width = round(layout.size.width);
    layout.size.height = round(layout.size.height);

    // Satisfy the borrow checker here by re-indexing to shorten the lifetime to the loop scope
    for x in 0..tree.children(root).len() {
        let child = tree.child(root, x);
        round_layout(tree, child, abs_x, abs_y);
    }
}

/// Saves intermediate results to a [`Cache`]
fn cache(tree: &mut impl LayoutTree, node: Node, main_size: bool) -> &mut Option<Cache> {
    if main_size {
        tree.primary_cache(node)
    } else {
        tree.secondary_cache(node)
    }
}

/// Try to get the computation result from the cache.
#[inline]
fn compute_from_cache(
    tree: &mut impl LayoutTree,
    node: Node,
    node_size: Size<Option<f32>>,
    parent_size: Size<Option<f32>>,
    run_mode: RunMode,
    main_size: bool,
) -> Option<Size<f32>> {
    if let Some(ref cache) = cache(tree, node, main_size) {
        // Cached ComputeSize results are not valid if we are running in PerformLayout mode
        if cache.run_mode == RunMode::ComputeSize && run_mode == RunMode::PeformLayout {
            return None;
        }

        let width_compatible = if let Some(width) = node_size.width {
            abs(width - cache.size.width) < f32::EPSILON
        } else {
            cache.node_size.width.is_none()
        };

        let height_compatible = if let Some(height) = node_size.height {
            abs(height - cache.size.height) < f32::EPSILON
        } else {
            cache.node_size.height.is_none()
        };

        if width_compatible && height_compatible {
            return Some(cache.size);
        }

        if cache.node_size == node_size && cache.parent_size == parent_size {
            return Some(cache.size);
        }
    }

    None
}

/// Compute constants that can be reused during the flexbox algorithm.
#[inline]
fn compute_constants(
    style: &FlexboxLayout,
    node_size: Size<Option<f32>>,
    parent_size: Size<Option<f32>>,
) -> AlgoConstants {
    let dir = style.flex_direction;
    let is_row = dir.is_row();
    let is_column = dir.is_column();
    let is_wrap_reverse = style.flex_wrap == FlexWrap::WrapReverse;

    let margin = style.margin.resolve_or_default(parent_size.width);
    let padding = style.padding.resolve_or_default(parent_size.width);
    let border = style.border.resolve_or_default(parent_size.width);

    let padding_border = Rect {
        start: padding.start + border.start,
        end: padding.end + border.end,
        top: padding.top + border.top,
        bottom: padding.bottom + border.bottom,
    };

    let node_inner_size = Size {
        width: node_size.width.maybe_sub(padding_border.horizontal_axis_sum()),
        height: node_size.height.maybe_sub(padding_border.vertical_axis_sum()),
    };

    let container_size = Size::ZERO;
    let inner_container_size = Size::ZERO;

    AlgoConstants {
        dir,
        is_row,
        is_column,
        is_wrap_reverse,
        margin,
        border,
        padding_border,
        node_inner_size,
        container_size,
        inner_container_size,
    }
}

/// Generate anonymous flex items.
///
/// # [9.1. Initial Setup](https://www.w3.org/TR/css-flexbox-1/#box-manip)
///
/// - [**Generate anonymous flex items**](https://www.w3.org/TR/css-flexbox-1/#algo-anon-box) as described in [§4 Flex Items](https://www.w3.org/TR/css-flexbox-1/#flex-items).
#[inline]
fn generate_anonymous_flex_items(tree: &impl LayoutTree, node: Node, constants: &AlgoConstants) -> Vec<FlexItem> {
    tree.children(node)
        .iter()
        .map(|child| (child, tree.style(*child)))
        .filter(|(_, style)| style.position_type != PositionType::Absolute)
        .filter(|(_, style)| style.display != Display::None)
        .map(|(child, child_style)| FlexItem {
            node: *child,
            size: child_style.size.maybe_resolve(constants.node_inner_size),
            min_size: child_style.min_size.maybe_resolve(constants.node_inner_size),
            max_size: child_style.max_size.maybe_resolve(constants.node_inner_size),

            position: child_style.position.zip_size(constants.node_inner_size, |p, s| p.maybe_resolve(s)),
            margin: child_style.margin.resolve_or_default(constants.node_inner_size.width),
            padding: child_style.padding.resolve_or_default(constants.node_inner_size.width),
            border: child_style.border.resolve_or_default(constants.node_inner_size.width),
            flex_basis: 0.0,
            inner_flex_basis: 0.0,
            violation: 0.0,
            frozen: false,

            hypothetical_inner_size: Size::ZERO,
            hypothetical_outer_size: Size::ZERO,
            target_size: Size::ZERO,
            outer_target_size: Size::ZERO,

            baseline: 0.0,

            offset_main: 0.0,
            offset_cross: 0.0,
        })
        .collect()
}

/// Determine the available main and cross space for the flex items.
///
/// # [9.2. Line Length Determination](https://www.w3.org/TR/css-flexbox-1/#line-sizing)
///
/// - [**Determine the available main and cross space for the flex items**](https://www.w3.org/TR/css-flexbox-1/#algo-available).
/// For each dimension, if that dimension of the flex container’s content box is a definite size, use that;
/// if that dimension of the flex container is being sized under a min or max-content constraint, the available space in that dimension is that constraint;
/// otherwise, subtract the flex container’s margin, border, and padding from the space available to the flex container in that dimension and use that value.
/// **This might result in an infinite value**.
#[inline]
#[must_use]
fn determine_available_space(
    node_size: Size<Option<f32>>,
    parent_size: Size<Option<f32>>,
    constants: &AlgoConstants,
) -> Size<Option<f32>> {
    let width = match node_size.width {
        Some(node_width) => Some(node_width),
        None => parent_size
            .width
            .maybe_sub(constants.margin.horizontal_axis_sum())
            .maybe_sub(constants.padding_border.horizontal_axis_sum()),
    };

    let height = match node_size.height {
        Some(node_height) => Some(node_height),
        None => parent_size
            .height
            .maybe_sub(constants.margin.vertical_axis_sum())
            .maybe_sub(constants.padding_border.vertical_axis_sum()),
    };

    Size { width, height }
}

/// Determine the flex base size and hypothetical main size of each item.
///
/// # [9.2. Line Length Determination](https://www.w3.org/TR/css-flexbox-1/#line-sizing)
///
/// - [**Determine the flex base size and hypothetical main size of each item:**](https://www.w3.org/TR/css-flexbox-1/#algo-main-item)
///
///     - A. If the item has a definite used flex basis, that’s the flex base size.
///
///     - B. If the flex item has ...
///
///         - an intrinsic aspect ratio,
///         - a used flex basis of content, and
///         - a definite cross size,
///
///     then the flex base size is calculated from its inner cross size and the flex item’s intrinsic aspect ratio.
///
///     - C. If the used flex basis is content or depends on its available space, and the flex container is being sized under a min-content
///         or max-content constraint (e.g. when performing automatic table layout \[CSS21\]), size the item under that constraint.
///         The flex base size is the item’s resulting main size.
///
///     - E. Otherwise, size the item into the available space using its used flex basis in place of its main size, treating a value of content as max-content.
///         If a cross size is needed to determine the main size (e.g. when the flex item’s main size is in its block axis) and the flex item’s cross size is auto and not definite,
///         in this calculation use fit-content as the flex item’s cross size. The flex base size is the item’s resulting main size.
///
///     When determining the flex base size, the item’s min and max main sizes are ignored (no clamping occurs).
///     Furthermore, the sizing calculations that floor the content box size at zero when applying box-sizing are also ignored.
///     (For example, an item with a specified size of zero, positive padding, and box-sizing: border-box will have an outer flex base size of zero—and hence a negative inner flex base size.)
#[inline]
fn determine_flex_base_size(
    tree: &mut impl LayoutTree,
    node: Node,
    node_size: Size<Option<f32>>,
    constants: &AlgoConstants,
    available_space: Size<Option<f32>>,
    flex_items: &mut Vec<FlexItem>,
) {
    // TODO - this does not follow spec. See the TODOs below
    for child in flex_items.iter_mut() {
        let child_style = tree.style(child.node);

        // A. If the item has a definite used flex basis, that’s the flex base size.

        let flex_basis = child_style.flex_basis.maybe_resolve(constants.node_inner_size.main(constants.dir));
        if flex_basis.is_some() {
            child.flex_basis = flex_basis.unwrap_or(0.0);
            continue;
        };

        // B. If the flex item has an intrinsic aspect ratio,
        //    a used flex basis of content, and a definite cross size,
        //    then the flex base size is calculated from its inner
        //    cross size and the flex item’s intrinsic aspect ratio.

        if let Some(ratio) = child_style.aspect_ratio {
            if let Some(cross) = node_size.cross(constants.dir) {
                if child_style.flex_basis == Dimension::Auto {
                    child.flex_basis = cross * ratio;
                    continue;
                }
            }
        }

        // C. If the used flex basis is content or depends on its available space,
        //    and the flex container is being sized under a min-content or max-content
        //    constraint (e.g. when performing automatic table layout [CSS21]),
        //    size the item under that constraint. The flex base size is the item’s
        //    resulting main size.

        // TODO - Probably need to cover this case in future

        // D. Otherwise, if the used flex basis is content or depends on its
        //    available space, the available main size is infinite, and the flex item’s
        //    inline axis is parallel to the main axis, lay the item out using the rules
        //    for a box in an orthogonal flow [CSS3-WRITING-MODES]. The flex base size
        //    is the item’s max-content main size.

        // TODO - Probably need to cover this case in future

        // E. Otherwise, size the item into the available space using its used flex basis
        //    in place of its main size, treating a value of content as max-content.
        //    If a cross size is needed to determine the main size (e.g. when the
        //    flex item’s main size is in its block axis) and the flex item’s cross size
        //    is auto and not definite, in this calculation use fit-content as the
        //    flex item’s cross size. The flex base size is the item’s resulting main size.

        let width: Option<f32> = if child.size.width.is_none()
            && child_style.align_self(tree.style(node)) == AlignSelf::Stretch
            && constants.is_column
        {
            available_space.width
        } else {
            child.size.width
        };

        let height: Option<f32> = if child.size.height.is_none()
            && child_style.align_self(tree.style(node)) == AlignSelf::Stretch
            && constants.is_row
        {
            available_space.height
        } else {
            child.size.height
        };

        child.flex_basis = compute_preliminary(
            tree,
            child.node,
            Size { width: width.maybe_min(child.max_size.width), height: height.maybe_min(child.max_size.height) },
            available_space,
            RunMode::ComputeSize,
            true,
        )
        .main(constants.dir)
        .maybe_min(child.max_size.main(constants.dir));
    }

    // The hypothetical main size is the item’s flex base size clamped according to its
    // used min and max main sizes (and flooring the content box size at zero).

    for child in flex_items {
        child.inner_flex_basis =
            child.flex_basis - child.padding.main_axis_sum(constants.dir) - child.border.main_axis_sum(constants.dir);

        // TODO - not really spec abiding but needs to be done somewhere. probably somewhere else though.
        // The following logic was developed not from the spec but by trail and error looking into how
        // webkit handled various scenarios. Can probably be solved better by passing in
        // min-content max-content constraints from the top
        let min_main = compute_preliminary(tree, child.node, Size::NONE, available_space, RunMode::ComputeSize, false)
            .main(constants.dir)
            .maybe_max(child.min_size.main(constants.dir))
            .maybe_min(child.size.main(constants.dir))
            .into();

        child.hypothetical_inner_size.set_main(
            constants.dir,
            child.flex_basis.maybe_max(min_main).maybe_min(child.max_size.main(constants.dir)),
        );

        child.hypothetical_outer_size.set_main(
            constants.dir,
            child.hypothetical_inner_size.main(constants.dir) + child.margin.main_axis_sum(constants.dir),
        );
    }
}

/// Collect flex items into flex lines.
///
/// # [9.3. Main Size Determination](https://www.w3.org/TR/css-flexbox-1/#main-sizing)
///
/// - [**Collect flex items into flex lines**](https://www.w3.org/TR/css-flexbox-1/#algo-line-break):
///
///     - If the flex container is single-line, collect all the flex items into a single flex line.
///
///     - Otherwise, starting from the first uncollected item, collect consecutive items one by one until the first time that the next collected item would not fit into the flex container’s inner main size
///         (or until a forced break is encountered, see [§10 Fragmenting Flex Layout](https://www.w3.org/TR/css-flexbox-1/#pagination)).
///         If the very first uncollected item wouldn't fit, collect just it into the line.
///
///         For this step, the size of a flex item is its outer hypothetical main size. (**Note: This can be negative**.)
///
///         Repeat until all flex items have been collected into flex lines.
///
///         **Note that the "collect as many" line will collect zero-sized flex items onto the end of the previous line even if the last non-zero item exactly "filled up" the line**.
#[inline]
fn collect_flex_lines<'a>(
    tree: &impl LayoutTree,
    node: Node,
    constants: &AlgoConstants,
    available_space: Size<Option<f32>>,
    flex_items: &'a mut Vec<FlexItem>,
) -> Vec<FlexLine<'a>> {
    let mut lines = crate::sys::new_vec_with_capacity(1);

    if tree.style(node).flex_wrap == FlexWrap::NoWrap {
        lines.push(FlexLine { items: flex_items.as_mut_slice(), cross_size: 0.0, offset_cross: 0.0 });
    } else {
        let mut flex_items = &mut flex_items[..];

        while !flex_items.is_empty() {
            let mut line_length = 0.0;
            let index = flex_items
                .iter()
                .enumerate()
                .find(|&(idx, child)| {
                    line_length += child.hypothetical_outer_size.main(constants.dir);
                    if let Some(main) = available_space.main(constants.dir) {
                        line_length > main && idx != 0
                    } else {
                        false
                    }
                })
                .map(|(idx, _)| idx)
                .unwrap_or(flex_items.len());

            let (items, rest) = flex_items.split_at_mut(index);
            lines.push(FlexLine { items, cross_size: 0.0, offset_cross: 0.0 });
            flex_items = rest;
        }
    }

    lines
}

/// Resolve the flexible lengths of the items within a flex line.
///
/// # [9.7. Resolving Flexible Lengths](https://www.w3.org/TR/css-flexbox-1/#resolve-flexible-lengths)
#[inline]
fn resolve_flexible_lengths(
    tree: &mut impl LayoutTree,
    line: &mut FlexLine,
    constants: &AlgoConstants,
    available_space: Size<Option<f32>>,
) {
    // 1. Determine the used flex factor. Sum the outer hypothetical main sizes of all
    //    items on the line. If the sum is less than the flex container’s inner main size,
    //    use the flex grow factor for the rest of this algorithm; otherwise, use the
    //    flex shrink factor.

    let used_flex_factor: f32 = line.items.iter().map(|child| child.hypothetical_outer_size.main(constants.dir)).sum();
    let growing = used_flex_factor < constants.node_inner_size.main(constants.dir).unwrap_or(0.0);
    let shrinking = !growing;

    // 2. Size inflexible items. Freeze, setting its target main size to its hypothetical main size
    //    - Any item that has a flex factor of zero
    //    - If using the flex grow factor: any item that has a flex base size
    //      greater than its hypothetical main size
    //    - If using the flex shrink factor: any item that has a flex base size
    //      smaller than its hypothetical main size

    for child in line.items.iter_mut() {
        // TODO - This is not found by reading the spec. Maybe this can be done in some other place
        // instead. This was found by trail and error fixing tests to align with webkit output.
        if constants.node_inner_size.main(constants.dir).is_none() && constants.is_row {
            child.target_size.set_main(
                constants.dir,
                compute_preliminary(
                    tree,
                    child.node,
                    Size {
                        width: child.size.width.maybe_max(child.min_size.width).maybe_min(child.max_size.width),
                        height: child.size.height.maybe_max(child.min_size.height).maybe_min(child.max_size.height),
                    },
                    available_space,
                    RunMode::ComputeSize,
                    false,
                )
                .main(constants.dir)
                .maybe_max(child.min_size.main(constants.dir))
                .maybe_min(child.max_size.main(constants.dir)),
            );
        } else {
            child.target_size.set_main(constants.dir, child.hypothetical_inner_size.main(constants.dir));
        }

        // TODO this should really only be set inside the if-statement below but
        // that causes the target_main_size to never be set for some items

        child
            .outer_target_size
            .set_main(constants.dir, child.target_size.main(constants.dir) + child.margin.main_axis_sum(constants.dir));

        let child_style = tree.style(child.node);
        if (child_style.flex_grow == 0.0 && child_style.flex_shrink == 0.0)
            || (growing && child.flex_basis > child.hypothetical_inner_size.main(constants.dir))
            || (shrinking && child.flex_basis < child.hypothetical_inner_size.main(constants.dir))
        {
            child.frozen = true;
        }
    }

    // 3. Calculate initial free space. Sum the outer sizes of all items on the line,
    //    and subtract this from the flex container’s inner main size. For frozen items,
    //    use their outer target main size; for other items, use their outer flex base size.

    let used_space: f32 = line
        .items
        .iter()
        .map(|child| {
            child.margin.main_axis_sum(constants.dir)
                + if child.frozen { child.target_size.main(constants.dir) } else { child.flex_basis }
        })
        .sum();

    let initial_free_space = constants.node_inner_size.main(constants.dir).maybe_sub(used_space).unwrap_or(0.0);

    // 4. Loop

    loop {
        // a. Check for flexible items. If all the flex items on the line are frozen,
        //    free space has been distributed; exit this loop.

        if line.items.iter().all(|child| child.frozen) {
            break;
        }

        // b. Calculate the remaining free space as for initial free space, above.
        //    If the sum of the unfrozen flex items’ flex factors is less than one,
        //    multiply the initial free space by this sum. If the magnitude of this
        //    value is less than the magnitude of the remaining free space, use this
        //    as the remaining free space.

        let used_space: f32 = line
            .items
            .iter()
            .map(|child| {
                child.margin.main_axis_sum(constants.dir)
                    + if child.frozen { child.target_size.main(constants.dir) } else { child.flex_basis }
            })
            .sum();

        let mut unfrozen: Vec<&mut FlexItem> = line.items.iter_mut().filter(|child| !child.frozen).collect();

        let (sum_flex_grow, sum_flex_shrink): (f32, f32) =
            unfrozen.iter().fold((0.0, 0.0), |(flex_grow, flex_shrink), item| {
                let style = tree.style(item.node);
                (flex_grow + style.flex_grow, flex_shrink + style.flex_shrink)
            });

        let free_space = if growing && sum_flex_grow < 1.0 {
            (initial_free_space * sum_flex_grow)
                .maybe_min(constants.node_inner_size.main(constants.dir).maybe_sub(used_space))
        } else if shrinking && sum_flex_shrink < 1.0 {
            (initial_free_space * sum_flex_shrink)
                .maybe_max(constants.node_inner_size.main(constants.dir).maybe_sub(used_space))
        } else {
            (constants.node_inner_size.main(constants.dir).maybe_sub(used_space)).unwrap_or(0.0)
        };

        // c. Distribute free space proportional to the flex factors.
        //    - If the remaining free space is zero
        //        Do Nothing
        //    - If using the flex grow factor
        //        Find the ratio of the item’s flex grow factor to the sum of the
        //        flex grow factors of all unfrozen items on the line. Set the item’s
        //        target main size to its flex base size plus a fraction of the remaining
        //        free space proportional to the ratio.
        //    - If using the flex shrink factor
        //        For every unfrozen item on the line, multiply its flex shrink factor by
        //        its inner flex base size, and note this as its scaled flex shrink factor.
        //        Find the ratio of the item’s scaled flex shrink factor to the sum of the
        //        scaled flex shrink factors of all unfrozen items on the line. Set the item’s
        //        target main size to its flex base size minus a fraction of the absolute value
        //        of the remaining free space proportional to the ratio. Note this may result
        //        in a negative inner main size; it will be corrected in the next step.
        //    - Otherwise
        //        Do Nothing

        if free_space.is_normal() {
            if growing && sum_flex_grow > 0.0 {
                for child in &mut unfrozen {
                    child.target_size.set_main(
                        constants.dir,
                        child.flex_basis + free_space * (tree.style(child.node).flex_grow / sum_flex_grow),
                    );
                }
            } else if shrinking && sum_flex_shrink > 0.0 {
                let sum_scaled_shrink_factor: f32 =
                    unfrozen.iter().map(|child| child.inner_flex_basis * tree.style(child.node).flex_shrink).sum();

                if sum_scaled_shrink_factor > 0.0 {
                    for child in &mut unfrozen {
                        let scaled_shrink_factor = child.inner_flex_basis * tree.style(child.node).flex_shrink;
                        child.target_size.set_main(
                            constants.dir,
                            child.flex_basis + free_space * (scaled_shrink_factor / sum_scaled_shrink_factor),
                        )
                    }
                }
            }
        }

        // d. Fix min/max violations. Clamp each non-frozen item’s target main size by its
        //    used min and max main sizes and floor its content-box size at zero. If the
        //    item’s target main size was made smaller by this, it’s a max violation.
        //    If the item’s target main size was made larger by this, it’s a min violation.

        let total_violation = unfrozen.iter_mut().fold(0.0, |acc, child| -> f32 {
            // TODO - not really spec abiding but needs to be done somewhere. probably somewhere else though.
            // The following logic was developed not from the spec but by trial and error looking into how
            // webkit handled various scenarios. Can probably be solved better by passing in
            // min-content max-content constraints from the top. Need to figure out correct thing to do here as
            // just piling on more conditionals.
            let min_main = if constants.is_row && !tree.needs_measure(child.node) {
                compute_preliminary(tree, child.node, Size::NONE, available_space, RunMode::ComputeSize, false)
                    .width
                    .maybe_min(child.size.width)
                    .maybe_max(child.min_size.width)
                    .into()
            } else {
                child.min_size.main(constants.dir)
            };

            let max_main = child.max_size.main(constants.dir);
            let clamped = child.target_size.main(constants.dir).maybe_min(max_main).maybe_max(min_main).max(0.0);
            child.violation = clamped - child.target_size.main(constants.dir);
            child.target_size.set_main(constants.dir, clamped);
            child.outer_target_size.set_main(
                constants.dir,
                child.target_size.main(constants.dir) + child.margin.main_axis_sum(constants.dir),
            );

            acc + child.violation
        });

        // e. Freeze over-flexed items. The total violation is the sum of the adjustments
        //    from the previous step ∑(clamped size - unclamped size). If the total violation is:
        //    - Zero
        //        Freeze all items.
        //    - Positive
        //        Freeze all the items with min violations.
        //    - Negative
        //        Freeze all the items with max violations.

        for child in &mut unfrozen {
            match total_violation {
                v if v > 0.0 => child.frozen = child.violation > 0.0,
                v if v < 0.0 => child.frozen = child.violation < 0.0,
                _ => child.frozen = true,
            }
        }

        // f. Return to the start of this loop.
    }
}

/// Determine the hypothetical cross size of each item.
///
/// # [9.4. Cross Size Determination](https://www.w3.org/TR/css-flexbox-1/#cross-sizing)
///
/// - [**Determine the hypothetical cross size of each item**](https://www.w3.org/TR/css-flexbox-1/#algo-cross-item)
///     by performing layout with the used main size and the available space, treating auto as fit-content.
#[inline]
fn determine_hypothetical_cross_size(
    tree: &mut impl LayoutTree,
    line: &mut FlexLine,
    constants: &AlgoConstants,
    available_space: Size<Option<f32>>,
) {
    for child in line.items.iter_mut() {
        let child_cross = child
            .size
            .cross(constants.dir)
            .maybe_max(child.min_size.cross(constants.dir))
            .maybe_min(child.max_size.cross(constants.dir));

        child.hypothetical_inner_size.set_cross(
            constants.dir,
            compute_preliminary(
                tree,
                child.node,
                Size {
                    width: if constants.is_row { child.target_size.width.into() } else { child_cross },
                    height: if constants.is_row { child_cross } else { child.target_size.height.into() },
                },
                Size {
                    width: if constants.is_row {
                        constants.container_size.main(constants.dir).into()
                    } else {
                        available_space.width
                    },
                    height: if constants.is_row {
                        available_space.height
                    } else {
                        constants.container_size.main(constants.dir).into()
                    },
                },
                RunMode::ComputeSize,
                false,
            )
            .cross(constants.dir)
            .maybe_max(child.min_size.cross(constants.dir))
            .maybe_min(child.max_size.cross(constants.dir)),
        );

        child.hypothetical_outer_size.set_cross(
            constants.dir,
            child.hypothetical_inner_size.cross(constants.dir) + child.margin.cross_axis_sum(constants.dir),
        );
    }
}

/// Calculate the base lines of the children.
#[inline]
fn calculate_children_base_lines(
    tree: &mut impl LayoutTree,
    node: Node,
    node_size: Size<Option<f32>>,
    flex_lines: &mut [FlexLine],
    constants: &AlgoConstants,
) {
    /// Recursively calculates the baseline for children
    fn calc_baseline(db: &impl LayoutTree, node: Node, layout: &Layout) -> f32 {
        let children = db.children(node);
        if children.is_empty() {
            layout.size.height
        } else {
            let child = children[0];
            let layout = db.layout(child);
            calc_baseline(db, child, layout)
        }
        // if db.children[node].is_empty() {
        //     layout.size.height
        // } else {
        //     let child = db.children[node][0];
        //     calc_baseline(db, child, &db.nodes[child].layout)
        // }
    }

    for line in flex_lines {
        for child in line.items.iter_mut() {
            let preliminary_size = compute_preliminary(
                tree,
                child.node,
                Size {
                    width: if constants.is_row {
                        child.target_size.width.into()
                    } else {
                        child.hypothetical_inner_size.width.into()
                    },
                    height: if constants.is_row {
                        child.hypothetical_inner_size.height.into()
                    } else {
                        child.target_size.height.into()
                    },
                },
                Size {
                    width: if constants.is_row { constants.container_size.width.into() } else { node_size.width },
                    height: if constants.is_row { node_size.height } else { constants.container_size.height.into() },
                },
                RunMode::PeformLayout,
                false,
            );

            child.baseline = calc_baseline(
                tree,
                child.node,
                &Layout {
                    order: tree.children(node).iter().position(|n| *n == child.node).unwrap() as u32,
                    size: preliminary_size,
                    location: Point::ZERO,
                },
            );
        }
    }
}

/// Calculate the cross size of each flex line.
///
/// # [9.4. Cross Size Determination](https://www.w3.org/TR/css-flexbox-1/#cross-sizing)
///
/// - [**Calculate the cross size of each flex line**](https://www.w3.org/TR/css-flexbox-1/#algo-cross-line).
///
///     If the flex container is single-line and has a definite cross size, the cross size of the flex line is the flex container’s inner cross size.
///
///     Otherwise, for each flex line:
///
///     1. Collect all the flex items whose inline-axis is parallel to the main-axis, whose align-self is baseline, and whose cross-axis margins are both non-auto.
///         Find the largest of the distances between each item’s baseline and its hypothetical outer cross-start edge,
///         and the largest of the distances between each item’s baseline and its hypothetical outer cross-end edge, and sum these two values.
///
///     2. Among all the items not collected by the previous step, find the largest outer hypothetical cross size.
///
///     3. The used cross-size of the flex line is the largest of the numbers found in the previous two steps and zero.
///
///         If the flex container is single-line, then clamp the line’s cross-size to be within the container’s computed min and max cross sizes.
///         **Note that if CSS 2.1’s definition of min/max-width/height applied more generally, this behavior would fall out automatically**.
#[inline]
fn calculate_cross_size(
    tree: &mut impl LayoutTree,
    flex_lines: &mut [FlexLine],
    node: Node,
    node_size: Size<Option<f32>>,
    constants: &AlgoConstants,
) {
    if flex_lines.len() == 1 && node_size.cross(constants.dir).is_some() {
        flex_lines[0].cross_size =
            (node_size.cross(constants.dir).maybe_sub(constants.padding_border.cross_axis_sum(constants.dir)))
                .unwrap_or(0.0);
    } else {
        for line in flex_lines.iter_mut() {
            //    1. Collect all the flex items whose inline-axis is parallel to the main-axis, whose
            //       align-self is baseline, and whose cross-axis margins are both non-auto. Find the
            //       largest of the distances between each item’s baseline and its hypothetical outer
            //       cross-start edge, and the largest of the distances between each item’s baseline
            //       and its hypothetical outer cross-end edge, and sum these two values.

            //    2. Among all the items not collected by the previous step, find the largest
            //       outer hypothetical cross size.

            //    3. The used cross-size of the flex line is the largest of the numbers found in the
            //       previous two steps and zero.

            let max_baseline: f32 = line.items.iter().map(|child| child.baseline).fold(0.0, |acc, x| acc.max(x));
            line.cross_size = line
                .items
                .iter()
                .map(|child| {
                    let child_style = tree.style(child.node);
                    if child_style.align_self(tree.style(node)) == AlignSelf::Baseline
                        && child_style.cross_margin_start(constants.dir) != Dimension::Auto
                        && child_style.cross_margin_end(constants.dir) != Dimension::Auto
                        && child_style.cross_size(constants.dir) == Dimension::Auto
                    {
                        max_baseline - child.baseline + child.hypothetical_outer_size.cross(constants.dir)
                    } else {
                        child.hypothetical_outer_size.cross(constants.dir)
                    }
                })
                .fold(0.0, |acc, x| acc.max(x));
        }
    }
}

/// Handle 'align-content: stretch'.
///
/// # [9.4. Cross Size Determination](https://www.w3.org/TR/css-flexbox-1/#cross-sizing)
///
/// - [**Handle 'align-content: stretch'**](https://www.w3.org/TR/css-flexbox-1/#algo-line-stretch). If the flex container has a definite cross size, align-content is stretch,
///     and the sum of the flex lines' cross sizes is less than the flex container’s inner cross size,
///     increase the cross size of each flex line by equal amounts such that the sum of their cross sizes exactly equals the flex container’s inner cross size.
#[inline]
fn handle_align_content_stretch(
    tree: &mut impl LayoutTree,
    flex_lines: &mut [FlexLine],
    node: Node,
    node_size: Size<Option<f32>>,
    constants: &AlgoConstants,
) {
    if tree.style(node).align_content == AlignContent::Stretch && node_size.cross(constants.dir).is_some() {
        let total_cross: f32 = flex_lines.iter().map(|line| line.cross_size).sum();
        let inner_cross =
            (node_size.cross(constants.dir).maybe_sub(constants.padding_border.cross_axis_sum(constants.dir)))
                .unwrap_or(0.0);

        if total_cross < inner_cross {
            let remaining = inner_cross - total_cross;
            let addition = remaining / flex_lines.len() as f32;
            flex_lines.iter_mut().for_each(|line| line.cross_size += addition);
        }
    }
}

/// Determine the used cross size of each flex item.
///
/// # [9.4. Cross Size Determination](https://www.w3.org/TR/css-flexbox-1/#cross-sizing)
///
/// - [**Determine the used cross size of each flex item**](https://www.w3.org/TR/css-flexbox-1/#algo-stretch). If a flex item has align-self: stretch, its computed cross size property is auto,
///     and neither of its cross-axis margins are auto, the used outer cross size is the used cross size of its flex line, clamped according to the item’s used min and max cross sizes.
///     Otherwise, the used cross size is the item’s hypothetical cross size.
///
///     If the flex item has align-self: stretch, redo layout for its contents, treating this used size as its definite cross size so that percentage-sized children can be resolved.
///
///     **Note that this step does not affect the main size of the flex item, even if it has an intrinsic aspect ratio**.
#[inline]
fn determine_used_cross_size(
    tree: &mut impl LayoutTree,
    flex_lines: &mut [FlexLine],
    node: Node,
    constants: &AlgoConstants,
) {
    for line in flex_lines {
        let line_cross_size = line.cross_size;

        for child in line.items.iter_mut() {
            let child_style = tree.style(child.node);
            child.target_size.set_cross(
                constants.dir,
                if child_style.align_self(tree.style(node)) == AlignSelf::Stretch
                    && child_style.cross_margin_start(constants.dir) != Dimension::Auto
                    && child_style.cross_margin_end(constants.dir) != Dimension::Auto
                    && child_style.cross_size(constants.dir) == Dimension::Auto
                {
                    (line_cross_size - child.margin.cross_axis_sum(constants.dir))
                        .maybe_max(child.min_size.cross(constants.dir))
                        .maybe_min(child.max_size.cross(constants.dir))
                } else {
                    child.hypothetical_inner_size.cross(constants.dir)
                },
            );

            child.outer_target_size.set_cross(
                constants.dir,
                child.target_size.cross(constants.dir) + child.margin.cross_axis_sum(constants.dir),
            );
        }
    }
}

/// Distribute any remaining free space.
///
/// # [9.5. Main-Axis Alignment](https://www.w3.org/TR/css-flexbox-1/#main-alignment)
///
/// - [**Distribute any remaining free space**](https://www.w3.org/TR/css-flexbox-1/#algo-main-align). For each flex line:
///
///     1. If the remaining free space is positive and at least one main-axis margin on this line is `auto`, distribute the free space equally among these margins.
///         Otherwise, set all `auto` margins to zero.
///
///     2. Align the items along the main-axis per `justify-content`.
#[inline]
fn distribute_remaining_free_space(
    tree: &mut impl LayoutTree,
    flex_lines: &mut [FlexLine],
    node: Node,
    constants: &AlgoConstants,
) {
    for line in flex_lines {
        let used_space: f32 = line.items.iter().map(|child| child.outer_target_size.main(constants.dir)).sum();
        let free_space = constants.inner_container_size.main(constants.dir) - used_space;
        let mut num_auto_margins = 0;

        for child in line.items.iter_mut() {
            let child_style = tree.style(child.node);
            if child_style.main_margin_start(constants.dir) == Dimension::Auto {
                num_auto_margins += 1;
            }
            if child_style.main_margin_end(constants.dir) == Dimension::Auto {
                num_auto_margins += 1;
            }
        }

        if free_space > 0.0 && num_auto_margins > 0 {
            let margin = free_space / num_auto_margins as f32;

            for child in line.items.iter_mut() {
                let child_style = tree.style(child.node);
                if child_style.main_margin_start(constants.dir) == Dimension::Auto {
                    if constants.is_row {
                        child.margin.start = margin;
                    } else {
                        child.margin.top = margin;
                    }
                }
                if child_style.main_margin_end(constants.dir) == Dimension::Auto {
                    if constants.is_row {
                        child.margin.end = margin;
                    } else {
                        child.margin.bottom = margin;
                    }
                }
            }
        } else {
            let num_items = line.items.len();
            let layout_reverse = constants.dir.is_reverse();

            let justify_item = |(i, child): (usize, &mut FlexItem)| {
                let is_first = i == 0;

                child.offset_main = match tree.style(node).justify_content {
                    JustifyContent::FlexStart => {
                        if layout_reverse && is_first {
                            free_space
                        } else {
                            0.0
                        }
                    }
                    JustifyContent::Center => {
                        if is_first {
                            free_space / 2.0
                        } else {
                            0.0
                        }
                    }
                    JustifyContent::FlexEnd => {
                        if is_first && !layout_reverse {
                            free_space
                        } else {
                            0.0
                        }
                    }
                    JustifyContent::SpaceBetween => {
                        if is_first {
                            0.0
                        } else {
                            free_space / (num_items - 1) as f32
                        }
                    }
                    JustifyContent::SpaceAround => {
                        if is_first {
                            (free_space / num_items as f32) / 2.0
                        } else {
                            free_space / num_items as f32
                        }
                    }
                    JustifyContent::SpaceEvenly => free_space / (num_items + 1) as f32,
                };
            };

            if layout_reverse {
                line.items.iter_mut().rev().enumerate().for_each(justify_item);
            } else {
                line.items.iter_mut().enumerate().for_each(justify_item);
            }
        }
    }
}

/// Resolve cross-axis `auto` margins.
///
/// # [9.6. Cross-Axis Alignment](https://www.w3.org/TR/css-flexbox-1/#cross-alignment)
///
/// - [**Resolve cross-axis `auto` margins**](https://www.w3.org/TR/css-flexbox-1/#algo-cross-margins).
///     If a flex item has auto cross-axis margins:
///
///     - If its outer cross size (treating those auto margins as zero) is less than the cross size of its flex line,
///         distribute the difference in those sizes equally to the auto margins.
///
///     - Otherwise, if the block-start or inline-start margin (whichever is in the cross axis) is auto, set it to zero.
///         Set the opposite margin so that the outer cross size of the item equals the cross size of its flex line.
#[inline]
fn resolve_cross_axis_auto_margins(
    tree: &mut impl LayoutTree,
    flex_lines: &mut [FlexLine],
    node: Node,
    constants: &AlgoConstants,
) {
    for line in flex_lines {
        let line_cross_size = line.cross_size;
        let max_baseline: f32 = line.items.iter_mut().map(|child| child.baseline).fold(0.0, |acc, x| acc.max(x));

        for child in line.items.iter_mut() {
            let free_space = line_cross_size - child.outer_target_size.cross(constants.dir);
            let child_style = tree.style(child.node);

            if child_style.cross_margin_start(constants.dir) == Dimension::Auto
                && child_style.cross_margin_end(constants.dir) == Dimension::Auto
            {
                if constants.is_row {
                    child.margin.top = free_space / 2.0;
                    child.margin.bottom = free_space / 2.0;
                } else {
                    child.margin.start = free_space / 2.0;
                    child.margin.end = free_space / 2.0;
                }
            } else if child_style.cross_margin_start(constants.dir) == Dimension::Auto {
                if constants.is_row {
                    child.margin.top = free_space;
                } else {
                    child.margin.start = free_space;
                }
            } else if child_style.cross_margin_end(constants.dir) == Dimension::Auto {
                if constants.is_row {
                    child.margin.bottom = free_space;
                } else {
                    child.margin.end = free_space;
                }
            } else {
                // 14. Align all flex items along the cross-axis.
                child.offset_cross = align_flex_items_along_cross_axis(
                    tree,
                    node,
                    child,
                    child_style,
                    free_space,
                    max_baseline,
                    constants,
                );
            }
        }
    }
}

/// Align all flex items along the cross-axis.
///
/// # [9.6. Cross-Axis Alignment](https://www.w3.org/TR/css-flexbox-1/#cross-alignment)
///
/// - [**Align all flex items along the cross-axis**](https://www.w3.org/TR/css-flexbox-1/#algo-cross-align) per `align-self`,
///     if neither of the item's cross-axis margins are `auto`.
#[inline]
fn align_flex_items_along_cross_axis(
    tree: &impl LayoutTree,
    node: Node,
    child: &mut FlexItem,
    child_style: &FlexboxLayout,
    free_space: f32,
    max_baseline: f32,
    constants: &AlgoConstants,
) -> f32 {
    match child_style.align_self(tree.style(node)) {
        AlignSelf::Auto => unreachable!(),
        AlignSelf::FlexStart => {
            if constants.is_wrap_reverse {
                free_space
            } else {
                0.0
            }
        }
        AlignSelf::FlexEnd => {
            if constants.is_wrap_reverse {
                0.0
            } else {
                free_space
            }
        }
        AlignSelf::Center => free_space / 2.0,
        AlignSelf::Baseline => {
            if constants.is_row {
                max_baseline - child.baseline
            } else {
                // baseline alignment only makes sense if the constants.direction is row
                // we treat it as flex-start alignment in columns.
                if constants.is_wrap_reverse {
                    free_space
                } else {
                    0.0
                }
            }
        }
        AlignSelf::Stretch => {
            if constants.is_wrap_reverse {
                free_space
            } else {
                0.0
            }
        }
    }
}

/// Determine the flex container’s used cross size.
///
/// # [9.6. Cross-Axis Alignment](https://www.w3.org/TR/css-flexbox-1/#cross-alignment)
///
/// - [**Determine the flex container’s used cross size**](https://www.w3.org/TR/css-flexbox-1/#algo-cross-container):
///
///     - If the cross size property is a definite size, use that, clamped by the used min and max cross sizes of the flex container.
///
///     - Otherwise, use the sum of the flex lines' cross sizes, clamped by the used min and max cross sizes of the flex container.
#[inline]
#[must_use]
fn determine_container_cross_size(
    flex_lines: &mut [FlexLine],
    node_size: Size<Option<f32>>,
    constants: &mut AlgoConstants,
) -> f32 {
    let total_cross_size: f32 = flex_lines.iter().map(|line| line.cross_size).sum();

    constants.container_size.set_cross(
        constants.dir,
        node_size
            .cross(constants.dir)
            .unwrap_or(total_cross_size + constants.padding_border.cross_axis_sum(constants.dir)),
    );

    constants.inner_container_size.set_cross(
        constants.dir,
        constants.container_size.cross(constants.dir) - constants.padding_border.cross_axis_sum(constants.dir),
    );

    total_cross_size
}

/// Align all flex lines per `align-content`.
///
/// # [9.6. Cross-Axis Alignment](https://www.w3.org/TR/css-flexbox-1/#cross-alignment)
///
/// - [**Align all flex lines**](https://www.w3.org/TR/css-flexbox-1/#algo-line-align) per `align-content`.
#[inline]
fn align_flex_lines_per_align_content(
    tree: &impl LayoutTree,
    flex_lines: &mut [FlexLine],
    node: Node,
    constants: &AlgoConstants,
    total_cross_size: f32,
) {
    let free_space = constants.inner_container_size.cross(constants.dir) - total_cross_size;
    let num_lines = flex_lines.len();

    let align_line = |(i, line): (usize, &mut FlexLine)| {
        let is_first = i == 0;

        line.offset_cross = match tree.style(node).align_content {
            AlignContent::FlexStart => {
                if is_first && constants.is_wrap_reverse {
                    free_space
                } else {
                    0.0
                }
            }
            AlignContent::FlexEnd => {
                if is_first && !constants.is_wrap_reverse {
                    free_space
                } else {
                    0.0
                }
            }
            AlignContent::Center => {
                if is_first {
                    free_space / 2.0
                } else {
                    0.0
                }
            }
            AlignContent::Stretch => 0.0,
            AlignContent::SpaceEvenly => free_space / (num_lines + 1) as f32,
            AlignContent::SpaceBetween => {
                if is_first {
                    0.0
                } else {
                    free_space / (num_lines - 1) as f32
                }
            }
            AlignContent::SpaceAround => {
                if is_first {
                    (free_space / num_lines as f32) / 2.0
                } else {
                    free_space / num_lines as f32
                }
            }
        };
    };

    if constants.is_wrap_reverse {
        flex_lines.iter_mut().rev().enumerate().for_each(align_line);
    } else {
        flex_lines.iter_mut().enumerate().for_each(align_line);
    }
}

/// Calculates the layout for a flex-item
#[allow(clippy::too_many_arguments)]
fn calculate_flex_item(
    tree: &mut impl LayoutTree,
    node: Node,
    item: &mut FlexItem,
    total_offset_main: &mut f32,
    total_offset_cross: f32,
    line_offset_cross: f32,
    container_size: Size<f32>,
    direction: FlexDirection,
) {
    let preliminary_size = compute_preliminary(
        tree,
        item.node,
        item.target_size.map(|s| s.into()),
        container_size.map(|s| s.into()),
        RunMode::PeformLayout,
        false,
    );

    let offset_main = *total_offset_main
        + item.offset_main
        + item.margin.main_start(direction)
        + (item.position.main_start(direction).unwrap_or(0.0) - item.position.main_end(direction).unwrap_or(0.0));

    let offset_cross = total_offset_cross
        + item.offset_cross
        + line_offset_cross
        + item.margin.cross_start(direction)
        + (item.position.cross_start(direction).unwrap_or(0.0) - item.position.cross_end(direction).unwrap_or(0.0));

    *tree.layout_mut(item.node) = Layout {
        order: tree.children(node).iter().position(|n| *n == item.node).unwrap() as u32,
        size: preliminary_size,
        location: Point {
            x: if direction.is_row() { offset_main } else { offset_cross },
            y: if direction.is_column() { offset_main } else { offset_cross },
        },
    };

    *total_offset_main += item.offset_main + item.margin.main_axis_sum(direction) + preliminary_size.main(direction);
}

/// Calculates the layout line
fn calculate_layout_line(
    tree: &mut impl LayoutTree,
    node: Node,
    line: &mut FlexLine,
    total_offset_cross: &mut f32,
    container_size: Size<f32>,
    padding_border: Rect<f32>,
    direction: FlexDirection,
) {
    let mut total_offset_main = padding_border.main_start(direction);
    let line_offset_cross = line.offset_cross;

    if direction.is_reverse() {
        for item in line.items.iter_mut().rev() {
            calculate_flex_item(
                tree,
                node,
                item,
                &mut total_offset_main,
                *total_offset_cross,
                line_offset_cross,
                container_size,
                direction,
            );
        }
    } else {
        for item in line.items.iter_mut() {
            calculate_flex_item(
                tree,
                node,
                item,
                &mut total_offset_main,
                *total_offset_cross,
                line_offset_cross,
                container_size,
                direction,
            );
        }
    }

    *total_offset_cross += line_offset_cross + line.cross_size;
}

/// Do a final layout pass and collect the resulting layouts.
#[inline]
fn final_layout_pass(tree: &mut impl LayoutTree, node: Node, flex_lines: &mut [FlexLine], constants: &AlgoConstants) {
    let mut total_offset_cross = constants.padding_border.cross_start(constants.dir);

    if constants.is_wrap_reverse {
        for line in flex_lines.iter_mut().rev() {
            calculate_layout_line(
                tree,
                node,
                line,
                &mut total_offset_cross,
                constants.container_size,
                constants.padding_border,
                constants.dir,
            );
        }
    } else {
        for line in flex_lines.iter_mut() {
            calculate_layout_line(
                tree,
                node,
                line,
                &mut total_offset_cross,
                constants.container_size,
                constants.padding_border,
                constants.dir,
            );
        }
    }
}

/// Perform absolute layout on all absolutely positioned children.
#[inline]
fn perform_absolute_layout_on_absolute_children(tree: &mut impl LayoutTree, node: Node, constants: &AlgoConstants) {
    // TODO: remove number of Vec<_> generated
    let candidates = tree
        .children(node)
        .iter()
        .cloned()
        .enumerate()
        .filter(|(_, child)| tree.style(*child).position_type == PositionType::Absolute)
        .collect::<Vec<_>>();

    for (order, child) in candidates {
        let container_width = constants.container_size.width.into();
        let container_height = constants.container_size.height.into();

        let child_style = tree.style(child);

        // X-axis
        let child_position_start = child_style.position.start.maybe_resolve(container_width);
        let child_margin_start = child_style.margin.start.maybe_resolve(container_width);
        let start = child_position_start.maybe_add(child_margin_start);

        let child_position_end = child_style.position.end.maybe_resolve(container_width);
        let child_margin_end = child_style.margin.end.maybe_resolve(container_width);
        let end = child_position_end.maybe_add(child_margin_end);

        // Y-axis
        let child_position_top = child_style.position.top.maybe_resolve(container_height);
        let child_margin_top = child_style.margin.top.maybe_resolve(container_height);
        let top = child_position_top.maybe_add(child_margin_top);

        let child_position_bottom = child_style.position.bottom.maybe_resolve(container_height);
        let child_margin_bottom = child_style.margin.bottom.maybe_resolve(container_height);
        let bottom = child_position_bottom.maybe_add(child_margin_bottom);

        let (start_main, end_main) = if constants.is_row { (start, end) } else { (top, bottom) };
        let (start_cross, end_cross) = if constants.is_row { (top, bottom) } else { (start, end) };

        let mut width = child_style
            .size
            .width
            .maybe_resolve(container_width)
            .maybe_max(child_style.min_size.width.maybe_resolve(container_width))
            .maybe_min(child_style.max_size.width.maybe_resolve(container_width));

        if width.is_none() && start.is_some() && end.is_some() {
            width = container_width.maybe_sub(start).maybe_sub(end);
        }

        let mut height: Option<f32> = child_style
            .size
            .height
            .maybe_resolve(container_height)
            .maybe_max(child_style.min_size.height.maybe_resolve(container_height))
            .maybe_min(child_style.max_size.height.maybe_resolve(container_height));

        if height.is_none() && top.is_some() && bottom.is_some() {
            height = container_height.maybe_sub(top).maybe_sub(bottom);
        }

        let preliminary_size = compute_preliminary(
            tree,
            child,
            Size { width, height },
            Size { width: container_width, height: container_height },
            RunMode::PeformLayout,
            false,
        );

        // Satisfy the borrow checker by re-requesting the style from above.
        // This shortens the lifetime of the original binding
        let child_style = tree.style(child);

        let free_main_space = constants.container_size.main(constants.dir)
            - preliminary_size
                .main(constants.dir)
                .maybe_max(
                    child_style
                        .min_main_size(constants.dir)
                        .maybe_resolve(constants.node_inner_size.main(constants.dir)),
                )
                .maybe_min(
                    child_style
                        .max_main_size(constants.dir)
                        .maybe_resolve(constants.node_inner_size.main(constants.dir)),
                );

        let free_cross_space = constants.container_size.cross(constants.dir)
            - preliminary_size
                .cross(constants.dir)
                .maybe_max(
                    child_style
                        .min_cross_size(constants.dir)
                        .maybe_resolve(constants.node_inner_size.cross(constants.dir)),
                )
                .maybe_min(
                    child_style
                        .max_cross_size(constants.dir)
                        .maybe_resolve(constants.node_inner_size.cross(constants.dir)),
                );

        let offset_main = if start_main.is_some() {
            start_main.unwrap_or(0.0) + constants.border.main_start(constants.dir)
        } else if end_main.is_some() {
            free_main_space - end_main.unwrap_or(0.0) - constants.border.main_end(constants.dir)
        } else {
            match tree.style(node).justify_content {
                JustifyContent::SpaceBetween | JustifyContent::FlexStart => {
                    constants.padding_border.main_start(constants.dir)
                }
                JustifyContent::FlexEnd => free_main_space - constants.padding_border.main_end(constants.dir),
                JustifyContent::SpaceEvenly | JustifyContent::SpaceAround | JustifyContent::Center => {
                    free_main_space / 2.0
                }
            }
        };

        let offset_cross = if start_cross.is_some() {
            start_cross.unwrap_or(0.0) + constants.border.cross_start(constants.dir)
        } else if end_cross.is_some() {
            free_cross_space - end_cross.unwrap_or(0.0) - constants.border.cross_end(constants.dir)
        } else {
            match child_style.align_self(tree.style(node)) {
                AlignSelf::Auto => unreachable!(),
                AlignSelf::FlexStart => {
                    if constants.is_wrap_reverse {
                        free_cross_space - constants.padding_border.cross_end(constants.dir)
                    } else {
                        constants.padding_border.cross_start(constants.dir)
                    }
                }
                AlignSelf::FlexEnd => {
                    if constants.is_wrap_reverse {
                        constants.padding_border.cross_start(constants.dir)
                    } else {
                        free_cross_space - constants.padding_border.cross_end(constants.dir)
                    }
                }
                AlignSelf::Center => free_cross_space / 2.0,
                AlignSelf::Baseline => free_cross_space / 2.0, // Treat as center for now until we have baseline support
                AlignSelf::Stretch => {
                    if constants.is_wrap_reverse {
                        free_cross_space - constants.padding_border.cross_end(constants.dir)
                    } else {
                        constants.padding_border.cross_start(constants.dir)
                    }
                }
            }
        };

        *tree.layout_mut(child) = Layout {
            order: order as u32,
            size: preliminary_size,
            location: Point {
                x: if constants.is_row { offset_main } else { offset_cross },
                y: if constants.is_column { offset_main } else { offset_cross },
            },
        };
    }
}

/// Creates a layout for this node and its children, recursively.
/// Each hidden node has zero size and is placed at the origin
fn hidden_layout(tree: &mut impl LayoutTree, node: Node, order: u32) {
    *tree.layout_mut(node) = Layout::with_order(order);

    let len = tree.children(node).len();
    for order in 0..len {
        hidden_layout(tree, tree.child(node, order), order as _);
    }
}

#[cfg(test)]
mod tests {
    use crate::geometry::Point;
    use crate::style::Display;
    use crate::style::Display::Flex;
    use crate::{
        math::MaybeMath,
        prelude::{Rect, Size},
        resolve::ResolveOrDefault,
        style::{FlexWrap, FlexboxLayout},
        Taffy,
    };
    use super::hidden_layout;

    // Make sure we get correct constants
    #[test]
    fn correct_constants() {
        let mut tree = Taffy::with_capacity(16);

        let style = FlexboxLayout::default();
        let node_id = tree.new_leaf(style).unwrap();

        let node_size = Size::NONE;
        let parent_size = Size::NONE;

        let constants = super::compute_constants(tree.style(node_id).unwrap(), node_size, parent_size);
        // let constants = super::compute_constants(&tree.nodes[node_id], node_size, parent_size);

        assert!(constants.dir == style.flex_direction);
        assert!(constants.is_row == style.flex_direction.is_row());
        assert!(constants.is_column == style.flex_direction.is_column());
        assert!(constants.is_wrap_reverse == (style.flex_wrap == FlexWrap::WrapReverse));

        let margin = style.margin.resolve_or_default(parent_size);
        assert_eq!(constants.margin, margin);

        let border = style.border.resolve_or_default(parent_size);
        assert_eq!(constants.border, border);

        let padding = style.padding.resolve_or_default(parent_size);

        // TODO: Replace with something less hardcoded?
        let padding_border = Rect {
            start: padding.start + border.start,
            end: padding.end + border.end,
            top: padding.top + border.top,
            bottom: padding.bottom + border.bottom,
        };

        assert_eq!(constants.padding_border, padding_border);

        // TODO: Replace with something less hardcoded?
        let inner_size = Size {
            width: node_size.width.maybe_sub(padding_border.horizontal_axis_sum()),
            height: node_size.height.maybe_sub(padding_border.vertical_axis_sum()),
        };
        assert_eq!(constants.node_inner_size, inner_size);

        assert_eq!(constants.container_size, Size::ZERO);
        assert_eq!(constants.inner_container_size, Size::ZERO);
    }

    #[test]
    fn hidden_layout_should_hide_recursively() {
        let mut taffy = Taffy::new();

        let style: FlexboxLayout =
            FlexboxLayout { display: Flex, size: Size::from_points(50.0, 50.0), ..Default::default() };

        let grandchild_00 = taffy.new_leaf(style).unwrap();

        let grandchild_01 = taffy.new_leaf(style).unwrap();

        let grandchild_02 = taffy.new_leaf(style).unwrap();

        let child_00 = taffy.new_with_children(style, &[grandchild_00, grandchild_01]).unwrap();

        let child_01 = taffy.new_with_children(style, &[grandchild_02]).unwrap();

        let root = taffy
            .new_with_children(
                FlexboxLayout { display: Display::None, size: Size::from_points(50.0, 50.0), ..Default::default() },
                &[child_00, child_01],
            )
            .unwrap();

        hidden_layout(&mut taffy, root, 0);

        // Whatever size and display-mode the nodes had previously,
        // all layouts should resolve to ZERO due to the root's DISPLAY::NONE
        for (node, _) in &taffy.nodes {
            if let Ok(layout) = taffy.layout(node) {
                assert_eq!(layout.size, Size::ZERO);
                assert_eq!(layout.location, Point::ZERO);
            }
        }
    }
}
