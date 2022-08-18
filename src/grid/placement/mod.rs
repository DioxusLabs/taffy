pub mod cell_occupancy;
pub mod coordinates;
pub mod estimate_size;
pub mod placement_algo;

pub(in crate::grid) use cell_occupancy::{CellOccupancyMatrix, CellOccupancyState, TrackCounts};
pub(in crate::grid) use estimate_size::compute_grid_size_estimate;
pub(in crate::grid) use placement_algo::place_grid_items;
