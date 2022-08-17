use super::super::types::{GridAxisTracks, GridTrack};
use crate::style::{Dimension, Style, TrackSizingFunction};
use crate::sys::GridTrackVec;
use core::cmp::max;

pub(crate) fn compute_explicit_grid_size(style: &Style) -> (u16, u16) {
    let explicit_col_count = max(style.grid_template_columns.len(), 1) as u16;
    let explicit_row_count = max(style.grid_template_rows.len(), 1) as u16;

    (explicit_col_count, explicit_row_count)
}

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
