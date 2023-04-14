//! Contains both [a high-level interface to Taffy](crate::Taffy) using a ready-made node tree, and [a trait for defining a custom node trees](crate::tree::LayoutTree) / utility types to help with that.

use crate::geometry::{Point, Size};
use crate::style::{AvailableSpace, Style};

// Submodules
mod cache;
pub use cache::{Cache, CacheEntry};
mod measure_func;
pub use measure_func::{Measurable, MeasureFunc};
mod node;
pub(self) use node::NodeData;
pub use node::NodeId;
mod taffy_tree;
mod taffy_tree_error;
pub use taffy_tree::{Taffy, TaffyChildIter};
pub use taffy_tree_error::{TaffyError, TaffyResult};
mod layout;

/// Any item that implements the LayoutTree can be layed out using Taffy's algorithms.
///
/// Generally, Taffy expects your Node tree to be indexable by stable indices. A "stable" index means that the Node's ID
/// remains the same between re-layouts.
pub trait LayoutTree {
    /// Type representing an iterator of the children of a node
    type ChildIter<'a>: Iterator<Item = NodeId>
    where
        Self: 'a;

    /// Get the list of children IDs for the given node
    fn children(&self, node: NodeId) -> Self::ChildIter<'_>;

    /// Get the number of children for the given node
    fn child_count(&self, node: NodeId) -> usize;

    /// Get a specific child of a node, where the index represents the nth child
    fn child(&self, node: NodeId, index: usize) -> NodeId;

    /// Get the [`Style`] for this node.
    fn style(&self, node: NodeId) -> &Style;

    /// Modify the node's output layout
    fn layout_mut(&mut self, node: NodeId) -> &mut Layout;

    /// Compute the size of the node given the specified constraints
    fn measure_child_size(
        &mut self,
        node: NodeId,
        known_dimensions: Size<Option<f32>>,
        parent_size: Size<Option<f32>>,
        available_space: Size<AvailableSpace>,
        sizing_mode: SizingMode,
    ) -> Size<f32>;

    /// Perform a full layout on the node given the specified constraints
    fn perform_child_layout(
        &mut self,
        node: NodeId,
        known_dimensions: Size<Option<f32>>,
        parent_size: Size<Option<f32>>,
        available_space: Size<AvailableSpace>,
        sizing_mode: SizingMode,
    ) -> SizeAndBaselines;
}

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
