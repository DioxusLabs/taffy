//! Computes size using styles and measure functions

use crate::geometry::{Line, Size};
use crate::style::{AvailableSpace, Overflow, Style};
use crate::tree::Measurable;
use crate::tree::{SizeBaselinesAndMargins, SizingMode};
use crate::util::sys::f32_max;
use crate::util::MaybeMath;
use crate::util::{MaybeResolve, ResolveOrZero};

#[cfg(feature = "debug")]
use crate::util::debug::NODE_LOGGER;

/// Perform full layout on a leaf node
pub(crate) fn perform_layout(
    style: &Style,
    measurable: Option<&impl Measurable>,
    known_dimensions: Size<Option<f32>>,
    parent_size: Size<Option<f32>>,
    available_space: Size<AvailableSpace>,
    sizing_mode: SizingMode,
    _vertical_margins_are_collapsible: Line<bool>,
) -> SizeBaselinesAndMargins {
    compute(style, measurable, known_dimensions, parent_size, available_space, sizing_mode)
}

/// Measure a leaf node's size
pub(crate) fn measure_size(
    style: &Style,
    measurable: Option<&impl Measurable>,
    known_dimensions: Size<Option<f32>>,
    parent_size: Size<Option<f32>>,
    available_space: Size<AvailableSpace>,
    sizing_mode: SizingMode,
    _vertical_margins_are_collapsible: Line<bool>,
) -> Size<f32> {
    compute(style, measurable, known_dimensions, parent_size, available_space, sizing_mode).size
}

/// Compute the size of a leaf node (node with no children)
pub fn compute(
    style: &Style,
    measurable: Option<&impl Measurable>,
    known_dimensions: Size<Option<f32>>,
    parent_size: Size<Option<f32>>,
    available_space: Size<AvailableSpace>,
    sizing_mode: SizingMode,
) -> SizeBaselinesAndMargins {
    // Resolve node's preferred/min/max sizes (width/heights) against the available space (percentages resolve to pixel values)
    // For ContentSize mode, we pretend that the node has no size styles as these should be ignored.
    let (node_size, node_min_size, node_max_size, aspect_ratio) = match sizing_mode {
        SizingMode::ContentSize => {
            let node_size = known_dimensions;
            let node_min_size = Size::NONE;
            let node_max_size = Size::NONE;
            (node_size, node_min_size, node_max_size, None)
        }
        SizingMode::InherentSize => {
            let aspect_ratio = style.aspect_ratio;
            let style_size = style.size.maybe_resolve(parent_size).maybe_apply_aspect_ratio(aspect_ratio);
            let style_min_size = style.min_size.maybe_resolve(parent_size).maybe_apply_aspect_ratio(aspect_ratio);
            let style_max_size = style.max_size.maybe_resolve(parent_size).maybe_apply_aspect_ratio(aspect_ratio);

            let node_size = known_dimensions.or(style_size);
            (node_size, style_min_size, style_max_size, aspect_ratio)
        }
    };

    // Note: both horizontal and vertical percentage padding/borders are resolved against the container's inline size (i.e. width).
    // This is not a bug, but is how CSS is specified (see: https://developer.mozilla.org/en-US/docs/Web/CSS/padding#values)
    let padding = style.padding.resolve_or_zero(parent_size.width);
    let border = style.border.resolve_or_zero(parent_size.width);
    let padding_border = padding + border;

    // Scrollbar gutters are reserved when the `overflow` property is set to `Overflow::Scroll`.
    // However, the axis are switched (transposed) because a node that scrolls vertically needs
    // *horizontal* space to be reserved for a scrollbar
    let scrollbar_gutter = style.overflow.transpose().map(|overflow| match overflow {
        Overflow::Scroll => style.scrollbar_width,
        _ => 0.0,
    });
    // TODO: make side configurable based on the `direction` property
    let mut content_box_inset = padding_border;
    content_box_inset.right += scrollbar_gutter.x;
    content_box_inset.bottom += scrollbar_gutter.y;

    #[cfg(feature = "debug")]
    NODE_LOGGER.log("LEAF");
    #[cfg(feature = "debug")]
    NODE_LOGGER.labelled_debug_log("node_size", node_size);
    #[cfg(feature = "debug")]
    NODE_LOGGER.labelled_debug_log("min_size ", node_min_size);
    #[cfg(feature = "debug")]
    NODE_LOGGER.labelled_debug_log("max_size ", node_max_size);

    // Return early if both width and height are known
    if let Size { width: Some(width), height: Some(height) } = node_size {
        let size = Size { width, height }
            .maybe_clamp(node_min_size, node_max_size)
            .maybe_max(padding_border.sum_axes().map(Some));
        return size.into();
    };

    if let Some(measurable) = measurable {
        // Compute available space
        let available_space = Size {
            width: available_space.width.maybe_set(node_size.width).maybe_set(node_max_size.width).map_definite_value(
                |size| {
                    size.maybe_clamp(node_min_size.width, node_max_size.width) - content_box_inset.horizontal_axis_sum()
                },
            ),
            height: available_space
                .height
                .maybe_set(node_size.height)
                .maybe_set(node_max_size.height)
                .map_definite_value(|size| {
                    size.maybe_clamp(node_min_size.height, node_max_size.height) - content_box_inset.vertical_axis_sum()
                }),
        };

        // Measure node
        let measured_size = measurable.measure(known_dimensions, available_space);

        let measured_size = Size {
            width: measured_size.width,
            height: f32_max(measured_size.height, aspect_ratio.map(|ratio| measured_size.width / ratio).unwrap_or(0.0)),
        };

        let size = node_size.unwrap_or(measured_size).maybe_clamp(node_min_size, node_max_size);
        let size = size.maybe_max(padding_border.sum_axes().map(Some));
        return size.into();
    }

    let size = Size {
        width: node_size
            .width
            // .unwrap_or(0.0) + padding.horizontal_axis_sum() + border.horizontal_axis_sum(), // content-box
            .unwrap_or(content_box_inset.horizontal_axis_sum()) // border-box
            .maybe_clamp(node_min_size.width, node_max_size.width)
            .maybe_max(padding_border.horizontal_axis_sum().into()),
        height: node_size
            .height
            // .unwrap_or(0.0) + padding.vertical_axis_sum() + border.vertical_axis_sum(), // content-box
            .unwrap_or(content_box_inset.vertical_axis_sum()) // border-box
            .maybe_clamp(node_min_size.height, node_max_size.height)
            .maybe_max(padding_border.vertical_axis_sum().into()),
    };

    let size = Size {
        width: f32_max(size.width, aspect_ratio.map(|ratio| size.height * ratio).unwrap_or(0.0)),
        height: f32_max(size.height, aspect_ratio.map(|ratio| size.width / ratio).unwrap_or(0.0)),
    };

    size.into()
}
