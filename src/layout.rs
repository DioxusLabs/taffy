//! Final and cached data structures that represent the high-level UI layout

use crate::geometry::{Point, Size};

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
    /// Creates a new zero-[`Layout`].
    ///
    /// The Zero-layout has size and location set to ZERO.
    /// The `order` value of this layout is set to the minimum value of 0.
    /// This means it should be rendered below all other [`Layout`]s.
    #[must_use]
    pub const fn new() -> Self {
        Self { order: 0, size: Size::ZERO, location: Point::ZERO }
    }

    /// Creates a new zero-[`Layout`] with the supplied `order` value.
    ///
    /// Nodes with a higher order should be rendered on top of those with a lower order.
    /// The Zero-layout has size and location set to ZERO.
    #[must_use]
    pub const fn with_order(order: u32) -> Self {
        Self { order, size: Size::ZERO, location: Point::ZERO }
    }
}

/// Cached intermediate layout results
#[derive(Debug, Clone)]
pub struct Cache {
    /// The initial cached size of the node itself
    pub(crate) node_size: Size<Option<f32>>,
    /// The initial cached size of the parent's node
    pub(crate) parent_size: Size<Option<f32>>,
    /// Whether or not layout should be recomputed
    pub(crate) perform_layout: bool,

    /// The cached size of the item
    pub(crate) size: Size<f32>,
}
