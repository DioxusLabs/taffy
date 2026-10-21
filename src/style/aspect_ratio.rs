//! The `aspect-ratio` property

/// The value of the `aspect-ratio` property: a preferred aspect ratio (width / height), on its own or
/// together with `auto`. With `auto`, a replaced element with a natural aspect ratio uses that instead, and
/// the ratio applies to the content box regardless of `box-sizing` (<https://drafts.csswg.org/css-sizing-4/#aspect-ratio>).
#[derive(Copy, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(into = "AspectRatioRepr", from = "AspectRatioRepr"))]
pub struct AspectRatio(
    /// The ratio, negated for `auto && <ratio>`, or NaN if there is no preferred aspect ratio
    f32,
);

impl AspectRatio {
    /// No preferred aspect ratio (`aspect-ratio: auto`)
    pub const NONE: Self = Self(f32::NAN);

    /// `aspect-ratio: <ratio>`. A degenerate ratio (not finite and positive) gives [`AspectRatio::NONE`].
    pub fn new(ratio: f32) -> Self {
        if ratio.is_finite() && ratio > 0.0 {
            Self(ratio)
        } else {
            Self::NONE
        }
    }

    /// `aspect-ratio: auto && <ratio>`. A degenerate ratio (not finite and positive) gives [`AspectRatio::NONE`].
    pub fn auto_and(ratio: f32) -> Self {
        if ratio.is_finite() && ratio > 0.0 {
            Self(-ratio)
        } else {
            Self::NONE
        }
    }

    /// The preferred aspect ratio (width / height), if any
    #[inline(always)]
    pub fn ratio(self) -> Option<f32> {
        (!self.0.is_nan()).then_some(self.0.abs())
    }

    /// Whether the ratio is combined with `auto` (`auto && <ratio>`)
    #[inline(always)]
    pub fn is_auto(self) -> bool {
        self.0 < 0.0
    }
}

impl Default for AspectRatio {
    fn default() -> Self {
        Self::NONE
    }
}

impl PartialEq for AspectRatio {
    fn eq(&self, other: &Self) -> bool {
        self.0.to_bits() == other.0.to_bits()
    }
}

impl core::fmt::Debug for AspectRatio {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self.ratio() {
            None => f.write_str("AspectRatio::NONE"),
            Some(ratio) if self.is_auto() => write!(f, "AspectRatio::auto_and({ratio:?})"),
            Some(ratio) => write!(f, "AspectRatio::new({ratio:?})"),
        }
    }
}

impl From<f32> for AspectRatio {
    fn from(ratio: f32) -> Self {
        Self::new(ratio)
    }
}

impl From<Option<f32>> for AspectRatio {
    fn from(ratio: Option<f32>) -> Self {
        ratio.map_or(Self::NONE, Self::new)
    }
}

/// The serialized form of [`AspectRatio`]
#[cfg(feature = "serde")]
#[derive(Serialize, Deserialize)]
struct AspectRatioRepr {
    /// The preferred aspect ratio, if any
    ratio: Option<f32>,
    /// Whether the ratio is combined with `auto`
    auto: bool,
}

#[cfg(feature = "serde")]
impl From<AspectRatio> for AspectRatioRepr {
    fn from(value: AspectRatio) -> Self {
        Self { ratio: value.ratio(), auto: value.is_auto() }
    }
}

#[cfg(feature = "serde")]
impl From<AspectRatioRepr> for AspectRatio {
    fn from(value: AspectRatioRepr) -> Self {
        match value.ratio {
            Some(ratio) if value.auto => Self::auto_and(ratio),
            Some(ratio) => Self::new(ratio),
            None => Self::NONE,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::AspectRatio;

    #[test]
    fn accessors() {
        assert_eq!(AspectRatio::NONE.ratio(), None);
        assert!(!AspectRatio::NONE.is_auto());
        assert_eq!(AspectRatio::new(2.0).ratio(), Some(2.0));
        assert!(!AspectRatio::new(2.0).is_auto());
        assert_eq!(AspectRatio::auto_and(2.0).ratio(), Some(2.0));
        assert!(AspectRatio::auto_and(2.0).is_auto());
    }

    #[test]
    fn degenerate_ratios_are_none() {
        for ratio in [0.0, -0.0, -1.0, f32::INFINITY, f32::NAN] {
            assert_eq!(AspectRatio::new(ratio), AspectRatio::NONE);
            assert_eq!(AspectRatio::auto_and(ratio), AspectRatio::NONE);
        }
    }

    #[test]
    fn equality() {
        assert_eq!(AspectRatio::NONE, AspectRatio::NONE);
        assert_eq!(AspectRatio::new(2.0), AspectRatio::new(2.0));
        assert_ne!(AspectRatio::new(2.0), AspectRatio::auto_and(2.0));
    }
}
