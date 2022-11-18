//! Node Data - important layout and styling data for nodes
//!
//! Used to compute layout for Taffy trees
//!
use crate::layout::{Cache, Layout};
use crate::style::FlexboxLayout;

/// Layout information for a given [`Node`](crate::node::Node)
///
/// Stored in a [`Taffy`].
pub(crate) struct NodeData {
    /// The layout strategy used by this node
    pub(crate) style: FlexboxLayout,
    /// The results of the layout computation
    pub(crate) layout: Layout,

    /// Should we try and measure this node?
    pub(crate) needs_measure: bool,

    /// The primary cached results of the layout computation
    pub(crate) size_cache: [Option<Cache>; 4],
    /// Does this node's layout need to be recomputed?
    pub(crate) is_dirty: bool,
}

impl NodeData {
    /// Create the data for a new node
    #[must_use]
    pub const fn new(style: FlexboxLayout) -> Self {
        Self { style, size_cache: [None; 4], layout: Layout::new(), is_dirty: true, needs_measure: false }
    }

    /// Marks a node and all of its parents (recursively) as dirty
    ///
    /// This clears any cached data and signals that the data must be recomputed.
    #[inline]
    pub fn mark_dirty(&mut self) {
        self.size_cache = [None; 4];
        self.is_dirty = true;
    }
}
