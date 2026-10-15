//! Axis helpers ported from `yoga/algorithm/FlexDirection.h`, `BoundAxis.h`, `Align.h` and the
//! `constrainMaxSizeForMode`/`calculateAvailableInnerDimension` helpers of `CalculateLayout.cpp`
//! at Yoga commit `08250431e09bc4b864917e51f9e9d52998ee2dce`.
//!
//! Values follow Yoga's conventions: `f32::NAN` is "undefined", and min/max constraints are
//! `Option<f32>` (Yoga's `FloatOptional`).

#![allow(dead_code)]

use crate::geometry::AbsoluteAxis;
use crate::style::{AlignContent, AlignItems, Direction, FlexDirection, JustifyContent};

use super::enums::YogaSizingMode;
use super::numeric::{is_defined, max_or_defined, min_or_defined};

/// One physical edge of a box (Yoga's `PhysicalEdge`)
#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub(crate) enum PhysicalEdge {
    /// The left edge
    Left,
    /// The top edge
    Top,
    /// The right edge
    Right,
    /// The bottom edge
    Bottom,
}

/// Yoga's `isRow`
#[inline]
pub(crate) fn is_row(flex_direction: FlexDirection) -> bool {
    matches!(flex_direction, FlexDirection::Row | FlexDirection::RowReverse)
}

/// Yoga's `isColumn`
#[inline]
pub(crate) fn is_column(flex_direction: FlexDirection) -> bool {
    matches!(flex_direction, FlexDirection::Column | FlexDirection::ColumnReverse)
}

/// Yoga's `resolveDirection`: swap row directions in RTL
#[inline]
pub(crate) fn resolve_direction(flex_direction: FlexDirection, direction: Direction) -> FlexDirection {
    match (direction, flex_direction) {
        (Direction::Rtl, FlexDirection::Row) => FlexDirection::RowReverse,
        (Direction::Rtl, FlexDirection::RowReverse) => FlexDirection::Row,
        _ => flex_direction,
    }
}

/// Yoga's `resolveCrossDirection`
#[inline]
pub(crate) fn resolve_cross_direction(flex_direction: FlexDirection, direction: Direction) -> FlexDirection {
    if is_column(flex_direction) {
        resolve_direction(FlexDirection::Row, direction)
    } else {
        FlexDirection::Column
    }
}

/// Yoga's `flexStartEdge`
#[inline]
pub(crate) fn flex_start_edge(flex_direction: FlexDirection) -> PhysicalEdge {
    match flex_direction {
        FlexDirection::Column => PhysicalEdge::Top,
        FlexDirection::ColumnReverse => PhysicalEdge::Bottom,
        FlexDirection::Row => PhysicalEdge::Left,
        FlexDirection::RowReverse => PhysicalEdge::Right,
    }
}

/// Yoga's `flexEndEdge`
#[inline]
pub(crate) fn flex_end_edge(flex_direction: FlexDirection) -> PhysicalEdge {
    match flex_direction {
        FlexDirection::Column => PhysicalEdge::Bottom,
        FlexDirection::ColumnReverse => PhysicalEdge::Top,
        FlexDirection::Row => PhysicalEdge::Right,
        FlexDirection::RowReverse => PhysicalEdge::Left,
    }
}

/// Yoga's `inlineStartEdge`
#[inline]
pub(crate) fn inline_start_edge(flex_direction: FlexDirection, direction: Direction) -> PhysicalEdge {
    if is_row(flex_direction) {
        if direction == Direction::Rtl {
            PhysicalEdge::Right
        } else {
            PhysicalEdge::Left
        }
    } else {
        PhysicalEdge::Top
    }
}

/// Yoga's `inlineEndEdge`
#[inline]
pub(crate) fn inline_end_edge(flex_direction: FlexDirection, direction: Direction) -> PhysicalEdge {
    if is_row(flex_direction) {
        if direction == Direction::Rtl {
            PhysicalEdge::Left
        } else {
            PhysicalEdge::Right
        }
    } else {
        PhysicalEdge::Bottom
    }
}

/// Yoga's `dimension`: the physical axis a flex direction runs along
#[inline]
pub(crate) fn dimension(flex_direction: FlexDirection) -> AbsoluteAxis {
    if is_row(flex_direction) {
        AbsoluteAxis::Horizontal
    } else {
        AbsoluteAxis::Vertical
    }
}

/// Yoga's `needsTrailingPosition`
#[inline]
pub(crate) fn needs_trailing_position(axis: FlexDirection) -> bool {
    matches!(axis, FlexDirection::RowReverse | FlexDirection::ColumnReverse)
}

/// Yoga's `getPositionOfOppositeEdge`
#[inline]
pub(crate) fn position_of_opposite_edge(position: f32, container_size: f32, child_size: f32) -> f32 {
    container_size - child_size - position
}

/// Yoga's `boundAxisWithinMinAndMax`, with the node's resolved min/max for the axis already
/// looked up. A constraint only applies when defined and non-negative; `value` may be undefined.
#[inline]
pub(crate) fn bound_axis_within_min_and_max(value: f32, min: Option<f32>, max: Option<f32>) -> f32 {
    if let Some(max) = max {
        if max >= 0.0 && value > max {
            return max;
        }
    }
    if let Some(min) = min {
        if min >= 0.0 && value < min {
            return min;
        }
    }
    value
}

/// Yoga's `boundAxis`: min/max, then never smaller than padding + border
#[inline]
pub(crate) fn bound_axis(value: f32, min: Option<f32>, max: Option<f32>, padding_and_border: f32) -> f32 {
    max_or_defined(bound_axis_within_min_and_max(value, min, max), padding_and_border)
}

