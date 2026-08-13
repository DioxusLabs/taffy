//! Shared resolution logic for sizing keywords (`min-content`, `max-content`, `fit-content`,
//! `fit-content(...)`, and `stretch`) on the `width`/`height` style properties
use crate::style::AvailableSpace;
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
    stretch_size: Option<f32>,
    percent_resolution_basis: Option<f32>,
) -> Option<SizingKeywordResolution> {
    match style.tag() {
        CompactLength::MIN_CONTENT_TAG => Some(SizingKeywordResolution::Measure(AvailableSpace::MinContent)),
        CompactLength::MAX_CONTENT_TAG => Some(SizingKeywordResolution::Measure(AvailableSpace::MaxContent)),
        CompactLength::FIT_CONTENT_PX_TAG => {
            Some(SizingKeywordResolution::Measure(AvailableSpace::Definite(style.value())))
        }
        CompactLength::FIT_CONTENT_PERCENT_TAG => percent_resolution_basis
            .map(|basis| SizingKeywordResolution::Measure(AvailableSpace::Definite(basis * style.value()))),
        CompactLength::FIT_CONTENT_KEYWORD_TAG => {
            stretch_size.map(|size| SizingKeywordResolution::Measure(AvailableSpace::Definite(size)))
        }
        CompactLength::STRETCH_TAG => stretch_size.map(SizingKeywordResolution::Exact),
        _ => None,
    }
}
