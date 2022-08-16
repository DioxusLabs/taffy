use super::super::types::{GridAxisTracks, GridTrack};
use crate::style::{Dimension, TrackSizingFunction};
use crate::sys::GridTrackVec;

/// 7.1. The Explicit Grid
/// Initialise the `rows` and `columns` fields of the `Grid` based on following style properties:
/// - `grid-template-rows`
/// - `grid-template-columns`
pub(in crate::grid) fn resolve_explicit_grid_tracks(
    axis: &mut GridAxisTracks,
    track_template: &GridTrackVec<TrackSizingFunction>,
    gap: Dimension,
) {
    debug_assert!(
        axis.len() == axis.origin,
        "Number of populated tracks should be equal to origin when calling resolve_explicit_grid_track"
    );

    let mut track_count = 0;
    track_template.iter().enumerate().for_each(|(index, track_sizing_function): (usize, &TrackSizingFunction)| {
        // Generate gutter in between each track
        if index != 0 {
            axis.push(GridTrack::gutter(gap))
        }

        // Generate track
        axis.push(GridTrack::new(
            track_sizing_function.min_sizing_function(),
            track_sizing_function.max_sizing_function(),
        ));

        // Count track
        track_count += 1;
    });

    axis.explicit_track_count = track_count;
}
