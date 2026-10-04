//! Computes size using styles and measure functions

#[cfg(feature = "content_size")]
use crate::geometry::Rect;
use crate::geometry::Size;
use crate::style::{AvailableSpace, Overflow};
use crate::tree::{Baselines, CollapsibleMarginSet, RunMode};
use crate::tree::{LayoutInput, LayoutOutput};
use crate::util::debug::debug_log;
use crate::util::MaybeMath;
use crate::util::ResolveOrZero;
use crate::CoreStyle;
use core::unreachable;

/// Compute the size of a leaf node (node with no children)
///
/// A definite `inputs.available_space` is treated as the space available to the node's border box:
/// the caller is expected to have already subtracted the node's margins from it.
pub fn compute_leaf_layout<MeasureFunction>(
    inputs: LayoutInput,
    style: &impl CoreStyle,
    resolve_calc_value: impl Fn(*const (), f32) -> f32,
    measure_function: MeasureFunction,
) -> LayoutOutput
where
    MeasureFunction: FnOnce(Size<Option<f32>>, Size<AvailableSpace>) -> Size<f32>,
{
    let LayoutInput { known_dimensions, parent_size, available_space, run_mode, .. } = inputs;

    // Note: both horizontal and vertical percentage padding/borders are resolved against the container's inline size (i.e. width).
    // This is not a bug, but is how CSS is specified (see: https://developer.mozilla.org/en-US/docs/Web/CSS/padding#values)
    let padding = style.padding().resolve_or_zero(parent_size.width, &resolve_calc_value);
    let border = style.border().resolve_or_zero(parent_size.width, &resolve_calc_value);
    let padding_border = padding + border;
    // A leaf node never applies its own preferred/min/max size or aspect-ratio styles: these are
    // resolved by its parent, which passes the result down as known dimensions and clamps the
    // size that the leaf reports.
    //
    // Consequently the only sizes that a leaf node knows about are the known dimensions that it
    // is passed and the size of its content (as determined by the measure function).

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
        || matches!(known_dimensions.height, Some(h) if h > 0.0);

    debug_log!("LEAF");
    debug_log!("known_dimensions", dbg:known_dimensions);

    // Return early if both width and height are known
    if run_mode == RunMode::ComputeSize && has_styles_preventing_being_collapsed_through {
        if let Size { width: Some(width), height: Some(height) } = known_dimensions {
            let size = Size { width, height }.maybe_max(padding_border.sum_axes().map(Some));
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

    // Compute available space
    let available_space = Size {
        width: known_dimensions
            .width
            .map(AvailableSpace::from)
            .unwrap_or(available_space.width)
            .maybe_set(known_dimensions.width)
            .map_definite_value(|size| size - content_box_inset.horizontal_axis_sum()),
        height: known_dimensions
            .height
            .map(AvailableSpace::from)
            .unwrap_or(available_space.height)
            .maybe_set(known_dimensions.height)
            .map_definite_value(|size| size - content_box_inset.vertical_axis_sum()),
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
    let size = known_dimensions
        .unwrap_or(measured_size + content_box_inset.sum_axes())
        .maybe_max(padding_border.sum_axes().map(Some));

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
