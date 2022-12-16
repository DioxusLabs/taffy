//! Alignment of tracks and final positioning of items
use super::types::{GridItem, GridTrack};
use crate::axis::InBothAbsAxis;
use crate::compute::common::alignment::compute_alignment_offset;
use crate::compute::compute_node_layout;
use crate::geometry::{Line, Point, Size};
use crate::layout::{Layout, RunMode, SizingMode};
use crate::resolve::MaybeResolve;
use crate::style::{AlignContent, AlignItems, AlignSelf, AvailableSpace};
use crate::sys::{f32_max, f32_min};
use crate::tree::LayoutTree;

/// Align the grid tracks within the grid according to the align-content (rows) or
/// justify-content (columns) property. This only does anything if the size of the
/// grid is not equal to the size of the grid container in the axis being aligned.
pub(super) fn align_tracks(
    grid_container_size: Option<f32>,
    tracks: &mut [GridTrack],
    track_alignment_style: AlignContent,
) {
    let used_size: f32 = tracks.iter().map(|track| track.base_size).sum();
    let size_diff = grid_container_size.map(|container_size| container_size - used_size).unwrap_or(0.0);
    let free_space = f32_max(size_diff, 0.0);
    let overflow = f32_min(size_diff, 0.0);

    // If the used_size > grid> container_size then the tracks must overflow their container
    // The direction in which they do so is determined by the alignment style
    let origin = match track_alignment_style {
        AlignContent::Start => 0.0,
        AlignContent::End => overflow,
        AlignContent::Center => overflow / 2.0,
        AlignContent::Stretch => 0.0,
        AlignContent::SpaceBetween => 0.0,
        AlignContent::SpaceEvenly => 0.0,
        AlignContent::SpaceAround => 0.0,
    };

    // Count the number of tracks (not counting gutters)
    let num_tracks = (tracks.len() - 1) / 2;

    // Grid layout treats gaps as full tracks rather than applying them at alignment so we
    // simply pass zero here. Grid layout is never reversed.
    let gap = 0.0;
    let layout_is_reversed = false;

    // Compute offsets
    let mut total_offset = origin;
    tracks.iter_mut().enumerate().for_each(|(i, track)| {
        // Odd tracks are gutters (but slices are zero-indexed, so odd tracks have even indicies)
        let is_gutter = i % 2 == 0;

        // The first non-gutter track is index 1
        let is_first = i == 1;

        let offset = if is_gutter {
            0.0
        } else {
            compute_alignment_offset(free_space, num_tracks, gap, track_alignment_style, layout_is_reversed, is_first)
        };

        track.offset = total_offset + offset;
        total_offset = total_offset + offset + track.base_size;
    });
}

/// Align and size a grid item into it's final position
pub(super) fn align_and_position_item(
    tree: &mut impl LayoutTree,
    order: u32,
    item: &GridItem,
    tracks: InBothAbsAxis<&[GridTrack]>,
    available_space: Size<AvailableSpace>,
    alignment_styles: InBothAbsAxis<Option<AlignItems>>,
) {
    let style = tree.style(item.node);
    let aspect_ratio = style.aspect_ratio;
    let justify_self = style.justify_self;
    let align_self = style.align_self;
    let inherent_size = style.size.maybe_resolve(available_space.into_options());

    let mut measure_node = |axis_available_space| {
        compute_node_layout(
            tree,
            item.node,
            Size::NONE,
            Size { width: AvailableSpace::Definite(axis_available_space), height: AvailableSpace::MaxContent },
            RunMode::ComputeSize,
            SizingMode::InherentSize,
        )
        .width
    };
    let (x, width) = align_and_size_item_within_area(
        tracks.horizontal,
        item.column_indexes,
        justify_self.or(alignment_styles.horizontal),
        inherent_size.width,
        aspect_ratio,
        &mut measure_node,
    );

    let mut measure_node = |axis_available_space| {
        compute_node_layout(
            tree,
            item.node,
            Size { width: Some(width), height: None },
            Size { width: AvailableSpace::MaxContent, height: AvailableSpace::Definite(axis_available_space) },
            RunMode::ComputeSize,
            SizingMode::InherentSize,
        )
        .height
    };
    let (y, height) = align_and_size_item_within_area(
        tracks.vertical,
        item.row_indexes,
        align_self.or(alignment_styles.vertical),
        inherent_size.height,
        aspect_ratio,
        &mut measure_node,
    );

    *tree.layout_mut(item.node) = Layout { order, size: Size { width, height }, location: Point { x, y } };
}

/// Align and size a grid item along a single axis
pub(super) fn align_and_size_item_within_area(
    tracks: &[GridTrack],
    indexes: Line<u16>,
    alignment_style: Option<AlignSelf>,
    size: Option<f32>,
    aspect_ratio: Option<f32>,
    mut measure_node: impl FnMut(f32) -> f32,
) -> (f32, f32) {
    // Calculate grid area dimension in the axis
    let area_start = tracks[(indexes.start + 1) as usize].offset;
    let area_end = {
        let row = &tracks[(indexes.end - 1) as usize];
        row.offset + row.base_size
    };
    let free_space = area_end - area_start;
    let origin = f32_max(free_space, 0.0);

    // Compute default alignment style if it set on neither the parent or the node itself
    let alignment_style = alignment_style.unwrap_or_else(|| {
        if size.is_some() || aspect_ratio.is_some() {
            AlignSelf::Start
        } else {
            AlignSelf::Stretch
        }
    });

    // Compute size in the axis
    let size = size.unwrap_or_else(|| match alignment_style {
        AlignItems::Stretch => free_space,
        _ => measure_node(free_space),
    });

    // Compute offset in the axis
    let offset_within_area = match alignment_style {
        AlignSelf::Start => 0.0,
        AlignSelf::End => origin - size,
        AlignSelf::Center => (origin - size) / 2.0,
        // TODO: Add support for baseline alignment. For now we treat it as "start".
        AlignSelf::Baseline => 0.0,
        AlignSelf::Stretch => 0.0,
    };

    (area_start + offset_within_area, size)
}
