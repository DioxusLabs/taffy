//! Computes the [flexbox](https://css-tricks.com/snippets/css/a-guide-to-flexbox/) layout algorithm on [`Taffy`](crate::Taffy) according to the [spec](https://www.w3.org/TR/css-flexbox-1/)
use crate::compute::LayoutAlgorithm;
use crate::geometry::{Point, Rect, Size};
use crate::prelude::{TaffyMaxContent, TaffyMinContent};
use crate::style::Style;
use crate::style::{
    AlignContent, AlignItems, AlignSelf, AvailableSpace, Dimension, Display, FlexWrap, JustifyContent,
    LengthPercentageAuto, Overflow, Position,
};
use crate::tree::{Layout, RunMode, SizeAndBaselines, SizingMode};
use crate::tree::{LayoutTree, NodeId};
use crate::util::sys::Vec;
use crate::util::sys::{f32_max, new_vec_with_capacity};
use crate::util::MaybeMath;
use crate::util::{MaybeResolve, ResolveOrZero};

#[cfg(feature = "debug")]
use crate::util::debug::NODE_LOGGER;

/// The public interface to Taffy's Flexbox algorithm implementation
pub struct BlockAlgorithm;
impl LayoutAlgorithm for BlockAlgorithm {
    const NAME: &'static str = "FLEXBOX";

    fn perform_layout(
        tree: &mut impl LayoutTree,
        node: NodeId,
        known_dimensions: Size<Option<f32>>,
        parent_size: Size<Option<f32>>,
        available_space: Size<AvailableSpace>,
        _sizing_mode: SizingMode,
    ) -> SizeAndBaselines {
        compute(tree, node, known_dimensions, parent_size, available_space, RunMode::PeformLayout)
    }

    fn measure_size(
        tree: &mut impl LayoutTree,
        node: NodeId,
        known_dimensions: Size<Option<f32>>,
        parent_size: Size<Option<f32>>,
        available_space: Size<AvailableSpace>,
        _sizing_mode: SizingMode,
    ) -> Size<f32> {
        compute(tree, node, known_dimensions, parent_size, available_space, RunMode::ComputeSize).size
    }
}

/// The intermediate results of a flexbox calculation for a single item
struct BlockItem {
    /// The identifier for the associated node
    node: NodeId,

    /// The base size of this item
    size: Size<Option<f32>>,
    /// The minimum allowable size of this item
    min_size: Size<Option<f32>>,
    /// The maximum allowable size of this item
    max_size: Size<Option<f32>>,

    /// The overflow style of the item
    overflow: Point<Overflow>,

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

    /// The computed content box size of this item
    computed_content_box_size: Size<f32>,
    /// The computed border box size of this item
    computed_border_box_size: Size<f32>,

    /// The position of the bottom edge of this item
    baseline: f32,
}

/// Computes the layout of [`LayoutTree`] according to the block layout algorithm
pub fn compute(
    tree: &mut impl LayoutTree,
    node: NodeId,
    known_dimensions: Size<Option<f32>>,
    parent_size: Size<Option<f32>>,
    available_space: Size<AvailableSpace>,
    run_mode: RunMode,
) -> SizeAndBaselines {
    let style = tree.style(node);

    // Pull these out earlier to avoid borrowing issues
    let aspect_ratio = style.aspect_ratio;
    let min_size = style.min_size.maybe_resolve(parent_size).maybe_apply_aspect_ratio(aspect_ratio);
    let max_size = style.max_size.maybe_resolve(parent_size).maybe_apply_aspect_ratio(aspect_ratio);
    let clamped_style_size =
        style.size.maybe_resolve(parent_size).maybe_apply_aspect_ratio(aspect_ratio).maybe_clamp(min_size, max_size);

    // If both min and max in a given axis are set and max <= min then this determines the size in that axis
    let min_max_definite_size = min_size.zip_map(max_size, |min, max| match (min, max) {
        (Some(min), Some(max)) if max <= min => Some(min),
        _ => None,
    });
    let styled_based_known_dimensions = known_dimensions.or(min_max_definite_size).or(clamped_style_size);

    // Short-circuit layout if the container's size is fully determined by the container's size and the run mode
    // is ComputeSize (and thus the container's size is all that we're interested in)
    if run_mode == RunMode::ComputeSize {
        if let Size { width: Some(width), height: Some(height) } = styled_based_known_dimensions {
            return Size { width, height }.into();
        }
    }

    #[cfg(feature = "debug")]
    NODE_LOGGER.log("BLOCK");
    compute_inner(tree, node, styled_based_known_dimensions, parent_size, available_space, run_mode)
}

