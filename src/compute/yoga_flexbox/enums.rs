//! Port of `yoga/enums/MeasureMode.h`, `yoga/enums/Errata.h`, `yoga/enums/ExperimentalFeature.h` and
//! `yoga/algorithm/SizingMode.h`.

use core::ops::{BitAnd, BitAndAssign, BitOr, BitOrAssign, Not};

/// The constraint Yoga passes to a measure function for one axis.
#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum YogaMeasureMode {
    /// No constraint: the node's max-content size.
    Undefined,
    /// The node must be exactly this size.
    Exactly,
    /// The node may be at most this size.
    AtMost,
}

/// How Yoga sizes a node in one axis; the algorithm-side counterpart of [`YogaMeasureMode`].
#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum YogaSizingMode {
    /// The size a box would take if its outer size filled the available space in the given axis.
    StretchFit,
    /// A box's "ideal" size when given infinite available space.
    MaxContent,
    /// When the available space is definite, `clamp(min-content, stretch-fit, max-content)`.
    FitContent,
}

impl YogaSizingMode {
    /// The measure mode a measure function is called with for this sizing mode.
    pub fn measure_mode(self) -> YogaMeasureMode {
        match self {
            YogaSizingMode::StretchFit => YogaMeasureMode::Exactly,
            YogaSizingMode::MaxContent => YogaMeasureMode::Undefined,
            YogaSizingMode::FitContent => YogaMeasureMode::AtMost,
        }
    }
}

impl From<YogaMeasureMode> for YogaSizingMode {
    fn from(mode: YogaMeasureMode) -> Self {
        match mode {
            YogaMeasureMode::Exactly => YogaSizingMode::StretchFit,
            YogaMeasureMode::Undefined => YogaSizingMode::MaxContent,
            YogaMeasureMode::AtMost => YogaSizingMode::FitContent,
        }
    }
}

/// Yoga's deliberate deviations from the CSS flexbox specification, kept for backwards compatibility.
/// A set of flags: combine with `|`, test with [`contains`](Self::contains).
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub struct YogaErrata(u32);

impl YogaErrata {
    /// Fully conformant behaviour.
    pub const NONE: YogaErrata = YogaErrata(0);
    /// Stretch the flex basis of a stretched item to the container's cross size.
    pub const STRETCH_FLEX_BASIS: YogaErrata = YogaErrata(1);
    /// An absolute child without insets is positioned relative to the content box, not the padding box.
    pub const ABSOLUTE_POSITION_WITHOUT_INSETS_EXCLUDES_PADDING: YogaErrata = YogaErrata(2);
    /// Percentages on absolute children resolve against the containing block's inner size.
    pub const ABSOLUTE_PERCENT_AGAINST_INNER_SIZE: YogaErrata = YogaErrata(4);
    /// An unset minimum size means no minimum, rather than the CSS automatic minimum size.
    pub const MIN_SIZE_UNDEFINED_INSTEAD_OF_AUTO: YogaErrata = YogaErrata(8);
    /// The first pass of flexible length resolution uses running totals.
    pub const FLEX_FIRST_PASS_USES_RUNNING_TOTALS: YogaErrata = YogaErrata(16);
    /// Every errata, including any added in the future.
    pub const ALL: YogaErrata = YogaErrata(0x7fff_ffff);
    /// Yoga's pre-3.0 behaviour (every errata but `STRETCH_FLEX_BASIS`).
    pub const CLASSIC: YogaErrata = YogaErrata(0x7fff_fffe);

    /// The raw flags, with Yoga's `YGErrata` values.
    pub const fn bits(self) -> u32 {
        self.0
    }

    /// True if any flag of `errata` is set (Yoga's `Config::hasErrata`).
    pub const fn contains(self, errata: YogaErrata) -> bool {
        self.0 & errata.0 != 0
    }
}

impl BitOr for YogaErrata {
    type Output = YogaErrata;
    fn bitor(self, rhs: YogaErrata) -> YogaErrata {
        YogaErrata(self.0 | rhs.0)
    }
}

impl BitOrAssign for YogaErrata {
    fn bitor_assign(&mut self, rhs: YogaErrata) {
        self.0 |= rhs.0;
    }
}

impl BitAnd for YogaErrata {
    type Output = YogaErrata;
    fn bitand(self, rhs: YogaErrata) -> YogaErrata {
        YogaErrata(self.0 & rhs.0)
    }
}

impl BitAndAssign for YogaErrata {
    fn bitand_assign(&mut self, rhs: YogaErrata) {
        self.0 &= rhs.0;
    }
}

impl Not for YogaErrata {
    type Output = YogaErrata;
    fn not(self) -> YogaErrata {
        YogaErrata(!self.0)
    }
}

/// Opt-in behaviour changes that are not yet Yoga's default.
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub enum YogaExperimentalFeature {
    /// Recompute a child's flex basis whenever its last computation is from an earlier layout pass.
    WebFlexBasis,
    /// Measure a child's flex basis with fit-content sizing instead of stretch-fit where CSS does.
    FixFlexBasisFitContent,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sizing_mode_round_trips() {
        for mode in [YogaMeasureMode::Undefined, YogaMeasureMode::Exactly, YogaMeasureMode::AtMost] {
            assert_eq!(YogaSizingMode::from(mode).measure_mode(), mode);
        }
    }

    #[test]
    fn errata_flags() {
        let errata = YogaErrata::STRETCH_FLEX_BASIS | YogaErrata::MIN_SIZE_UNDEFINED_INSTEAD_OF_AUTO;
        assert!(errata.contains(YogaErrata::STRETCH_FLEX_BASIS));
        assert!(!errata.contains(YogaErrata::ABSOLUTE_PERCENT_AGAINST_INNER_SIZE));
        assert!(YogaErrata::ALL.contains(YogaErrata::STRETCH_FLEX_BASIS));
        assert!(!YogaErrata::CLASSIC.contains(YogaErrata::STRETCH_FLEX_BASIS));
        assert!(YogaErrata::CLASSIC.contains(YogaErrata::FLEX_FIRST_PASS_USES_RUNNING_TOTALS));
        assert!(!YogaErrata::NONE.contains(YogaErrata::ALL));
        assert_eq!(errata & !YogaErrata::STRETCH_FLEX_BASIS, YogaErrata::MIN_SIZE_UNDEFINED_INSTEAD_OF_AUTO);
    }
}
