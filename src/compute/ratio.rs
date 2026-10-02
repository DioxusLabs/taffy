//! Sizing a box through its preferred aspect ratio (the `aspect-ratio` property).
//!
//! [CSS Sizing 4 §4](https://www.w3.org/TR/css-sizing-4/#aspect-ratio) relates a box's sizes
//! through its ratio as follows, and the layout algorithms share these rules by resolving a box's
//! size styles with [`sizes_through_ratio`]:
//!
//! - The ratio relates the sizes of the box that `box-sizing` names, except for natural ratios
//!   and `auto <ratio>`, which always relate content-box sizes
//!   ([§4.1](https://www.w3.org/TR/css-sizing-4/#aspect-ratio)).
//! - If one dimension is given then it is clamped by its own minimum and maximum, and the other
//!   dimension is derived from the clamped value and then clamped by its own minimum and maximum.
//!   If both dimensions are given then the ratio does nothing.
//! - A minimum or maximum size transfers through the ratio only into a dimension that is not
//!   given ([§4.4](https://www.w3.org/TR/css-sizing-4/#aspect-ratio-size-transfers)).
//!   A box with a stretched width takes its `max-height` as a maximum width. A box with a `width` does not.
//! - The height derived from a width is a minimum that the box's content can exceed, rather than
//!   a size, if the box is neither a replaced element nor a scroll container and its `min-height`
//!   is `auto` ([§4.3](https://www.w3.org/TR/css-sizing-4/#aspect-ratio-minimum)).
use crate::geometry::Size;
use crate::util::sys::{f32_max, f32_min};
use crate::util::MaybeMath;
use crate::{BoxSizing, CoreStyle};

/// The size, minimum size and maximum size of a box, as border-box sizes
#[derive(Debug, Clone, Copy)]
pub(crate) struct BoxSizes {
    /// The dimensions of the box that are determined
    pub(crate) size: Size<Option<f32>>,
    /// The minimum size of the box
    pub(crate) min_size: Size<Option<f32>>,
    /// The maximum size of the box
    pub(crate) max_size: Size<Option<f32>>,
}

/// A usable preferred aspect ratio, along with the difference between the border box
/// and the box whose sizes the ratio relates
#[derive(Debug, Clone, Copy)]
pub(crate) struct Ratio {
    /// The width divided by the height. Finite and positive.
    ratio: f32,
    /// The amount that is subtracted from a border-box size to get the size that the ratio relates
    inset: Size<f32>,
}

impl Ratio {
    /// The preferred aspect ratio of `style`, if it has a usable one.
    ///
    /// `padding_border_sum` is the sum of the style's resolved padding and border in each axis.
    pub(crate) fn of(style: &impl CoreStyle, padding_border_sum: Size<f32>) -> Option<Self> {
        Self::from_parts(style.aspect_ratio(), style.box_sizing(), style.aspect_ratio_content_box(), padding_border_sum)
    }

    /// The same as [`Ratio::of`], for algorithms that store the relevant styles rather than the style itself
    pub(crate) fn from_parts(
        ratio: Option<f32>,
        box_sizing: BoxSizing,
        content_box: bool,
        padding_border_sum: Size<f32>,
    ) -> Option<Self> {
        let ratio = ratio.filter(|ratio| ratio.is_finite() && *ratio > 0.0)?;
        let inset = if box_sizing == BoxSizing::ContentBox || content_box { padding_border_sum } else { Size::ZERO };
        Some(Self { ratio, inset })
    }

    /// The border-box height that corresponds to a border-box width
    pub(crate) fn height(self, width: f32) -> f32 {
        f32_max(width - self.inset.width, 0.0) / self.ratio + self.inset.height
    }

    /// The border-box width that corresponds to a border-box height
    pub(crate) fn width(self, height: f32) -> f32 {
        f32_max(height - self.inset.height, 0.0) * self.ratio + self.inset.width
    }

