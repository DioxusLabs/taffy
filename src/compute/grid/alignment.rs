use super::types::GridTrack;
use crate::compute::common::alignment::compute_alignment_offset;
use crate::style::AlignContent;
use crate::sys::f32_max;

pub(super) fn align_tracks(
    grid_container_size: Option<f32>,
    tracks: &mut [GridTrack],
    track_alignment_style: AlignContent,
) {
    let used_size: f32 = tracks.iter().map(|track| track.base_size).sum();
    let free_space = grid_container_size.map(|container_size| f32_max(container_size - used_size, 0.0)).unwrap_or(0.0);

    // If the used_size > grid> container_size then the tracks must overflow their container
    // The direction in which they do so is determined by the alignment style
    let origin = grid_container_size
        .map(|container_size| match track_alignment_style {
            AlignContent::FlexStart => 0.0,
            AlignContent::FlexEnd => container_size - used_size,
            AlignContent::Center => (container_size - used_size) / 2.0,
            AlignContent::Stretch => 0.0,
            AlignContent::SpaceBetween => 0.0,
            AlignContent::SpaceEvenly => 0.0,
            AlignContent::SpaceAround => 0.0,
        })
        .unwrap_or(0.0);

    // Count the number of tracks (not counting gutters)
    let num_tracks = (tracks.len() - 1) / 2;

    // Grid layout treats gaps as full tracks rather than applying them at alignment so we
    // simply pass zero here. Grid layout is never reversed.
    let gap = 0.0;
    let layout_is_reversed = false;

    // Compute offsets
    let mut total_offset = origin;
    tracks.iter_mut().enumerate().for_each(|(i, track)| {
        let is_first = i == 0;
        let offset =
            compute_alignment_offset(free_space, num_tracks, gap, track_alignment_style, layout_is_reversed, is_first);
        track.offset = total_offset + offset;
        total_offset = total_offset + offset + track.base_size;
    });
}
