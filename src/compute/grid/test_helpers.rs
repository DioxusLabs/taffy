use crate::prelude::*;
use crate::style::Dimension::Points;
use crate::style::GridPlacement::{self, *};
use crate::style::Style;
use crate::style::TrackSizingFunction;

pub(crate) trait CreateParentTestNode {
    fn into_grid(&self) -> Style;
}
impl CreateParentTestNode for (f32, f32, i32, i32) {
    fn into_grid(&self) -> Style {
        Style {
            display: Display::Grid,
            size: Size { width: Points(self.0 as f32), height: Points(self.1 as f32) },
            grid_template_columns: vec![TrackSizingFunction::Flex(1f32); self.2 as usize],
            grid_template_rows: vec![TrackSizingFunction::Flex(1f32); self.3 as usize],
            ..Default::default()
        }
    }
}
pub(crate) trait CreateChildTestNode {
    fn into_grid_child(&self) -> Style;
}
impl CreateChildTestNode for (GridPlacement, GridPlacement, GridPlacement, GridPlacement) {
    fn into_grid_child(&self) -> Style {
        Style {
            display: Display::Grid,
            grid_column: Line { start: self.0, end: self.1 },
            grid_row: Line { start: self.2, end: self.3 },
            ..Default::default()
        }
    }
}