    /// Resolve the size, minimum size and maximum size of a box through the ratio.
    ///
    /// `given` are the dimensions of the box that are already determined (by its size styles or by
    /// its parent) and `min` and `max` are its resolved `min-width`/`min-height` and
    /// `max-width`/`max-height` styles. All are border-box sizes.
    ///
    /// `derived_height_is_minimum` is whether a height derived from a given width is a minimum that
    /// the box's content can exceed, rather than a size (see [`derived_height_is_minimum`]). If it is
    /// then the returned size has no height, and the derived height is returned as the minimum height.
    pub(crate) fn resolve(
        self,
        given: Size<Option<f32>>,
        min: Size<Option<f32>>,
        max: Size<Option<f32>>,
        derived_height_is_minimum: bool,
    ) -> BoxSizes {
        // Minimum and maximum sizes transfer through the ratio only into a dimension that is not given
        let (mut min_out, mut max_out) = (min, max);
        if given.width.is_none() {
            if let Some(min_height) = min.height {
                min_out.width = Some(f32_max(min.width.unwrap_or(0.0), self.width(min_height)));
            }
            if let Some(max_height) = max.height {
                let transferred = self.width(max_height);
                max_out.width = Some(max.width.map_or(transferred, |max_width| f32_min(max_width, transferred)));
            }
        }
        if given.height.is_none() {
            if let Some(min_width) = min.width {
                min_out.height = Some(f32_max(min.height.unwrap_or(0.0), self.height(min_width)));
            }
            // A maximum width limits the height that is derived from the width. But it does not limit
            // the height of content that exceeds the derived height: only `max-height` does that.
            if !derived_height_is_minimum {
                if let Some(max_width) = max.width {
                    let transferred = self.height(max_width);
                    max_out.height =
                        Some(max.height.map_or(transferred, |max_height| f32_min(max_height, transferred)));
                }
            }
        }

        // A given dimension is clamped by its own minimum and maximum before the other is derived from it
        let mut size = given;
        match (given.width, given.height) {
            (Some(width), None) => {
                let derived_height = self.height(width.maybe_clamp(min_out.width, max_out.width));
                if derived_height_is_minimum {
                    let minimum = derived_height.maybe_min(max_out.height);
                    min_out.height = Some(f32_max(min_out.height.unwrap_or(0.0), minimum));
                } else {
                    size.height = Some(derived_height.maybe_clamp(min_out.height, max_out.height));
                }
            }
            (None, Some(height)) => {
                let derived_width = self.width(height.maybe_clamp(min_out.height, max_out.height));
                size.width = Some(derived_width.maybe_clamp(min_out.width, max_out.width));
            }
            _ => {}
        }

        BoxSizes { size, min_size: min_out, max_size: max_out }
    }
}

/// Whether the height that a box's aspect ratio derives from its width is a minimum that the
/// box's content can exceed rather than a size: true for a box that is neither a replaced
/// element nor a scroll container and whose `min-height` is `auto`.
///
/// <https://www.w3.org/TR/css-sizing-4/#aspect-ratio-minimum>
pub(crate) fn derived_height_is_minimum(style: &impl CoreStyle, _min_height: Option<f32>) -> bool {
    let overflow = style.overflow();
    !style.is_compressible_replaced()
        && style.min_size().height.is_auto()
        && !overflow.x.is_scroll_container()
        && !overflow.y.is_scroll_container()
}

/// Resolve the size, minimum size and maximum size of a box through its preferred aspect ratio
/// (see [`Ratio::resolve`]). The sizes are returned unchanged if the box does not have one.
pub(crate) fn sizes_through_ratio(
    style: &impl CoreStyle,
    padding_border_sum: Size<f32>,
    size: Size<Option<f32>>,
    min: Size<Option<f32>>,
    max: Size<Option<f32>>,
) -> BoxSizes {
    match Ratio::of(style, padding_border_sum) {
        Some(ratio) => ratio.resolve(size, min, max, derived_height_is_minimum(style, min.height)),
        None => BoxSizes { size, min_size: min, max_size: max },
    }
}

/// The height that a box's percentage-height children resolve against when the height of the box is
/// derived from its width by its preferred aspect ratio: the derived height, even if the box's
/// content makes the box taller than that.
pub(crate) fn percentage_basis_height(
    style: &impl CoreStyle,
    padding_border_sum: Size<f32>,
    width: Option<f32>,
) -> Option<f32> {
    if !style.size().height.is_auto() {
        return None;
    }
    Some(Ratio::of(style, padding_border_sum)?.height(width?))
}

