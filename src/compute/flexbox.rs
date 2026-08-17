//! Computes the [flexbox](https://css-tricks.com/snippets/css/a-guide-to-flexbox/) layout algorithm on [`TaffyTree`](crate::TaffyTree) according to the [spec](https://www.w3.org/TR/css-flexbox-1/)
use crate::compute::common::alignment::{compute_alignment_offset, resolve_self_alignment_safety};
use crate::geometry::{Line, Point, Rect, Size};
use crate::style::{
    AlignContent, AlignContentKeyword, AlignItems, AlignItemsKeyword, AlignSelf, AvailableSpace, JustifyContent,
    LengthPercentageAuto, Overflow, Position,
};
use crate::style::{CoreStyle, FlexDirection, FlexboxContainerStyle, FlexboxItemStyle};
use crate::style_helpers::{TaffyMaxContent, TaffyMinContent};
use crate::tree::{Baselines, Layout, LayoutInput, LayoutOutput, RunMode, SizingMode};
use crate::tree::{LayoutFlexboxContainer, LayoutPartialTreeExt, NodeId};
use crate::util::debug::debug_log;
use crate::util::sys::{f32_max, f32_min, new_vec_with_capacity, Vec};
use crate::util::MaybeMath;
use crate::util::{MaybeResolve, ResolveOrZero};
use crate::{BoxGenerationMode, BoxSizing, Dimension, Direction, RequestedAxis};

use super::common::alignment::apply_alignment_fallback;
#[cfg(feature = "content_size")]
use super::common::content_size::compute_content_size_contribution;
use super::common::sizing_keyword::{
    resolve_absolute_sizing_keywords, resolve_sizing_keyword, SizingKeywordResolution,
};

/// The intermediate results of a flexbox calculation for a single item
struct FlexItem {
    /// The identifier for the associated node
    node: NodeId,

    /// The order of the node relative to it's siblings
    order: u32,

    /// The base size of this item
    size: Size<Option<f32>>,
    /// The raw size style of this item. Used to detect and resolve sizing
    /// keywords (`min-content`, `max-content`, `fit-content`, `fit-content(...)`, and `stretch`)
    size_style: Size<Dimension>,
    /// The minimum allowable size of this item
    min_size: Size<Option<f32>>,
    /// The maximum allowable size of this item
    max_size: Size<Option<f32>>,
    /// The aspect ratio of this item
    aspect_ratio: Option<f32>,
    /// The cross-alignment of this item
    align_self: AlignSelf,

    /// The overflow style of the item
    overflow: Point<Overflow>,
    /// The width of the scrollbars (if it has any)
    scrollbar_width: f32,
    /// The flex shrink style of the item
    flex_shrink: f32,
    /// The flex grow style of the item
    flex_grow: f32,
    /// Whether the item's used flex basis is definite (rather than derived from the item's content)
    flex_basis_is_definite: bool,

    /// The minimum size of the item. This differs from min_size above because it also
    /// takes into account content based automatic minimum sizes
    resolved_minimum_main_size: f32,

    /// The final offset of this item
    inset: Rect<Option<f32>>,
    /// The margin of this item
    margin: Rect<f32>,
    /// Whether each margin is an auto margin or not
    margin_is_auto: Rect<bool>,
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

    /// Either the max- or min- content flex fraction
    /// See https://www.w3.org/TR/css-flexbox-1/#intrinsic-main-sizes
    content_flex_fraction: f32,

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

impl FlexItem {
    /// Returns true if the item is a <https://www.w3.org/TR/css-overflow-3/#scroll-container>
    fn is_scroll_container(&self) -> bool {
        self.overflow.x.is_scroll_container() | self.overflow.y.is_scroll_container()
    }

    /// Returns true if the item participates in baseline alignment: it has `align-self: baseline`
    /// and neither of its cross-axis margins are `auto`.
    /// See <https://www.w3.org/TR/css-flexbox-1/#baseline-participation>
    fn participates_in_baseline_alignment(&self, dir: FlexDirection) -> bool {
        self.align_self == AlignSelf::BASELINE
            && !self.margin_is_auto.cross_start(dir)
            && !self.margin_is_auto.cross_end(dir)
    }
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
    /// The layout direction of the current segment being laid out
    layout_direction: Direction,
    /// Is this segment a row
    is_row: bool,
    /// Is this segment a column
    is_column: bool,
    /// Is wrapping enabled (in either direction)
    is_wrap: bool,
    /// Is the wrap direction inverted
    is_wrap_reverse: bool,
    /// Are items balanced across lines (`flex-wrap: balance`)?
    #[cfg(feature = "flexbox_balance")]
    is_balance: bool,
    /// The requested minimum number of lines (`flex-line-count`). `Some` for every
    /// multi-line container, `None` for `nowrap`.
    #[cfg(feature = "flexbox_balance")]
    line_count: Option<u16>,

    /// The item's min_size style
    min_size: Size<Option<f32>>,
    /// The item's max_size style
    max_size: Size<Option<f32>>,
    /// The margin of this section
    margin: Rect<f32>,
    /// The border of this section
    border: Rect<f32>,
    /// The space between the content box and the border box.
    /// This consists of padding + border + scrollbar_gutter.
    content_box_inset: Rect<f32>,
    /// The size reserved for scrollbar gutters in each axis
    scrollbar_gutter: Point<f32>,
    /// Whether the node being laid out is a scroll container
    #[cfg(feature = "content_size")]
    is_scroll_container: bool,
    /// The gap of this section
    gap: Size<f32>,
    /// The align_items property of this node
    align_items: AlignItems,
    /// The align_content property of this node
    align_content: AlignContent,
    /// The justify_content property of this node
    justify_content: Option<JustifyContent>,

    /// The border-box size of the node being laid out (if known)
    node_outer_size: Size<Option<f32>>,
    /// The content-box size of the node being laid out (if known)
    node_inner_size: Size<Option<f32>>,
    /// Whether the known main size of the node (if any) is definite. This is `false` when a parent
    /// imposes a main size on this node that is derived from the node's own content, in which case
    /// it is indefinite for the purposes of resolving percentage sizes of items and collecting items
    /// into flex lines. See <https://www.w3.org/TR/css-flexbox-1/#definite-sizes>.
    known_main_size_is_definite: bool,
    /// Whether the node has a known main size which is definite (as of the start of layout,
    /// before the main size is determined from the node's contents)
    has_definite_main_size: bool,
    /// Whether the node has a known cross size which is definite
    has_definite_cross_size: bool,

