//! Style types for Block layout
use crate::{CoreStyle, Style};

/// The set of styles required for a Block layout container
pub trait BlockContainerStyle: CoreStyle {
    /// Defines which row in the grid the item should start and end at
    #[inline(always)]
    fn text_align(&self) -> TextAlign {
        Style::DEFAULT.text_align
    }
}

/// The set of styles required for a Block layout item (child of a Block container)
pub trait BlockItemStyle: CoreStyle {
    /// Whether the item is a table. Table children are handled specially in block layout.
    #[inline(always)]
    fn is_table(&self) -> bool {
        false
    }
}

/// Used by block layout to implement the legacy behaviour of `<center>` and `<div align="left | right | center">`
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum TextAlign {
    /// No special legacy text align behaviour.
    #[default]
    Auto,
    /// Corresponds to `-webkit-left` or `-moz-left` in browsers
    LegacyLeft,
    /// Corresponds to `-webkit-right` or `-moz-right` in browsers
    LegacyRight,
    /// Corresponds to `-webkit-center` or `-moz-center` in browsers
    LegacyCenter,
}
