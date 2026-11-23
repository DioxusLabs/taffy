//! Style types for Table layout
use crate::geometry::Size;
use crate::style::{AlignContent, LengthPercentage};
use crate::{CoreStyle, Style};

/// The value of the `table-layout` property, which selects the algorithm
/// used to resolve column widths.
///
/// <https://www.w3.org/TR/css-tables-3/#propdef-table-layout>
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum TableLayout {
    /// Column widths are derived from the content of all cells
    #[default]
    Auto,
    /// Column widths are derived from the table's width and the first row of cells
    Fixed,
}

#[cfg(feature = "parse")]
crate::util::parse::impl_parse_for_keyword_enum!(TableLayout,
    "auto" => Auto,
    "fixed" => Fixed,
);

/// The value of the `visibility` property, as far as layout is concerned.
///
/// Only table layout reads this: a row, row group, column or column group with
/// `visibility: collapse` is removed from the table and the space it took up is given to the
/// content after it (CSS 2.1 §17.5.5). Elsewhere `visibility` affects painting only, which is
/// the responsibility of the code painting the tree.
///
/// <https://www.w3.org/TR/CSS21/visufx.html#propdef-visibility>
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum Visibility {
    /// The box is visible
    #[default]
    Visible,
    /// The box is invisible but still takes up space
    Hidden,
    /// Rows, row groups, columns and column groups are removed from the table; for any other
    /// box this is the same as `hidden`
    Collapse,
}

#[cfg(feature = "parse")]
crate::util::parse::impl_parse_for_keyword_enum!(Visibility,
    "visible" => Visible,
    "hidden" => Hidden,
    "collapse" => Collapse,
);

/// Which side of the table a caption is placed on (CSS `caption-side`)
///
/// <https://www.w3.org/TR/css-tables-3/#caption-side-property>
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum CaptionSide {
    /// The caption is placed above the table box
    #[default]
    Top,
    /// The caption is placed below the table box
    Bottom,
}

#[cfg(feature = "parse")]
crate::util::parse::impl_parse_for_keyword_enum!(CaptionSide,
    "top" => Top,
    "bottom" => Bottom,
);

/// The role a child node plays within a table, as determined by its `display` value.
///
/// Taffy expects a well-formed table tree (table → row group / row → cell) as produced
/// by the box fixup described in <https://www.w3.org/TR/css-tables-3/#fixup>. Fixup itself
/// is the responsibility of the code constructing the tree.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum TableRole {
    /// `display: table-cell`
    #[default]
    Cell,
    /// `display: table-row`
    Row,
    /// `display: table-row-group` (thead, tbody, tfoot)
    RowGroup,
    /// `display: table-column-group` (colgroup)
    ColumnGroup,
    /// `display: table-column` (col)
    Column,
    /// `display: table-caption` (caption)
    Caption,
    /// Any other display value
    Other,
}

/// The set of styles required for a Table layout container
pub trait TableContainerStyle: CoreStyle {
    /// The distance between adjacent cell borders (CSS `border-spacing`)
    #[inline(always)]
    fn border_spacing(&self) -> Size<LengthPercentage> {
        Style::<Self::CustomIdent>::DEFAULT.border_spacing
    }

    /// Which column sizing algorithm to use (CSS `table-layout`)
    #[inline(always)]
    fn table_layout(&self) -> TableLayout {
        Style::<Self::CustomIdent>::DEFAULT.table_layout
    }
}

/// The set of styles required for the descendants of a Table container
/// (row groups, rows, and cells)
pub trait TableItemStyle: CoreStyle {
    /// The item's role within the table, derived from its `display` value
    #[inline(always)]
    fn table_role(&self) -> TableRole {
        TableRole::Other
    }

    /// How a cell aligns its content in the block axis. `normal` means the cell takes part in
    /// its row's baseline alignment
    #[inline(always)]
    fn align_content(&self) -> AlignContent {
        Style::<Self::CustomIdent>::DEFAULT.align_content
    }

    /// The number of columns this cell spans (HTML `colspan`), or the number of columns a column
    /// or column group spans (HTML `span`)
    #[inline(always)]
    fn colspan(&self) -> u16 {
        Style::<Self::CustomIdent>::DEFAULT.colspan
    }

    /// Whether a row, row group, column or column group is collapsed out of the table
    /// (`visibility: collapse`)
    #[inline(always)]
    fn visibility(&self) -> Visibility {
        Style::<Self::CustomIdent>::DEFAULT.visibility
    }

    /// The number of rows this cell spans (HTML `rowspan`)
    #[inline(always)]
    fn rowspan(&self) -> u16 {
        Style::<Self::CustomIdent>::DEFAULT.rowspan
    }

    /// Which side of the table a caption is placed on (`caption-side`)
    #[inline(always)]
    fn caption_side(&self) -> CaptionSide {
        Style::<Self::CustomIdent>::DEFAULT.caption_side
    }
}
