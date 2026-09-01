//! Shared resolution logic for sizing keywords (`min-content`, `max-content`, `fit-content`,
//! `fit-content(...)`, and `stretch`) on the `width`/`height` style properties
use crate::geometry::{AbsoluteAxis, Line, Rect, Size};
use crate::tree::{LayoutPartialTree, LayoutPartialTreeExt, NodeId, SizingMode};
use crate::util::sys::f32_max;
use crate::util::OptF32;
use crate::AvailableSpace;
use crate::{CompactLength, Dimension};

/// How a sizing keyword resolves to a used size
pub(crate) enum SizingKeywordResolution {
    /// The size is the result of measuring the item under the given available space constraint
    Measure(AvailableSpace),
    /// The size resolves to an exact value without measuring the item
    Exact(f32),
}

/// Resolve an item's size style in one axis if it is a sizing keyword (`min-content`,
/// `max-content`, `fit-content`, `fit-content(...)`, or `stretch`).
///
/// - `stretch_size` is the size the item would take if stretched to fill the available space
///   (available space minus margins). Used by `fit-content` and `stretch`.
/// - `percent_resolution_basis` is the size that percentages resolve against in this axis.
///   Used by `fit-content(<percentage>)`.
///
/// Returns `None` if the size style is not a sizing keyword, or if it cannot
/// be resolved in the current context (in which case it behaves as `auto`).
#[inline]
pub(crate) fn resolve_sizing_keyword(
    style: Dimension,
    stretch_size: OptF32,
    percent_resolution_basis: OptF32,
) -> Option<SizingKeywordResolution> {
    match style.tag() {
        CompactLength::MIN_CONTENT_TAG => Some(SizingKeywordResolution::Measure(AvailableSpace::MinContent)),
        CompactLength::MAX_CONTENT_TAG => Some(SizingKeywordResolution::Measure(AvailableSpace::MaxContent)),
        CompactLength::FIT_CONTENT_PX_TAG => {
            Some(SizingKeywordResolution::Measure(AvailableSpace::Definite(style.value())))
        }
        CompactLength::FIT_CONTENT_PERCENT_TAG => percent_resolution_basis
            .into_option()
            .map(|basis| SizingKeywordResolution::Measure(AvailableSpace::Definite(basis * style.value()))),
        CompactLength::FIT_CONTENT_KEYWORD_TAG => {
            stretch_size.into_option().map(|size| SizingKeywordResolution::Measure(AvailableSpace::Definite(size)))
        }
        CompactLength::STRETCH_TAG => stretch_size.into_option().map(SizingKeywordResolution::Exact),
        _ => None,
    }
}

/// Resolve the sizing keywords (`min-content`, `max-content`, `fit-content`, `fit-content(...)`,
/// and `stretch`) on the size styles of an absolutely positioned item, filling in the
/// corresponding `known_dimensions` axes.
///
/// - `area_size` is the size of the item's containing block (which insets and percentages
///   resolve against).
/// - The stretch size in each axis is the containing block minus the item's insets and margins
///   in that axis.
#[allow(clippy::too_many_arguments)]
pub(crate) fn resolve_absolute_sizing_keywords(
    tree: &mut impl LayoutPartialTree,
    node: NodeId,
    known_dimensions: &mut Size<OptF32>,
    size_style: Size<Dimension>,
    area_size: Size<f32>,
    inset: Rect<OptF32>,
    margin: Rect<OptF32>,
    sizing_mode: SizingMode,
) {
    let stretch_size = Size {
        width: f32_max(
            area_size.width
                - inset.left.unwrap_or(0.0)
                - inset.right.unwrap_or(0.0)
                - margin.left.unwrap_or(0.0)
                - margin.right.unwrap_or(0.0),
            0.0,
        ),
        height: f32_max(
            area_size.height
                - inset.top.unwrap_or(0.0)
                - inset.bottom.unwrap_or(0.0)
                - margin.top.unwrap_or(0.0)
                - margin.bottom.unwrap_or(0.0),
            0.0,
        ),
    };

    let keyword_width = if known_dimensions.width.is_none() {
        resolve_sizing_keyword(size_style.width, OptF32::some(stretch_size.width), OptF32::some(area_size.width))
    } else {
        None
    };
    let keyword_height = if known_dimensions.height.is_none() {
        resolve_sizing_keyword(size_style.height, OptF32::some(stretch_size.height), OptF32::some(area_size.height))
    } else {
        None
    };

    match (keyword_width, keyword_height) {
        // If both axes need to be measured then resolve them with a single measure call
        (
            Some(SizingKeywordResolution::Measure(available_width)),
            Some(SizingKeywordResolution::Measure(available_height)),
        ) => {
            let measured_size = tree.measure_child_size_both(
                node,
                Size::NONE,
                area_size.map(OptF32::some),
                Size { width: available_width, height: available_height },
                sizing_mode,
                Line::FALSE,
            );
            *known_dimensions = measured_size.map(OptF32::some);
        }
        (keyword_width, keyword_height) => {
            if let Some(resolution) = keyword_width {
                known_dimensions.width = OptF32::some(match resolution {
                    SizingKeywordResolution::Exact(width) => width,
                    SizingKeywordResolution::Measure(available_width) => tree.measure_child_size(
                        node,
                        *known_dimensions,
                        area_size.map(OptF32::some),
                        Size { width: available_width, height: AvailableSpace::Definite(stretch_size.height) },
                        sizing_mode,
                        AbsoluteAxis::Horizontal,
                        Line::FALSE,
                    ),
                });
            }
            if let Some(resolution) = keyword_height {
                known_dimensions.height = OptF32::some(match resolution {
                    SizingKeywordResolution::Exact(height) => height,
                    SizingKeywordResolution::Measure(available_height) => tree.measure_child_size(
                        node,
                        *known_dimensions,
                        area_size.map(OptF32::some),
                        Size {
                            width: AvailableSpace::Definite(known_dimensions.width.unwrap_or(stretch_size.width)),
                            height: available_height,
                        },
                        sizing_mode,
                        AbsoluteAxis::Vertical,
                        Line::FALSE,
                    ),
                });
            }
        }
    }
}
