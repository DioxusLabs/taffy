//! Transferring sizes through a box's preferred aspect ratio
use crate::geometry::Size;
use crate::style::{AspectRatio, BoxSizing, CoreStyle};
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
    #[inline(always)]
    pub(crate) fn new(aspect_ratio: AspectRatio, box_sizing: BoxSizing, padding_border: Size<f32>) -> Option<Self> {
        if aspect_ratio == AspectRatio::NONE {
            return None;
        }
        Some(Self::new_unchecked(aspect_ratio, box_sizing, padding_border))
    }

    /// [`ResolvedAspectRatio::new`] for an `aspect_ratio` that is not [`AspectRatio::NONE`]
    #[inline(never)]
    #[cold]
    fn new_unchecked(aspect_ratio: AspectRatio, box_sizing: BoxSizing, padding_border: Size<f32>) -> Self {
        let ratio = aspect_ratio.ratio().unwrap_or(1.0);
        let inset =
            if aspect_ratio.is_auto() && box_sizing == BoxSizing::BorderBox { padding_border } else { Size::ZERO };
        Self { ratio, inset }
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
        let Some(ratio) = aspect_ratio else { return self };
        match (self.width, self.height) {
            (Some(width), None) => Size { width: Some(width), height: Some(ratio.height_from_width(width)) },
            (None, Some(height)) => Size { width: Some(ratio.width_from_height(height)), height: Some(height) },
            _ => self,
        }
    }
}

/// A box's size after transferring it through the box's preferred aspect ratio
pub(crate) struct AspectRatioTransfer {
    /// The size, with an unset axis transferred from the other axis unless it has an automatic minimum
    pub size: Size<Option<f32>>,
    /// The transferred size of each axis that has an automatic minimum. Such an axis is sized by its
    /// content (its min-content size in the inline axis), floored by this size.
    pub automatic_minimum: Size<Option<f32>>,
}

/// Transfers a size in one axis through `aspect_ratio` into the other axis, if that axis is unset.
///
/// An axis whose `min-*` is `auto` has an automatic minimum of its min-content size, unless the box
/// is replaced or a scroll container in that axis (<https://drafts.csswg.org/css-sizing-4/#aspect-ratio-minimum>).
pub(crate) fn transfer_aspect_ratio(
    style: &impl CoreStyle,
    size: Size<Option<f32>>,
    aspect_ratio: Option<ResolvedAspectRatio>,
) -> AspectRatioTransfer {
    let transferred = size.apply_aspect_ratio(aspect_ratio);
    if transferred == size || style.is_replaced() {
        return AspectRatioTransfer { size: transferred, automatic_minimum: Size::NONE };
    }
    let min_size = style.min_size();
    let overflow = style.overflow();
    let has_automatic_minimum = Size {
        width: size.width.is_none() && min_size.width.is_auto() && !overflow.x.is_scroll_container(),
        height: size.height.is_none() && min_size.height.is_auto() && !overflow.y.is_scroll_container(),
    };
    AspectRatioTransfer {
        size: Size {
            width: if has_automatic_minimum.width { None } else { transferred.width },
            height: if has_automatic_minimum.height { None } else { transferred.height },
        },
        automatic_minimum: Size {
            width: transferred.width.filter(|_| has_automatic_minimum.width),
            height: transferred.height.filter(|_| has_automatic_minimum.height),
        },
    }
}