/// Yoga's `fallbackAlignment(Align)` for `align-items`/`align-self`: `stretch` falls back to
/// flex-start (Yoga's distribution keywords have no `AlignItems` equivalent in Taffy)
#[inline]
pub(crate) fn fallback_align_items(align: AlignItems) -> AlignItems {
    if align == AlignItems::STRETCH {
        AlignItems::FLEX_START
    } else {
        align
    }
}

/// Yoga's `fallbackAlignment(Align)` for `align-content`
#[inline]
pub(crate) fn fallback_align_content(align: AlignContent) -> AlignContent {
    if align == AlignContent::SPACE_BETWEEN
        || align == AlignContent::STRETCH
        || align == AlignContent::SPACE_AROUND
        || align == AlignContent::SPACE_EVENLY
    {
        AlignContent::FLEX_START
    } else {
        align
    }
}

/// Yoga's `fallbackAlignment(Justify)`
#[inline]
pub(crate) fn fallback_justify_content(justify: JustifyContent) -> JustifyContent {
    if justify == JustifyContent::SPACE_BETWEEN
        || justify == JustifyContent::SPACE_AROUND
        || justify == JustifyContent::SPACE_EVENLY
    {
        JustifyContent::FLEX_START
    } else {
        justify
    }
}

/// Yoga's `constrainMaxSizeForMode`: cap a child's available size by its max size (+ margin).
///
/// `max_size_with_margin` is `resolvedMaxDimension + computeMarginForAxis` (`None` when the max is
/// undefined).
#[inline]
pub(crate) fn constrain_max_size_for_mode(
    max_size_with_margin: Option<f32>,
    mode: &mut YogaSizingMode,
    size: &mut f32,
) {
    match *mode {
        YogaSizingMode::StretchFit | YogaSizingMode::FitContent => {
            if let Some(max) = max_size_with_margin {
                if size.is_nan() || *size >= max {
                    *size = max;
                }
            }
        }
        YogaSizingMode::MaxContent => {
            if let Some(max) = max_size_with_margin {
                *mode = YogaSizingMode::FitContent;
                *size = max;
            }
        }
    }
}

/// Yoga's `calculateAvailableInnerDimension`: the available size inside padding and border, clamped
/// to the node's min/max (also made inner). `available` may be undefined.
#[inline]
pub(crate) fn calculate_available_inner_dimension(
    available: f32,
    padding_and_border: f32,
    min: Option<f32>,
    max: Option<f32>,
) -> f32 {
    let mut available_inner = available - padding_and_border;
    if is_defined(available_inner) {
        let min_inner = min.map(|min| min - padding_and_border).unwrap_or(0.0);
        let max_inner = max.map(|max| max - padding_and_border).unwrap_or(f32::MAX);
        available_inner = max_or_defined(min_or_defined(available_inner, max_inner), min_inner);
    }
    available_inner
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn directions_resolve_like_yoga() {
        assert_eq!(resolve_direction(FlexDirection::Row, Direction::Rtl), FlexDirection::RowReverse);
        assert_eq!(resolve_direction(FlexDirection::RowReverse, Direction::Rtl), FlexDirection::Row);
        assert_eq!(resolve_direction(FlexDirection::Column, Direction::Rtl), FlexDirection::Column);
        assert_eq!(resolve_cross_direction(FlexDirection::Column, Direction::Rtl), FlexDirection::RowReverse);
        assert_eq!(resolve_cross_direction(FlexDirection::RowReverse, Direction::Rtl), FlexDirection::Column);
        assert_eq!(inline_start_edge(FlexDirection::Row, Direction::Rtl), PhysicalEdge::Right);
        assert_eq!(inline_end_edge(FlexDirection::ColumnReverse, Direction::Rtl), PhysicalEdge::Bottom);
        assert_eq!(flex_start_edge(FlexDirection::ColumnReverse), PhysicalEdge::Bottom);
        assert_eq!(dimension(FlexDirection::RowReverse), AbsoluteAxis::Horizontal);
    }

    #[test]
    fn constrain_max_size_for_mode_matches_yoga() {
        let mut mode = YogaSizingMode::MaxContent;
        let mut size = f32::NAN;
        constrain_max_size_for_mode(Some(50.0), &mut mode, &mut size);
        assert_eq!((mode, size), (YogaSizingMode::FitContent, 50.0));

        let mut mode = YogaSizingMode::FitContent;
        let mut size = 80.0;
        constrain_max_size_for_mode(Some(50.0), &mut mode, &mut size);
        assert_eq!((mode, size), (YogaSizingMode::FitContent, 50.0));

        let mut size = f32::NAN;
        constrain_max_size_for_mode(Some(50.0), &mut mode, &mut size);
        assert_eq!(size, 50.0);

        let mut mode = YogaSizingMode::MaxContent;
        let mut size = f32::NAN;
        constrain_max_size_for_mode(None, &mut mode, &mut size);
        assert_eq!(mode, YogaSizingMode::MaxContent);
        assert!(size.is_nan());
    }

    #[test]
    fn available_inner_dimension_matches_yoga() {
        assert!(calculate_available_inner_dimension(f32::NAN, 10.0, None, None).is_nan());
        assert_eq!(calculate_available_inner_dimension(100.0, 10.0, None, None), 90.0);
        assert_eq!(calculate_available_inner_dimension(100.0, 10.0, Some(120.0), None), 110.0);
        assert_eq!(calculate_available_inner_dimension(100.0, 10.0, None, Some(50.0)), 40.0);
        assert_eq!(calculate_available_inner_dimension(5.0, 10.0, None, None), 0.0);
    }
}
