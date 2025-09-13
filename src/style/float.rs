//! Style types for float layout

/// Floats a box to the left or right.
/// This property only applies to children of a block layout
///
/// See <https://developer.mozilla.org/en-US/docs/Web/CSS/float>
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum Float {
    /// The box is floated to the left
    Left,
    /// The box is floated to the right
    Right,
    /// The box is not floated
    #[default]
    None,
}

impl Float {
    pub fn is_floated(self) -> bool {
        matches!(self, Self::Left | Self::Right)
    }
}

/// Gives a box "clearance", which moves it below floated boxes which precede
/// it in the tree.
///
/// See <https://developer.mozilla.org/en-US/docs/Web/CSS/clear>
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum Clear {
    /// The box clears left-floated boxes
    Left,
    /// The box clears right-floated boxes
    Right,
    /// The box clears boxes floated in either direction
    Both,
    /// The box does not clear floated boxes
    #[default]
    None,
}
