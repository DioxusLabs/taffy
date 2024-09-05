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