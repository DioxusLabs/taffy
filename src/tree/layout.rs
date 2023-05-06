//! Final data structures that represent the high-level UI layout

use crate::{
    geometry::{Point, Size},
    util::sys::{f32_max, f32_min},
};

/// Whether we are performing a full layout, or we merely need to size the node
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum RunMode {
    /// A full layout for this node and all children should be computed
    PeformLayout,
    /// The layout algorithm should be executed such that an accurate container size for the node can be determined.
    /// Layout steps that aren't necessary for determining the container size of the current node can be skipped.
    ComputeSize,
}

/// Whether styles should be taken into account when computing size
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum SizingMode {
    /// Only content contributions should be taken into account
    ContentSize,
    /// Inherent size styles should be taken into account in addition to content contributions
    InherentSize,
}

/// A set of margins that are available for collapsing with for block layout's margin collapsing
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct CollapsibleMarginSet {
    /// The largest positive margin
    positive: f32,
    /// The smallest negative margin (with largest absolute value)
    negative: f32,
}

impl CollapsibleMarginSet {
    /// A default margin set with no collapsible margins
    pub const ZERO: Self = Self { positive: 0.0, negative: 0.0 };

    /// Collapse a single margin with this set
    pub fn collapse_with_margin(&mut self, margin: f32) {
        if margin >= 0.0 {
            self.positive = f32_max(self.positive, margin);
        } else {
            self.negative = f32_min(self.negative, margin);
        }
    }

    /// Collapse another margin set with this set
    pub fn collapse_with_set(&mut self, other: CollapsibleMarginSet) {
        self.positive = f32_max(self.positive, other.positive);
        self.negative = f32_min(self.negative, other.negative);
    }

    /// Resolve the resultant margin from this set once all collapsible margins
    /// have been collapsed into it
    pub fn resolve(&self) -> f32 {
        self.positive + self.negative
    }
}

/// A struct containing both the size of a node and it's first baseline in each dimension (if it has any)
///
/// A baseline is the line on which text sits. Your node likely has a baseline if it is a text node, or contains
/// children that may be text nodes. See <https://www.w3.org/TR/css-writing-modes-3/#intro-baselines> for details.
/// If your node does not have a baseline (or you are unsure how to compute it), then simply return `Point::NONE`
/// for the first_baselines field
#[derive(Debug, Copy, Clone)]
pub struct SizeBaselinesAndMargins {
    /// The size of the node
    pub size: Size<f32>,
    /// Top margin that can be collapsed with. This is used for CSS block layout and can be set to
    /// `CollapsibleMarginSet::ZERO` for other layout modes that don't support margin collapsing
    pub top_margin: CollapsibleMarginSet,
    /// Bottom margin that can be collapsed with. This is used for CSS block layout and can be set to
    /// `CollapsibleMarginSet::ZERO` for other layout modes that don't support margin collapsing
    pub bottom_margin: CollapsibleMarginSet,
    /// The first baseline of the node in each dimension, if any
    pub first_baselines: Point<Option<f32>>,
}

impl SizeBaselinesAndMargins {
    /// An all-zero `SizeBaselinesAndMargins` for hidden nodes
    pub const HIDDEN: Self = Self {
        size: Size::ZERO,
        first_baselines: Point::NONE,
        top_margin: CollapsibleMarginSet::ZERO,
        bottom_margin: CollapsibleMarginSet::ZERO,
    };

    /// Constructor to create a `SizeBaselinesAndMargins` from just the size and baselines
    pub fn from_size_and_baselines(size: Size<f32>, first_baselines: Point<Option<f32>>) -> Self {
        Self {
            size,
            first_baselines,
            top_margin: CollapsibleMarginSet::ZERO,
            bottom_margin: CollapsibleMarginSet::ZERO,
        }
    }
}

impl From<Size<f32>> for SizeBaselinesAndMargins {
    fn from(size: Size<f32>) -> Self {
        Self {
            size,
            top_margin: CollapsibleMarginSet::ZERO,
            bottom_margin: CollapsibleMarginSet::ZERO,
            first_baselines: Point::NONE,
        }
    }
}

/// The final result of a layout algorithm for a single node.
#[derive(Debug, Copy, Clone)]
pub struct Layout {
    /// The relative ordering of the node
    ///
    /// Nodes with a higher order should be rendered on top of those with a lower order.
    /// This is effectively a topological sort of each tree.
    pub order: u32,
    /// The width and height of the node
    pub size: Size<f32>,
    /// The bottom-left corner of the node
    pub location: Point<f32>,
}

impl Layout {
    /// Creates a new zero-[`Layout`].
    ///
    /// The Zero-layout has size and location set to ZERO.
    /// The `order` value of this layout is set to the minimum value of 0.
    /// This means it should be rendered below all other [`Layout`]s.
    #[must_use]
    pub const fn new() -> Self {
        Self { order: 0, size: Size::zero(), location: Point::ZERO }
    }

    /// Creates a new zero-[`Layout`] with the supplied `order` value.
    ///
    /// Nodes with a higher order should be rendered on top of those with a lower order.
    /// The Zero-layout has size and location set to ZERO.
    #[must_use]
    pub const fn with_order(order: u32) -> Self {
        Self { order, size: Size::zero(), location: Point::ZERO }
    }
}
