//! Helpers for use in unit tests within the grid module
use super::super::types::{GridItem, NamedLineResolver};
use super::super::{ItemPlacement, OriginZeroLine};
use crate::geometry::InBothAbsAxis;
use crate::prelude::*;
use crate::style::{Dimension, GridPlacement, Style};
use crate::util::sys::Vec;

/// Create a `GridItem` (with start alignment) for each `(index, style)` pair, as `compute_grid_layout` does
/// for the in-flow children of a grid container
pub(crate) fn grid_items<'a>(children: impl Iterator<Item = (usize, &'a Style)>) -> Vec<GridItem> {
    children
        .map(|(index, style)| {
            GridItem::new_with_style_and_order(
                NodeId::from(index),
                style,
                AlignSelf::START,
                AlignSelf::START,
                index as u16,
            )
        })
        .collect()
}

/// Resolve the named lines in the `grid-row`/`grid-column` styles of the given child styles (and convert them to
/// origin-zero coordinates), as `compute_grid_layout` does before estimating the grid size and placing items
pub(crate) fn resolve_named_lines<'a>(
    child_styles: impl Iterator<Item = &'a Style>,
    name_resolver: &NamedLineResolver<String>,
    explicit_col_count: u16,
    explicit_row_count: u16,
) -> Vec<ItemPlacement> {
    child_styles
        .map(|style| InBothAbsAxis {
            horizontal: name_resolver.resolve_column_names(&style.grid_column).into_origin_zero(explicit_col_count),
            vertical: name_resolver.resolve_row_names(&style.grid_row).into_origin_zero(explicit_row_count),
        })
        .collect()
}

pub(crate) trait CreateParentTestNode {
    fn into_grid(self) -> Style;
}
impl CreateParentTestNode for (f32, f32, i32, i32) {
    fn into_grid(self) -> Style {
        Style {
            display: Display::Grid,
            size: Size { width: Dimension::from_length(self.0), height: Dimension::from_length(self.1) },
            grid_template_columns: vec![fr(1f32); self.2 as usize],
            grid_template_rows: vec![fr(1f32); self.3 as usize],
            ..Default::default()
        }
    }
}
pub(crate) trait CreateChildTestNode {
    fn into_grid_child(self) -> Style;
}
impl CreateChildTestNode
    for (GridPlacement<String>, GridPlacement<String>, GridPlacement<String>, GridPlacement<String>)
{
    fn into_grid_child(self) -> Style<String> {
        Style {
            display: Display::Grid,
            grid_column: Line { start: self.0, end: self.1 },
            grid_row: Line { start: self.2, end: self.3 },
            ..Default::default()
        }
    }
}

pub(crate) trait CreateExpectedPlacement {
    fn into_oz(self) -> (OriginZeroLine, OriginZeroLine, OriginZeroLine, OriginZeroLine);
}
impl CreateExpectedPlacement for (i16, i16, i16, i16) {
    fn into_oz(self) -> (OriginZeroLine, OriginZeroLine, OriginZeroLine, OriginZeroLine) {
        (OriginZeroLine(self.0), OriginZeroLine(self.1), OriginZeroLine(self.2), OriginZeroLine(self.3))
    }
}