    /// The size of the virtual container containing the flex items.
    container_size: Size<f32>,
    /// The size of the internal container
    inner_container_size: Size<f32>,
}

impl AlgoConstants {
    /// When a multi-line container requests a minimum number of lines (`flex-line-count`),
    /// definite cross-axis available space for measuring items is divided between the requested
    /// number of lines (after subtracting the cross-axis gaps between them).
    /// See <https://github.com/w3c/csswg-drafts/issues/13414>
    #[inline]
    fn divided_cross_space(&self, cross_available_space: f32) -> f32 {
        #[cfg(feature = "flexbox_balance")]
        if let Some(line_count) = self.line_count {
            if line_count > 1 {
                let line_count = line_count as f32;
                return (cross_available_space - (line_count - 1.0) * self.gap.cross(self.dir)) / line_count;
            }
        }
        cross_available_space
    }
}

/// Computes the layout of a box according to the flexbox algorithm
pub fn compute_flexbox_layout(
    tree: &mut impl LayoutFlexboxContainer,
    node: NodeId,
    inputs: LayoutInput,
) -> LayoutOutput {
    let LayoutInput { known_dimensions, parent_size, run_mode, .. } = inputs;
    let style = tree.get_flexbox_container_style(node);

    // Pull these out earlier to avoid borrowing issues
    let aspect_ratio = style.aspect_ratio();
    let padding = style.padding().resolve_or_zero(parent_size.width, |val, basis| tree.calc(val, basis));
    let border = style.border().resolve_or_zero(parent_size.width, |val, basis| tree.calc(val, basis));
    let padding_border_sum = padding.sum_axes() + border.sum_axes();
    let box_sizing_adjustment =
        if style.box_sizing() == BoxSizing::ContentBox { padding_border_sum } else { Size::ZERO };

    let min_size = style
        .min_size()
        .maybe_resolve(parent_size, |val, basis| tree.calc(val, basis))
        .maybe_apply_aspect_ratio(aspect_ratio)
        .maybe_add(box_sizing_adjustment);
    let max_size = style
        .max_size()
        .maybe_resolve(parent_size, |val, basis| tree.calc(val, basis))
        .maybe_apply_aspect_ratio(aspect_ratio)
        .maybe_add(box_sizing_adjustment);
    let clamped_style_size = if inputs.sizing_mode == SizingMode::InherentSize {
        style
            .size()
            .maybe_resolve(parent_size, |val, basis| tree.calc(val, basis))
            .maybe_apply_aspect_ratio(aspect_ratio)
            .maybe_add(box_sizing_adjustment)
            .maybe_clamp(min_size, max_size)
    } else {
        Size::NONE
    };

    // If both min and max in a given axis are set and max <= min then this determines the size in that axis
    let min_max_definite_size = min_size.zip_map(max_size, |min, max| match (min, max) {
        (Some(min), Some(max)) if max <= min => Some(min),
        _ => None,
    });

    // The size of the container should be floored by the padding and border
    let styled_based_known_dimensions =
        known_dimensions.or(min_max_definite_size.or(clamped_style_size).maybe_max(padding_border_sum));

    // Short-circuit layout if the container's size is fully determined by the container's size and the run mode
    // is ComputeSize (and thus the container's size is all that we're interested in)
    if run_mode == RunMode::ComputeSize {
        if let Size { width: Some(width), height: Some(height) } = styled_based_known_dimensions {
            return LayoutOutput::from_outer_size(Size { width, height });
        }

        // We can also short-circuit if the width is known and only the width has been requested.
        if inputs.axis == RequestedAxis::Horizontal {
            if let Some(width) = styled_based_known_dimensions.width {
                return LayoutOutput::from_outer_size(Size { width, height: 0.0 });
            }
        }
    }

    // Short-circuit layout if the container's size is fully determined by the container's size and the run mode
    // is ComputeSize (and thus the container's size is all that we're interested in)
    if run_mode == RunMode::ComputeSize {
        if let Size { width: Some(width), height: Some(height) } = styled_based_known_dimensions {
            return LayoutOutput::from_outer_size(Size { width, height });
        }
    }

    debug_log!("FLEX:", dbg:style.flex_direction());
    drop(style);

    // Normalize the definiteness flags: they only apply to dimensions which were passed in as known
    // by the parent. Dimensions resolved from the node's own style are always definite.
    let known_dimensions_are_definite = inputs
        .known_dimensions_are_definite
        .zip_map(known_dimensions, |is_definite, known_dimension| is_definite || known_dimension.is_none());

    compute_preliminary(
        tree,
        node,
        LayoutInput { known_dimensions: styled_based_known_dimensions, known_dimensions_are_definite, ..inputs },
    )
}

/// Compute a preliminary size for an item
fn compute_preliminary(tree: &mut impl LayoutFlexboxContainer, node: NodeId, inputs: LayoutInput) -> LayoutOutput {
    let LayoutInput { known_dimensions, parent_size, available_space, run_mode, .. } = inputs;

    // Define some general constants we will need for the remainder of the algorithm.
    let mut constants = compute_constants(
        tree,
        tree.get_flexbox_container_style(node),
        known_dimensions,
        inputs.known_dimensions_are_definite,
        parent_size,
    );

    // 9. Flex Layout Algorithm

    // 9.1. Initial Setup

    // 1. Generate anonymous flex items as described in §4 Flex Items.
    debug_log!("generate_anonymous_flex_items");
    let mut flex_items = generate_anonymous_flex_items(tree, node, &constants);

    // 9.2. Line Length Determination

    // 2. Determine the available main and cross space for the flex items
    debug_log!("determine_available_space");
    let available_space = determine_available_space(known_dimensions, available_space, &constants);

    // 3. Determine the flex base size and hypothetical main size of each item.
    debug_log!("determine_flex_base_size");
    determine_flex_base_size(tree, &constants, available_space, &mut flex_items);

    #[cfg(feature = "debug")]
    for item in flex_items.iter() {
        debug_log!("item.flex_basis", item.flex_basis);
        debug_log!("item.inner_flex_basis", item.inner_flex_basis);
        debug_log!("item.hypothetical_outer_size", dbg:item.hypothetical_outer_size);
        debug_log!("item.hypothetical_inner_size", dbg:item.hypothetical_inner_size);
        debug_log!("item.resolved_minimum_main_size", dbg:item.resolved_minimum_main_size);
    }

    // 4. Determine the main size of the flex container
    // This has already been done as part of compute_constants. The inner size is exposed as constants.node_inner_size.

    // 9.3. Main Size Determination

    // 5. Collect flex items into flex lines.
    debug_log!("collect_flex_lines");
    #[cfg(feature = "flexbox_balance")]
    let mut flex_lines = if constants.is_balance {
        collect_balanced_flex_lines(&constants, available_space, &mut flex_items)
    } else {
        collect_flex_lines(&constants, available_space, &mut flex_items)
    };
    #[cfg(not(feature = "flexbox_balance"))]
    let mut flex_lines = collect_flex_lines(&constants, available_space, &mut flex_items);

    // If container size is undefined, determine the container's main size
    // and then re-resolve gaps based on newly determined size
    debug_log!("determine_container_main_size");
    if let Some(inner_main_size) = constants.node_inner_size.main(constants.dir) {
        let outer_main_size = inner_main_size + constants.content_box_inset.main_axis_sum(constants.dir);
        constants.inner_container_size.set_main(constants.dir, inner_main_size);
        constants.container_size.set_main(constants.dir, outer_main_size);
    } else {
        // Sets constants.container_size and constants.outer_container_size
        determine_container_main_size(tree, available_space, &mut flex_lines, &mut constants);
        constants.node_inner_size.set_main(constants.dir, Some(constants.inner_container_size.main(constants.dir)));
        constants.node_outer_size.set_main(constants.dir, Some(constants.container_size.main(constants.dir)));

        debug_log!("constants.node_outer_size", dbg:constants.node_outer_size);
        debug_log!("constants.node_inner_size", dbg:constants.node_inner_size);

        // Re-resolve percentage gaps
        let style = tree.get_flexbox_container_style(node);
        let inner_container_size = constants.inner_container_size.main(constants.dir);
        let new_gap = style
            .gap()
            .main(constants.dir)
            .maybe_resolve(inner_container_size, |val, basis| tree.calc(val, basis))
            .unwrap_or(0.0);
        constants.gap.set_main(constants.dir, new_gap);
    }

    // 6. Resolve the flexible lengths of all the flex items to find their used main size.
    debug_log!("resolve_flexible_lengths");
    for line in &mut flex_lines {
        resolve_flexible_lengths(line, &constants);
    }

    // 9.4. Cross Size Determination

    // 7. Determine the hypothetical cross size of each item.
    debug_log!("determine_hypothetical_cross_size");
    for line in &mut flex_lines {
        determine_hypothetical_cross_size(tree, line, &constants, available_space);
    }

    // Calculate child baselines. This function is internally smart and only computes child baselines
    // if they are necessary.
    debug_log!("calculate_children_base_lines");
    calculate_children_base_lines(tree, known_dimensions, available_space, &mut flex_lines, &constants);

    // 8. Calculate the cross size of each flex line.
    debug_log!("calculate_cross_size");
    calculate_cross_size(&mut flex_lines, known_dimensions, &constants);

    // 9. Handle 'align-content: stretch'.
    debug_log!("handle_align_content_stretch");
    handle_align_content_stretch(&mut flex_lines, known_dimensions, &constants);

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
    debug_log!("determine_used_cross_size");
    determine_used_cross_size(tree, &mut flex_lines, &constants);

    // 9.5. Main-Axis Alignment

    // 12. Distribute any remaining free space.
    debug_log!("distribute_remaining_free_space");
    distribute_remaining_free_space(&mut flex_lines, &constants);

    // 9.6. Cross-Axis Alignment

    // 13. Resolve cross-axis auto margins (also includes 14).
    debug_log!("resolve_cross_axis_auto_margins");
    resolve_cross_axis_auto_margins(&mut flex_lines, &constants);

    // 15. Determine the flex container’s used cross size.
    debug_log!("determine_container_cross_size");
    let total_line_cross_size = determine_container_cross_size(&flex_lines, known_dimensions, &mut constants);

    // We have the container size.
    // If our caller does not care about performing layout we are done now.
    if run_mode == RunMode::ComputeSize {
        return LayoutOutput::from_outer_size(constants.container_size);
    }

    // 16. Align all flex lines per align-content.
    debug_log!("align_flex_lines_per_align_content");
    align_flex_lines_per_align_content(&mut flex_lines, &constants, total_line_cross_size);

    // Do a final layout pass and gather the resulting layouts
    debug_log!("final_layout_pass");
    let inflow_content_size = final_layout_pass(tree, &mut flex_lines, &constants);

    // Before returning we perform absolute layout on all absolutely positioned children
    debug_log!("perform_absolute_layout_on_absolute_children");
    let absolute_content_size = perform_absolute_layout_on_absolute_children(tree, node, &constants);

    debug_log!("hidden_layout");
    let len = tree.child_count(node);
    for order in 0..len {
        let child = tree.get_child_id(node, order);
        if tree.get_flexbox_child_style(child).box_generation_mode() == BoxGenerationMode::None {
            tree.set_unrounded_layout(child, &Layout::with_order(order as u32));
            tree.perform_child_layout(
                child,
                Size::NONE,
                Size::NONE,
                Size::MAX_CONTENT,
                SizingMode::InherentSize,
                Line::FALSE,
            );
        }
    }

    // 8.5. Flex Container Baselines: calculate the flex container's first baseline
    // See https://www.w3.org/TR/css-flexbox-1/#flex-baselines
    // For wrap-reverse containers the cross-start-most line is the last line rather than the first,
    // and it is that line which the container's first baseline is generated from.
    let first_line = if constants.is_wrap_reverse { flex_lines.last() } else { flex_lines.first() };
    let first_vertical_baseline = first_line.and_then(|line| {
        line.items
            .iter()
            .find(|item| constants.is_column || item.participates_in_baseline_alignment(constants.dir))
            .or_else(|| line.items.iter().next())
            .map(|child| child.baseline)
    });

    LayoutOutput::from_sizes_and_baselines(
        constants.container_size,
        inflow_content_size.f32_max(absolute_content_size),
        Baselines::from_first(first_vertical_baseline),
    )
}

/// Compute constants that can be reused during the flexbox algorithm.
#[inline]
fn compute_constants(
    tree: &impl LayoutFlexboxContainer,
    style: impl FlexboxContainerStyle,
    known_dimensions: Size<Option<f32>>,
    known_dimensions_are_definite: Size<bool>,
    parent_size: Size<Option<f32>>,
) -> AlgoConstants {
    let dir = style.flex_direction();
    let is_row = dir.is_row();
    let is_column = dir.is_column();
    let flex_wrap = style.flex_wrap();
    let is_wrap = flex_wrap.is_multi_line();
    let is_wrap_reverse = flex_wrap.is_reverse();
    #[cfg(feature = "flexbox_balance")]
    let is_balance = flex_wrap.is_balance();
    #[cfg(feature = "flexbox_balance")]
    let line_count = if is_wrap { Some(style.flex_line_count().max(1)) } else { None };

    let aspect_ratio = style.aspect_ratio();
    let margin = style.margin().resolve_or_zero(parent_size.width, |val, basis| tree.calc(val, basis));
    let padding = style.padding().resolve_or_zero(parent_size.width, |val, basis| tree.calc(val, basis));
    let border = style.border().resolve_or_zero(parent_size.width, |val, basis| tree.calc(val, basis));
    let padding_border_sum = padding.sum_axes() + border.sum_axes();
    let box_sizing_adjustment =
        if style.box_sizing() == BoxSizing::ContentBox { padding_border_sum } else { Size::ZERO };

    let align_items = style.align_items().unwrap_or(AlignItems::STRETCH);
    let align_content = style.align_content().unwrap_or(AlignContent::STRETCH);
    let justify_content = style.justify_content();
    let layout_direction = style.direction();

    // Scrollbar gutters are reserved when the `overflow` property is set to `Overflow::Scroll`.
    // However, the axis are switched (transposed) because a node that scrolls vertically needs
    // *horizontal* space to be reserved for a scrollbar
    let scrollbar_gutter = style.overflow().transpose().map(|overflow| match overflow {
        Overflow::Scroll => style.scrollbar_width(),
        _ => 0.0,
    });
    #[cfg(feature = "content_size")]
    let is_scroll_container = {
        let overflow = style.overflow();
        overflow.x.is_scroll_container() || overflow.y.is_scroll_container()
    };
    let mut content_box_inset = padding + border;
    content_box_inset.bottom += scrollbar_gutter.y;

    match layout_direction {
        Direction::Ltr => content_box_inset.right += scrollbar_gutter.x,
        Direction::Rtl => content_box_inset.left += scrollbar_gutter.x,
    };

    let node_outer_size = known_dimensions;
    let node_inner_size = node_outer_size.maybe_sub(content_box_inset.sum_axes());
    let known_main_size_is_definite = known_dimensions_are_definite.main(dir);
    let has_definite_main_size = known_main_size_is_definite && known_dimensions.main(dir).is_some();
    let has_definite_cross_size = known_dimensions_are_definite.cross(dir) && known_dimensions.cross(dir).is_some();
    let gap = style.gap().resolve_or_zero(node_inner_size.or(Size::zero()), |val, basis| tree.calc(val, basis));

    let container_size = Size::zero();
    let inner_container_size = Size::zero();

    AlgoConstants {
        dir,
        layout_direction,
        is_row,
        is_column,
        is_wrap,
        is_wrap_reverse,
        #[cfg(feature = "flexbox_balance")]
        is_balance,
        #[cfg(feature = "flexbox_balance")]
        line_count,
        min_size: style
            .min_size()
            .maybe_resolve(parent_size, |val, basis| tree.calc(val, basis))
            .maybe_apply_aspect_ratio(aspect_ratio)
            .maybe_add(box_sizing_adjustment),
        max_size: style
            .max_size()
            .maybe_resolve(parent_size, |val, basis| tree.calc(val, basis))
            .maybe_apply_aspect_ratio(aspect_ratio)
            .maybe_add(box_sizing_adjustment),
        margin,
        border,
        gap,
        content_box_inset,
        scrollbar_gutter,
        #[cfg(feature = "content_size")]
        is_scroll_container,
        align_items,
        align_content,
        justify_content,
        node_outer_size,
        node_inner_size,
        known_main_size_is_definite,
        has_definite_main_size,
        has_definite_cross_size,
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
fn generate_anonymous_flex_items(
    tree: &impl LayoutFlexboxContainer,
    node: NodeId,
    constants: &AlgoConstants,
) -> Vec<FlexItem> {
    // Percentage sizes of items resolve against the container's inner size, but only if that size
    // is definite. A known main size which is derived from the container's own content is treated
    // as indefinite here.
    let percent_resolution_size = if constants.known_main_size_is_definite {
        constants.node_inner_size
    } else {
        constants.node_inner_size.with_main(constants.dir, None)
    };

    tree.child_ids(node)
        .enumerate()
        .map(|(index, child)| (index, child, tree.get_flexbox_child_style(child)))
        .filter(|(_, _, style)| style.position() != Position::Absolute)
        .filter(|(_, _, style)| style.box_generation_mode() != BoxGenerationMode::None)
        .map(|(index, child, child_style)| {
            let aspect_ratio = child_style.aspect_ratio();
            let padding = child_style
                .padding()
                .resolve_or_zero(constants.node_inner_size.width, |val, basis| tree.calc(val, basis));
            let border = child_style
                .border()
                .resolve_or_zero(constants.node_inner_size.width, |val, basis| tree.calc(val, basis));
            let pb_sum = (padding + border).sum_axes();
            let box_sizing_adjustment =
                if child_style.box_sizing() == BoxSizing::ContentBox { pb_sum } else { Size::ZERO };
            FlexItem {
                node: child,
                order: index as u32,
                size: child_style
                    .size()
                    .maybe_resolve(percent_resolution_size, |val, basis| tree.calc(val, basis))
                    .maybe_apply_aspect_ratio(aspect_ratio)
                    .maybe_add(box_sizing_adjustment),
                size_style: child_style.size(),
                min_size: child_style
                    .min_size()
                    .maybe_resolve(percent_resolution_size, |val, basis| tree.calc(val, basis))
                    .maybe_add(box_sizing_adjustment),
                max_size: child_style
                    .max_size()
                    .maybe_resolve(percent_resolution_size, |val, basis| tree.calc(val, basis))
                    .maybe_add(box_sizing_adjustment),
                aspect_ratio,

                inset: child_style
                    .inset()
                    .zip_size(constants.node_inner_size, |p, s| p.maybe_resolve(s, |val, basis| tree.calc(val, basis))),
                margin: child_style
                    .margin()
                    .resolve_or_zero(constants.node_inner_size.width, |val, basis| tree.calc(val, basis)),
                margin_is_auto: child_style.margin().map(LengthPercentageAuto::is_auto),
                padding: child_style
                    .padding()
                    .resolve_or_zero(constants.node_inner_size.width, |val, basis| tree.calc(val, basis)),
                border: child_style
                    .border()
                    .resolve_or_zero(constants.node_inner_size.width, |val, basis| tree.calc(val, basis)),
                align_self: child_style.align_self().unwrap_or(constants.align_items).resolve_self_relative(
                    child_style.direction(),
                    constants.layout_direction,
                    constants.is_column,
                ),
                overflow: child_style.overflow(),
                scrollbar_width: child_style.scrollbar_width(),
                flex_grow: child_style.flex_grow(),
                flex_shrink: child_style.flex_shrink(),
                flex_basis_is_definite: false,
                flex_basis: 0.0,
                inner_flex_basis: 0.0,
                violation: 0.0,
                frozen: false,

                resolved_minimum_main_size: 0.0,
                hypothetical_inner_size: Size::zero(),
                hypothetical_outer_size: Size::zero(),
                target_size: Size::zero(),
                outer_target_size: Size::zero(),
                content_flex_fraction: 0.0,

                baseline: 0.0,

                offset_main: 0.0,
                offset_cross: 0.0,
            }
        })
        .collect()
}

/// Determine the available main and cross space for the flex items.
///
/// # [9.2. Line Length Determination](https://www.w3.org/TR/css-flexbox-1/#line-sizing)
///
/// - [**Determine the available main and cross space for the flex items**](https://www.w3.org/TR/css-flexbox-1/#algo-available).
///
/// For each dimension, if that dimension of the flex container’s content box is a definite size, use that;
/// if that dimension of the flex container is being sized under a min or max-content constraint, the available space in that dimension is that constraint;
/// otherwise, subtract the flex container’s margin, border, and padding from the space available to the flex container in that dimension and use that value.
/// **This might result in an infinite value**.
#[inline]
#[must_use]
fn determine_available_space(
    known_dimensions: Size<Option<f32>>,
    outer_available_space: Size<AvailableSpace>,
    constants: &AlgoConstants,
) -> Size<AvailableSpace> {
    // Note: min/max/preferred size styles have already been applied to known_dimensions in the `compute` function above
    let width = match known_dimensions.width {
        Some(node_width) => AvailableSpace::Definite(node_width - constants.content_box_inset.horizontal_axis_sum()),
        None => outer_available_space
            .width
            .maybe_sub(constants.margin.horizontal_axis_sum())
            .maybe_sub(constants.content_box_inset.horizontal_axis_sum()),
    };

    let height = match known_dimensions.height {
        Some(node_height) => AvailableSpace::Definite(node_height - constants.content_box_inset.vertical_axis_sum()),
        None => outer_available_space
            .height
            .maybe_sub(constants.margin.vertical_axis_sum())
            .maybe_sub(constants.content_box_inset.vertical_axis_sum()),
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
///       then the flex base size is calculated from its inner cross size and the flex item’s intrinsic aspect ratio.
///
///     - C. If the used flex basis is content or depends on its available space, and the flex container is being sized under a min-content
///       or max-content constraint (e.g. when performing automatic table layout \[CSS21\]), size the item under that constraint.
///       The flex base size is the item’s resulting main size.
///
///     - E. Otherwise, size the item into the available space using its used flex basis in place of its main size, treating a value of content as max-content.
///       If a cross size is needed to determine the main size (e.g. when the flex item’s main size is in its block axis) and the flex item’s cross size is auto and not definite,
///       in this calculation use fit-content as the flex item’s cross size. The flex base size is the item’s resulting main size.
///
///   When determining the flex base size, the item’s min and max main sizes are ignored (no clamping occurs).
///   Furthermore, the sizing calculations that floor the content box size at zero when applying box-sizing are also ignored.
///   (For example, an item with a specified size of zero, positive padding, and box-sizing: border-box will have an outer flex base size of zero—and hence a negative inner flex base size.)
#[inline]
fn determine_flex_base_size(
    tree: &mut impl LayoutFlexboxContainer,
    constants: &AlgoConstants,
    available_space: Size<AvailableSpace>,
    flex_items: &mut [FlexItem],
) {
    let dir = constants.dir;

    for child in flex_items.iter_mut() {
        let child_style = tree.get_flexbox_child_style(child.node);

        // Parent size for child sizing
        let cross_axis_parent_size = constants.node_inner_size.cross(dir);
        let child_parent_size = Size::from_cross(dir, cross_axis_parent_size);

        // Available space for child sizing
        // Min/max sizes transferred through the aspect ratio are taken into account here
        // https://github.com/w3c/csswg-drafts/issues/10997
        let cross_axis_margin_sum = constants.margin.cross_axis_sum(dir);
        let transferred_min_size = child.min_size.maybe_apply_aspect_ratio(child.aspect_ratio);
        let transferred_max_size = child.max_size.maybe_apply_aspect_ratio(child.aspect_ratio);
        let child_min_cross = transferred_min_size.cross(dir).maybe_add(cross_axis_margin_sum);
        let child_max_cross = transferred_max_size.cross(dir).maybe_add(cross_axis_margin_sum);

        // Clamp available space by min- and max- size
        let cross_axis_available_space: AvailableSpace = match available_space.cross(dir) {
            AvailableSpace::Definite(val) => AvailableSpace::Definite(
                constants
                    .divided_cross_space(cross_axis_parent_size.unwrap_or(val))
                    .maybe_clamp(child_min_cross, child_max_cross),
            ),
            AvailableSpace::MinContent => match child_min_cross {
                Some(min) => AvailableSpace::Definite(min),
                None => AvailableSpace::MinContent,
            },
            AvailableSpace::MaxContent => match child_max_cross {
                Some(max) => AvailableSpace::Definite(max),
                None => AvailableSpace::MaxContent,
            },
        };

        // Known dimensions for child sizing
        let mut child_cross_size_is_definite = child.size.cross(dir).is_some();
        let child_known_dimensions = {
            let mut ckd = child.size.with_main(dir, None);
            // Clamp the definite cross size by the cross min/max sizes so that sizes
            // transferred through an intrinsic aspect ratio (e.g. for replaced elements)
            // are based on the used cross size.
            ckd.set_cross(
                dir,
                ckd.cross(dir).maybe_clamp(transferred_min_size.cross(dir), transferred_max_size.cross(dir)),
            );
            if child.align_self == AlignSelf::STRETCH
                && !child.margin_is_auto.cross_start(constants.dir)
                && !child.margin_is_auto.cross_end(constants.dir)
                && ckd.cross(dir).is_none()
            {
                ckd.set_cross(
                    dir,
                    cross_axis_available_space.into_option().maybe_sub(child.margin.cross_axis_sum(dir)),
                );
                // The cross size of a stretched item is definite if the container has a definite
                // cross size (https://www.w3.org/TR/css-flexbox-1/#definite-sizes)
                child_cross_size_is_definite =
                    !constants.is_wrap && constants.has_definite_cross_size && cross_axis_parent_size.is_some();
            }
            ckd
        };

        let container_width = constants.node_inner_size.main(dir);
        let box_sizing_adjustment = if child_style.box_sizing() == BoxSizing::ContentBox {
            let padding = child_style.padding().resolve_or_zero(container_width, |val, basis| tree.calc(val, basis));
            let border = child_style.border().resolve_or_zero(container_width, |val, basis| tree.calc(val, basis));
            (padding + border).sum_axes()
        } else {
            Size::ZERO
        }
        .main(dir);
        // Percentage flex basis values resolve against the container's inner main size, but only
        // if that size is definite. A known main size which is derived from the container's own
        // content is treated as indefinite here.
        let percent_resolution_main_size =
            if constants.known_main_size_is_definite { constants.node_inner_size.main(dir) } else { None };
        let flex_basis_style = child_style.flex_basis();
        let flex_basis = flex_basis_style
            .maybe_resolve(percent_resolution_main_size, |val, basis| tree.calc(val, basis))
            .maybe_add(box_sizing_adjustment);

        drop(child_style);

        child.flex_basis = 'flex_basis: {
            // A. If the item has a definite used flex basis, that’s the flex base size.

            // B. If the flex item has an intrinsic aspect ratio,
            //    a used flex basis of content, and a definite cross size,
            //    then the flex base size is calculated from its inner
            //    cross size and the flex item’s intrinsic aspect ratio.

            // Note: `child.size` has already been resolved against aspect_ratio in generate_anonymous_flex_items
            // So B will just work here by using main_size without special handling for aspect_ratio
            let main_size = child.size.main(dir);
            let main_stretch_size = percent_resolution_main_size.maybe_sub(child.margin.main_axis_sum(dir));

            // A flex basis that is a sizing keyword (min-content, max-content, fit-content,
            // fit-content(...), stretch) is used in place of the main size property: `stretch`
            // resolves to an exact (definite) size while the other keywords determine the
            // available space constraint the item is measured under. A keyword that cannot be
            // resolved in the current context behaves as `content`.
            let keyword_main_available_space = if flex_basis_style.is_content() {
                // A flex basis of `content` indicates an automatic size based on the item's
                // content: the item is measured (ignoring its main size property) under the
                // default constraint below
                None
            } else if flex_basis_style.is_sizing_keyword() {
                match resolve_sizing_keyword(flex_basis_style, main_stretch_size, percent_resolution_main_size) {
                    Some(SizingKeywordResolution::Exact(size)) => {
                        child.flex_basis_is_definite = true;
                        break 'flex_basis size;
                    }
                    Some(SizingKeywordResolution::Measure(available)) => Some(available),
                    None => None,
                }
            } else {
                if let Some(flex_basis) = flex_basis.or(main_size) {
                    child.flex_basis_is_definite = true;
                    break 'flex_basis flex_basis;
                };

                // A main size that is a sizing keyword either resolves to an exact size or
                // determines the available space constraint the item is measured under
                match resolve_sizing_keyword(
                    child.size_style.main(dir),
                    main_stretch_size,
                    percent_resolution_main_size,
                ) {
                    Some(SizingKeywordResolution::Exact(size)) => {
                        child.flex_basis_is_definite = true;
                        break 'flex_basis size;
                    }
                    Some(SizingKeywordResolution::Measure(available)) => Some(available),
                    None => None,
                }
            };

            // C. If the used flex basis is content or depends on its available space,
            //    and the flex container is being sized under a min-content or max-content
            //    constraint (e.g. when performing automatic table layout [CSS21]),
            //    size the item under that constraint. The flex base size is the item’s
            //    resulting main size.

            // This is covered by the implementation of E below, which passes the available_space constraint
            // through to the child size computation. It may need a separate implementation if/when D is implemented.

            // D. Otherwise, if the used flex basis is content or depends on its
            //    available space, the available main size is infinite, and the flex item’s
            //    inline axis is parallel to the main axis, lay the item out using the rules
            //    for a box in an orthogonal flow [CSS3-WRITING-MODES]. The flex base size
            //    is the item’s max-content main size.

            // TODO if/when vertical writing modes are supported

            // If the item has an aspect ratio and a definite cross size then the flex base size
            // is derived from that cross size via the aspect ratio (case B above, as applied by
            // the child's own layout below), and is therefore definite.
            if child.aspect_ratio.is_some()
                && child_cross_size_is_definite
                && child_known_dimensions.cross(dir).is_some()
            {
                child.flex_basis_is_definite = true;
            }

            // E. Otherwise, size the item into the available space using its used flex basis
            //    in place of its main size, treating a value of content as max-content.
            //    If a cross size is needed to determine the main size (e.g. when the
            //    flex item’s main size is in its block axis) and the flex item’s cross size
            //    is auto and not definite, in this calculation use fit-content as the
            //    flex item’s cross size. The flex base size is the item’s resulting main size.

            let child_available_space = Size::MAX_CONTENT
                .with_main(
                    dir,
                    keyword_main_available_space.unwrap_or(
                        // Map AvailableSpace::Definite to AvailableSpace::MaxContent
                        if available_space.main(dir) == AvailableSpace::MinContent {
                            AvailableSpace::MinContent
                        } else {
                            AvailableSpace::MaxContent
                        },
                    ),
                )
                .with_cross(dir, cross_axis_available_space);

            debug_log!("COMPUTE CHILD BASE SIZE:");
            break 'flex_basis tree.measure_child_size(
                child.node,
                child_known_dimensions,
                child_parent_size,
                child_available_space,
                SizingMode::ContentSize,
                dir.main_axis(),
                Line::FALSE,
            );
        };

        // Floor flex-basis by the padding_border_sum (floors inner_flex_basis at zero)
        // This seems to be in violation of the spec which explicitly states that the content box should not be floored at zero
        // (like it usually is) when calculating the flex-basis. But including this matches both Chrome and Firefox's behaviour.
        //
        // TODO: resolve spec violation
        // Spec: https://www.w3.org/TR/css-flexbox-1/#intrinsic-item-contributions
        // Spec: https://www.w3.org/TR/css-flexbox-1/#change-2016-max-contribution
        let padding_border_sum = child.padding.main_axis_sum(constants.dir) + child.border.main_axis_sum(constants.dir);
        child.flex_basis = child.flex_basis.max(padding_border_sum);

        // The hypothetical main size is the item’s flex base size clamped according to its
        // used min and max main sizes (and flooring the content box size at zero).

        child.inner_flex_basis =
            child.flex_basis - child.padding.main_axis_sum(constants.dir) - child.border.main_axis_sum(constants.dir);

        let padding_border_axes_sums = (child.padding + child.border).sum_axes().map(Some);

        // Note that it is important that the `parent_size` parameter in the main axis is not set for this
        // function call as it used for resolving percentages, and percentage size in an axis should not contribute
        // to a min-content contribution in that same axis. However the `parent_size` and `available_space` *should*
        // be set to their usual values in the cross axis so that wrapping content can wrap correctly.
        //
        // See https://drafts.csswg.org/css-sizing-3/#min-percentage-contribution
        let style_min_main_size =
            child.min_size.or(child.overflow.map(Overflow::maybe_into_automatic_min_size).into()).main(dir);

        child.resolved_minimum_main_size = style_min_main_size.unwrap_or_else(|| {
            let min_content_main_size = {
                let child_available_space = Size::MIN_CONTENT.with_cross(dir, cross_axis_available_space);

                debug_log!("COMPUTE CHILD MIN SIZE:");
                tree.measure_child_size(
                    child.node,
                    child_known_dimensions,
                    child_parent_size,
                    child_available_space,
                    SizingMode::ContentSize,
                    dir.main_axis(),
                    Line::FALSE,
                )
            };

            // 4.5. Automatic Minimum Size of Flex Items
            // https://www.w3.org/TR/css-flexbox-1/#min-size-auto
            let clamped_min_content_size =
                min_content_main_size.maybe_min(child.size.main(dir)).maybe_min(transferred_max_size.main(dir));
            clamped_min_content_size.maybe_max(padding_border_axes_sums.main(dir))
        });

        // Sizes transferred through the aspect ratio clamp the hypothetical main size,
        // but do not participate in resolving flexible lengths or clamping the final size.
        // https://github.com/w3c/csswg-drafts/issues/10997
        let hypothetical_inner_min_main = child
            .resolved_minimum_main_size
            .maybe_max(transferred_min_size.main(constants.dir))
            .maybe_max(padding_border_axes_sums.main(constants.dir));
        let hypothetical_inner_size =
            child.flex_basis.maybe_clamp(Some(hypothetical_inner_min_main), transferred_max_size.main(constants.dir));
        let hypothetical_outer_size = hypothetical_inner_size + child.margin.main_axis_sum(constants.dir);

        child.hypothetical_inner_size.set_main(constants.dir, hypothetical_inner_size);
        child.hypothetical_outer_size.set_main(constants.dir, hypothetical_outer_size);
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
///       (or until a forced break is encountered, see [§10 Fragmenting Flex Layout](https://www.w3.org/TR/css-flexbox-1/#pagination)).
///       If the very first uncollected item wouldn't fit, collect just it into the line.
///
///       For this step, the size of a flex item is its outer hypothetical main size. (**Note: This can be negative**.)
///
///       Repeat until all flex items have been collected into flex lines.
///
///       **Note that the "collect as many" line will collect zero-sized flex items onto the end of the previous line even if the last non-zero item exactly "filled up" the line**.
#[inline]
fn collect_flex_lines<'a>(
    constants: &AlgoConstants,
    available_space: Size<AvailableSpace>,
    flex_items: &'a mut Vec<FlexItem>,
) -> Vec<FlexLine<'a>> {
    // Wrapping into multiple lines requires a definite main size. If the container's known main size
    // is derived from its own content (and is thus indefinite) then all items are collected into a
    // single flex line, matching how the container was sized under a min/max-content constraint.
    if !constants.is_wrap || !constants.known_main_size_is_definite {
        let mut lines = new_vec_with_capacity(1);
        lines.push(FlexLine { items: flex_items.as_mut_slice(), cross_size: 0.0, offset_cross: 0.0 });
        lines
    } else {
        let main_axis_available_space = match constants.max_size.main(constants.dir) {
            Some(max_size) => AvailableSpace::Definite({
                let available = available_space.main(constants.dir).into_option().unwrap_or(max_size);
                // If the container's main size is not definite then it is at most the max main size,
                // so the max size (and not the available space) is the limit that items wrap against.
                let available = if constants.has_definite_main_size { available } else { available.min(max_size) };
                available.maybe_max(constants.min_size.main(constants.dir))
            }),
            None => available_space.main(constants.dir),
        };

        match main_axis_available_space {
            // If we're sizing under a max-content constraint then the flex items will never wrap
            // (at least for now - future extensions to the CSS spec may add provisions for forced wrap points)
            AvailableSpace::MaxContent => {
                let mut lines = new_vec_with_capacity(1);
                lines.push(FlexLine { items: flex_items.as_mut_slice(), cross_size: 0.0, offset_cross: 0.0 });
                lines
            }
            // If flex-wrap is Wrap and we're sizing under a min-content constraint, then we take every possible wrapping opportunity
            // and place each item in it's own line
            AvailableSpace::MinContent => {
                let mut lines = new_vec_with_capacity(flex_items.len());
                let mut items = &mut flex_items[..];
                while !items.is_empty() {
                    let (line_items, rest) = items.split_at_mut(1);
                    lines.push(FlexLine { items: line_items, cross_size: 0.0, offset_cross: 0.0 });
                    items = rest;
                }
                lines
            }
            AvailableSpace::Definite(main_axis_available_space) => {
                let mut lines = new_vec_with_capacity(1);
                let mut flex_items = &mut flex_items[..];
                let main_axis_gap = constants.gap.main(constants.dir);

                while !flex_items.is_empty() {
                    // Find index of the first item in the next line
                    // (or the last item if all remaining items are in the current line)
                    let mut line_length = 0.0;
                    let index = flex_items
                        .iter()
                        .enumerate()
                        .find(|&(idx, child)| {
                            // Gaps only occur between items (not before the first one or after the last one)
                            // So first item in the line does not contribute a gap to the line length
                            let gap_contribution = if idx == 0 { 0.0 } else { main_axis_gap };
                            line_length += child.hypothetical_outer_size.main(constants.dir) + gap_contribution;
                            line_length > main_axis_available_space && idx != 0
                        })
                        .map(|(idx, _)| idx)
                        .unwrap_or(flex_items.len());

                    let (items, rest) = flex_items.split_at_mut(index);
                    lines.push(FlexLine { items, cross_size: 0.0, offset_cross: 0.0 });
                    flex_items = rest;
                }
                lines
            }
        }
    }
}

/// Collect flex items into balanced flex lines (`flex-wrap: balance`), such that the largest
/// line is as small as possible.
///
/// See <https://drafts.csswg.org/css-flexbox-2/#balance-values>
#[cfg(feature = "flexbox_balance")]
fn collect_balanced_flex_lines<'a>(
    constants: &AlgoConstants,
    available_space: Size<AvailableSpace>,
    flex_items: &'a mut [FlexItem],
) -> Vec<FlexLine<'a>> {
    if flex_items.is_empty() {
        return new_vec_with_capacity(0);
    }

    // If the container's known main size is derived from its own content (and is thus indefinite)
    // then items are balanced without a size limit, matching how the container was sized under a
    // min/max-content constraint.
    let main_axis_available_space = if constants.known_main_size_is_definite {
        match constants.max_size.main(constants.dir) {
            Some(max_size) => AvailableSpace::Definite({
                let available = available_space.main(constants.dir).into_option().unwrap_or(max_size);
                // If the container's main size is not definite then it is at most the max main size,
                // so the max size (and not the available space) is the limit that items wrap against.
                let available = if constants.has_definite_main_size { available } else { available.min(max_size) };
                available.maybe_max(constants.min_size.main(constants.dir))
            }),
            None => available_space.main(constants.dir),
        }
    } else {
        AvailableSpace::MaxContent
    };

    // If we're sizing under a min-content constraint then we take every possible wrapping
    // opportunity and place each item in its own line, the same as greedy wrapping (the
    // min-content main size of a multi-line container is the size of its largest item)
    if main_axis_available_space == AvailableSpace::MinContent {
        let mut lines = new_vec_with_capacity(flex_items.len());
        let mut items = &mut flex_items[..];
        while !items.is_empty() {
            let (line_items, rest) = items.split_at_mut(1);
            lines.push(FlexLine { items: line_items, cross_size: 0.0, offset_cross: 0.0 });
            items = rest;
        }
        return lines;
    }

    let line_break_size = main_axis_available_space.into_option().unwrap_or(f32::INFINITY);
    let min_line_count = constants.line_count.unwrap_or(1) as usize;
    let item_counts = balance::balanced_line_item_counts(
        flex_items.iter().map(|item| item.hypothetical_outer_size.main(constants.dir)),
        line_break_size,
        constants.gap.main(constants.dir),
        min_line_count,
    );

    let mut lines = new_vec_with_capacity(item_counts.len());
    let mut items = &mut flex_items[..];
    for count in item_counts {
        let (line_items, rest) = items.split_at_mut(count);
        lines.push(FlexLine { items: line_items, cross_size: 0.0, offset_cross: 0.0 });
        items = rest;
    }
    debug_assert!(items.is_empty());
    lines
}

/// Compute whether each of an item's known dimensions should be treated as definite when performing
/// layout on the item. An item's post-flexing main size is only treated as definite if the container
/// has a definite main size or the item's used flex basis is definite.
/// See <https://www.w3.org/TR/css-flexbox-1/#definite-sizes>
#[inline]
fn item_definiteness(constants: &AlgoConstants, item: &FlexItem) -> Size<bool> {
    let main_is_definite = constants.has_definite_main_size || item.flex_basis_is_definite;
    Size { width: true, height: true }.with_main(constants.dir, main_is_definite)
}

/// Determine the container's main size (if not already known)
fn determine_container_main_size(
    tree: &mut impl LayoutFlexboxContainer,
    available_space: Size<AvailableSpace>,
    lines: &mut [FlexLine<'_>],
    constants: &mut AlgoConstants,
) {
    let dir = constants.dir;
    let main_content_box_inset = constants.content_box_inset.main_axis_sum(constants.dir);

    let outer_main_size: f32 = constants.node_outer_size.main(constants.dir).unwrap_or_else(|| {
        match available_space.main(dir) {
            AvailableSpace::Definite(main_axis_available_space) => {
                let main_axis_gap = constants.gap.main(constants.dir);
                let item_main_length = |child: &FlexItem| {
                    let padding_border_sum = (child.padding + child.border).main_axis_sum(constants.dir);
                    (child.flex_basis.maybe_max(child.min_size.main(constants.dir))
                        + child.margin.main_axis_sum(constants.dir))
                    .max(padding_border_sum)
                };
                let longest_line_length: f32 = lines
                    .iter()
                    .map(|line| {
                        let line_main_axis_gap = sum_axis_gaps(main_axis_gap, line.items.len());
                        let total_target_size = line.items.iter().map(item_main_length).sum::<f32>();
                        total_target_size + line_main_axis_gap
                    })
                    .max_by(|a, b| a.total_cmp(b))
                    .unwrap_or(0.0);
                let size = longest_line_length + main_content_box_inset;

                // A balanced container can produce multiple lines that all fit within the
                // available space (via `flex-line-count`), in which case fit-content sizing
                // uses its max-content size: the longest line when items are balanced across
                // the minimum line count without a size limit.
                #[cfg(feature = "flexbox_balance")]
                if constants.is_balance {
                    let min_line_count = constants.line_count.unwrap_or(1);
                    let item_count = lines.iter().map(|line| line.items.len()).sum::<usize>();
                    if item_count == 0 {
                        return size;
                    }
                    let mut item_lengths: Vec<f32> = new_vec_with_capacity(item_count);
                    for line in lines.iter() {
                        for child in line.items.iter() {
                            item_lengths.push(item_main_length(child));
                        }
                    }
                    let item_counts = balance::balanced_line_item_counts(
                        item_lengths.iter().copied(),
                        f32::INFINITY,
                        main_axis_gap,
                        min_line_count.max(1) as usize,
                    );
                    let mut widest_line_length: f32 = 0.0;
                    let mut index = 0;
                    for count in item_counts {
                        let line_length: f32 = item_lengths[index..index + count].iter().sum::<f32>()
                            + sum_axis_gaps(main_axis_gap, count);
                        widest_line_length = widest_line_length.max(line_length);
                        index += count;
                    }
                    let max_content_size = widest_line_length + main_content_box_inset;
                    return f32_max(size, f32_min(max_content_size, main_axis_available_space));
                }

                if lines.len() > 1 {
                    f32_max(size, main_axis_available_space)
                } else {
                    size
                }
            }
            AvailableSpace::MinContent if constants.is_wrap => {
                let longest_line_length: f32 = lines
                    .iter()
                    .map(|line| {
                        let line_main_axis_gap = sum_axis_gaps(constants.gap.main(constants.dir), line.items.len());
                        let total_target_size = line
                            .items
                            .iter()
                            .map(|child| {
                                let padding_border_sum = (child.padding + child.border).main_axis_sum(constants.dir);
                                (child.flex_basis.maybe_max(child.min_size.main(constants.dir))
                                    + child.margin.main_axis_sum(constants.dir))
                                .max(padding_border_sum)
                            })
                            .sum::<f32>();
                        total_target_size + line_main_axis_gap
                    })
                    .max_by(|a, b| a.total_cmp(b))
                    .unwrap_or(0.0);
                longest_line_length + main_content_box_inset
            }
            AvailableSpace::MinContent | AvailableSpace::MaxContent => {
                // Define a base main_size variable. This is mutated once for iteration over the outer
                // loop over the flex lines as:
                //   "The flex container’s max-content size is the largest sum of the afore-calculated sizes of all items within a single line."
                let mut main_size = 0.0;

                for line in lines.iter_mut() {
                    for item in line.items.iter_mut() {
                        let style_min = item.min_size.main(constants.dir);
                        let style_preferred = item.size.main(constants.dir);
                        let style_max = item.max_size.main(constants.dir);

                        // The spec seems a bit unclear on this point (my initial reading was that the `.maybe_max(style_preferred)` should
                        // not be included here), however this matches both Chrome and Firefox as of 9th March 2023.
                        //
                        // Spec: https://www.w3.org/TR/css-flexbox-1/#intrinsic-item-contributions
                        // Spec modification: https://www.w3.org/TR/css-flexbox-1/#change-2016-max-contribution
                        // Issue: https://github.com/w3c/csswg-drafts/issues/1435
                        // Gentest: padding_border_overrides_size_flex_basis_0.html
                        let clamping_basis = Some(item.flex_basis).maybe_max(style_preferred);
                        let flex_basis_min = clamping_basis.filter(|_| item.flex_shrink == 0.0);
                        let flex_basis_max = clamping_basis.filter(|_| item.flex_grow == 0.0);

                        let min_main_size = style_min
                            .maybe_max(flex_basis_min)
                            .or(flex_basis_min)
                            .unwrap_or(item.resolved_minimum_main_size)
                            .max(item.resolved_minimum_main_size);
                        let max_main_size =
                            style_max.maybe_min(flex_basis_max).or(flex_basis_max).unwrap_or(f32::INFINITY);

                        let content_contribution = match (min_main_size, style_preferred, max_main_size) {
                            // If the clamping values are such that max <= min, then we can avoid the expensive step of computing the content size
                            // as we know that the clamping values will override it anyway
                            (min, Some(pref), max) if max <= min || max <= pref => {
                                pref.min(max).max(min) + item.margin.main_axis_sum(constants.dir)
                            }
                            (min, _, max) if max <= min => min + item.margin.main_axis_sum(constants.dir),

                            // Else compute the min- or -max content size and apply the full formula for computing the
                            // min- or max- content contribution
                            _ if item.is_scroll_container() => {
                                item.flex_basis + item.margin.main_axis_sum(constants.dir)
                            }
                            _ => {
                                // Parent size for child sizing
                                let cross_axis_parent_size = constants.node_inner_size.cross(dir);

                                // Available space for child sizing
                                let cross_axis_margin_sum = constants.margin.cross_axis_sum(dir);
                                let child_min_cross = item.min_size.cross(dir).maybe_add(cross_axis_margin_sum);
                                let child_max_cross = item.max_size.cross(dir).maybe_add(cross_axis_margin_sum);
                                let cross_axis_available_space: AvailableSpace = available_space
                                    .cross(dir)
                                    .map_definite_value(|val| {
                                        constants.divided_cross_space(cross_axis_parent_size.unwrap_or(val))
                                    })
                                    .maybe_clamp(child_min_cross, child_max_cross);

                                let child_available_space = available_space.with_cross(dir, cross_axis_available_space);

                                // Known dimensions for child sizing
                                let child_known_dimensions = {
                                    let mut ckd = item.size.with_main(dir, None);
                                    if item.align_self == AlignSelf::STRETCH && ckd.cross(dir).is_none() {
                                        ckd.set_cross(
                                            dir,
                                            cross_axis_available_space
                                                .into_option()
                                                .maybe_sub(item.margin.cross_axis_sum(dir)),
                                        );
                                    }
                                    ckd
                                };

                                // Either the min- or max- content size depending on which constraint we are sizing under.
                                // TODO: Optimise by using already computed values where available
                                debug_log!("COMPUTE CHILD BASE SIZE (for intrinsic main size):");
                                let content_main_size = tree.measure_child_size(
                                    item.node,
                                    child_known_dimensions,
                                    constants.node_inner_size,
                                    child_available_space,
                                    SizingMode::InherentSize,
                                    dir.main_axis(),
                                    Line::FALSE,
                                ) + item.margin.main_axis_sum(constants.dir);

                                // This is somewhat bizarre in that it's asymmetrical depending whether the flex container is a column or a row.
                                //
                                // I *think* this might relate to https://drafts.csswg.org/css-flexbox-1/#algo-main-container:
                                //
                                //    "The automatic block size of a block-level flex container is its max-content size."
                                //
                                // Which could suggest that flex-basis defining a vertical size does not shrink because it is in the block axis, and the automatic size
                                // in the block axis is a MAX content size. Whereas a flex-basis defining a horizontal size does shrink because the automatic size in
                                // inline axis is MIN content size (although I don't have a reference for that).
                                //
                                // Ultimately, this was not found by reading the spec, but by trial and error fixing tests to align with Webkit/Firefox output.
                                // (see the `flex_basis_unconstraint_row` and `flex_basis_uncontraint_column` generated tests which demonstrate this)
                                if constants.is_row {
                                    content_main_size.maybe_clamp(style_min, style_max)
                                } else {
                                    content_main_size.max(item.flex_basis).maybe_clamp(style_min, style_max)
                                }
                            }
                        };
                        item.content_flex_fraction = {
                            let diff = content_contribution - item.flex_basis;
                            if diff > 0.0 {
                                diff / f32_max(1.0, item.flex_grow)
                            } else if diff < 0.0 {
                                let scaled_shrink_factor = f32_max(1.0, item.flex_shrink * item.inner_flex_basis);
                                diff / scaled_shrink_factor
                            } else {
                                // We are assuming that diff is 0.0 here and that we haven't accidentally introduced a NaN
                                0.0
                            }
                        };
                    }

                    // TODO Spec says to scale everything by the line's max flex fraction. But neither Chrome nor firefox implement this
                    // so we don't either. But if we did want to, we'd need this computation here (and to use it below):
                    //
                    // Within each line, find the largest max-content flex fraction among all the flex items.
                    // let line_flex_fraction = line
                    //     .items
                    //     .iter()
                    //     .map(|item| item.content_flex_fraction)
                    //     .max_by(|a, b| a.total_cmp(b))
                    //     .unwrap_or(0.0); // Unwrap case never gets hit because there is always at least one item a line

                    // Add each item’s flex base size to the product of:
                    //   - its flex grow factor (or scaled flex shrink factor,if the chosen max-content flex fraction was negative)
                    //   - the chosen max-content flex fraction
                    // then clamp that result by the max main size floored by the min main size.
                    //
                    // The flex container’s max-content size is the largest sum of the afore-calculated sizes of all items within a single line.
                    let item_main_size_sum = line
                        .items
                        .iter_mut()
                        .map(|item| {
                            let flex_fraction = item.content_flex_fraction;
                            // let flex_fraction = line_flex_fraction;

                            let flex_contribution = if item.content_flex_fraction > 0.0 {
                                f32_max(1.0, item.flex_grow) * flex_fraction
                            } else if item.content_flex_fraction < 0.0 {
                                let scaled_shrink_factor = f32_max(1.0, item.flex_shrink) * item.inner_flex_basis;
                                scaled_shrink_factor * flex_fraction
                            } else {
                                0.0
                            };
                            let size = item.flex_basis + flex_contribution;
                            item.outer_target_size.set_main(constants.dir, size);
                            item.target_size.set_main(constants.dir, size);
                            size
                        })
                        .sum::<f32>();

                    let gap_sum = sum_axis_gaps(constants.gap.main(constants.dir), line.items.len());
                    main_size = f32_max(main_size, item_main_size_sum + gap_sum)
                }

                main_size + main_content_box_inset
            }
        }
    });

    let outer_main_size = outer_main_size
        .maybe_clamp(constants.min_size.main(constants.dir), constants.max_size.main(constants.dir))
        .max(main_content_box_inset - constants.scrollbar_gutter.main(constants.dir));

    // let outer_main_size = inner_main_size + constants.padding_border.main_axis_sum(constants.dir);
    let inner_main_size = f32_max(outer_main_size - main_content_box_inset, 0.0);
    constants.container_size.set_main(constants.dir, outer_main_size);
    constants.inner_container_size.set_main(constants.dir, inner_main_size);
    constants.node_inner_size.set_main(constants.dir, Some(inner_main_size));
}

/// Resolve the flexible lengths of the items within a flex line.
/// Sets the `main` component of each item's `target_size` and `outer_target_size`
///
/// # [9.7. Resolving Flexible Lengths](https://www.w3.org/TR/css-flexbox-1/#resolve-flexible-lengths)
#[inline]
fn resolve_flexible_lengths(line: &mut FlexLine, constants: &AlgoConstants) {
    let total_main_axis_gap = sum_axis_gaps(constants.gap.main(constants.dir), line.items.len());

    // 1. Determine the used flex factor. Sum the outer hypothetical main sizes of all
    //    items on the line. If the sum is less than the flex container’s inner main size,
    //    use the flex grow factor for the rest of this algorithm; otherwise, use the
    //    flex shrink factor.

    let total_hypothetical_outer_main_size =
        line.items.iter().map(|child| child.hypothetical_outer_size.main(constants.dir)).sum::<f32>();
    let used_flex_factor: f32 = total_main_axis_gap + total_hypothetical_outer_main_size;
    let growing = used_flex_factor < constants.node_inner_size.main(constants.dir).unwrap_or(0.0);
    let shrinking = used_flex_factor > constants.node_inner_size.main(constants.dir).unwrap_or(0.0);
    let exactly_sized = !growing & !shrinking;

    // 2. Size inflexible items. Freeze, setting its target main size to its hypothetical main size
    //    - Any item that has a flex factor of zero
    //    - If using the flex grow factor: any item that has a flex base size
    //      greater than its hypothetical main size
    //    - If using the flex shrink factor: any item that has a flex base size
    //      smaller than its hypothetical main size

    for child in line.items.iter_mut() {
        let inner_target_size = child.hypothetical_inner_size.main(constants.dir);
        child.target_size.set_main(constants.dir, inner_target_size);

        if exactly_sized
            || (child.flex_grow == 0.0 && child.flex_shrink == 0.0)
            || (growing && child.flex_basis > child.hypothetical_inner_size.main(constants.dir))
            || (shrinking && child.flex_basis < child.hypothetical_inner_size.main(constants.dir))
        {
            child.frozen = true;
            let outer_target_size = inner_target_size + child.margin.main_axis_sum(constants.dir);
            child.outer_target_size.set_main(constants.dir, outer_target_size);
        }
    }

    if exactly_sized {
        return;
    }

    // 3. Calculate initial free space. Sum the outer sizes of all items on the line,
    //    and subtract this from the flex container’s inner main size. For frozen items,
    //    use their outer target main size; for other items, use their outer flex base size.

    let used_space: f32 = total_main_axis_gap
        + line
            .items
            .iter()
            .map(|child| {
                if child.frozen {
                    child.outer_target_size.main(constants.dir)
                } else {
                    child.flex_basis + child.margin.main_axis_sum(constants.dir)
                }
            })
            .sum::<f32>();

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

        let used_space: f32 = total_main_axis_gap
            + line
                .items
                .iter()
                .map(|child| {
                    if child.frozen {
                        child.outer_target_size.main(constants.dir)
                    } else {
                        child.flex_basis + child.margin.main_axis_sum(constants.dir)
                    }
                })
                .sum::<f32>();

        let mut unfrozen: Vec<&mut FlexItem> = line.items.iter_mut().filter(|child| !child.frozen).collect();

        let (sum_flex_grow, sum_flex_shrink): (f32, f32) =
            unfrozen.iter().fold((0.0, 0.0), |(flex_grow, flex_shrink), item| {
                (flex_grow + item.flex_grow, flex_shrink + item.flex_shrink)
            });

        let free_space = if growing && sum_flex_grow < 1.0 {
            (initial_free_space * sum_flex_grow - total_main_axis_gap)
                .maybe_min(constants.node_inner_size.main(constants.dir).maybe_sub(used_space))
        } else if shrinking && sum_flex_shrink < 1.0 {
            (initial_free_space * sum_flex_shrink - total_main_axis_gap)
                .maybe_max(constants.node_inner_size.main(constants.dir).maybe_sub(used_space))
        } else {
            (constants.node_inner_size.main(constants.dir).maybe_sub(used_space))
                .unwrap_or(used_flex_factor - used_space)
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
                    child
                        .target_size
                        .set_main(constants.dir, child.flex_basis + free_space * (child.flex_grow / sum_flex_grow));
                }
            } else if shrinking && sum_flex_shrink > 0.0 {
                let sum_scaled_shrink_factor: f32 =
                    unfrozen.iter().map(|child| child.inner_flex_basis * child.flex_shrink).sum();

                if sum_scaled_shrink_factor > 0.0 {
                    for child in &mut unfrozen {
                        let scaled_shrink_factor = child.inner_flex_basis * child.flex_shrink;
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
            let resolved_min_main: Option<f32> = child.resolved_minimum_main_size.into();
            let max_main = child.max_size.main(constants.dir);
            let clamped = child.target_size.main(constants.dir).maybe_clamp(resolved_min_main, max_main).max(0.0);
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
///   by performing layout with the used main size and the available space, treating auto as fit-content.
#[inline]
fn determine_hypothetical_cross_size(
    tree: &mut impl LayoutFlexboxContainer,
    line: &mut FlexLine,
    constants: &AlgoConstants,
    available_space: Size<AvailableSpace>,
) {
    for child in line.items.iter_mut() {
        let padding_border_sum = (child.padding + child.border).cross_axis_sum(constants.dir);

        let child_known_main = constants.container_size.main(constants.dir).into();

        // Sizes transferred through the aspect ratio clamp the hypothetical cross size
        // https://github.com/w3c/csswg-drafts/issues/10997
        let transferred_min_cross = child.min_size.maybe_apply_aspect_ratio(child.aspect_ratio).cross(constants.dir);
        let transferred_max_cross = child.max_size.maybe_apply_aspect_ratio(child.aspect_ratio).cross(constants.dir);

        let child_cross = child
            .size
            .cross(constants.dir)
            .maybe_clamp(transferred_min_cross, transferred_max_cross)
            .maybe_max(padding_border_sum);

        let child_available_cross = available_space
            .cross(constants.dir)
            .map_definite_value(|val| constants.divided_cross_space(val))
            .maybe_clamp(transferred_min_cross, transferred_max_cross)
            .maybe_max(padding_border_sum);

        // A cross size that is a sizing keyword (min-content, max-content, fit-content,
        // fit-content(...)) determines the available space constraint the item is measured under.
        // The `stretch` keyword is not resolved here: it stretches to the flex line, which is
        // handled in `determine_used_cross_size`
        let cross_stretch_size = constants
            .node_inner_size
            .cross(constants.dir)
            .map(|val| constants.divided_cross_space(val))
            .maybe_sub(child.margin.cross_axis_sum(constants.dir));
        let child_available_cross = match resolve_sizing_keyword(
            child.size_style.cross(constants.dir),
            cross_stretch_size,
            constants.node_inner_size.cross(constants.dir),
        ) {
            Some(SizingKeywordResolution::Measure(available)) => available,
            _ => child_available_cross,
        };

        let child_inner_cross = child_cross.unwrap_or_else(|| {
            tree.compute_child_layout(
                child.node,
                LayoutInput {
                    run_mode: RunMode::ComputeSize,
                    sizing_mode: SizingMode::ContentSize,
                    axis: constants.dir.cross_axis().into(),
                    known_dimensions: Size {
                        width: if constants.is_row { child.target_size.width.into() } else { child_cross },
                        height: if constants.is_row { child_cross } else { child.target_size.height.into() },
                    },
                    known_dimensions_are_definite: item_definiteness(constants, child),
                    parent_size: constants.node_inner_size,
                    available_space: Size {
                        width: if constants.is_row { child_known_main } else { child_available_cross },
                        height: if constants.is_row { child_available_cross } else { child_known_main },
                    },
                    vertical_margins_are_collapsible: Line::FALSE,
                },
            )
            .size
            .get_abs(constants.dir.cross_axis())
            .maybe_clamp(transferred_min_cross, transferred_max_cross)
            .max(padding_border_sum)
        });
        let child_outer_cross = child_inner_cross + child.margin.cross_axis_sum(constants.dir);

        child.hypothetical_inner_size.set_cross(constants.dir, child_inner_cross);
        child.hypothetical_outer_size.set_cross(constants.dir, child_outer_cross);
    }
}

/// Calculate the base lines of the children.
#[inline]
fn calculate_children_base_lines(
    tree: &mut impl LayoutFlexboxContainer,
    node_size: Size<Option<f32>>,
    available_space: Size<AvailableSpace>,
    flex_lines: &mut [FlexLine],
    constants: &AlgoConstants,
) {
    // Only compute baselines for flex rows because we only support baseline alignment in the cross axis
    // where that axis is also the inline axis
    // TODO: this may need revisiting if/when we support vertical writing modes
    if !constants.is_row {
        return;
    }

    for line in flex_lines {
        // If a flex line has one or zero items participating in baseline alignment then baseline alignment is a no-op so we skip
        let line_baseline_child_count =
            line.items.iter().filter(|child| child.participates_in_baseline_alignment(constants.dir)).count();
        if line_baseline_child_count <= 1 {
            continue;
        }

        for child in line.items.iter_mut() {
            // Only calculate baselines for children participating in baseline alignment
            if !child.participates_in_baseline_alignment(constants.dir) {
                continue;
            }

            let measured_size_and_baselines = tree.compute_child_layout(
                child.node,
                LayoutInput {
                    run_mode: RunMode::PerformLayout,
                    sizing_mode: SizingMode::ContentSize,
                    axis: RequestedAxis::Both,
                    known_dimensions: Size {
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
                    known_dimensions_are_definite: item_definiteness(constants, child),
                    parent_size: constants.node_inner_size,
                    available_space: Size {
                        width: if constants.is_row {
                            constants.container_size.width.into()
                        } else {
                            available_space.width.maybe_set(node_size.width)
                        },
                        height: if constants.is_row {
                            available_space.height.maybe_set(node_size.height)
                        } else {
                            constants.container_size.height.into()
                        },
                    },
                    vertical_margins_are_collapsible: Line::FALSE,
                },
            );

            let baseline = measured_size_and_baselines.baselines.first;
            let height = measured_size_and_baselines.size.height;

            // Scroll containers' baselines are determined from their content as if scrolled to the
            // initial position, but are additionally clamped to their border box.
            // See https://github.com/w3c/csswg-drafts/issues/7660
            let baseline = if child.overflow.y.is_scroll_container() {
                baseline.unwrap_or(height).min(height).max(0.0)
            } else {
                baseline.unwrap_or(height)
            };

            child.baseline = baseline + child.margin.top;
        }
    }
}

/// Calculate the cross size of each flex line.
///
/// # [9.4. Cross Size Determination](https://www.w3.org/TR/css-flexbox-1/#cross-sizing)
///
/// - [**Calculate the cross size of each flex line**](https://www.w3.org/TR/css-flexbox-1/#algo-cross-line).
#[inline]
fn calculate_cross_size(flex_lines: &mut [FlexLine], node_size: Size<Option<f32>>, constants: &AlgoConstants) {
    // If the flex container is single-line and has a definite cross size,
    // the cross size of the flex line is the flex container’s inner cross size.
    if !constants.is_wrap && node_size.cross(constants.dir).is_some() {
        let cross_axis_padding_border = constants.content_box_inset.cross_axis_sum(constants.dir);
        let cross_min_size = constants.min_size.cross(constants.dir);
        let cross_max_size = constants.max_size.cross(constants.dir);
        flex_lines[0].cross_size = node_size
            .cross(constants.dir)
            .maybe_clamp(cross_min_size, cross_max_size)
            .maybe_sub(cross_axis_padding_border)
            .maybe_max(0.0)
            .unwrap_or(0.0);
    } else {
        // Otherwise, for each flex line:
        //
        //    1. Collect all the flex items whose inline-axis is parallel to the main-axis, whose
        //       align-self is baseline, and whose cross-axis margins are both non-auto. Find the
        //       largest of the distances between each item’s baseline and its hypothetical outer
        //       cross-start edge, and the largest of the distances between each item’s baseline
        //       and its hypothetical outer cross-end edge, and sum these two values.

        //    2. Among all the items not collected by the previous step, find the largest
        //       outer hypothetical cross size.

        //    3. The used cross-size of the flex line is the largest of the numbers found in the
        //       previous two steps and zero.
        for line in flex_lines.iter_mut() {
            let max_baseline: f32 = line.items.iter().map(|child| child.baseline).fold(0.0, |acc, x| acc.max(x));
            line.cross_size = line
                .items
                .iter()
                .map(|child| {
                    if child.participates_in_baseline_alignment(constants.dir) {
                        max_baseline - child.baseline + child.hypothetical_outer_size.cross(constants.dir)
                    } else {
                        child.hypothetical_outer_size.cross(constants.dir)
                    }
                })
                .fold(0.0, |acc, x| acc.max(x));
        }

        // If the flex container is single-line, then clamp the line’s cross-size to be within the container’s computed min and max cross sizes.
        // Note that if CSS 2.1’s definition of min/max-width/height applied more generally, this behavior would fall out automatically.
        if !constants.is_wrap {
            let cross_axis_padding_border = constants.content_box_inset.cross_axis_sum(constants.dir);
            let cross_min_size = constants.min_size.cross(constants.dir);
            let cross_max_size = constants.max_size.cross(constants.dir);
            flex_lines[0].cross_size = flex_lines[0].cross_size.maybe_clamp(
                cross_min_size.maybe_sub(cross_axis_padding_border),
                cross_max_size.maybe_sub(cross_axis_padding_border),
            );
        }
    }
}

/// Handle 'align-content: stretch'.
///
/// # [9.4. Cross Size Determination](https://www.w3.org/TR/css-flexbox-1/#cross-sizing)
///
/// - [**Handle 'align-content: stretch'**](https://www.w3.org/TR/css-flexbox-1/#algo-line-stretch). If the flex container has a definite cross size, align-content is stretch,
///   and the sum of the flex lines' cross sizes is less than the flex container’s inner cross size,
///   increase the cross size of each flex line by equal amounts such that the sum of their cross sizes exactly equals the flex container’s inner cross size.
#[inline]
fn handle_align_content_stretch(flex_lines: &mut [FlexLine], node_size: Size<Option<f32>>, constants: &AlgoConstants) {
    if constants.align_content == AlignContent::STRETCH {
        let cross_axis_padding_border = constants.content_box_inset.cross_axis_sum(constants.dir);
        let cross_min_size = constants.min_size.cross(constants.dir);
        let cross_max_size = constants.max_size.cross(constants.dir);
        let container_min_inner_cross = node_size
            .cross(constants.dir)
            .or(cross_min_size)
            .maybe_clamp(cross_min_size, cross_max_size)
            .maybe_sub(cross_axis_padding_border)
            .maybe_max(0.0)
            .unwrap_or(0.0);

        let total_cross_axis_gap = sum_axis_gaps(constants.gap.cross(constants.dir), flex_lines.len());
        let lines_total_cross: f32 = flex_lines.iter().map(|line| line.cross_size).sum::<f32>() + total_cross_axis_gap;

        if lines_total_cross < container_min_inner_cross {
            let remaining = container_min_inner_cross - lines_total_cross;
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
///   and neither of its cross-axis margins are auto, the used outer cross size is the used cross size of its flex line, clamped according to the item’s used min and max cross sizes.
///   Otherwise, the used cross size is the item’s hypothetical cross size.
///
///   If the flex item has align-self: stretch, redo layout for its contents, treating this used size as its definite cross size so that percentage-sized children can be resolved.
///
///   **Note that this step does not affect the main size of the flex item, even if it has an intrinsic aspect ratio**.
#[inline]
fn determine_used_cross_size(
    tree: &impl LayoutFlexboxContainer,
    flex_lines: &mut [FlexLine],
    constants: &AlgoConstants,
) {
    for line in flex_lines {
        let line_cross_size = line.cross_size;

        for child in line.items.iter_mut() {
            let child_style = tree.get_flexbox_child_style(child.node);
            // A cross size of `stretch` stretches to the flex line like align-self: stretch
            // (but regardless of the alignment style)
            let cross_is_stretch = child.size_style.cross(constants.dir).is_stretch();
            child.target_size.set_cross(
                constants.dir,
                if !child.margin_is_auto.cross_start(constants.dir)
                    && !child.margin_is_auto.cross_end(constants.dir)
                    && (cross_is_stretch
                        || (child.align_self == AlignSelf::STRETCH
                            && child_style.size().cross(constants.dir).is_auto()))
                {
                    // For some reason this particular usage of max_width is an exception to the rule that max_width's transfer
                    // using the aspect_ratio (if set). Both Chrome and Firefox agree on this. And reading the spec, it seems like
                    // a reasonable interpretation. Although it seems to me that the spec *should* apply aspect_ratio here.
                    let padding = child_style
                        .padding()
                        .resolve_or_zero(constants.node_inner_size, |val, basis| tree.calc(val, basis));
                    let border = child_style
                        .border()
                        .resolve_or_zero(constants.node_inner_size, |val, basis| tree.calc(val, basis));
                    let pb_sum = (padding + border).sum_axes();
                    let box_sizing_adjustment =
                        if child_style.box_sizing() == BoxSizing::ContentBox { pb_sum } else { Size::ZERO };

                    let max_size_ignoring_aspect_ratio = child_style
                        .max_size()
                        .maybe_resolve(constants.node_inner_size, |val, basis| tree.calc(val, basis))
                        .maybe_add(box_sizing_adjustment);

                    (line_cross_size - child.margin.cross_axis_sum(constants.dir)).maybe_clamp(
                        child.min_size.cross(constants.dir),
                        max_size_ignoring_aspect_ratio.cross(constants.dir),
                    )
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
///   1. If the remaining free space is positive and at least one main-axis margin on this line is `auto`, distribute the free space equally among these margins.
///      Otherwise, set all `auto` margins to zero.
///
///   2. Align the items along the main-axis per `justify-content`.
#[inline]
fn distribute_remaining_free_space(flex_lines: &mut [FlexLine], constants: &AlgoConstants) {
    for line in flex_lines {
        let total_main_axis_gap = sum_axis_gaps(constants.gap.main(constants.dir), line.items.len());
        let used_space: f32 = total_main_axis_gap
            + line.items.iter().map(|child| child.outer_target_size.main(constants.dir)).sum::<f32>();
        let mut free_space = constants.inner_container_size.main(constants.dir) - used_space;
        let mut num_auto_margins = 0;

        for child in line.items.iter_mut() {
            if child.margin_is_auto.main_start(constants.dir) {
                num_auto_margins += 1;
            }
            if child.margin_is_auto.main_end(constants.dir) {
                num_auto_margins += 1;
            }
        }

        if free_space > 0.0 && num_auto_margins > 0 {
            let margin = free_space / num_auto_margins as f32;

            for child in line.items.iter_mut() {
                if child.margin_is_auto.main_start(constants.dir) {
                    if constants.is_row {
                        child.margin.left = margin;
                    } else {
                        child.margin.top = margin;
                    }
                }
                if child.margin_is_auto.main_end(constants.dir) {
                    if constants.is_row {
                        child.margin.right = margin;
                    } else {
                        child.margin.bottom = margin;
                    }
                }
            }

            // The auto margins have absorbed all of the free space, leaving none for `justify-content`
            free_space = 0.0;
        }

        let num_items = line.items.len();
        let layout_reverse = constants.dir.is_reverse();
        let gap = constants.gap.main(constants.dir);
        let raw_justify_content_mode = constants.justify_content.unwrap_or(JustifyContent::FLEX_START);
        let justify_content_mode = apply_alignment_fallback(free_space, num_items, raw_justify_content_mode);

        let justify_item = |(i, child): (usize, &mut FlexItem)| {
            child.offset_main =
                compute_alignment_offset(free_space, num_items, gap, justify_content_mode, layout_reverse, i == 0);
        };

        if layout_reverse {
            line.items.iter_mut().rev().enumerate().for_each(justify_item);
        } else {
            line.items.iter_mut().enumerate().for_each(justify_item);
        }
    }
}

/// Resolve cross-axis `auto` margins.
///
/// # [9.6. Cross-Axis Alignment](https://www.w3.org/TR/css-flexbox-1/#cross-alignment)
///
/// - [**Resolve cross-axis `auto` margins**](https://www.w3.org/TR/css-flexbox-1/#algo-cross-margins).
///   If a flex item has auto cross-axis margins:
///
///   - If its outer cross size (treating those auto margins as zero) is less than the cross size of its flex line,
///     distribute the difference in those sizes equally to the auto margins.
///
///   - Otherwise, if the block-start or inline-start margin (whichever is in the cross axis) is auto, set it to zero.
///     Set the opposite margin so that the outer cross size of the item equals the cross size of its flex line.
#[inline]
fn resolve_cross_axis_auto_margins(flex_lines: &mut [FlexLine], constants: &AlgoConstants) {
    for line in flex_lines {
        let line_cross_size = line.cross_size;
        let max_baseline: f32 = line.items.iter_mut().map(|child| child.baseline).fold(0.0, |acc, x| acc.max(x));
        let max_baseline_to_bottom_distance: f32 = line
            .items
            .iter_mut()
            .filter(|child| child.participates_in_baseline_alignment(constants.dir))
            .map(|child| child.outer_target_size.cross(constants.dir) - child.baseline)
            .fold(0.0, |acc, x| acc.max(x));

        for child in line.items.iter_mut() {
            let free_space = line_cross_size - child.outer_target_size.cross(constants.dir);

            if child.margin_is_auto.cross_start(constants.dir) && child.margin_is_auto.cross_end(constants.dir) {
                if constants.is_row {
                    child.margin.top = free_space / 2.0;
                    child.margin.bottom = free_space / 2.0;
                } else {
                    child.margin.left = free_space / 2.0;
                    child.margin.right = free_space / 2.0;
                }
            } else if child.margin_is_auto.cross_start(constants.dir) {
                if constants.is_row {
                    child.margin.top = free_space;
                } else {
                    child.margin.left = free_space;
                }
            } else if child.margin_is_auto.cross_end(constants.dir) {
                if constants.is_row {
                    child.margin.bottom = free_space;
                } else {
                    child.margin.right = free_space;
                }
            } else {
                // 14. Align all flex items along the cross-axis.
                child.offset_cross = align_flex_items_along_cross_axis(
                    child,
                    free_space,
                    max_baseline,
                    max_baseline_to_bottom_distance,
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
///   if neither of the item's cross-axis margins are `auto`.
#[inline]
fn align_flex_items_along_cross_axis(
    child: &FlexItem,
    free_space: f32,
    max_baseline: f32,
    max_baseline_to_bottom_distance: f32,
    constants: &AlgoConstants,
) -> f32 {
    let cross_axis_should_reverse = constants.is_column && matches!(constants.layout_direction, Direction::Rtl);

    // If align-self uses a "safe" overflow-position keyword and the item would overflow its
    // line cross size, fall back to logical Start to avoid data loss. See CSS Box Alignment 3
    // §4.3 <https://www.w3.org/TR/css-align-3/#overflow-values>. Otherwise, drop the safety
    // field so the match below operates on a bare keyword and stays exhaustive.
    let align_keyword = if child.align_self.is_safe() && free_space < 0.0 {
        AlignItemsKeyword::Start
    } else {
        child.align_self.keyword
    };

    match align_keyword {
        AlignItemsKeyword::Start => {
            if cross_axis_should_reverse {
                free_space
            } else {
                0.0
            }
        }
        AlignItemsKeyword::FlexStart => {
            if constants.is_wrap_reverse ^ cross_axis_should_reverse {
                free_space
            } else {
                0.0
            }
        }
        AlignItemsKeyword::End => {
            if cross_axis_should_reverse {
                0.0
            } else {
                free_space
            }
        }
        AlignItemsKeyword::FlexEnd => {
            if constants.is_wrap_reverse ^ cross_axis_should_reverse {
                0.0
            } else {
                free_space
            }
        }
        AlignItemsKeyword::Center => free_space / 2.0,
        AlignItemsKeyword::Baseline => {
            if constants.is_row {
                if constants.is_wrap_reverse {
                    // In a wrap-reverse container the cross axis is flipped, so the baseline-aligned
                    // group of items is aligned to the cross-start edge, which is the bottom of the line.
                    let line_cross_size = free_space + child.outer_target_size.cross(constants.dir);
                    line_cross_size - max_baseline_to_bottom_distance - child.baseline
                } else {
                    max_baseline - child.baseline
                }
            } else {
                // Until we support vertical writing modes, baseline alignment only makes sense if
                // the constants.direction is row, so we treat it as flex-start alignment in columns.
                let baseline_column_should_reverse = cross_axis_should_reverse && !constants.is_wrap;
                if constants.is_wrap_reverse ^ baseline_column_should_reverse {
                    free_space
                } else {
                    0.0
                }
            }
        }
        AlignItemsKeyword::Stretch => {
            if constants.is_wrap_reverse ^ cross_axis_should_reverse {
                free_space
            } else {
                0.0
            }
        }
        // SelfStart/SelfEnd are resolved to Start/End against the item's own direction when
        // flex items are generated.
        AlignItemsKeyword::SelfStart | AlignItemsKeyword::SelfEnd => unreachable!(),
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
    flex_lines: &[FlexLine],
    node_size: Size<Option<f32>>,
    constants: &mut AlgoConstants,
) -> f32 {
    let total_cross_axis_gap = sum_axis_gaps(constants.gap.cross(constants.dir), flex_lines.len());
    let total_line_cross_size: f32 = flex_lines.iter().map(|line| line.cross_size).sum::<f32>();

    let padding_border_sum = constants.content_box_inset.cross_axis_sum(constants.dir);
    let cross_scrollbar_gutter = constants.scrollbar_gutter.cross(constants.dir);
    let min_cross_size = constants.min_size.cross(constants.dir);
    let max_cross_size = constants.max_size.cross(constants.dir);
    let outer_container_size = node_size
        .cross(constants.dir)
        .unwrap_or(total_line_cross_size + total_cross_axis_gap + padding_border_sum)
        .maybe_clamp(min_cross_size, max_cross_size)
        .max(padding_border_sum - cross_scrollbar_gutter);
    let inner_container_size = f32_max(outer_container_size - padding_border_sum, 0.0);

    constants.container_size.set_cross(constants.dir, outer_container_size);
    constants.inner_container_size.set_cross(constants.dir, inner_container_size);

    total_line_cross_size
}

/// Align all flex lines per `align-content`.
///
/// # [9.6. Cross-Axis Alignment](https://www.w3.org/TR/css-flexbox-1/#cross-alignment)
///
/// - [**Align all flex lines**](https://www.w3.org/TR/css-flexbox-1/#algo-line-align) per `align-content`.
#[inline]
fn align_flex_lines_per_align_content(flex_lines: &mut [FlexLine], constants: &AlgoConstants, total_cross_size: f32) {
    let num_lines = flex_lines.len();
    let gap = constants.gap.cross(constants.dir);
    let total_cross_axis_gap = sum_axis_gaps(gap, num_lines);
    let free_space = constants.inner_container_size.cross(constants.dir) - total_cross_size - total_cross_axis_gap;

    let align_content_mode = apply_alignment_fallback(free_space, num_lines, constants.align_content);

    let align_line = |(i, line): (usize, &mut FlexLine)| {
        line.offset_cross =
            compute_alignment_offset(free_space, num_lines, gap, align_content_mode, constants.is_wrap_reverse, i == 0);
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
    tree: &mut impl LayoutFlexboxContainer,
    item: &mut FlexItem,
    total_offset_main: &mut f32,
    total_offset_cross: f32,
    line_offset_cross: f32,
    #[cfg(feature = "content_size")] total_content_size: &mut Size<f32>,
    #[cfg(feature = "content_size")] border: Rect<f32>,
    container_size: Size<f32>,
    node_inner_size: Size<Option<f32>>,
    has_definite_main_size: bool,
    direction: FlexDirection,
    layout_direction: Direction,
) {
    let item_definiteness =
        Size { width: true, height: true }.with_main(direction, has_definite_main_size || item.flex_basis_is_definite);
    let layout_output = tree.compute_child_layout(
        item.node,
        LayoutInput {
            run_mode: RunMode::PerformLayout,
            sizing_mode: SizingMode::ContentSize,
            axis: RequestedAxis::Both,
            known_dimensions: item.target_size.map(|s| s.into()),
            known_dimensions_are_definite: item_definiteness,
            parent_size: node_inner_size,
            available_space: container_size.map(|s| s.into()),
            vertical_margins_are_collapsible: Line::FALSE,
        },
    );
    let LayoutOutput {
        size,
        #[cfg(feature = "content_size")]
        content_size,
        ..
    } = layout_output;

    let is_rtl_row = direction.is_row() && layout_direction.is_rtl();
    let is_rtl_column = direction.is_column() && layout_direction.is_rtl();
    let main_relative_inset = if is_rtl_row {
        item.inset.main_end(direction).or(item.inset.main_start(direction).map(|pos| -pos)).unwrap_or(0.0)
    } else {
        item.inset.main_start(direction).or(item.inset.main_end(direction).map(|pos| -pos)).unwrap_or(0.0)
    };
    let cross_relative_inset = if is_rtl_column {
        item.inset.cross_end(direction).map(|pos| -pos).or(item.inset.cross_start(direction)).unwrap_or(0.0)
    } else {
        item.inset.cross_start(direction).or(item.inset.cross_end(direction).map(|pos| -pos)).unwrap_or(0.0)
    };
    let effective_line_offset_cross = if is_rtl_column { 0.0 } else { line_offset_cross };

    let offset_main = if is_rtl_row {
        *total_offset_main - item.offset_main - item.margin.main_end(direction) - main_relative_inset - size.width
    } else {
        *total_offset_main + item.offset_main + item.margin.main_start(direction) + main_relative_inset
    };

    let offset_cross = total_offset_cross
        + item.offset_cross
        + effective_line_offset_cross
        + item.margin.cross_start(direction)
        + cross_relative_inset;

    if direction.is_row() {
        let baseline_offset_cross =
            total_offset_cross + item.offset_cross + effective_line_offset_cross + item.margin.cross_start(direction);
        // Scroll containers' baselines are determined from their content as if scrolled to the initial
        // position, but are additionally clamped to their border box.
        // See https://github.com/w3c/csswg-drafts/issues/7660
        let inner_baseline = {
            let baseline = layout_output.baselines.first.unwrap_or(size.height);
            if item.overflow.y.is_scroll_container() {
                baseline.min(size.height).max(0.0)
            } else {
                baseline
            }
        };
        item.baseline = baseline_offset_cross + inner_baseline;
    } else {
        let baseline_offset_main = *total_offset_main + item.offset_main + item.margin.main_start(direction);
        let inner_baseline = layout_output.baselines.first.unwrap_or(size.height);
        item.baseline = baseline_offset_main + inner_baseline;
    }

    let location = if direction.is_row() {
        Point { x: offset_main, y: offset_cross }
    } else {
        Point { x: offset_cross, y: offset_main }
    };
    let scrollbar_size = Size {
        width: if item.overflow.y == Overflow::Scroll { item.scrollbar_width } else { 0.0 },
        height: if item.overflow.x == Overflow::Scroll { item.scrollbar_width } else { 0.0 },
    };

    tree.set_unrounded_layout(
        item.node,
        &Layout {
            order: item.order,
            size,
            #[cfg(feature = "content_size")]
            content_size,
            scrollbar_size,
            location,
            padding: item.padding,
            border: item.border,
            margin: item.margin,
        },
    );

    if is_rtl_row {
        *total_offset_main -= item.offset_main + item.margin.main_axis_sum(direction) + size.main(direction);
    } else {
        *total_offset_main += item.offset_main + item.margin.main_axis_sum(direction) + size.main(direction);
    }

    #[cfg(feature = "content_size")]
    {
        let contribution_location = if layout_direction.is_rtl() {
            Point { x: container_size.width - (location.x + size.width) - border.right, y: location.y - border.top }
        } else {
            Point { x: location.x - border.left, y: location.y - border.top }
        };
        *total_content_size = total_content_size.f32_max(compute_content_size_contribution(
            contribution_location,
            size,
            content_size,
            item.overflow,
        ));
    }
}

/// Calculates the layout line
#[allow(clippy::too_many_arguments)]
fn calculate_layout_line(
    tree: &mut impl LayoutFlexboxContainer,
    line: &mut FlexLine,
    total_offset_cross: &mut f32,
    #[cfg(feature = "content_size")] content_size: &mut Size<f32>,
    #[cfg(feature = "content_size")] border: Rect<f32>,
    container_size: Size<f32>,
    node_inner_size: Size<Option<f32>>,
    has_definite_main_size: bool,
    padding_border: Rect<f32>,
    direction: FlexDirection,
    layout_direction: Direction,
) {
    let mut total_offset_main = if layout_direction.is_rtl() && direction.is_row() {
        container_size.width - padding_border.main_end(direction)
    } else {
        padding_border.main_start(direction)
    };
    let line_offset_cross = line.offset_cross;

    let is_rtl_column = layout_direction.is_rtl() && direction.is_column();
    if is_rtl_column {
        *total_offset_cross -= line_offset_cross + line.cross_size;
    }

    if direction.is_reverse() {
        for item in line.items.iter_mut().rev() {
            calculate_flex_item(
                tree,
                item,
                &mut total_offset_main,
                *total_offset_cross,
                line_offset_cross,
                #[cfg(feature = "content_size")]
                content_size,
                #[cfg(feature = "content_size")]
                border,
                container_size,
                node_inner_size,
                has_definite_main_size,
                direction,
                layout_direction,
            );
        }
    } else {
        for item in line.items.iter_mut() {
            calculate_flex_item(
                tree,
                item,
                &mut total_offset_main,
                *total_offset_cross,
                line_offset_cross,
                #[cfg(feature = "content_size")]
                content_size,
                #[cfg(feature = "content_size")]
                border,
                container_size,
                node_inner_size,
                has_definite_main_size,
                direction,
                layout_direction,
            );
        }
    }

    if !is_rtl_column {
        *total_offset_cross += line_offset_cross + line.cross_size;
    }
}

/// Do a final layout pass and collect the resulting layouts.
#[inline]
fn final_layout_pass(
    tree: &mut impl LayoutFlexboxContainer,
    flex_lines: &mut [FlexLine],
    constants: &AlgoConstants,
) -> Size<f32> {
    let mut total_offset_cross = if constants.is_column && constants.layout_direction.is_rtl() {
        constants.container_size.width - constants.content_box_inset.cross_end(constants.dir)
    } else {
        constants.content_box_inset.cross_start(constants.dir)
    };

    #[cfg_attr(not(feature = "content_size"), allow(unused_mut))]
    let mut content_size = Size::ZERO;

    if constants.is_wrap_reverse {
        for line in flex_lines.iter_mut().rev() {
            calculate_layout_line(
                tree,
                line,
                &mut total_offset_cross,
                #[cfg(feature = "content_size")]
                &mut content_size,
                #[cfg(feature = "content_size")]
                constants.border,
                constants.container_size,
                constants.node_inner_size,
                constants.has_definite_main_size,
                constants.content_box_inset,
                constants.dir,
                constants.layout_direction,
            );
        }
    } else {
        for line in flex_lines.iter_mut() {
            calculate_layout_line(
                tree,
                line,
                &mut total_offset_cross,
                #[cfg(feature = "content_size")]
                &mut content_size,
                #[cfg(feature = "content_size")]
                constants.border,
                constants.container_size,
                constants.node_inner_size,
                constants.has_definite_main_size,
                constants.content_box_inset,
                constants.dir,
                constants.layout_direction,
            );
        }
    }

    // A scroll container's own padding at the end of the content is part of its scrollable
    // overflow region, so it is included in the content size. Boxes that are not scroll
    // containers do not extend their overflow region by their own padding.
    #[cfg(feature = "content_size")]
    if constants.is_scroll_container {
        content_size.width += if constants.layout_direction.is_rtl() {
            constants.content_box_inset.left - constants.border.left - constants.scrollbar_gutter.x
        } else {
            constants.content_box_inset.right - constants.border.right - constants.scrollbar_gutter.x
        };
        content_size.height +=
            constants.content_box_inset.bottom - constants.border.bottom - constants.scrollbar_gutter.y;
    }

    content_size
}

/// Perform absolute layout on all absolutely positioned children.
#[inline]
fn perform_absolute_layout_on_absolute_children(
    tree: &mut impl LayoutFlexboxContainer,
    node: NodeId,
    constants: &AlgoConstants,
) -> Size<f32> {
    let container_width = constants.container_size.width;
    let container_height = constants.container_size.height;
    let inset_relative_size =
        constants.container_size - constants.border.sum_axes() - constants.scrollbar_gutter.into();

    #[cfg_attr(not(feature = "content_size"), allow(unused_mut))]
    let mut content_size = Size::ZERO;

    for order in 0..tree.child_count(node) {
        let child = tree.get_child_id(node, order);
        let child_style = tree.get_flexbox_child_style(child);

        // Skip items that are display:none or are not position:absolute
        if child_style.box_generation_mode() == BoxGenerationMode::None || child_style.position() != Position::Absolute
        {
            continue;
        }

        let overflow = child_style.overflow();
        let scrollbar_width = child_style.scrollbar_width();
        let aspect_ratio = child_style.aspect_ratio();
        let align_self = child_style.align_self().unwrap_or(constants.align_items).resolve_self_relative(
            child_style.direction(),
            constants.layout_direction,
            constants.is_column,
        );
        let margin = child_style
            .margin()
            .map(|margin| margin.resolve_to_option(inset_relative_size.width, |val, basis| tree.calc(val, basis)));
        let padding =
            child_style.padding().resolve_or_zero(Some(inset_relative_size.width), |val, basis| tree.calc(val, basis));
        let border =
            child_style.border().resolve_or_zero(Some(inset_relative_size.width), |val, basis| tree.calc(val, basis));
        let padding_border_sum = (padding + border).sum_axes();
        let box_sizing_adjustment =
            if child_style.box_sizing() == BoxSizing::ContentBox { padding_border_sum } else { Size::ZERO };

        // Resolve inset
        // Insets are resolved against the container size minus border
        let left =
            child_style.inset().left.maybe_resolve(inset_relative_size.width, |val, basis| tree.calc(val, basis));
        let right =
            child_style.inset().right.maybe_resolve(inset_relative_size.width, |val, basis| tree.calc(val, basis));
        let top = child_style.inset().top.maybe_resolve(inset_relative_size.height, |val, basis| tree.calc(val, basis));
        let bottom =
            child_style.inset().bottom.maybe_resolve(inset_relative_size.height, |val, basis| tree.calc(val, basis));

        // Compute known dimensions from min/max/inherent size styles
        let size_style = child_style.size();
        let style_size = size_style
            .maybe_resolve(inset_relative_size, |val, basis| tree.calc(val, basis))
            .maybe_apply_aspect_ratio(aspect_ratio)
            .maybe_add(box_sizing_adjustment);
        let min_size = child_style
            .min_size()
            .maybe_resolve(inset_relative_size, |val, basis| tree.calc(val, basis))
            .maybe_apply_aspect_ratio(aspect_ratio)
            .maybe_add(box_sizing_adjustment)
            .or(padding_border_sum.map(Some))
            .maybe_max(padding_border_sum);
        let max_size = child_style
            .max_size()
            .maybe_resolve(inset_relative_size, |val, basis| tree.calc(val, basis))
            .maybe_apply_aspect_ratio(aspect_ratio)
            .maybe_add(box_sizing_adjustment);
        let mut known_dimensions = style_size.maybe_clamp(min_size, max_size);

        drop(child_style);

        // Resolve any sizing keywords (min-content, max-content, fit-content, fit-content(...),
        // stretch) in the size styles. An explicitly sized axis takes precedence over the
        // inset-derived size below.
        if size_style.width.is_sizing_keyword() || size_style.height.is_sizing_keyword() {
            resolve_absolute_sizing_keywords(
                tree,
                child,
                &mut known_dimensions,
                size_style,
                inset_relative_size,
                Rect { left, right, top, bottom },
                margin,
                SizingMode::InherentSize,
            );
            known_dimensions = known_dimensions.maybe_apply_aspect_ratio(aspect_ratio).maybe_clamp(min_size, max_size);
        }

        // Fill in width from left/right and reapply aspect ratio if:
        //   - Width is not already known
        //   - Item has both left and right inset properties set
        if let (None, Some(left), Some(right)) = (known_dimensions.width, left, right) {
            let new_width_raw = inset_relative_size.width.maybe_sub(margin.left).maybe_sub(margin.right) - left - right;
            known_dimensions.width = Some(f32_max(new_width_raw, 0.0));
            known_dimensions = known_dimensions.maybe_apply_aspect_ratio(aspect_ratio).maybe_clamp(min_size, max_size);
        }

        // Fill in height from top/bottom and reapply aspect ratio if:
        //   - Height is not already known
        //   - Item has both top and bottom inset properties set
        if let (None, Some(top), Some(bottom)) = (known_dimensions.height, top, bottom) {
            let new_height_raw =
                inset_relative_size.height.maybe_sub(margin.top).maybe_sub(margin.bottom) - top - bottom;
            known_dimensions.height = Some(f32_max(new_height_raw, 0.0));
            known_dimensions = known_dimensions.maybe_apply_aspect_ratio(aspect_ratio).maybe_clamp(min_size, max_size);
        }
        let final_size = match (known_dimensions.width, known_dimensions.height) {
            (Some(width), Some(height)) => Size { width, height },
            _ => {
                let measured_size = tree.measure_child_size_both(
                    child,
                    known_dimensions,
                    constants.node_inner_size,
                    Size {
                        width: AvailableSpace::Definite(container_width.maybe_clamp(min_size.width, max_size.width)),
                        height: AvailableSpace::Definite(
                            container_height.maybe_clamp(min_size.height, max_size.height),
                        ),
                    },
                    SizingMode::InherentSize,
                    Line::FALSE,
                );
                known_dimensions.unwrap_or(measured_size)
            }
        }
        .maybe_clamp(min_size, max_size);

        let layout_output = tree.perform_child_layout(
            child,
            final_size.map(Some),
            constants.node_inner_size,
            Size {
                width: AvailableSpace::Definite(container_width.maybe_clamp(min_size.width, max_size.width)),
                height: AvailableSpace::Definite(container_height.maybe_clamp(min_size.height, max_size.height)),
            },
            SizingMode::InherentSize,
            Line::FALSE,
        );

        let non_auto_margin = margin.map(|m| m.unwrap_or(0.0));

        let free_space = Size {
            width: constants.container_size.width - final_size.width - non_auto_margin.horizontal_axis_sum(),
            height: constants.container_size.height - final_size.height - non_auto_margin.vertical_axis_sum(),
        }
        .f32_max(Size::ZERO);

        // Expand auto margins to fill available space. Auto margins only absorb free space
        // when the box is inset-constrained in that axis (both insets set); otherwise they
        // resolve to zero and the box is statically positioned (CSS2 §10.3.7 / §10.6.4).
        let resolved_margin = {
            let auto_margin_size = Size {
                width: {
                    let auto_margin_count = margin.left.is_none() as u8 + margin.right.is_none() as u8;
                    if auto_margin_count > 0 && left.is_some() && right.is_some() {
                        free_space.width / auto_margin_count as f32
                    } else {
                        0.0
                    }
                },
                height: {
                    let auto_margin_count = margin.top.is_none() as u8 + margin.bottom.is_none() as u8;
                    if auto_margin_count > 0 && top.is_some() && bottom.is_some() {
                        free_space.height / auto_margin_count as f32
                    } else {
                        0.0
                    }
                },
            };

            Rect {
                left: margin.left.unwrap_or(auto_margin_size.width),
                right: margin.right.unwrap_or(auto_margin_size.width),
                top: margin.top.unwrap_or(auto_margin_size.height),
                bottom: margin.bottom.unwrap_or(auto_margin_size.height),
            }
        };

        // Determine flex-relative insets
        let (start_main, end_main) = if constants.is_row { (left, right) } else { (top, bottom) };
        let (start_cross, end_cross) = if constants.is_row { (top, bottom) } else { (left, right) };
        let main_axis_is_horizontal = constants.is_row;
        let cross_axis_is_horizontal = !constants.is_row;
        let main_is_rtl = main_axis_is_horizontal && constants.layout_direction.is_rtl();
        let cross_is_rtl = cross_axis_is_horizontal && constants.layout_direction.is_rtl();
        let main_axis_flex_start_reversed = constants.dir.is_reverse() ^ main_is_rtl;
        let cross_axis_flex_start_reversed = constants.is_wrap_reverse ^ cross_is_rtl;
        let main_start_scrollbar_offset =
            if main_is_rtl { constants.scrollbar_gutter.main(constants.dir) } else { 0.0 };
        let cross_start_scrollbar_offset =
            if cross_is_rtl { constants.scrollbar_gutter.cross(constants.dir) } else { 0.0 };
        let main_end_scrollbar_offset = if main_is_rtl { 0.0 } else { constants.scrollbar_gutter.main(constants.dir) };
        let cross_end_scrollbar_offset =
            if cross_is_rtl { 0.0 } else { constants.scrollbar_gutter.cross(constants.dir) };

        // Apply main-axis alignment
        // let free_main_space = free_space.main(constants.dir) - resolved_margin.main_axis_sum(constants.dir);
        let offset_main = if start_main.is_some() || end_main.is_some() {
            if main_is_rtl && end_main.is_some() {
                constants.container_size.main(constants.dir)
                    - constants.border.main_end(constants.dir)
                    - main_end_scrollbar_offset
                    - final_size.main(constants.dir)
                    - end_main.unwrap_or(0.0)
                    - resolved_margin.main_end(constants.dir)
            } else if let Some(start) = start_main {
                start
                    + constants.border.main_start(constants.dir)
                    + main_start_scrollbar_offset
                    + resolved_margin.main_start(constants.dir)
            } else {
                constants.container_size.main(constants.dir)
                    - constants.border.main_end(constants.dir)
                    - main_end_scrollbar_offset
                    - final_size.main(constants.dir)
                    - end_main.unwrap_or(0.0)
                    - resolved_margin.main_end(constants.dir)
            }
        } else {
            // Stretch is an invalid value for justify_content in the flexbox algorithm, so we
            // treat it as if it wasn't set (and thus we default to FlexStart behaviour).
            //
            // The `safe` overflow-position keyword is intentionally NOT applied here, even when
            // the abs-positioned item would overflow the main axis: Chrome does not apply safe
            // fallback to `justify-content` on absolutely-positioned flex items (only the
            // cross-axis `align-self` does so). Matching the layout authority over a strict
            // spec read keeps gentest fixtures green; reconsider if Chromium changes behavior.
            // `start`/`end` are writing-mode relative (they flip for RTL but not for
            // reversed flex-directions), whereas `flex-start`/`flex-end` and the
            // distributed keywords' fallbacks are flex-relative.
            let start_position = match constants.justify_content.unwrap_or(JustifyContent::FLEX_START).keyword() {
                AlignContentKeyword::Start => !main_is_rtl,
                AlignContentKeyword::End => main_is_rtl,
                _ => true,
            };
            match (
                constants.justify_content.unwrap_or(JustifyContent::FLEX_START).keyword(),
                main_axis_flex_start_reversed,
            ) {
                (AlignContentKeyword::SpaceBetween, false)
                | (AlignContentKeyword::Stretch, false)
                | (AlignContentKeyword::FlexStart, false)
                | (AlignContentKeyword::FlexEnd, true) => {
                    constants.content_box_inset.main_start(constants.dir) + resolved_margin.main_start(constants.dir)
                }
                (AlignContentKeyword::Start | AlignContentKeyword::End, _) => {
                    if start_position {
                        constants.content_box_inset.main_start(constants.dir)
                            + resolved_margin.main_start(constants.dir)
                    } else {
                        constants.container_size.main(constants.dir)
                            - constants.content_box_inset.main_end(constants.dir)
                            - final_size.main(constants.dir)
                            - resolved_margin.main_end(constants.dir)
                    }
                }
                (AlignContentKeyword::FlexEnd, false)
                | (AlignContentKeyword::FlexStart, true)
                | (AlignContentKeyword::Stretch, true)
                | (AlignContentKeyword::SpaceBetween, true) => {
                    constants.container_size.main(constants.dir)
                        - constants.content_box_inset.main_end(constants.dir)
                        - final_size.main(constants.dir)
                        - resolved_margin.main_end(constants.dir)
                }
                (AlignContentKeyword::SpaceEvenly, _)
                | (AlignContentKeyword::SpaceAround, _)
                | (AlignContentKeyword::Center, _) => {
                    (constants.container_size.main(constants.dir)
                        + constants.content_box_inset.main_start(constants.dir)
                        - constants.content_box_inset.main_end(constants.dir)
                        - final_size.main(constants.dir)
                        + resolved_margin.main_start(constants.dir)
                        - resolved_margin.main_end(constants.dir))
                        / 2.0
                }
            }
        };

        // Apply cross-axis alignment
        // let free_cross_space = free_space.cross(constants.dir) - resolved_margin.cross_axis_sum(constants.dir);
        let offset_cross = if start_cross.is_some() || end_cross.is_some() {
            if cross_is_rtl && end_cross.is_some() {
                constants.container_size.cross(constants.dir)
                    - constants.border.cross_end(constants.dir)
                    - cross_end_scrollbar_offset
                    - final_size.cross(constants.dir)
                    - end_cross.unwrap_or(0.0)
                    - resolved_margin.cross_end(constants.dir)
            } else if let Some(start) = start_cross {
                start
                    + constants.border.cross_start(constants.dir)
                    + cross_start_scrollbar_offset
                    + resolved_margin.cross_start(constants.dir)
            } else {
                constants.container_size.cross(constants.dir)
                    - constants.border.cross_end(constants.dir)
                    - cross_end_scrollbar_offset
                    - final_size.cross(constants.dir)
                    - end_cross.unwrap_or(0.0)
                    - resolved_margin.cross_end(constants.dir)
            }
        } else {
            let cross_overflows = final_size.cross(constants.dir) + resolved_margin.cross_axis_sum(constants.dir)
                > constants.container_size.cross(constants.dir)
                    - constants.content_box_inset.cross_axis_sum(constants.dir);
            let cross_keyword = resolve_self_alignment_safety(align_self, cross_overflows);
            // `start`/`end` (and `baseline`, whose static-position fallback is `start`) are
            // writing-mode relative: they flip for RTL but not for `wrap-reverse`.
            // `flex-start`/`flex-end` and the `stretch` fallback are flex-relative.
            let start_position = match cross_keyword {
                AlignItemsKeyword::Start | AlignItemsKeyword::Baseline => !cross_is_rtl,
                AlignItemsKeyword::End => cross_is_rtl,
                _ => true,
            };
            match (cross_keyword, cross_axis_flex_start_reversed) {
                // Stretch alignment does not apply to absolutely positioned items
                // See "Example 3" at https://www.w3.org/TR/css-flexbox-1/#abspos-items
                // Note: Stretch should be FlexStart not Start when we support both
                (AlignItemsKeyword::Start | AlignItemsKeyword::End | AlignItemsKeyword::Baseline, _) => {
                    if start_position {
                        constants.content_box_inset.cross_start(constants.dir)
                            + resolved_margin.cross_start(constants.dir)
                    } else {
                        constants.container_size.cross(constants.dir)
                            - constants.content_box_inset.cross_end(constants.dir)
                            - final_size.cross(constants.dir)
                            - resolved_margin.cross_end(constants.dir)
                    }
                }
                (AlignItemsKeyword::Stretch | AlignItemsKeyword::FlexStart, false)
                | (AlignItemsKeyword::FlexEnd, true) => {
                    constants.content_box_inset.cross_start(constants.dir) + resolved_margin.cross_start(constants.dir)
                }
                (AlignItemsKeyword::Stretch | AlignItemsKeyword::FlexStart, true)
                | (AlignItemsKeyword::FlexEnd, false) => {
                    constants.container_size.cross(constants.dir)
                        - constants.content_box_inset.cross_end(constants.dir)
                        - final_size.cross(constants.dir)
                        - resolved_margin.cross_end(constants.dir)
                }
                (AlignItemsKeyword::Center, _) => {
                    (constants.container_size.cross(constants.dir)
                        + constants.content_box_inset.cross_start(constants.dir)
                        - constants.content_box_inset.cross_end(constants.dir)
                        - final_size.cross(constants.dir)
                        + resolved_margin.cross_start(constants.dir)
                        - resolved_margin.cross_end(constants.dir))
                        / 2.0
                }
                // SelfStart/SelfEnd are resolved to Start/End against the item's own direction
                // where `align_self` is read above.
                (AlignItemsKeyword::SelfStart | AlignItemsKeyword::SelfEnd, _) => unreachable!(),
            }
        };

        let location = match constants.is_row {
            true => Point { x: offset_main, y: offset_cross },
            false => Point { x: offset_cross, y: offset_main },
        };
        let scrollbar_size = Size {
            width: if overflow.y == Overflow::Scroll { scrollbar_width } else { 0.0 },
            height: if overflow.x == Overflow::Scroll { scrollbar_width } else { 0.0 },
        };
        tree.set_unrounded_layout(
            child,
            &Layout {
                order: order as u32,
                size: final_size,
                #[cfg(feature = "content_size")]
                content_size: layout_output.content_size,
                scrollbar_size,
                location,
                padding,
                border,
                margin: resolved_margin,
            },
        );

        #[cfg(feature = "content_size")]
        {
            let size_content_size_contribution = Size {
                width: match overflow.x {
                    Overflow::Visible => f32_max(final_size.width, layout_output.content_size.width),
                    _ => final_size.width,
                },
                height: match overflow.y {
                    Overflow::Visible => f32_max(final_size.height, layout_output.content_size.height),
                    _ => final_size.height,
                },
            };
            if size_content_size_contribution.has_non_zero_area() {
                let absolute_area_offset = Point {
                    x: constants.border.left
                        + if constants.layout_direction.is_rtl() { constants.scrollbar_gutter.x } else { 0.0 },
                    y: constants.border.top,
                };
                let relative_location =
                    Point { x: location.x - absolute_area_offset.x, y: location.y - absolute_area_offset.y };
                let content_size_contribution = Size {
                    width: if constants.layout_direction.is_rtl() {
                        let overflow_extra_width =
                            f32_max(size_content_size_contribution.width - final_size.width, 0.0);
                        f32_max(inset_relative_size.width - relative_location.x, 0.0) + overflow_extra_width
                    } else {
                        relative_location.x + size_content_size_contribution.width
                    },
                    height: relative_location.y + size_content_size_contribution.height,
                };
                content_size = content_size.f32_max(content_size_contribution);
            }
        }
    }

    content_size
}

/// Computes the total space taken up by gaps in an axis given:
///   - The size of each gap
///   - The number of items (children or flex-lines) between which there are gaps
#[inline(always)]
fn sum_axis_gaps(gap: f32, num_items: usize) -> f32 {
    // Gaps only exist between items, so...
    if num_items <= 1 {
        // ...if there are less than 2 items then there are no gaps
        0.0
    } else {
        // ...otherwise there are (num_items - 1) gaps
        gap * (num_items - 1) as f32
    }
}

/// Balanced line breaking for `flex-wrap: balance`.
///
/// Implements the balancing algorithm from the CSS Flexbox Level 2 draft
/// (<https://drafts.csswg.org/css-flexbox-2/#balancing>). Items are divided into exactly
/// `line_count` contiguous sequences (lines), where `line_count` is the number of lines that
/// greedy line breaking would produce (with item sizes floored at zero), raised to the minimum
/// flex line count (`flex-line-count`, clamped to the number of items), such that:
///
/// - every line holds at least one item;
/// - no line's size exceeds the container's inner main size, unless the line holds a single
///   (overflowing) item;
/// - a zero-sized item is assigned to the end of the preceding line rather than the beginning
///   of a line, unless no valid division can glue it there: because extending the preceding
///   line through the item would make it exceed the container's inner main size (which covers
///   a preceding single overflowing item), or because gluing leaves the remaining lines
///   without a valid division of the remaining items;
/// - calling the difference between a line's size and the container's inner main size the
///   line's *error*, the sum of the squared errors of all lines is minimized;
/// - ties are broken by assigning the most items to the first line, then the most items to the
///   second line, and so on.
///
/// Because every division has the same line count, total item size, and total gap size,
/// minimizing the sum of squared errors is equivalent to minimizing the sum of squared *line
/// sizes*, which is what is actually computed: it needs no target size, so an indefinite
/// (infinite) container size needs no special handling — no line overflows, and the
/// minimization equalizes the line sizes. All arithmetic uses `f64`: finite `f32` sizes
/// convert exactly, and the squares and sums involved are far from `f64`'s limits, so
/// equally-balanced divisions compare equal and are resolved by the tie-break rather than by
/// rounding noise.
///
/// The minimization is a dynamic program over (line count, first item of suffix) accelerated
/// with the divide-and-conquer optimization, running in `O(line_count * item_count *
/// log(item_count))` time; the naive `O(line_count * item_count²)` dynamic program is kept as
/// a test oracle.
#[cfg(feature = "flexbox_balance")]
mod balance {
    use crate::util::sys::{new_vec_with_capacity, Vec};

    /// Score assigned to divisions that violate the line size constraint
    const INFEASIBLE: f64 = f64::INFINITY;

    /// Convert an item size in pixels to an `f64`, flooring negative sizes at zero. (An
    /// infinite size is clamped to the largest finite `f32` so that sums stay meaningful.)
    fn to_size(value: f32) -> f64 {
        (value as f64).clamp(0.0, f32::MAX as f64)
    }

    /// Line sizing shared by the scoring and readback phases
    struct LineSizes {
        /// Per item, the prefix sum of the item sizes through it (each item also contributes
        /// one trailing gap, so the size of the line holding items `start..=end` is
        /// `sums[end].0 - sums[start - 1].0 - gap_between_items`), paired with whether the
        /// item's (floored) size is zero
        sums: Vec<(f64, bool)>,
        /// The size of the gap between adjacent items on a line
        gap_between_items: f64,
        /// The container's inner main size, which lines may not exceed (unless they hold a
        /// single item)
        limit: f64,
    }

    impl LineSizes {
        /// The total number of items
        fn item_count(&self) -> usize {
            self.sums.len()
        }

        /// Whether the item at `index` has a (floored) size of zero
        fn is_zero_item(&self, index: usize) -> bool {
            self.sums[index].1
        }

        /// The size of the line holding items `start..=end`
        fn line_size(&self, start: usize, end: usize) -> f64 {
            let start_sum = if start == 0 { 0.0 } else { self.sums[start - 1].0 };
            self.sums[end].0 - start_sum - self.gap_between_items
        }

        /// The squared size of the line holding items `start..=end`, or [`INFEASIBLE`] for a
        /// line of more than one item exceeding the limit
        fn line_cost(&self, start: usize, end: usize) -> f64 {
            let size = self.line_size(start, end);
            if end > start && size > self.limit {
                return INFEASIBLE;
            }
            size * size
        }

        /// For every suffix of the items, the number of lines that greedy line breaking
        /// produces for it: each line collects consecutive items until the next item no longer
        /// fits, and if even the first item of a line doesn't fit, that line takes just the one
        /// (overflowing) item. (The entry for the empty suffix is zero.)
        ///
        /// This is also the *fewest* lines each suffix can validly be divided into, and a
        /// suffix can validly be divided into any number of lines from that count up to its
        /// item count: dividing at greedy line ends never violates the zero-sized-item rule
        /// (a line that can hold no more of the following items can never glue them), and
        /// from any in-range line count the same holds with line ends capped to leave one
        /// item per remaining line.
        fn suffix_greedy_line_counts(&self) -> Vec<u32> {
            let item_count = self.item_count();
            let mut counts: Vec<u32> = new_vec_with_capacity(item_count + 1);
            counts.extend(core::iter::repeat(0).take(item_count + 1));
            // The (exclusive) end of the greedy first line of the suffix, which only moves
            // down as the suffix grows leftwards since lines starting earlier are larger
            let mut line_end = item_count;
            for start in (0..item_count).rev() {
                while line_end > start + 1 && self.line_size(start, line_end - 1) > self.limit {
                    line_end -= 1;
                }
                counts[start] = 1 + counts[line_end];
            }
            counts
        }

        /// For every `start`, the largest `end` such that the line holding items `start..=end`
        /// does not exceed the limit (or `start` itself if even that one item overflows)
        fn fit_ends(&self) -> Vec<u32> {
            let item_count = self.item_count();
            let mut fit_ends: Vec<u32> = new_vec_with_capacity(item_count);
            // Lines starting later are smaller, so the fit end only moves up
            let mut fit_end = 0;
            for start in 0..item_count {
                if fit_end < start {
                    fit_end = start;
                }
                while fit_end + 1 < item_count && self.line_size(start, fit_end + 1) <= self.limit {
                    fit_end += 1;
                }
                fit_ends.push(fit_end as u32);
            }
            fit_ends
        }
    }

    /// One row of the balancing dynamic program: divisions of item suffixes into exactly
    /// `lines` lines
    struct Row<'a> {
        /// The line sizing for the items being divided
        sizes: &'a LineSizes,
        /// See [`LineSizes::fit_ends`]
        fit_ends: &'a [u32],
        /// The previous row: `prev[start]` is the minimum score of any valid division of items
        /// `start..` into exactly `lines - 1` lines ([`INFEASIBLE`] if there is none)
        prev: &'a [f64],
        /// The line ends that precede a non-zero item, in ascending order and capped to
        /// `max_end`. Line ends preceding a *zero* item are constrained by the
        /// zero-sized-item rule and handled separately: for a first line starting at `start`,
        /// the only end preceding a zero item that can be part of a valid division is
        /// `min(fit_ends[start], max_end)` — the line extended as far as the limit and the
        /// remaining lines' one-item-each budget allow. Ending earlier is forbidden by the
        /// rule (the zero item could be glued: the extended line fits and, per
        /// [`LineSizes::suffix_greedy_line_counts`], what remains after even the longest glued
        /// line divides validly into the remaining lines whenever what remains after the
        /// shorter line does, which it must for the division to be feasible at all), and
        /// ending later exceeds the limit on a line of more than one item.
        nonzero_ends: &'a [u32],
        /// The largest possible end of the first line: the remaining `lines - 1` lines need
        /// one item each
        max_end: usize,
    }

    /// Compute `cur[start]` and `opts[start]` for `start` in `start_lo..=start_hi`, where
    /// `cur[start]` is the minimum over valid ends `end` of
    /// `row.sizes.line_cost(start, end) + row.prev[end + 1]` and `opts[start]` is the largest
    /// `end` achieving that minimum. Ends preceding a non-zero item are minimized over
    /// `row.nonzero_ends[col_lo..col_hi]`; the (at most one) end preceding a zero item allowed
    /// by the zero-sized-item rule is merged in afterwards.
    ///
    /// Uses the divide-and-conquer dynamic programming optimization: over the ends preceding
    /// non-zero items, the cost function satisfies the concave quadrangle inequality (it is
    /// the square of a quantity that increases in `end` and decreases in `start`, plus a term
    /// depending on `end` alone, with infeasibility monotone in line length), so the largest
    /// minimizing end is non-decreasing in `start`. Solving the middle `start` therefore
    /// splits the end range in two, and each recursion level scans each candidate end a
    /// bounded number of times. (The ends constrained by the zero-sized-item rule are *not*
    /// monotone this way, which is why they are excluded from the scan and merged in
    /// separately.)
    fn fill_row(
        row: &Row,
        cur: &mut [f64],
        opts: &mut [u32],
        start_lo: usize,
        start_hi: usize,
        col_lo: usize,
        col_hi: usize,
    ) {
        if start_lo > start_hi {
            return;
        }
        let start = start_lo + (start_hi - start_lo) / 2;

        // Minimize over the in-range ends preceding a non-zero item, starting from `start`
        // (the line must hold at least one item)
        let first_col = col_lo + row.nonzero_ends[col_lo..col_hi].partition_point(|&end| (end as usize) < start);
        let mut min_cost = INFEASIBLE;
        let mut min_col = None;
        for col in first_col..col_hi {
            let end = row.nonzero_ends[col] as usize;
            let line_cost = row.sizes.line_cost(start, end);
            if line_cost == INFEASIBLE {
                // Every longer line also exceeds the limit
                break;
            }
            let cost = line_cost + row.prev[end + 1];
            // `<=` keeps the *largest* minimizing end, assigning the most items to the
            // earliest lines as the tie-break requires
            if cost <= min_cost {
                min_cost = cost;
                min_col = Some(col);
            }
        }

        // Merge the single end preceding a zero item that the zero-sized-item rule allows
        let mut min_end = min_col.map(|col| row.nonzero_ends[col] as usize);
        let zero_end = (row.fit_ends[start] as usize).min(row.max_end);
        if row.sizes.is_zero_item(zero_end + 1) {
            let cost = row.sizes.line_cost(start, zero_end) + row.prev[zero_end + 1];
            if cost < min_cost || (cost == min_cost && min_end.map_or(true, |end| end < zero_end)) {
                min_cost = cost;
                min_end = Some(zero_end);
            }
        }

        // Infeasible states are excluded from the range by the caller, and every feasible
        // state has a valid transition (see [`LineSizes::suffix_greedy_line_counts`])
        debug_assert!(min_cost.is_finite());
        cur[start] = min_cost;
        opts[start] = min_end.unwrap_or(start) as u32;

        // The zero-rule end is excluded from the narrowing: only the non-zero ends' minima
        // are monotone
        if start > start_lo {
            let col_hi = min_col.map_or(col_hi, |col| col + 1);
            fill_row(row, cur, opts, start_lo, start - 1, col_lo, col_hi);
        }
        if start < start_hi {
            let col_lo = min_col.unwrap_or(first_col);
            fill_row(row, cur, opts, start + 1, start_hi, col_lo, col_hi);
        }
    }

    /// Determine the number of items on each line that balances items across lines, such that
    /// the largest line is as small as possible, with a minimum of `min_line_count` lines
    /// (or one line per item if there are fewer items).
    ///
    /// `item_sizes` must be non-empty. The returned line item counts are all non-zero and sum to
    /// the number of items.
    ///
    /// Runs in `O(line_count * item_count * log(item_count))` time using
    /// `O(line_count * item_count)` transient memory.
    pub(super) fn balanced_line_item_counts(
        item_sizes: impl ExactSizeIterator<Item = f32>,
        line_limit: f32,
        gap_between_items: f32,
        min_line_count: usize,
    ) -> Vec<usize> {
        let item_count = item_sizes.len();
        debug_assert!(item_count > 0);
        let gap_between_items = to_size(gap_between_items);
        let limit = line_limit as f64;

        let mut sums: Vec<(f64, bool)> = new_vec_with_capacity(item_count);
        let mut sum = 0.0;
        for size in item_sizes {
            let size = to_size(size);
            sum += size + gap_between_items;
            sums.push((sum, size == 0.0));
        }
        let sizes = LineSizes { sums, gap_between_items, limit };

        // `suffix_greedy[start]` is the number of lines greedy line breaking produces for items
        // `start..`, which is also the *fewest* lines the suffix can validly be divided into
        let suffix_greedy = sizes.suffix_greedy_line_counts();
        let line_count = (suffix_greedy[0] as usize).max(min_line_count.clamp(1, item_count));

        let mut item_counts: Vec<usize> = new_vec_with_capacity(line_count);
        if line_count == item_count {
            // One item per line is the only division (this covers `flex-line-count` of at least
            // the item count as well as every item overflowing a line of its own)
            item_counts.extend(core::iter::repeat(1).take(item_count));
            return item_counts;
        }

        let fit_ends = sizes.fit_ends();
        let mut nonzero_ends: Vec<u32> = new_vec_with_capacity(item_count - 1);
        for end in 0..item_count - 1 {
            if !sizes.is_zero_item(end + 1) {
                nonzero_ends.push(end as u32);
            }
        }

        // `prev[start]` is the minimum total score of any valid division of items `start..`
        // into exactly `lines - 1` lines ([`INFEASIBLE`] if there is none), and `cur` is the
        // row being computed for `lines` lines: a division into `lines` lines is a first line
        // `start..=end` plus a division of `end + 1..` into `lines - 1` lines.
        // `opts[(lines - 2) * item_count + start]` records the largest `end` achieving
        // `cur[start]`, from which the chosen division is read back.
        let mut prev: Vec<f64> = new_vec_with_capacity(item_count);
        for start in 0..item_count {
            prev.push(sizes.line_cost(start, item_count - 1));
        }
        let mut cur: Vec<f64> = new_vec_with_capacity(item_count);
        cur.extend(core::iter::repeat(INFEASIBLE).take(item_count));
        let mut opts: Vec<u32> = new_vec_with_capacity((line_count - 1) * item_count);
        opts.extend(core::iter::repeat(0).take((line_count - 1) * item_count));
        for lines in 2..=line_count {
            // The remaining `lines - 1` lines need one item each, bounding this line's end
            let max_end = item_count - lines;
            // Starts whose suffix doesn't fit in `lines` lines even with greedy breaking are
            // infeasible; they form a prefix (a longer suffix never needs fewer lines), and
            // excluding them keeps every state `fill_row` solves feasible
            let mut first_start = 0;
            while first_start < max_end && suffix_greedy[first_start] as usize > lines {
                cur[first_start] = INFEASIBLE;
                first_start += 1;
            }
            let col_hi = nonzero_ends.partition_point(|&end| (end as usize) <= max_end);
            let row = Row { sizes: &sizes, fit_ends: &fit_ends, prev: &prev, nonzero_ends: &nonzero_ends, max_end };
            let opts_row = &mut opts[(lines - 2) * item_count..(lines - 1) * item_count];
            fill_row(&row, &mut cur, opts_row, first_start, max_end, 0, col_hi);
            core::mem::swap(&mut prev, &mut cur);
        }

        // Read the division back out front to back
        debug_assert!(prev[0].is_finite());
        let mut start = 0;
        for lines in (2..=line_count).rev() {
            let end = opts[(lines - 2) * item_count + start] as usize;
            item_counts.push(end - start + 1);
            start = end + 1;
        }
        item_counts.push(item_count - start);
        item_counts
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        /// The number of lines that greedy line breaking produces: each line collects
        /// consecutive items until the next item no longer fits, and if even the first item of
        /// a line doesn't fit, that line takes just the one (overflowing) item
        fn greedy_line_count(sizes: &LineSizes) -> usize {
            let item_count = sizes.item_count();
            let mut line_count = 1;
            let mut index = 0;
            while index < item_count {
                let mut next = index;
                while next < item_count && sizes.line_size(index, next) <= sizes.limit {
                    next += 1;
                }
                if next == index {
                    next = index + 1;
                }
                index = next;
                if index < item_count {
                    line_count += 1;
                }
            }
            line_count
        }

        /// The zero-sized-item rule, brute force: a line `start..=end` (of a division of
        /// `start..` into `lines` lines) may only end before a zero-sized item if no division
        /// that extends the line through the item exists — every extension either exceeds the
        /// limit or leaves the remaining items without a valid division into the remaining
        /// lines. (`min_errors` rows below `lines` must already be computed.)
        fn zero_boundary_valid(sizes: &LineSizes, min_errors: &[f64], lines: usize, start: usize, end: usize) -> bool {
            let item_count = sizes.item_count();
            if !sizes.is_zero_item(end + 1) {
                return true;
            }
            for glued_end in end + 1..=item_count - lines {
                if sizes.line_size(start, glued_end) > sizes.limit {
                    break;
                }
                if min_errors[(lines - 2) * item_count + glued_end + 1].is_finite() {
                    return false;
                }
            }
            true
        }

        /// The naive `O(line_count * item_count²)` dynamic program from the spec, used as a
        /// test oracle for the divide-and-conquer optimized implementation
        fn naive_line_item_counts(
            item_sizes: &[f32],
            line_limit: f32,
            gap_between_items: f32,
            min_line_count: usize,
        ) -> Vec<usize> {
            let item_count = item_sizes.len();
            let gap_between_items = to_size(gap_between_items);
            let limit = line_limit as f64;

            let mut sums: Vec<(f64, bool)> = new_vec_with_capacity(item_count);
            let mut sum = 0.0;
            for size in item_sizes {
                let size = to_size(*size);
                sum += size + gap_between_items;
                sums.push((sum, size == 0.0));
            }
            let sizes = LineSizes { sums, gap_between_items, limit };

            let line_count = greedy_line_count(&sizes).max(min_line_count.clamp(1, item_count));

            // `min_errors[(lines - 1) * item_count + start]` is the minimum total score of any
            // valid division of items `start..` into exactly `lines` lines
            let mut min_errors: Vec<f64> = new_vec_with_capacity(line_count * item_count);
            for start in 0..item_count {
                min_errors.push(sizes.line_cost(start, item_count - 1));
            }
            for lines in 2..=line_count {
                let row = (lines - 1) * item_count;
                for start in 0..item_count {
                    let mut min_error = INFEASIBLE;
                    let mut end = start;
                    while end + lines <= item_count {
                        let line_cost = sizes.line_cost(start, end);
                        if line_cost == INFEASIBLE {
                            break;
                        }
                        if zero_boundary_valid(&sizes, &min_errors, lines, start, end) {
                            let error = line_cost + min_errors[row - item_count + end + 1];
                            if error < min_error {
                                min_error = error;
                            }
                        }
                        end += 1;
                    }
                    min_errors.push(min_error);
                }
            }

            // For each line, the *largest* end whose error plus the minimum remaining error
            // adds up to the (feasible) minimum is chosen
            assert!(min_errors[(line_count - 1) * item_count].is_finite());
            let mut item_counts: Vec<usize> = new_vec_with_capacity(line_count);
            let mut start = 0;
            for lines_after in (0..line_count).rev() {
                let target = min_errors[lines_after * item_count + start];
                let mut end = item_count - 1 - lines_after;
                loop {
                    if sizes.line_cost(start, end) != INFEASIBLE
                        && (lines_after == 0 || zero_boundary_valid(&sizes, &min_errors, lines_after + 1, start, end))
                    {
                        let remaining_error =
                            if lines_after == 0 { 0.0 } else { min_errors[(lines_after - 1) * item_count + end + 1] };
                        if sizes.line_cost(start, end) + remaining_error == target {
                            break;
                        }
                    }
                    assert!(end > start);
                    end -= 1;
                }
                item_counts.push(end - start + 1);
                start = end + 1;
            }
            assert_eq!(start, item_count);
            item_counts
        }

        #[test]
        fn zero_sized_item_rule() {
            // The zero item joins the first line even though the gap it brings makes the score
            // worse: [70, 0] / [20] has line sizes 80 / 20 (score 6800), while [70] / [0, 20]
            // would have line sizes 70 / 30 (score 5800)
            let counts = balanced_line_item_counts([70.0, 0.0, 20.0].iter().copied(), 100.0, 10.0, 1);
            assert_eq!(counts, [2, 1]);
            // The zero item is not glued to a single overflowing item's line
            let counts = balanced_line_item_counts([150.0, 0.0, 30.0].iter().copied(), 100.0, 0.0, 1);
            assert_eq!(counts, [1, 2]);
            // Zero items are glued as far as the requested line count allows
            let counts = balanced_line_item_counts([70.0, 0.0, 0.0].iter().copied(), 100.0, 0.0, 2);
            assert_eq!(counts, [2, 1]);
        }

        #[test]
        fn matches_naive_dp() {
            // A simple xorshift PRNG so the test is deterministic and dependency-free
            let mut state: u64 = 0x243F6A8885A308D3;
            let mut rand = move |bound: u32| -> u32 {
                state ^= state << 13;
                state ^= state >> 7;
                state ^= state << 17;
                ((state >> 32) as u32) % bound
            };

            for case in 0..5000 {
                // Mostly small item counts (quantized sizes maximize ties, exercising the
                // tie-break rules), with some larger ones exercising the recursion
                let item_count = if case % 20 == 0 { (rand(60) + 1) as usize } else { (rand(15) + 1) as usize };
                let mut item_sizes: Vec<f32> = new_vec_with_capacity(item_count);
                for _ in 0..item_count {
                    let size = match rand(8) {
                        // Frequent zero items exercise the zero-sized-item rule
                        0 | 1 => 0.0,
                        2 => -10.0,
                        _ => rand(8) as f32 * 25.0,
                    };
                    item_sizes.push(size);
                }
                let line_limit = match rand(5) {
                    0 => f32::INFINITY,
                    _ => rand(10) as f32 * 30.0,
                };
                let gap_between_items = rand(4) as f32 * 5.0;
                let min_line_count = rand(item_count as u32 + 2) as usize;

                let expected = naive_line_item_counts(&item_sizes, line_limit, gap_between_items, min_line_count);
                let actual = balanced_line_item_counts(
                    item_sizes.iter().copied(),
                    line_limit,
                    gap_between_items,
                    min_line_count,
                );
                assert_eq!(
                    actual, expected,
                    "case {case}: item_sizes={item_sizes:?} line_limit={line_limit} \
                     gap_between_items={gap_between_items} min_line_count={min_line_count}"
                );
            }
        }
    }
}
