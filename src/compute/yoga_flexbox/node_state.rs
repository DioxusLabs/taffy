//! Per-node layout state that Yoga keeps on `yoga::Node` (`LayoutResults.h`) and that the Yoga
//! flexbox algorithm reads back across layout passes.

/// Yoga layout state for one node that outlives a single layout pass.
///
/// Hosts store one of these per node and expose it through
/// [`LayoutYogaFlexboxContainer`](crate::tree::LayoutYogaFlexboxContainer). Marking a node dirty
/// must reset `computed_flex_basis` (Yoga's `markDirtyAndPropagate`), as `TaffyTree` does.
#[derive(Copy, Clone, Debug, Default, PartialEq)]
pub struct YogaFlexboxNodeState {
    /// The flex basis computed by the most recent layout pass (`LayoutResults::computedFlexBasis`).
    /// Yoga reuses it on later passes, which is why it must survive between layouts.
    pub computed_flex_basis: Option<f32>,
    /// Whether the node's content overflowed its container (`LayoutResults::hadOverflow`).
    pub had_overflow: bool,
}

impl YogaFlexboxNodeState {
    /// State for a node that has never been laid out.
    pub const NONE: Self = Self { computed_flex_basis: None, had_overflow: false };

    /// Forget the computed flex basis, as Yoga does when a node is marked dirty.
    #[inline]
    pub fn mark_dirty(&mut self) {
        self.computed_flex_basis = None;
    }
}
