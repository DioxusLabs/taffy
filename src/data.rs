//! Node Data - important layout and styling data for nodes
//!
//! Used to compute layout for Taffy trees
//!
use crate::layout::{Cache, Layout};
use crate::style::Style;

pub(crate) const CACHE_SIZE: usize = 5;

/// Layout information for a given [`Node`](crate::node::Node)
///
/// Stored in a [`Taffy`].
pub(crate) struct NodeData {
    /// The layout strategy used by this node
    pub(crate) style: Style,
    /// The results of the layout computation
    pub(crate) layout: Layout,

    /// Should we try and measure this node?
    pub(crate) needs_measure: bool,

    /// The primary cached results of the layout computation
    pub(crate) size_cache: [Option<Cache>; CACHE_SIZE],
}

impl NodeData {
    /// Create the data for a new node
    #[must_use]
    pub const fn new(style: Style) -> Self {
        Self { style, size_cache: [None; CACHE_SIZE], layout: Layout::new(), needs_measure: false }
    }

    /// Marks a node and all of its parents (recursively) as dirty
    ///
    /// This clears any cached data and signals that the data must be recomputed.
    #[inline]
    pub fn mark_dirty(&mut self) {
        self.size_cache = [None; CACHE_SIZE];
    }
}
