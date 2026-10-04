//! Low-level access to the layout algorithms themselves. For a higher-level API, see the [`TaffyTree`](crate::TaffyTree) struct.
//!
//! ### Layout functions
//!
//! The layout functions all take an [`&mut impl LayoutPartialTree`](crate::LayoutPartialTree) parameter, which represents a single container node and it's direct children.
//!
//! | Function                          | Purpose                                                                                                                                                                                            |
//! | ---                               | ---                                                                                                                                                                                                |
//! | [`compute_flexbox_layout`]        | Layout a Flexbox container and it's direct children                                                                                                                                                |
//! | [`compute_grid_layout`]           | Layout a CSS Grid container and it's direct children                                                                                                                                               |
//! | [`compute_block_layout`]          | Layout a Block container and it's direct children                                                                                                                                                  |
//! | [`compute_leaf_layout`]           | Applies common properties like padding/border/aspect-ratio to a node before deferring to a passed closure to determine it's size. Can be applied to nodes like text or image nodes.                |
//! | [`compute_root_layout`]           | Layout the root node of a tree (regardless of it's layout mode). This function is typically called once to begin a layout run.                                                                     |                                                                      |
//! | [`compute_hidden_layout`]         | Mark a node as hidden during layout (like `Display::None`)                                                                                                                                         |
//! | [`compute_cached_layout`]         | Attempts to find a cached layout for the specified node and layout inputs. Uses the provided closure to compute the layout (and then stores the result in the cache) if no cached layout is found. |
//!
//! ### Other functions
//!
//! | Function                          | Requires                                                                                                                                                                                           | Purpose                                                              |
//! | ---                               | ---                                                                                                                                                                                                | ---                                                                  |
//! | [`round_layout`]                  | [`RoundTree`]                                                                                                                                                                                      | Round a tree of float-valued layouts to integer pixels               |
//! | [`print_tree`](crate::print_tree) | [`PrintTree`](crate::PrintTree)                                                                                                                                                                    | Print a debug representation of a node tree and it's computed layout |
//!
pub(crate) mod common;
pub(crate) mod leaf;
pub(crate) mod oof;

#[cfg(feature = "block_layout")]
pub(crate) mod block;

#[cfg(feature = "float_layout")]
pub(crate) mod float;

#[cfg(feature = "flexbox")]
pub(crate) mod flexbox;

#[cfg(feature = "grid")]
pub(crate) mod grid;

pub use leaf::compute_leaf_layout;
pub use oof::{compute_oof_layout, compute_oof_layout_for_area, resolve_static_offset, OofLayoutResult};

#[cfg(feature = "block_layout")]
pub use self::block::{compute_block_align_content_offset, compute_block_layout, BlockContext, BlockFormattingContext};

#[cfg(feature = "flexbox")]
pub use self::flexbox::compute_flexbox_layout;

#[cfg(feature = "grid")]
pub use self::grid::compute_grid_layout;

#[cfg(feature = "float_layout")]
pub use self::float::{BfcSlot, ContentSlot, FloatContext, FloatIntrinsicWidthCalculator};

use crate::geometry::{Line, Point, Size};
use crate::style::{AvailableSpace, ContainingBlockClaims, CoreStyle, Overflow, Position};
use crate::tree::{
    AxisStaticAlign, AxisStaticEdge, AxisStaticPosition, Layout, LayoutInput, LayoutOutput, LayoutPartialTree,
    LayoutPartialTreeExt, NodeId, OofCandidate, OofCandidates, RequestedAxis, RoundTree, RunMode, SizingMode,
};
use crate::util::debug::{debug_log, debug_log_node, debug_pop_node, debug_push_node};
use crate::util::sys::Vec;
use crate::util::ResolveOrZero;
use crate::{CacheTree, MaybeMath, MaybeResolve};

