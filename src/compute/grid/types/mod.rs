//! Structs and enums that are used within the grid module
use crate::geometry::{InBothAbsAxis, Line};
use crate::style::OriginZeroGridPlacement;

mod cell_occupancy;
mod coordinates;
mod grid_item;
mod grid_track;
mod grid_track_counts;
mod named;

// Publish only locally in the grid module
pub(super) use cell_occupancy::{CellOccupancyMatrix, CellOccupancyState};
pub(crate) use coordinates::{GridCoordinate, GridLine, OriginZeroLine, MAX_GRID_TRACKS, MAX_OZ_LINE, MIN_OZ_LINE};
pub(super) use grid_item::GridItem;
pub(super) use grid_track::GridTrack;
pub(super) use grid_track_counts::TrackCounts;
pub(super) use named::{GridAreasMap, NamedLineResolver};

/// A grid item's placement styles (`grid-row`/`grid-column`) resolved to origin-zero coordinates
pub(super) type ItemPlacement = InBothAbsAxis<Line<OriginZeroGridPlacement>>;
pub use named::{GridLineNames, GridLineNamesIter};

#[allow(unused_imports)]
pub(super) use grid_track::GridTrackKind;
