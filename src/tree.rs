//! The baseline requirements of any UI Tree so Taffy can efficiently calculate the layout

use crate::{
    layout::{Cache, Layout, SizeAndBaselines, SizingMode},
    prelude::*,
};
use core::fmt::Debug;

/// Any item that implements the LayoutTree can be layed out using Taffy's algorithms.
///
/// Generally, Taffy expects your Node tree to be indexable by stable indices. A "stable" index means that the Node's ID
/// remains the same between re-layouts.
pub trait LayoutTree {
    /// Type of an id that represents a child of the current node
    /// This can be a usize if you are storing children in a vector
    type ChildId: Copy + PartialEq + Debug;

    /// Type representing an iterator of the children of a node
    type ChildIter<'a>: Iterator<Item = Self::ChildId>
    where
        Self: 'a;

    // Current node methods

    /// Get the [`Style`] for this Node.
    fn style(&self) -> &Style;

    /// Modify the node's output layout
    fn layout_mut(&mut self) -> &mut Layout;

    /// Get a cache entry for this Node by index
    fn cache_mut(&mut self, index: usize) -> &mut Option<Cache>;

    // Child methods

    /// Get the list of children IDs for the given node
    fn children(&self) -> Self::ChildIter<'_>;

    /// Get the number of children for the given node
    fn child_count(&self) -> usize;

    /// Get a specific child of a node, where the index represents the nth child
    fn child(&self, index: usize) -> Self::ChildId;

    /// Get the [`Style`] for this child.
    fn child_style(&self, child_node_id: Self::ChildId) -> &Style;

    /// Modify the child's output layout
    fn child_layout_mut(&mut self, child_node_id: Self::ChildId) -> &mut Layout;

    /// Compute the size of the node given the specified constraints
    fn measure_child_size(
        &mut self,
        child_node_id: Self::ChildId,
        known_dimensions: Size<Option<f32>>,
        parent_size: Size<Option<f32>>,
        available_space: Size<AvailableSpace>,
        sizing_mode: SizingMode,
    ) -> Size<f32>;

    /// Perform a full layout on the node given the specified constraints
    fn perform_child_layout(
        &mut self,
        child_node_id: Self::ChildId,
        known_dimensions: Size<Option<f32>>,
        parent_size: Size<Option<f32>>,
        available_space: Size<AvailableSpace>,
        sizing_mode: SizingMode,
    ) -> SizeAndBaselines;

    /// Perform a hidden layout (mark the node as invisible)
    fn perform_child_hidden_layout(&mut self, child_node_id: Self::ChildId, order: u32);
}