/// Compute layout for the root node in the tree
///
/// The root's containing block is the initial containing block (ICB), which has the dimensions of the
/// viewport (`available_space`) and is anchored at the canvas origin. The root's `position` and `inset`
/// styles are resolved against it:
///
/// - A `position: absolute`/`fixed` root is laid out by the out-of-flow positioning routine used for
///   every other out-of-flow box (insets, shrink-to-fit sizing, auto margins, ...), with its static
///   position at the ICB origin. This requires a definite `available_space` in both axes: otherwise there
///   is no ICB to resolve against and the root is laid out as if it were in flow.
/// - A `position: relative` root is laid out in flow and then offset by its insets.
pub fn compute_root_layout(
    tree: &mut (impl crate::tree::LayoutContainingBlock + CacheTree),
    root: NodeId,
    available_space: Size<AvailableSpace>,
) {
    let style = tree.get_core_container_style(root);
    let position = style.position();
    let direction = style.direction();
    drop(style);

    let icb_size = available_space.into_options();
    let oof_root_area = match (position.is_out_of_flow(), icb_size.width, icb_size.height) {
        (true, Some(width), Some(height)) => Some(Size { width, height }),
        _ => None,
    };

    // When the root's layout is served from the cache its layout algorithm does not run, so the
    // hoisted children it recorded on a previous run (including those added by the root
    // positioning pass below) are still in place and must not be re-added.
    let mut root_is_cached = false;

    let (layout, candidates) = if let Some(area_size) = oof_root_area {
        // The static position of the root places its margin box at the origin of the ICB
        let (start, end) = if direction.is_rtl() { (area_size.width, area_size.width) } else { (0.0, 0.0) };
        let candidate = OofCandidate {
            node: root,
            order: 0,
            position,
            static_position: Point {
                x: AxisStaticPosition {
                    area: Line { start, end },
                    align: AxisStaticAlign::from_keyword(if direction.is_rtl() {
                        AxisStaticEdge::End
                    } else {
                        AxisStaticEdge::Start
                    }),
                },
                y: AxisStaticPosition::from_edge(0.0, AxisStaticEdge::Start),
            },
        };
        let oof::OofBoxLayout { layout, surfaced, .. } = oof::layout_oof_box(
            tree,
            candidate,
            area_size,
            Point::ZERO,
            direction,
            // The initial containing block is not a scroll container
            Point { x: Overflow::Visible, y: Overflow::Visible },
            #[cfg(feature = "grid")]
            None,
            |tree, inputs| root_is_cached = tree.cache_get(root, inputs).is_some(),
        );
        (layout, surfaced)
    } else {
        compute_in_flow_root_layout(tree, root, available_space, &mut root_is_cached)
    };

    // Final positioning pass for out-of-flow boxes with no nearer containing block: the initial
    // containing block is the containing block for `position: fixed` boxes and for
    // `position: absolute` boxes with no positioned ancestor.
    if !candidates.is_empty() {
        let overflow = tree.get_core_container_style(root).overflow();

        // The initial containing block has the dimensions of the viewport (the available space) and is
        // anchored at the canvas origin. In an axis where the available space is indefinite, fall back
        // to the root's padding box.
        let area_inset = layout.border
            + crate::geometry::Rect {
                left: 0.0,
                right: layout.scrollbar_size.width,
                top: 0.0,
                bottom: layout.scrollbar_size.height,
            };
        let root_padding_box_size =
            layout.size - Size { width: area_inset.horizontal_axis_sum(), height: area_inset.vertical_axis_sum() };
        let (area_width, area_x) = match icb_size.width {
            Some(width) => ((width - layout.scrollbar_size.width).max(0.0), -layout.location.x),
            None => (root_padding_box_size.width, area_inset.left),
        };
        let (area_height, area_y) = match icb_size.height {
            Some(height) => ((height - layout.scrollbar_size.height).max(0.0), -layout.location.y),
            None => (root_padding_box_size.height, area_inset.top),
        };
        let area_size = Size { width: area_width, height: area_height };
        let area_offset = Point { x: area_x, y: area_y };

        let mut hoisted: Vec<NodeId> = Vec::new();
        let mut unclaimed = OofCandidates::new();
        oof::perform_oof_layout(
            tree,
            root,
            candidates.as_slice(),
            area_size,
            area_offset,
            direction,
            // The root is the initial containing block and claims all remaining candidates
            ContainingBlockClaims::ALL,
            overflow,
            &mut hoisted,
            &mut unclaimed,
        );
        debug_assert!(unclaimed.is_empty(), "the root positioning pass must claim all remaining candidates");
        if !root_is_cached {
            tree.add_hoisted_children(root, &hoisted);
        }
    }
}

