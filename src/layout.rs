//! Final and cached data structures that represent the high-level UI layout

use crate::flexbox::ComputeResult;
use crate::geometry::{Point, Size};
use crate::number::Number;

/// The final result of a layout algorithm for a single [`Node`](crate::node::Node).
#[derive(Copy, Debug, Clone)]
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
    /// Creates a new [`Layout`] struct with zero size positioned at the origin
    #[must_use]
    pub(crate) fn new() -> Self {
        Self { order: 0, size: Size::zero(), location: Point::zero() }
    }
}

/// Cached intermediate layout results
#[derive(Debug, Clone)]
pub(crate) struct Cache {
    /// The initial cached size of the node itself
    pub(crate) node_size: Size<Number>,
    /// The initial cached size of the parent's node
    pub(crate) parent_size: Size<Number>,
    /// Whether or not layout should be recomputed
    pub(crate) perform_layout: bool,

    /// The stored result of the layout calculations
    pub(crate) result: ComputeResult,
}
