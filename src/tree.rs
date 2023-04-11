//! The baseline requirements of any UI Tree so Taffy can efficiently calculate the layout
use crate::{
    layout::{Layout, SizeAndBaselines, SizingMode},
    prelude::*,
};
use slotmap::{DefaultKey, Key, KeyData};

/// A type representing the id of a single node in a tree of nodes
///
/// Internally it is a wrapper around a u64 and a `NodeId` can be converted to and from
/// and u64 if needed.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct NodeId(u64);
impl NodeId {
    /// Create a new NodeId from a u64 value
    pub const fn new(val: u64) -> Self {
        Self(val)
    }
}

impl From<u64> for NodeId {
    #[inline]
    fn from(raw: u64) -> Self {
        Self(raw)
    }
}
impl From<NodeId> for u64 {
    #[inline]
    fn from(id: NodeId) -> Self {
        id.0
    }
}
impl From<usize> for NodeId {
    #[inline]
    fn from(raw: usize) -> Self {
        Self(raw as u64)
    }
}
impl From<NodeId> for usize {
    #[inline]
    fn from(id: NodeId) -> Self {
        id.0 as usize
    }
}
impl From<DefaultKey> for NodeId {
    #[inline]
    fn from(key: DefaultKey) -> Self {
        Self(key.data().as_ffi())
    }
}
impl From<NodeId> for DefaultKey {
    #[inline]
    fn from(key: NodeId) -> Self {
        KeyData::from_ffi(key.0).into()
    }
}

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

    // todo: allow abstractions over this so we don't prescribe how layout works
    // for reference, CSS cascades require context, and storing a full flexbox layout for each node could be inefficient
    //
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