/// Lay out an in-flow (or `position: relative`) root node against the available space, store its layout
/// and return it along with the out-of-flow candidates bubbled up from its subtree.
#[inline(always)]
fn compute_in_flow_root_layout(
    tree: &mut (impl crate::tree::LayoutContainingBlock + CacheTree),
    root: NodeId,
    available_space: Size<AvailableSpace>,
    root_is_cached: &mut bool,
) -> (Layout, OofCandidates) {
    let mut known_dimensions = Size::NONE;

    // The available space passed to the root excludes the root's margins
    let root_available_space = {
        let style = tree.get_core_container_style(root);
        let margin =
            style.margin().resolve_or_zero(available_space.width.into_option(), |val, basis| tree.calc(val, basis));
        available_space.maybe_sub(margin.sum_axes()).maybe_max(Size::ZERO)
    };

    #[cfg(feature = "block_layout")]
    {
        use crate::BoxSizing;

        let parent_size = available_space.into_options();
        let style = tree.get_core_container_style(root);

        if style.is_block() {
            // Pull these out earlier to avoid borrowing issues
            let aspect_ratio = style.aspect_ratio();
            let padding = style.padding().resolve_or_zero(parent_size.width, |val, basis| tree.calc(val, basis));
            let border = style.border().resolve_or_zero(parent_size.width, |val, basis| tree.calc(val, basis));
            let padding_border_size = (padding + border).sum_axes();
            let box_sizing_adjustment =
                if style.box_sizing() == BoxSizing::ContentBox { padding_border_size } else { Size::ZERO };

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
            let clamped_style_size = style
                .size()
                .maybe_resolve(parent_size, |val, basis| tree.calc(val, basis))
                .maybe_apply_aspect_ratio(aspect_ratio)
                .maybe_add(box_sizing_adjustment)
                .maybe_clamp(min_size, max_size);

            // If both min and max in a given axis are set and max <= min then this determines the size in that axis
            let min_max_definite_size = min_size.zip_map(max_size, |min, max| match (min, max) {
                (Some(min), Some(max)) if max <= min => Some(min),
                _ => None,
            });

            // Block nodes automatically stretch fit their width to fit available space if available space is definite
            let available_space_based_size = Size { width: root_available_space.width.into_option(), height: None };

            let styled_based_known_dimensions = known_dimensions
                .or(min_max_definite_size)
                .or(clamped_style_size)
                .or(available_space_based_size)
                .maybe_max(padding_border_size);

            known_dimensions = styled_based_known_dimensions;
        }
    }

    let inputs = LayoutInput {
        known_dimensions,
        known_dimensions_are_definite: Size { width: true, height: true },
        parent_size: available_space.into_options(),
        available_space: root_available_space,
        sizing_mode: SizingMode::InherentSize,
        axis: RequestedAxis::Both,
        run_mode: RunMode::PerformLayout,
        vertical_margins_are_collapsible: Line::FALSE,
    };
    *root_is_cached = tree.cache_get(root, &inputs).is_some();

    // Recursively compute node layout
    let mut output = tree.compute_child_layout(root, inputs);
    let style = tree.get_core_container_style(root);
    let padding =
        style.padding().resolve_or_zero(available_space.width.into_option(), |val, basis| tree.calc(val, basis));
    let border =
        style.border().resolve_or_zero(available_space.width.into_option(), |val, basis| tree.calc(val, basis));
    let margin =
        style.margin().resolve_or_zero(available_space.width.into_option(), |val, basis| tree.calc(val, basis));
    let scrollbar_size = Size {
        width: if style.overflow().y == Overflow::Scroll { style.scrollbar_width() } else { 0.0 },
        height: if style.overflow().x == Overflow::Scroll { style.scrollbar_width() } else { 0.0 },
    };
    let is_rtl = style.direction().is_rtl();

    // The root's margin box is placed at the origin of the initial containing block
    let mut location = Point {
        x: match (is_rtl, available_space.width.into_option()) {
            (true, Some(available_width)) => available_width - output.size.width - margin.right,
            _ => margin.left,
        },
        y: margin.top,
    };

    // A relatively positioned root is offset by its insets (resolved against the initial containing block)
    if style.position() == Position::Relative {
        let icb_size = available_space.into_options();
        let inset = style.inset();
        let left = inset.left.maybe_resolve(icb_size.width, |val, basis| tree.calc(val, basis));
        let right = inset.right.maybe_resolve(icb_size.width, |val, basis| tree.calc(val, basis));
        let top = inset.top.maybe_resolve(icb_size.height, |val, basis| tree.calc(val, basis));
        let bottom = inset.bottom.maybe_resolve(icb_size.height, |val, basis| tree.calc(val, basis));
        location.x += if is_rtl {
            right.map(|right| -right).or(left).unwrap_or(0.0)
        } else {
            left.or(right.map(|right| -right)).unwrap_or(0.0)
        };
        location.y += top.or(bottom.map(|bottom| -bottom)).unwrap_or(0.0);
    }
    drop(style);

    let layout = Layout {
        order: 0,
        location,
        size: output.size,
        #[cfg(feature = "content_size")]
        scrollable_overflow_rect: output.scrollable_overflow_rect,
        scrollbar_size,
        padding,
        border,
        // TODO: support auto margins for root node?
        margin,
    };
    tree.set_unrounded_layout(root, &layout);

    (layout, output.oof_candidates.take())
}