/// Whether the automatic minimum width of a box is its min-content width: true for a box whose
/// width is derived from a definite height by its preferred aspect ratio, and which is neither
/// a replaced element nor a scroll container and whose `min-width` is `auto`.
///
/// `height` is the resolved height of the box (if it is definite).
///
/// <https://www.w3.org/TR/css-sizing-4/#aspect-ratio-minimum>
pub(crate) fn derived_width_has_content_minimum(style: &impl CoreStyle, height: Option<f32>) -> bool {
    let overflow = style.overflow();
    height.is_some()
        && style.aspect_ratio().is_some_and(|ratio| ratio.is_finite() && ratio > 0.0)
        && style.size().width.is_auto()
        && style.min_size().width.is_auto()
        && !style.is_compressible_replaced()
        && !overflow.x.is_scroll_container()
        && !overflow.y.is_scroll_container()
}

/// Transfer `limits` (a box's resolved minimum or maximum sizes) through the box's preferred
/// aspect ratio into the axes that the box's size styles do not size (as given by `sized`).
/// An axis that is sized keeps only its own limit.
pub(crate) fn transfer_into_unsized(
    limits: Size<Option<f32>>,
    aspect_ratio: Option<Ratio>,
    sized: Size<bool>,
) -> Size<Option<f32>> {
    let transferred = match aspect_ratio {
        Some(ratio) => Size {
            width: limits.width.or_else(|| limits.height.map(|height| ratio.width(height))),
            height: limits.height.or_else(|| limits.width.map(|width| ratio.height(width))),
        },
        None => limits,
    };
    Size {
        width: if sized.width { limits.width } else { transferred.width },
        height: if sized.height { limits.height } else { transferred.height },
    }
}

/// The automatic inline minimum of a non-replaced ratio box whose width
/// derives from a definite height is its min-content width, capped by max-width.
pub(crate) fn minimum_ratio_width(
    tree: &mut impl crate::LayoutPartialTree,
    node: crate::NodeId,
    parent: Size<Option<f32>>,
) -> Option<f32> {
    use crate::{LayoutPartialTreeExt, MaybeResolve, ResolveOrZero};
    let style = tree.get_core_container_style(node);
    if !style.aspect_ratio().is_some_and(|ratio| ratio.is_finite() && ratio > 0.0)
        || !style.size().width.is_auto()
        || !style.min_size().width.is_auto()
        || style.is_compressible_replaced()
        || style.overflow().x.is_scroll_container()
        || style.overflow().y.is_scroll_container()
        || style.size().height.maybe_resolve(parent.height, |v, b| tree.calc(v, b)).is_none()
    {
        return None;
    }
    let pb = (style.padding().resolve_or_zero(parent.width, |v, b| tree.calc(v, b))
        + style.border().resolve_or_zero(parent.width, |v, b| tree.calc(v, b)))
    .sum_axes();
    let adjustment = if style.box_sizing() == BoxSizing::ContentBox { pb.width } else { 0.0 };
    let max = style.max_size().width.maybe_resolve(parent.width, |v, b| tree.calc(v, b)).map(|w| w + adjustment);
    drop(style);
    let intrinsic = tree.measure_child_size(
        node,
        Size::NONE,
        parent,
        Size { width: crate::AvailableSpace::MinContent, height: crate::AvailableSpace::MinContent },
        crate::SizingMode::ContentSize,
        crate::AbsoluteAxis::Horizontal,
        crate::geometry::Line::FALSE,
    );
    Some(intrinsic.maybe_min(max))
}

/// Resolve sizes through a ratio already retained by a layout algorithm.
pub(crate) fn resolve_through(
    ratio: Option<Ratio>,
    size: Size<Option<f32>>,
    min: Size<Option<f32>>,
    max: Size<Option<f32>>,
    floor_height: bool,
) -> BoxSizes {
    match ratio {
        Some(ratio) => ratio.resolve(size, min, max, floor_height),
        None => BoxSizes { size, min_size: min, max_size: max },
    }
}
