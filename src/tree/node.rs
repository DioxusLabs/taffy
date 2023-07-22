//! UI node types and related data structures.
//!
//! Layouts are composed of multiple nodes, which live in a tree-like data structure.
use crate::style::Style;
use crate::tree::Cache;
use crate::tree::Layout;

#[cfg(feature = "taffy_tree")]
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

#[cfg(feature = "taffy_tree")]
impl From<DefaultKey> for NodeId {
    #[inline]
    fn from(key: DefaultKey) -> Self {
        Self(key.data().as_ffi())
    }
}

#[cfg(feature = "taffy_tree")]
impl From<NodeId> for DefaultKey {
    #[inline]
    fn from(key: NodeId) -> Self {
        KeyData::from_ffi(key.0).into()
    }
}

/// Layout information for a given [`Node`](crate::node::Node)
///
/// Stored in a [`Taffy`].
pub(crate) struct NodeData {
    /// The layout strategy used by this node
    pub(crate) style: Style,

    /// The always unrounded results of the layout computation. We must store this separately from the rounded
    /// layout to avoid errors from rounding already-rounded values. See <https://github.com/DioxusLabs/taffy/issues/501>.
    pub(crate) unrounded_layout: Layout,

    /// The final (possibly rounded) results of the layout computation
    pub(crate) final_layout: Layout,

    /// Should we try and measure this node?
    pub(crate) needs_measure: bool,

    /// The cached results of the layout computation
    pub(crate) cache: Cache,
}

impl NodeData {
    /// Create the data for a new node
    #[must_use]
    pub const fn new(style: Style) -> Self {
        Self {
            style,
            cache: Cache::new(),
            unrounded_layout: Layout::new(),
            final_layout: Layout::new(),
            needs_measure: false,
        }
    }

    /// Marks a node and all of its parents (recursively) as dirty
    ///
    /// This clears any cached data and signals that the data must be recomputed.
    #[inline]
    pub fn mark_dirty(&mut self) {
        self.cache.clear()
    }
}
