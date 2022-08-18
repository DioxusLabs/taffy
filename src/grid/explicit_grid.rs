use super::placement::TrackCounts;
use super::types::GridTrack;
use crate::style::{Dimension, Style, TrackSizingFunction};
use crate::sys::GridTrackVec;
use core::cmp::max;

pub(crate) fn compute_explicit_grid_size(style: &Style) -> (u16, u16) {
    let explicit_col_count = max(style.grid_template_columns.len(), 1) as u16;
    let explicit_row_count = max(style.grid_template_rows.len(), 1) as u16;

    (explicit_col_count, explicit_row_count)
}

pub(in crate::grid) fn initialize_grid_tracks(
    tracks: &mut Vec<GridTrack>,
    counts: TrackCounts,
    track_template: &GridTrackVec<TrackSizingFunction>,
    auto_tracks: &Vec<TrackSizingFunction>,
    gap: Dimension,
) {
    // Clear vector (in case this is a re-layout), reserve space for all tracks ahead of time to reduce allocations,
    // and push the initial gutter
    tracks.clear();
    tracks.reserve((counts.len() * 2) + 1);
    tracks.push(GridTrack::gutter(gap));

    // Create negative implicit tracks
    if auto_tracks.is_empty() {
        let iter = core::iter::repeat(TrackSizingFunction::Auto);
        create_implicit_tracks(tracks, counts.negative_implicit, iter, gap)
    } else {
        let offset = auto_tracks.len() % counts.negative_implicit as usize;
        let iter = auto_tracks.iter().copied().skip(offset).cycle();
        create_implicit_tracks(tracks, counts.negative_implicit, iter, gap)
    }

    // Create explicit tracks
    track_template.iter().for_each(|track_sizing_function| {
        tracks.push(GridTrack::new(
            track_sizing_function.min_sizing_function(),
            track_sizing_function.max_sizing_function(),
        ));
        tracks.push(GridTrack::gutter(gap))
    });

    // Create positive implicit tracks
    if auto_tracks.is_empty() {
        let iter = core::iter::repeat(TrackSizingFunction::Auto);
        create_implicit_tracks(tracks, counts.negative_implicit, iter, gap)
    } else {
        let iter = auto_tracks.iter().copied().cycle();
        create_implicit_tracks(tracks, counts.negative_implicit, iter, gap)
    }
}

// Utility function for repeating logic of creating implicit tracks
fn create_implicit_tracks(
    tracks: &mut Vec<GridTrack>,
    count: u16,
    mut auto_tracks_iter: impl Iterator<Item = TrackSizingFunction>,
    gap: Dimension,
) {
    for _ in 0..count {
        let track_def = auto_tracks_iter.next().unwrap();
        tracks.push(GridTrack::new(track_def.min_sizing_function(), track_def.max_sizing_function()));
        tracks.push(GridTrack::gutter(gap));
    }
}

#[cfg(test)]
mod test {
    use super::compute_explicit_grid_size;
    use crate::grid::test_helpers::*;

    #[test]
    fn explicit_grid_sizing() {
        let grid_style = (600.0, 600.0, 2, 4).into_grid();
        let (width, height) = compute_explicit_grid_size(&grid_style);
        assert_eq!(width, 2);
        assert_eq!(height, 4);
    }
}
