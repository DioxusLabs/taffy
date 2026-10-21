//! Transferring sizes through a box's preferred aspect ratio
use crate::geometry::Size;
use crate::style::{AspectRatio, BoxSizing};
use crate::util::sys::f32_max;

/// A box's preferred aspect ratio, and the padding and border that the box it applies to excludes. `auto && <ratio>`
/// applies to the content box (<https://drafts.csswg.org/css-sizing-4/#aspect-ratio>), so on a `border-box` box
/// it relates the border-box sizes minus `inset`.
#[derive(Debug, Clone, Copy, PartialEq)]
pub(crate) struct ResolvedAspectRatio {
    /// The ratio of width to height
    ratio: f32,
    /// The size subtracted from the box's size before applying the ratio, and added back after
    inset: Size<f32>,
}

impl ResolvedAspectRatio {
    /// Resolves a box's `aspect-ratio` against its `box-sizing` and its padding and border
    pub(crate) fn new(aspect_ratio: AspectRatio, box_sizing: BoxSizing, padding_border: Size<f32>) -> Option<Self> {
        let ratio = aspect_ratio.ratio()?;
        let inset =
            if aspect_ratio.is_auto() && box_sizing == BoxSizing::BorderBox { padding_border } else { Size::ZERO };
        Some(Self { ratio, inset })
    }

    /// The width that corresponds to `height`
    #[inline(always)]
    pub(crate) fn width_from_height(self, height: f32) -> f32 {
        f32_max(height - self.inset.height, 0.0) * self.ratio + self.inset.width
    }

    /// The height that corresponds to `width`
    #[inline(always)]
    pub(crate) fn height_from_width(self, width: f32) -> f32 {
        f32_max(width - self.inset.width, 0.0) / self.ratio + self.inset.height
    }
}

/// Applies a [`ResolvedAspectRatio`] to a size
pub(crate) trait ApplyAspectRatio {
    /// Sets an unset axis from the other axis through `aspect_ratio`, if there is one and the other axis is set
    fn apply_aspect_ratio(self, aspect_ratio: Option<ResolvedAspectRatio>) -> Self;
}

impl ApplyAspectRatio for Size<Option<f32>> {
    #[inline(always)]
    fn apply_aspect_ratio(self, aspect_ratio: Option<ResolvedAspectRatio>) -> Self {
        match (aspect_ratio, self.width, self.height) {
            (Some(ratio), Some(width), None) => {
                Size { width: Some(width), height: Some(ratio.height_from_width(width)) }
            }
            (Some(ratio), None, Some(height)) => {
                Size { width: Some(ratio.width_from_height(height)), height: Some(height) }
            }
            _ => self,
        }
    }
}