/// Attempts to find a cached layout for the specified node and layout inputs.
///
/// Uses the provided closure to compute the layout (and then stores the result in the cache) if no cached layout is found.
#[inline(always)]
pub fn compute_cached_layout<Tree: CacheTree + ?Sized, ComputeFunction>(
    tree: &mut Tree,
    node: NodeId,
    inputs: LayoutInput,
    compute_uncached: ComputeFunction,
) -> LayoutOutput
where
    ComputeFunction: FnOnce(&mut Tree, NodeId, LayoutInput) -> LayoutOutput,
{
    debug_push_node!(node);

    // First we check if we have a cached result for the given input
    let cache_entry = tree.cache_get(node, &inputs);
    if let Some(cached_size_and_baselines) = cache_entry {
        debug_log_node!(inputs);
        debug_log!("RESULT (CACHED)", dbg:cached_size_and_baselines.size);
        debug_pop_node!();
        return cached_size_and_baselines;
    }

    debug_log_node!(inputs);

    let computed_size_and_baselines = compute_uncached(tree, node, inputs);

    // Cache result
    tree.cache_store(node, &inputs, computed_size_and_baselines.clone());

    debug_log!("RESULT", dbg:computed_size_and_baselines.size);
    debug_pop_node!();

    computed_size_and_baselines
}

/// Rounds the calculated layout to exact pixel values
///
/// In order to ensure that no gaps in the layout are introduced we:
///   - Always round based on the cumulative x/y coordinates (relative to the viewport) rather than
///     parent-relative coordinates
///   - Compute width/height by first rounding the top/bottom/left/right and then computing the difference
///     rather than rounding the width/height directly
///
/// See <https://github.com/facebook/yoga/commit/aa5b296ac78f7a22e1aeaf4891243c6bb76488e2> for more context
///
/// In order to prevent innacuracies caused by rounding already-rounded values, we read from `unrounded_layout`
/// and write to `final_layout`.
pub fn round_layout(tree: &mut impl RoundTree, node_id: NodeId) {
    // Detect the instructions that the CPU supports once, and use them to round every node
    #[cfg(feature = "simd_rounding")]
    return fearless_simd::dispatch!(fearless_simd::Level::new(), simd => {
        round_layout_inner(tree, node_id, 0.0, 0.0, SimdRounder(simd))
    });

    #[cfg(not(feature = "simd_rounding"))]
    round_layout_inner(tree, node_id, 0.0, 0.0, ScalarRounder)
}

/// A way of rounding values to the nearest whole number
trait Rounder: Copy + Send + Sync {
    /// Rounds four values to the nearest whole number.
    /// Gives the same result as [`round`](crate::util::sys::round) for every value.
    fn round(self, values: [f32; 4]) -> [f32; 4];
    /// Call a function in a context where [`Rounder::round`] can be inlined
    fn vectorize<R>(self, f: impl FnOnce() -> R) -> R;
}

/// Rounds using [`round`](crate::util::sys::round)
#[cfg_attr(feature = "simd_rounding", allow(dead_code))]
#[derive(Copy, Clone)]
struct ScalarRounder;

impl Rounder for ScalarRounder {
    #[inline(always)]
    fn round(self, values: [f32; 4]) -> [f32; 4] {
        values.map(crate::util::sys::round)
    }
    #[inline(always)]
    fn vectorize<R>(self, f: impl FnOnce() -> R) -> R {
        f()
    }
}

/// Rounds using the best instructions that the CPU was detected to support
#[cfg(feature = "simd_rounding")]
#[derive(Copy, Clone)]
struct SimdRounder<S: fearless_simd::Simd>(S);

