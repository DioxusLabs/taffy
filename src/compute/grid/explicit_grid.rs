use super::placement::TrackCounts;
use super::types::GridTrack;
use crate::style::{Dimension, Style, TrackSizingFunction};
use crate::sys::{GridTrackVec, Vec};
use core::cmp::{max, min};

pub(crate) fn compute_explicit_grid_size(style: &Style) -> (u16, u16) {
    let explicit_col_count = max(style.grid_template_columns.len(), 1) as u16;
    let explicit_row_count = max(style.grid_template_rows.len(), 1) as u16;

    (explicit_col_count, explicit_row_count)
}

pub(super) fn initialize_grid_tracks(
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
        let max_count = max(auto_tracks.len(), counts.negative_implicit as usize);
        let min_count = min(auto_tracks.len(), counts.negative_implicit as usize);
        let offset = max_count % min_count as usize;
        let iter = auto_tracks.iter().copied().cycle().skip(offset);
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
    use super::initialize_grid_tracks;
    use crate::compute::grid::placement::TrackCounts;
    use crate::compute::grid::test_helpers::*;
    use crate::compute::grid::types::GridTrackKind;
    use crate::style::MaxTrackSizingFunction;
    use crate::style::MinTrackSizingFunction;
    use crate::style::{Dimension, TrackSizingFunction};

    #[test]
    fn explicit_grid_sizing() {
        let grid_style = (600.0, 600.0, 2, 4).into_grid();
        let (width, height) = compute_explicit_grid_size(&grid_style);
        assert_eq!(width, 2);
        assert_eq!(height, 4);
    }

    #[test]
    fn test_initialize_grid_tracks() {
        let px20 = Dimension::Points(20.0);
        let px100 = Dimension::Points(100.0);

        // Setup test
        let track_template = vec![
            TrackSizingFunction::Fixed(px100),
            TrackSizingFunction::MinMax {
                min: MinTrackSizingFunction::Fixed(px100),
                max: MaxTrackSizingFunction::Flex(2.0),
            },
            TrackSizingFunction::Flex(1.0),
        ];
        let track_counts =
            TrackCounts { negative_implicit: 3, explicit: track_template.len() as u16, positive_implicit: 3 };
        let auto_tracks = vec![TrackSizingFunction::Auto, TrackSizingFunction::Fixed(px100)];
        let gap = px20;

        // Call function
        let mut tracks = Vec::new();
        initialize_grid_tracks(&mut tracks, track_counts, &track_template, &auto_tracks, gap);

        // Assertions
        let expected = vec![
            // Gutter
            (GridTrackKind::Gutter, MinTrackSizingFunction::Fixed(px20), MaxTrackSizingFunction::Fixed(px20)),
            // Negative implict tracks
            (GridTrackKind::Track, MinTrackSizingFunction::Fixed(px100), MaxTrackSizingFunction::Fixed(px100)),
            (GridTrackKind::Gutter, MinTrackSizingFunction::Fixed(px20), MaxTrackSizingFunction::Fixed(px20)),
            (GridTrackKind::Track, MinTrackSizingFunction::Auto, MaxTrackSizingFunction::Auto),
            (GridTrackKind::Gutter, MinTrackSizingFunction::Fixed(px20), MaxTrackSizingFunction::Fixed(px20)),
            (GridTrackKind::Track, MinTrackSizingFunction::Fixed(px100), MaxTrackSizingFunction::Fixed(px100)),
            (GridTrackKind::Gutter, MinTrackSizingFunction::Fixed(px20), MaxTrackSizingFunction::Fixed(px20)),
            // Explicit tracks
            (GridTrackKind::Track, MinTrackSizingFunction::Fixed(px100), MaxTrackSizingFunction::Fixed(px100)),
            (GridTrackKind::Gutter, MinTrackSizingFunction::Fixed(px20), MaxTrackSizingFunction::Fixed(px20)),
            (GridTrackKind::Track, MinTrackSizingFunction::Fixed(px100), MaxTrackSizingFunction::Flex(2.0)), // Note: separate min-max functions
            (GridTrackKind::Gutter, MinTrackSizingFunction::Fixed(px20), MaxTrackSizingFunction::Fixed(px20)),
            (GridTrackKind::Track, MinTrackSizingFunction::Auto, MaxTrackSizingFunction::Flex(1.0)), // Note: min sizing function of flex sizing functions is auto
            (GridTrackKind::Gutter, MinTrackSizingFunction::Fixed(px20), MaxTrackSizingFunction::Fixed(px20)),
            // Positive implict tracks
            (GridTrackKind::Track, MinTrackSizingFunction::Auto, MaxTrackSizingFunction::Auto),
            (GridTrackKind::Gutter, MinTrackSizingFunction::Fixed(px20), MaxTrackSizingFunction::Fixed(px20)),
            (GridTrackKind::Track, MinTrackSizingFunction::Fixed(px100), MaxTrackSizingFunction::Fixed(px100)),
            (GridTrackKind::Gutter, MinTrackSizingFunction::Fixed(px20), MaxTrackSizingFunction::Fixed(px20)),
            (GridTrackKind::Track, MinTrackSizingFunction::Auto, MaxTrackSizingFunction::Auto),
            (GridTrackKind::Gutter, MinTrackSizingFunction::Fixed(px20), MaxTrackSizingFunction::Fixed(px20)),
        ];

        assert_eq!(tracks.len(), expected.len(), "Number of tracks doesn't match");

        for (idx, (actual, (kind, min, max))) in tracks.into_iter().zip(expected).enumerate() {
            assert_eq!(actual.kind, kind, "Track {idx} (0-based index)");
            assert_eq!(actual.min_track_sizing_function, min, "Track {idx} (0-based index)");
            assert_eq!(actual.max_track_sizing_function, max, "Track {idx} (0-based index)");
        }
    }
}
