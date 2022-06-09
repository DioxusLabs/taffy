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
    // The width and height of the node
    pub size: Size<f32>,
    // The bottom-left corner of the node
    pub location: Point<f32>,
}

impl Layout {
    #[must_use]
    pub(crate) fn new() -> Self {
        Self { order: 0, size: Size::zero(), location: Point::zero() }
    }
}

#[derive(Debug, Clone)]
pub(crate) struct Cache {
    pub(crate) node_size: Size<Number>,
    pub(crate) parent_size: Size<Number>,
    pub(crate) perform_layout: bool,

    pub(crate) result: ComputeResult,
}
