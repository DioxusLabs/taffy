//! The baseline requirements of any UI Tree so Taffy can efficiently calculate the layout
use crate::{
    layout::{Cache, Layout, SizeAndBaselines, SizingMode},
    prelude::*,
};

/// Any item that implements the LayoutTree can be layed out using Taffy's algorithms.
///
/// Generally, Taffy expects your Node tree to be indexable by stable indices. A "stable" index means that the Node's ID
/// remains the same between re-layouts.
pub trait LayoutTree {
    /// Type representing an iterator of the children of a node
    type ChildIter<'a>: Iterator<Item = u64>
    where
        Self: 'a;

    /// Get the list of children IDs for the given node
    fn children(&self, node: u64) -> Self::ChildIter<'_>;

    /// Get the number of children for the given node
    fn child_count(&self, node: u64) -> usize;

    /// Get a specific child of a node, where the index represents the nth child
    fn child(&self, node: u64, index: usize) -> u64;

    // todo: allow abstractions over this so we don't prescribe how layout works
    // for reference, CSS cascades require context, and storing a full flexbox layout for each node could be inefficient
    //
    /// Get the [`Style`] for this node.
    fn style(&self, node: u64) -> &Style;

    /// Modify the node's output layout
    fn layout_mut(&mut self, node: u64) -> &mut Layout;

    /// Measure a node. Taffy uses this to force reflows of things like text and overflowing content.
    fn measure_node(
        &self,
        node: u64,
        known_dimensions: Size<Option<f32>>,
        available_space: Size<AvailableSpace>,
    ) -> Size<f32>;

    /// Node needs to be measured
    fn needs_measure(&self, node: u64) -> bool;

    /// Get a cache entry for this Node by index
    fn cache_mut(&mut self, node: u64, index: usize) -> &mut Option<Cache>;

    /// Compute the size of the node given the specified constraints
    fn measure_child_size(
        &mut self,
        node: u64,
        known_dimensions: Size<Option<f32>>,
        parent_size: Size<Option<f32>>,
        available_space: Size<AvailableSpace>,
        sizing_mode: SizingMode,
    ) -> Size<f32>;

    /// Perform a full layout on the node given the specified constraints
    fn perform_child_layout(
        &mut self,
        node: u64,
        known_dimensions: Size<Option<f32>>,
        parent_size: Size<Option<f32>>,
        available_space: Size<AvailableSpace>,
        sizing_mode: SizingMode,
    ) -> SizeAndBaselines;
}
