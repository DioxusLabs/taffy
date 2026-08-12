//! Style types for Table layout
use crate::geometry::Size;
use crate::style::LengthPercentage;
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
        TableRole::Cell
    }

    /// The number of columns this cell spans (HTML `colspan`)
    #[inline(always)]
    fn colspan(&self) -> u16 {
        Style::<Self::CustomIdent>::DEFAULT.colspan
    }

    /// The number of rows this cell spans (HTML `rowspan`)
    #[inline(always)]
    fn rowspan(&self) -> u16 {
        Style::<Self::CustomIdent>::DEFAULT.rowspan
    }
}
