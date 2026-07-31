//! Layout of a single absolutely positioned child against its containing block.
//!
//! This is the CSS2 "absolutely positioned, non-replaced elements" sizing/positioning
//! algorithm as used by block containers. It is exposed publicly so that tree implementations
//! can lay out children which were deferred via
//! [`LayoutPartialTree::defer_absolute_child`](crate::LayoutPartialTree::defer_absolute_child)
//! against their actual containing block.
use crate::geometry::Line;
use crate::geometry::{Point, Rect, Size};
use crate::style::{AvailableSpace, BoxSizing, CoreStyle, Direction, Overflow, Position};
use crate::tree::{Layout, LayoutPartialTree, LayoutPartialTreeExt, NodeId, SizingMode};
use crate::util::sys::f32_max;
use crate::util::MaybeMath;
use crate::util::MaybeResolve;
use crate::util::ResolveOrZero;

/// The result of [`compute_absolute_child_layout`]
pub struct AbsoluteChildLayout {
    /// The layout that was set on the child (location is relative to the containing block's border box)
    pub layout: Layout,
    /// The child's content size (for scrollable overflow computation)
    pub content_size: Size<f32>,
    /// The child's overflow style
    pub overflow: Point<Overflow>,
}

/// Size and position a single absolutely positioned child against a containing block.
///
/// - `area_size` is the size of the containing block's positioning area (the padding box,
///   i.e. border box minus borders and scrollbar gutters).
/// - `area_offset` is the offset of the positioning area from the containing block's border box
///   origin (i.e. border + scrollbar gutter on the start sides).
/// - `static_position` is the child's static position, relative to the containing block's
///   border box origin. If `static_position_direction` is [`Direction::Rtl`] then its `x`
///   coordinate is the static position of the child's *right* edge, otherwise its left edge.
/// - `direction` is the `direction` style (LTR/RTL) of the containing block.
///
/// The child's unrounded layout is written to the tree with a location relative to the
/// containing block's border box, and also returned.
#[allow(clippy::too_many_arguments)]
pub fn compute_absolute_child_layout(
    tree: &mut impl LayoutPartialTree,
    child_id: NodeId,
    order: u32,
    area_size: Size<f32>,
    area_offset: Point<f32>,
    static_position: Point<f32>,
    static_position_direction: Direction,
    direction: Direction,
) -> AbsoluteChildLayout {
    let area_width = area_size.width;
    let area_height = area_size.height;

    let child_style = tree.get_core_container_style(child_id);

    let aspect_ratio = child_style.aspect_ratio();
    let overflow = child_style.overflow();
    let scrollbar_width = child_style.scrollbar_width();
    let margin =
        child_style.margin().map(|margin| margin.resolve_to_option(area_width, |val, basis| tree.calc(val, basis)));
    let padding = child_style.padding().resolve_or_zero(Some(area_width), |val, basis| tree.calc(val, basis));
    let border = child_style.border().resolve_or_zero(Some(area_width), |val, basis| tree.calc(val, basis));
    let padding_border_sum = (padding + border).sum_axes();
    let box_sizing_adjustment =
        if child_style.box_sizing() == BoxSizing::ContentBox { padding_border_sum } else { Size::ZERO };

    // Resolve inset
    let left = child_style.inset().left.maybe_resolve(area_width, |val, basis| tree.calc(val, basis));
    let right = child_style.inset().right.maybe_resolve(area_width, |val, basis| tree.calc(val, basis));
    let top = child_style.inset().top.maybe_resolve(area_height, |val, basis| tree.calc(val, basis));
    let bottom = child_style.inset().bottom.maybe_resolve(area_height, |val, basis| tree.calc(val, basis));

    // Compute known dimensions from min/max/inherent size styles
    let style_size = child_style
        .size()
        .maybe_resolve(area_size, |val, basis| tree.calc(val, basis))
        .maybe_apply_aspect_ratio(aspect_ratio)
        .maybe_add(box_sizing_adjustment);
    let min_size = child_style
        .min_size()
        .maybe_resolve(area_size, |val, basis| tree.calc(val, basis))
        .maybe_apply_aspect_ratio(aspect_ratio)
        .maybe_add(box_sizing_adjustment)
        .or(padding_border_sum.map(Some))
        .maybe_max(padding_border_sum);
    let max_size = child_style
        .max_size()
        .maybe_resolve(area_size, |val, basis| tree.calc(val, basis))
        .maybe_apply_aspect_ratio(aspect_ratio)
        .maybe_add(box_sizing_adjustment);
    let mut known_dimensions = style_size.maybe_clamp(min_size, max_size);

    drop(child_style);

    // Fill in width from left/right and reapply aspect ratio if:
    //   - Width is not already known
    //   - Item has both left and right inset properties set
    if let (None, Some(left), Some(right)) = (known_dimensions.width, left, right) {
        let new_width_raw = area_width.maybe_sub(margin.left).maybe_sub(margin.right) - left - right;
        known_dimensions.width = Some(f32_max(new_width_raw, 0.0));
        known_dimensions = known_dimensions.maybe_apply_aspect_ratio(aspect_ratio).maybe_clamp(min_size, max_size);
    }

    // Fill in height from top/bottom and reapply aspect ratio if:
    //   - Height is not already known
    //   - Item has both top and bottom inset properties set
    if let (None, Some(top), Some(bottom)) = (known_dimensions.height, top, bottom) {
        let new_height_raw = area_height.maybe_sub(margin.top).maybe_sub(margin.bottom) - top - bottom;
        known_dimensions.height = Some(f32_max(new_height_raw, 0.0));
        known_dimensions = known_dimensions.maybe_apply_aspect_ratio(aspect_ratio).maybe_clamp(min_size, max_size);
    }

    let measured_size = tree.measure_child_size_both(
        child_id,
        known_dimensions,
        area_size.map(Some),
        Size {
            width: AvailableSpace::Definite(area_width.maybe_clamp(min_size.width, max_size.width)),
            height: AvailableSpace::Definite(area_height.maybe_clamp(min_size.height, max_size.height)),
        },
        SizingMode::ContentSize,
        Line::FALSE,
    );

    let final_size = known_dimensions.unwrap_or(measured_size).maybe_clamp(min_size, max_size);

    let layout_output = tree.perform_child_layout(
        child_id,
        final_size.map(Some),
        area_size.map(Some),
        Size {
            width: AvailableSpace::Definite(area_width.maybe_clamp(min_size.width, max_size.width)),
            height: AvailableSpace::Definite(area_height.maybe_clamp(min_size.height, max_size.height)),
        },
        SizingMode::ContentSize,
        Line::FALSE,
    );

    let non_auto_margin = Rect {
        left: if left.is_some() { margin.left.unwrap_or(0.0) } else { 0.0 },
        right: if right.is_some() { margin.right.unwrap_or(0.0) } else { 0.0 },
        top: if top.is_some() { margin.top.unwrap_or(0.0) } else { 0.0 },
        bottom: if bottom.is_some() { margin.bottom.unwrap_or(0.0) } else { 0.0 },
    };

    // Expand auto margins to fill available space
    // https://www.w3.org/TR/CSS21/visudet.html#abs-non-replaced-width
    let auto_margin = {
        // Auto margins for absolutely positioned elements in block containers only resolve
        // if inset is set. Otherwise they resolve to 0.
        let absolute_auto_margin_space = Point {
            x: right.map(|right| area_size.width - right - left.unwrap_or(0.0)).unwrap_or(final_size.width),
            y: bottom.map(|bottom| area_size.height - bottom - top.unwrap_or(0.0)).unwrap_or(final_size.height),
        };
        let free_space = Size {
            width: absolute_auto_margin_space.x - final_size.width - non_auto_margin.horizontal_axis_sum(),
            height: absolute_auto_margin_space.y - final_size.height - non_auto_margin.vertical_axis_sum(),
        };

        let auto_margin_size = Size {
            width: {
                let auto_margin_count = margin.left.is_none() as u8 + margin.right.is_none() as u8;
                if auto_margin_count == 2
                    && (style_size.width.is_none() || style_size.width.unwrap() >= free_space.width)
                {
                    0.0
                } else if auto_margin_count > 0 {
                    free_space.width / auto_margin_count as f32
                } else {
                    0.0
                }
            },
            height: {
                let auto_margin_count = margin.top.is_none() as u8 + margin.bottom.is_none() as u8;
                if auto_margin_count == 2
                    && (style_size.height.is_none() || style_size.height.unwrap() >= free_space.height)
                {
                    0.0
                } else if auto_margin_count > 0 {
                    free_space.height / auto_margin_count as f32
                } else {
                    0.0
                }
            },
        };

        Rect {
            left: margin.left.map(|_| 0.0).unwrap_or(auto_margin_size.width),
            right: margin.right.map(|_| 0.0).unwrap_or(auto_margin_size.width),
            top: margin.top.map(|_| 0.0).unwrap_or(auto_margin_size.height),
            bottom: margin.bottom.map(|_| 0.0).unwrap_or(auto_margin_size.height),
        }
    };

    let resolved_margin = Rect {
        left: margin.left.unwrap_or(auto_margin.left),
        right: margin.right.unwrap_or(auto_margin.right),
        top: margin.top.unwrap_or(auto_margin.top),
        bottom: margin.bottom.unwrap_or(auto_margin.bottom),
    };

    let x_offset = match (left, right) {
        (Some(left), Some(right)) => {
            if direction.is_rtl() {
                area_size.width - final_size.width - right - resolved_margin.right
            } else {
                left + resolved_margin.left
            }
        }
        (Some(left), None) => left + resolved_margin.left,
        (None, Some(right)) => area_size.width - final_size.width - right - resolved_margin.right,
        (None, None) => {
            if static_position_direction.is_rtl() {
                static_position.x - final_size.width - resolved_margin.right - area_offset.x
            } else {
                static_position.x + resolved_margin.left - area_offset.x
            }
        }
    };
    let location = Point {
        x: x_offset + area_offset.x,
        y: top
            .map(|top| top + resolved_margin.top)
            .or(bottom.map(|bottom| area_size.height - final_size.height - bottom - resolved_margin.bottom))
            .maybe_add(area_offset.y)
            .unwrap_or(static_position.y + resolved_margin.top),
    };
    // Note: axis intentionally switched here as scrollbars take up space in the opposite axis
    // to the axis in which scrolling is enabled.
    let scrollbar_size = Size {
        width: if overflow.y == Overflow::Scroll { scrollbar_width } else { 0.0 },
        height: if overflow.x == Overflow::Scroll { scrollbar_width } else { 0.0 },
    };

    let layout = Layout {
        order,
        size: final_size,
        #[cfg(feature = "content_size")]
        content_size: layout_output.content_size,
        scrollbar_size,
        location,
        padding,
        border,
        margin: resolved_margin,
    };
    tree.set_unrounded_layout(child_id, &layout);

    #[cfg(feature = "content_size")]
    let content_size = layout_output.content_size;
    #[cfg(not(feature = "content_size"))]
    let content_size = Size::ZERO;

    AbsoluteChildLayout { layout, content_size, overflow }
}

/// Whether `Position::Absolute` children of a container with the given position style should be
/// deferred to the tree implementation (rather than laid out by the container itself).
#[inline(always)]
pub(crate) fn should_defer_absolute_child(child_position: Position, container_position: Position) -> bool {
    child_position == Position::Fixed || container_position == Position::Static
}