#[cfg(feature = "simd_rounding")]
impl<S: fearless_simd::Simd> Rounder for SimdRounder<S> {
    #[inline(always)]
    fn round(self, values: [f32; 4]) -> [f32; 4] {
        use fearless_simd::SimdInto;
        let values: fearless_simd::f32x4<S> = values.simd_into(self.0);
        *self.0.floor_f32x4(self.0.add_f32x4(values, self.0.splat_f32x4(0.5)))
    }
    #[inline(always)]
    fn vectorize<R>(self, f: impl FnOnce() -> R) -> R {
        self.0.vectorize(f)
    }
}

/// Recursive function to apply rounding to all descendents
fn round_layout_inner<R: Rounder>(
    tree: &mut impl RoundTree,
    node_id: NodeId,
    parent_x: f32,
    parent_y: f32,
    rounder: R,
) {
    // The layout is read and written within the same context as it is rounded in,
    // so that it does not need to be copied into and out of that context
    let (cumulative_x, cumulative_y) = rounder.vectorize(
        #[inline(always)]
        || {
            let unrounded_layout = tree.get_unrounded_layout(node_id);
            let cumulative_x = parent_x + unrounded_layout.location.x;
            let cumulative_y = parent_y + unrounded_layout.location.y;
            let layout = round_node_layout(rounder, &unrounded_layout, parent_x, parent_y, cumulative_x, cumulative_y);
            tree.set_final_layout(node_id, &layout);
            (cumulative_x, cumulative_y)
        },
    );

    // Recurse into the node's in-flow children and into the out-of-flow boxes
    // for which this node is the containing block
    tree.round_child_subtrees(node_id, cumulative_x, cumulative_y, move |tree, node_id, x, y| {
        round_layout_inner(tree, node_id, x, y, rounder)
    });
}

/// Round the layout of a single node, given the unrounded position of its parent (`parent_x`, `parent_y`)
/// and of the node itself (`cumulative_x`, `cumulative_y`) relative to the root
#[inline(always)]
fn round_node_layout<R: Rounder>(
    rounder: R,
    unrounded_layout: &Layout,
    parent_x: f32,
    parent_y: f32,
    cumulative_x: f32,
    cumulative_y: f32,
) -> Layout {
    let mut layout = *unrounded_layout;
    let Layout { size, border, padding, scrollbar_size, .. } = *unrounded_layout;

    // The far edges of the node
    let right = cumulative_x + size.width;
    let bottom = cumulative_y + size.height;

    // Four values are rounded at a time, so that they can be rounded by a single instruction
    let [parent_x, parent_y, x, y] = rounder.round([parent_x, parent_y, cumulative_x, cumulative_y]);
    let [rounded_right, rounded_bottom, scrollbar_width, scrollbar_height] =
        rounder.round([right, bottom, scrollbar_size.width, scrollbar_size.height]);
    let [border_left, border_right, border_top, border_bottom] = rounder.round([
        cumulative_x + border.left,
        right - border.right,
        cumulative_y + border.top,
        bottom - border.bottom,
    ]);
    let [padding_left, padding_right, padding_top, padding_bottom] = rounder.round([
        cumulative_x + padding.left,
        right - padding.right,
        cumulative_y + padding.top,
        bottom - padding.bottom,
    ]);

    layout.location.x = x - parent_x;
    layout.location.y = y - parent_y;
    layout.size.width = rounded_right - x;
    layout.size.height = rounded_bottom - y;
    layout.scrollbar_size.width = scrollbar_width;
    layout.scrollbar_size.height = scrollbar_height;
    layout.border.left = border_left - x;
    layout.border.right = rounded_right - border_right;
    layout.border.top = border_top - y;
    layout.border.bottom = rounded_bottom - border_bottom;
    layout.padding.left = padding_left - x;
    layout.padding.right = rounded_right - padding_right;
    layout.padding.top = padding_top - y;
    layout.padding.bottom = rounded_bottom - padding_bottom;

    #[cfg(feature = "content_size")]
    {
        let unrounded_rect = unrounded_layout.scrollable_overflow_rect;
        let [left, right, top, bottom] = rounder.round([
            cumulative_x + unrounded_rect.left,
            cumulative_x + unrounded_rect.right,
            cumulative_y + unrounded_rect.top,
            cumulative_y + unrounded_rect.bottom,
        ]);
        layout.scrollable_overflow_rect.left = left - x;
        layout.scrollable_overflow_rect.right = right - x;
        layout.scrollable_overflow_rect.top = top - y;
        layout.scrollable_overflow_rect.bottom = bottom - y;
    }

    layout
}

