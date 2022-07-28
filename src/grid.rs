use crate::geometry::Size;
use crate::node::Node;
use crate::style::GridScalarTrackSizingFunction;
use crate::sys::GridTrackVec;

struct AreaOccupancyMatrix {
    areas: Vec<u16>,
    num_rows: u16,
}

enum GridTrackKind {
    Track,
    Gutter,
}

struct GridTrack {
    kind: GridTrackKind,
    min_track_sizing_function: GridScalarTrackSizingFunction,
    max_track_sizing_function: GridScalarTrackSizingFunction,
    base_size: f32,
    growth_limit: f32,         // Note: can be infinity
    infinitely_growable: bool, // https://www.w3.org/TR/css3-grid-layout/#infinitely-growable
}

struct GridLine {}

enum AvailableSpace {
    Definite(f32),
    MinContent,
    MaxContent,
}

enum GridPosition {
    Auto,
    Line(u8),
}

struct GridItem {
    node: Node,
    min_content_contribution: Option<Size<f32>>,
    max_content_contribution: Option<Size<f32>>,
    row_start: GridPosition,
    row_end: GridPosition,
    column_start: GridPosition,
    column_end: GridPosition,
}

impl GridItem {
    fn new(node: Node) -> Self {
        GridItem {
            node,
            min_content_contribution: None,
            max_content_contribution: None,
            row_start: GridPosition::Auto,
            row_end: GridPosition::Auto,
            column_start: GridPosition::Auto,
            column_end: GridPosition::Auto,
        }
    }
}

struct Grid {
    width: AvailableSpace,
    height: AvailableSpace,
    columns: GridTrackVec<GridTrack>,
    rows: GridTrackVec<GridTrack>,
    area_occupancy_matrix: AreaOccupancyMatrix,
    column_gutters: GridTrackVec<GridLine>,
    row_gutters: GridTrackVec<GridLine>,
    items: Vec<GridItem>,
}
