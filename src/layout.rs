//! Final and cached data structures that represent the high-level UI layout

use crate::geometry::{Point, Size};
use crate::style::AvailableSpace;
use crate::sys::round;

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

/// A struct containing both the size of a node and it's first baseline in each dimension (if it has any)
///
/// A baseline is the line on which text sits. Your node likely has a baseline if it is a text node, or contains
/// children that may be text nodes. See <https://www.w3.org/TR/css-writing-modes-3/#intro-baselines> for details.
/// If your node does not have a baseline (or you are unsure how to compute it), then simply return `Point::NONE`
/// for the first_baselines field
#[derive(Debug, Copy, Clone)]
pub struct SizeAndBaselines {
    /// The size of the node
    pub size: Size<f32>,
    /// The first baseline of the node in each dimension, if any
    pub first_baselines: Point<Option<f32>>,
}

impl From<Size<f32>> for SizeAndBaselines {
    fn from(size: Size<f32>) -> Self {
        Self { size, first_baselines: Point::NONE }
    }
}

/// The final result of a layout algorithm for a single [`Node`](crate::node::Node).
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

    /// Round layout to integer values
    pub fn round(&mut self) {
        self.location.x = round(self.location.x);
        self.location.y = round(self.location.y);
        self.size.width = round(self.size.width);
        self.size.height = round(self.size.height);
    }
}

/// Cached intermediate layout results
#[derive(Debug, Clone, Copy)]
pub struct Cache {
    /// The initial cached size of the node itself
    pub(crate) known_dimensions: Size<Option<f32>>,
    /// The initial cached size of the parent's node
    pub(crate) available_space: Size<AvailableSpace>,
    /// Whether or not layout should be recomputed
    pub(crate) run_mode: RunMode,

    /// The cached size and baselines of the item
    pub(crate) cached_size_and_baselines: SizeAndBaselines,
}