/// Creates a layout for this node and its children, recursively.
/// Each hidden node has zero size and is placed at the origin
pub fn compute_hidden_layout(tree: &mut (impl LayoutPartialTree + CacheTree), node: NodeId) -> LayoutOutput {
    // Clear cache and set zeroed-out layout for the node
    tree.cache_clear(node);
    tree.set_unrounded_layout(node, &Layout::with_order(0));

    // Perform hidden layout on all children
    for index in 0..tree.child_count(node) {
        let child_id = tree.get_child_id(node, index);
        tree.compute_child_layout(child_id, LayoutInput::HIDDEN);
    }

    LayoutOutput::HIDDEN
}

/// A module for unified re-exports of detailed layout info structs, used by low level API
pub mod detailed_info {
    #[cfg(feature = "grid")]
    pub use super::grid::{
        DetailedGridInfo, DetailedGridItemsInfo, DetailedGridTracksInfo, GridLineNames, GridLineNamesIter,
    };
}

#[cfg(test)]
mod tests {
    use super::compute_hidden_layout;
    use crate::geometry::{Point, Size};
    use crate::style::{Display, Style};
    use crate::TaffyTree;

    /// Rounding with the detected instructions gives exactly the same result as scalar rounding
    #[cfg(feature = "simd_rounding")]
    #[test]
    fn simd_rounding_matches_scalar_rounding() {
        use super::{Rounder, ScalarRounder, SimdRounder};

        fearless_simd::dispatch!(fearless_simd::Level::new(), simd => {
            let rounder = SimdRounder(simd);
            rounder.vectorize(|| {
                // Every 4099th bit pattern (4099 is odd, so this visits a spread of all exponents and signs),
                // and the values on either side of each multiple of 0.5 up to 4096
                let spread = (0..=u32::MAX).step_by(4099).map(f32::from_bits);
                let halves = (-8192..=8192).flat_map(|i| {
                    let value = i as f32 * 0.5;
                    [f32::from_bits(value.to_bits().wrapping_sub(1)), value, f32::from_bits(value.to_bits() + 1)]
                });
                for value in spread.chain(halves) {
                    // Each value is rounded in a different position, next to different values
                    let values = [value, -value, value + 1.0, 0.25];
                    for rotation in 0..4 {
                        let mut values = values;
                        values.rotate_left(rotation);
                        let expected = ScalarRounder.round(values);
                        let actual = rounder.round(values);
                        for index in 0..4 {
                            let (expected, actual) = (expected[index], actual[index]);
                            assert!(
                                expected.to_bits() == actual.to_bits() || (expected.is_nan() && actual.is_nan()),
                                "round({:?}): expected {expected:?}, got {actual:?}",
                                values[index]
                            );
                        }
                    }
                }
            })
        });
    }

    #[test]
    fn hidden_layout_should_hide_recursively() {
        let mut taffy: TaffyTree<()> = TaffyTree::new();

        let style: Style = Style { display: Display::Flex, size: Size::from_lengths(50.0, 50.0), ..Default::default() };

        let grandchild_00 = taffy.new_leaf(style.clone()).unwrap();
        let grandchild_01 = taffy.new_leaf(style.clone()).unwrap();
        let child_00 = taffy.new_with_children(style.clone(), &[grandchild_00, grandchild_01]).unwrap();

        let grandchild_02 = taffy.new_leaf(style.clone()).unwrap();
        let child_01 = taffy.new_with_children(style.clone(), &[grandchild_02]).unwrap();

        let root = taffy
            .new_with_children(
                Style { display: Display::None, size: Size::from_lengths(50.0, 50.0), ..Default::default() },
                &[child_00, child_01],
            )
            .unwrap();

        compute_hidden_layout(&mut taffy.as_layout_tree(), root);

        // Whatever size and display-mode the nodes had previously,
        // all layouts should resolve to ZERO due to the root's DISPLAY::NONE

        for node in [root, child_00, child_01, grandchild_00, grandchild_01, grandchild_02] {
            let layout = taffy.layout(node).unwrap();
            assert_eq!(layout.size, Size::zero());
            assert_eq!(layout.location, Point::zero());
        }
    }
}
