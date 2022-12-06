mod axis;
mod cell_occupancy;
mod grid_axis_tracks;
mod grid_item;
mod grid_track;
mod grid_track_counts;

// Publish publicly
pub use axis::{AbsoluteAxis, GridAxis};

// Publish only locally in the grid module
pub(super) use cell_occupancy::{CellOccupancyMatrix, CellOccupancyState};
pub(super) use grid_axis_tracks::GridAxisTracks;
pub(super) use grid_item::GridItem;
pub(super) use grid_track::{GridTrack, GridTrackKind};
pub(super) use grid_track_counts::TrackCounts;

// pub(super) enum GridPosition {
//     Auto,
//     LineIndex(i16),
//     LineName(u16),
//     // GridAreaStart(u16),
//     // GridAreaEnd(u16),
// }

// pub(super) struct NamedArea {
//     name: u16,
//     row_start: u16,
//     row_end: u16,
//     column_start: u16,
//     column_end: u16,
// }

// pub(super) struct CssGrid {
//     pub available_space: Size<AvailableSpace>,
//     pub cell_occupancy_matrix: CellOccupancyMatrix,
//     pub items: Vec<GridItem>,
//     pub columns: GridAxisTracks,
//     pub rows: GridAxisTracks,
//     pub named_areas: Vec<NamedArea>,
// }