/// Computes the layout of [`LayoutTree`] according to the block layout algorithm
fn compute_inner(
    tree: &mut impl LayoutTree,
    node: NodeId,
    known_dimensions: Size<Option<f32>>,
    parent_size: Size<Option<f32>>,
    available_space: Size<AvailableSpace>,
    run_mode: RunMode,
) -> SizeAndBaselines {
    let style = tree.style(node);
    let margin = style.margin.resolve_or_zero(parent_size.width);
    let padding = style.padding.resolve_or_zero(parent_size.width);
    let border = style.border.resolve_or_zero(parent_size.width);

    // Scrollbar gutters are reserved when the `overflow` property is set to `Overflow::Scroll`.
    // However, the axis are switched (transposed) because a node that scrolls vertically needs
    // *horizontal* space to be reserved for a scrollbar
    let scrollbar_gutter = style.overflow.transpose().map(|overflow| match overflow {
        Overflow::Scroll => style.scrollbar_width,
        _ => 0.0,
    });
    // TODO: make side configurable based on the `direction` property
    let mut content_box_inset = padding + border;
    content_box_inset.right += scrollbar_gutter.x;
    content_box_inset.bottom += scrollbar_gutter.y;

    let container_border_box_size = known_dimensions;
    let container_content_box_size = container_border_box_size.maybe_sub(content_box_inset.sum_axes());

    let items = generate_item_list(&*tree, node, container_content_box_size);

    todo!()
}

/// Generate anonymous flex items.
///
/// # [9.1. Initial Setup](https://www.w3.org/TR/css-flexbox-1/#box-manip)
///
/// - [**Generate anonymous flex items**](https://www.w3.org/TR/css-flexbox-1/#algo-anon-box) as described in [§4 Flex Items](https://www.w3.org/TR/css-flexbox-1/#flex-items).
#[inline]
fn generate_item_list(tree: &impl LayoutTree, node: NodeId, node_inner_size: Size<Option<f32>>) -> Vec<BlockItem> {
    tree.children(node)
        .map(|child| (child, tree.style(child)))
        .filter(|(_, style)| style.position != Position::Absolute)
        .filter(|(_, style)| style.display != Display::None)
        .map(|(child, child_style)| {
            let aspect_ratio = child_style.aspect_ratio;
            BlockItem {
                node: child,
                size: child_style.size.maybe_resolve(node_inner_size).maybe_apply_aspect_ratio(aspect_ratio),
                min_size: child_style.min_size.maybe_resolve(node_inner_size).maybe_apply_aspect_ratio(aspect_ratio),
                max_size: child_style.max_size.maybe_resolve(node_inner_size).maybe_apply_aspect_ratio(aspect_ratio),

                inset: child_style.inset.zip_size(node_inner_size, |p, s| p.maybe_resolve(s)),
                margin: child_style.margin.resolve_or_zero(node_inner_size.width),
                margin_is_auto: child_style.margin.map(|m| m == LengthPercentageAuto::Auto),
                padding: child_style.padding.resolve_or_zero(node_inner_size.width),
                border: child_style.border.resolve_or_zero(node_inner_size.width),
                overflow: child_style.overflow,

                // Fields to be computed later (for now we initialise with dummy values)
                computed_content_box_size: Size::zero(),
                computed_border_box_size: Size::zero(),
                baseline: 0.0,
            }
        })
        .collect()
}
