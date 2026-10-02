//! Computes size using styles and measure functions

use crate::compute::ratio::{derived_height_is_minimum, BoxSizes, Ratio};
#[cfg(feature = "content_size")]
use crate::geometry::Rect;
use crate::geometry::Size;
use crate::style::{AvailableSpace, Overflow};
use crate::tree::{Baselines, CollapsibleMarginSet, RunMode};
use crate::tree::{LayoutInput, LayoutOutput, SizingMode};
use crate::util::debug::debug_log;
use crate::util::sys::f32_max;
use crate::util::MaybeMath;
use crate::util::{MaybeResolve, ResolveOrZero};
use crate::{BoxSizing, CoreStyle};
use core::unreachable;

/// Compute the size of a leaf node (node with no children)
pub fn compute_leaf_layout<MeasureFunction>(
    inputs: LayoutInput,
    style: &impl CoreStyle,
    resolve_calc_value: impl Fn(*const (), f32) -> f32,
    measure_function: MeasureFunction,
) -> LayoutOutput
where
    MeasureFunction: FnOnce(Size<Option<f32>>, Size<AvailableSpace>) -> Size<f32>,
{
    let LayoutInput { known_dimensions, parent_size, available_space, sizing_mode, run_mode, .. } = inputs;

    // Note: both horizontal and vertical percentage padding/borders are resolved against the container's inline size (i.e. width).
    // This is not a bug, but is how CSS is specified (see: https://developer.mozilla.org/en-US/docs/Web/CSS/padding#values)
    let margin = style.margin().resolve_or_zero(parent_size.width, &resolve_calc_value);
    let padding = style.padding().resolve_or_zero(parent_size.width, &resolve_calc_value);
    let border = style.border().resolve_or_zero(parent_size.width, &resolve_calc_value);
    let padding_border = padding + border;
    let pb_sum = padding_border.sum_axes();
    let box_sizing_adjustment = if style.box_sizing() == BoxSizing::ContentBox { pb_sum } else { Size::ZERO };

    // Resolve node's preferred/min/max sizes (width/heights) against the available space (percentages resolve to pixel values)
    // For ContentSize mode, we pretend that the node has no size styles as these should be ignored.
    let (given_size, style_min_size, style_max_size) = match sizing_mode {
        SizingMode::ContentSize => (known_dimensions, Size::NONE, Size::NONE),
        SizingMode::InherentSize => {
            let style_size =
                style.size().maybe_resolve(parent_size, &resolve_calc_value).maybe_add(box_sizing_adjustment);
            let style_min_size =
                style.min_size().maybe_resolve(parent_size, &resolve_calc_value).maybe_add(box_sizing_adjustment);
            let style_max_size =
                style.max_size().maybe_resolve(parent_size, &resolve_calc_value).maybe_add(box_sizing_adjustment);
            (known_dimensions.or(style_size), style_min_size, style_max_size)
        }
    };

    // Resolve the sizes through the node's preferred aspect ratio (if it has one). As with the size
    // styles, the ratio is ignored in ContentSize mode.
    let ratio = match sizing_mode {
        SizingMode::ContentSize => None,
        SizingMode::InherentSize => Ratio::of(style, pb_sum),
    };
    let derived_height_is_minimum = derived_height_is_minimum(style, style_min_size.height);
    let BoxSizes { size: node_size, min_size: node_min_size, max_size: node_max_size } = match ratio {
        Some(ratio) => ratio.resolve(given_size, style_min_size, style_max_size, derived_height_is_minimum),
        None => BoxSizes { size: given_size, min_size: style_min_size, max_size: style_max_size },
    };

    // Scrollbar gutters are reserved when the `overflow` property is set to `Overflow::Scroll`.
    // However, the axis are switched (transposed) because a node that scrolls vertically needs
    // *horizontal* space to be reserved for a scrollbar
    let scrollbar_gutter = style.overflow().transpose().map(|overflow| match overflow {
        Overflow::Scroll => style.scrollbar_width(),
        _ => 0.0,
    });
    // TODO: make side configurable based on the `direction` property
    let mut content_box_inset = padding_border;
    content_box_inset.right += scrollbar_gutter.x;
    content_box_inset.bottom += scrollbar_gutter.y;

    let has_styles_preventing_being_collapsed_through = !style.is_block()
        || style.overflow().x.is_scroll_container()
        || style.overflow().y.is_scroll_container()
        || style.position().is_out_of_flow()
        || style.contain().establishes_independent_formatting_context()
        || padding.top > 0.0
        || padding.bottom > 0.0
        || border.top > 0.0
        || border.bottom > 0.0
        || matches!(node_size.height, Some(h) if h > 0.0)
        || matches!(node_min_size.height, Some(h) if h > 0.0);

    debug_log!("LEAF");
    debug_log!("node_size", dbg:node_size);
    debug_log!("min_size ", dbg:node_min_size);
    debug_log!("max_size ", dbg:node_max_size);

    // Return early if both width and height are known
    if run_mode == RunMode::ComputeSize && has_styles_preventing_being_collapsed_through {
        if let Size { width: Some(width), height: Some(height) } = node_size {
            let size = Size { width, height }
                .maybe_clamp(node_min_size, node_max_size)
                .maybe_max(padding_border.sum_axes().map(Some));
            return LayoutOutput {
                size,
                #[cfg(feature = "content_size")]
                scrollable_overflow_rect: Rect::ZERO,
                baselines: Baselines::NONE,
                top_margin: CollapsibleMarginSet::ZERO,
                bottom_margin: CollapsibleMarginSet::ZERO,
                margins_can_collapse_through: false,
                oof_candidates: crate::tree::OofCandidates::NONE,
                oof_positioning_area: None,
            };
        };
    }

    // A ratio-derived height is the preferred wrapping offer even when the
    // automatic content minimum may make the final box taller. In particular,
    // vertical text must not choose its inline extent from an unrelated parent's
    // height offer. Keep this a measurement offer, not a maximum on content.
    let measurement_height = node_size.height.or_else(|| {
        if derived_height_is_minimum {
            ratio.zip(node_size.width).map(|(ratio, width)| {
                ratio
                    .height(width.maybe_clamp(node_min_size.width, node_max_size.width))
                    .maybe_min(node_max_size.height)
            })
        } else {
            None
        }
    });

    // Compute available space
    let available_space = Size {
        width: known_dimensions
            .width
            .map(AvailableSpace::from)
            .unwrap_or(available_space.width)
            .maybe_sub(margin.horizontal_axis_sum())
            .maybe_set(known_dimensions.width)
            .maybe_set(node_size.width)
            .map_definite_value(|size| {
                size.maybe_clamp(node_min_size.width, node_max_size.width) - content_box_inset.horizontal_axis_sum()
            }),
        height: known_dimensions
            .height
            .map(AvailableSpace::from)
            .unwrap_or(available_space.height)
            .maybe_sub(margin.vertical_axis_sum())
            .maybe_set(known_dimensions.height)
            .maybe_set(measurement_height)
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
    // If the node has a preferred aspect ratio and neither of its dimensions are given, then its
    // height is derived from its content-based width. (A given dimension has already been resolved
    // through the ratio above.)
    let size = match ratio {
        Some(ratio) if node_size.width.is_none() && node_size.height.is_none() => {
            let derived_height = ratio.height(clamped_size.width);
            let height = if derived_height_is_minimum {
                f32_max(clamped_size.height, derived_height.maybe_min(node_max_size.height))
            } else if style.is_compressible_replaced() {
                f32_max(clamped_size.height, derived_height)
            } else {
                derived_height
            };
            Size { width: clamped_size.width, height: height.maybe_clamp(node_min_size.height, node_max_size.height) }
        }
        _ => clamped_size,
    };
    let size = size.maybe_max(padding_border.sum_axes().map(Some));

    // A scroll container's own padding at the end of the content is part of its scrollable
    // overflow region, so it is included in the overflow rect. Boxes that are not scroll
    // containers do not extend their overflow region by their own padding.
    #[cfg(feature = "content_size")]
    let scrollable_overflow_rect = {
        let is_scroll_container = style.overflow().x.is_scroll_container() || style.overflow().y.is_scroll_container();
        let is_rtl = style.direction().is_rtl();
        let start_padding = if is_rtl { padding.right } else { padding.left };
        let end_padding = if is_rtl { padding.left } else { padding.right };
        Rect {
            left: 0.0,
            right: start_padding + measured_size.width + if is_scroll_container { end_padding } else { 0.0 },
            top: 0.0,
            bottom: padding.top + measured_size.height + if is_scroll_container { padding.bottom } else { 0.0 },
        }
    };

    LayoutOutput {
        size,
        #[cfg(feature = "content_size")]
        scrollable_overflow_rect,
        baselines: Baselines::NONE,
        top_margin: CollapsibleMarginSet::ZERO,
        bottom_margin: CollapsibleMarginSet::ZERO,
        margins_can_collapse_through: !has_styles_preventing_being_collapsed_through
            && size.height == 0.0
            && measured_size.height == 0.0,
        oof_candidates: crate::tree::OofCandidates::NONE,
        oof_positioning_area: None,
    }
}
