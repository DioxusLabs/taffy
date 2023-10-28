//! Computes size using styles and measure functions

use crate::geometry::{Point, Size};
use crate::style::{AvailableSpace, Display, Overflow, Position, Style};
use crate::tree::{CollapsibleMarginSet, RunMode};
use crate::tree::{LayoutInput, LayoutOutput, SizingMode};
use crate::util::debug::debug_log;
use crate::util::sys::f32_max;
use crate::util::MaybeMath;
use crate::util::{MaybeResolve, ResolveOrZero};
use core::unreachable;

/// Compute the size of a leaf node (node with no children)
pub fn compute_leaf_layout<MeasureFunction>(
    inputs: LayoutInput,
    style: &Style,
    measure_function: Option<MeasureFunction>,
) -> LayoutOutput
where
    MeasureFunction: FnOnce(Size<Option<f32>>, Size<AvailableSpace>) -> Size<f32>,
{
    let LayoutInput { known_dimensions, parent_size, available_space, sizing_mode, run_mode, .. } = inputs;

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
            let style_max_size = style.max_size.maybe_resolve(parent_size);

            let node_size = known_dimensions.or(style_size);
            (node_size, style_min_size, style_max_size, aspect_ratio)
        }
    };

    // Note: both horizontal and vertical percentage padding/borders are resolved against the container's inline size (i.e. width).
    // This is not a bug, but is how CSS is specified (see: https://developer.mozilla.org/en-US/docs/Web/CSS/padding#values)
    let margin = style.margin.resolve_or_zero(parent_size.width);
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

    #[cfg(feature = "block_layout")]
    let is_block = style.display == Display::Block;
    #[cfg(not(feature = "block_layout"))]
    let is_block = false;

    let has_styles_preventing_being_collapsed_through = !is_block
        || style.overflow.x.is_scroll_container()
        || style.overflow.y.is_scroll_container()
        || style.position == Position::Absolute
        || padding.top > 0.0
        || padding.bottom > 0.0
        || border.top > 0.0
        || border.bottom > 0.0;

    debug_log!("LEAF");
    debug_log!("node_size", dbg:node_size);
    debug_log!("min_size ", dbg:node_min_size);
    debug_log!("max_size ", dbg:node_max_size);

    // Return early if both width and height are known
    if run_mode == RunMode::ComputeSize {
        if let Size { width: Some(width), height: Some(height) } = node_size {
            let size = Size { width, height }
                .maybe_clamp(node_min_size, node_max_size)
                .maybe_max(padding_border.sum_axes().map(Some));
            return LayoutOutput {
                size,
                content_size: Size::ZERO,
                first_baselines: Point::NONE,
                top_margin: CollapsibleMarginSet::ZERO,
                bottom_margin: CollapsibleMarginSet::ZERO,
                margins_can_collapse_through: !has_styles_preventing_being_collapsed_through
                    && size.height == 0.0
                    && measure_function.is_none(),
            };
        };
    }

    if let Some(measure_function) = measure_function {
        // Compute available space
        let available_space = Size {
            width: known_dimensions
                .width
                .map(AvailableSpace::from)
                .unwrap_or(available_space.width)
                .maybe_sub(margin.horizontal_axis_sum())
                .maybe_set(known_dimensions.width)
                .maybe_set(node_size.width)
                .maybe_set(node_max_size.width)
                .map_definite_value(|size| {
                    size.maybe_clamp(node_min_size.width, node_max_size.width) - content_box_inset.horizontal_axis_sum()
                }),
            height: known_dimensions
                .height
                .map(AvailableSpace::from)
                .unwrap_or(available_space.height)
                .maybe_sub(margin.vertical_axis_sum())
                .maybe_set(known_dimensions.height)
                .maybe_set(node_size.height)
                .maybe_set(node_max_size.height)
                .map_definite_value(|size| {
                    size.maybe_clamp(node_min_size.height, node_max_size.height) - content_box_inset.vertical_axis_sum()
                }),
        };

        // Measure node
        let measured_size = measure_function(
            match run_mode {
                RunMode::ComputeSize => known_dimensions,
                RunMode::PerformLayout => Size::NONE,
                RunMode::PerformHiddenLayout => unreachable!(),
            },
            available_space,
        );
        let clamped_size = known_dimensions
            .or(node_size)
            .unwrap_or(measured_size + content_box_inset.sum_axes())
            .maybe_clamp(node_min_size, node_max_size);
        let size = Size {
            width: clamped_size.width,
            height: f32_max(clamped_size.height, aspect_ratio.map(|ratio| clamped_size.width / ratio).unwrap_or(0.0)),
        };
        let size = size.maybe_max(padding_border.sum_axes().map(Some));

        return LayoutOutput {
            size,
            content_size: measured_size,
            first_baselines: Point::NONE,
            top_margin: CollapsibleMarginSet::ZERO,
            bottom_margin: CollapsibleMarginSet::ZERO,
            margins_can_collapse_through: !has_styles_preventing_being_collapsed_through
                && size.height == 0.0
                && measured_size.height == 0.0,
        };
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

    LayoutOutput {
        size,
        content_size: padding_border.sum_axes(),
        first_baselines: Point::NONE,
        top_margin: CollapsibleMarginSet::ZERO,
        bottom_margin: CollapsibleMarginSet::ZERO,
        margins_can_collapse_through: !has_styles_preventing_being_collapsed_through && size.height == 0.0,
    }
}
