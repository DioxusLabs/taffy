use super::placement::TrackCounts;
use crate::geometry::{Line, Size};
use crate::layout::{AvailableSpace, SizingMode};
use crate::node::Node;
use crate::style::{AlignContent, Dimension, MaxTrackSizingFunction, MinTrackSizingFunction, Style};
// use super::AbsoluteAxis;
use super::types::{GridAxis, GridItem, GridTrack};
use crate::sys::{f32_max, f32_min};
use core::cmp::Ordering;

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub(in super::super) enum AvailableSpaceMode {
    Estimates,
    OtherAxisSizes,
}

/// To make track sizing efficient we want to order tracks
/// Here a placement is either a Line<i16> representing a row-start/row-end or a column-start/column-end
#[inline(always)]
pub(in super::super) fn cmp_by_span_then_start(
    get_placement: impl Fn(&GridItem) -> Line<u16>,
) -> impl FnMut(&GridItem, &GridItem) -> Ordering {
    move |item_a: &GridItem, item_b: &GridItem| -> Ordering {
        let placement_a = get_placement(item_a);
        let placement_b = get_placement(item_b);
        let a_span = placement_a.end - placement_a.start;
        let b_span = placement_b.end - placement_b.start;
        match a_span.cmp(&b_span) {
            Ordering::Less => Ordering::Less,
            Ordering::Greater => Ordering::Greater,
            Ordering::Equal => placement_a.start.cmp(&placement_b.start),
        }
    }
}

pub(in super::super) fn compute_alignment_gutter_adjustment(
    alignment: AlignContent,
    available_space: AvailableSpace,
    get_track_size_estimate: impl Fn(&GridTrack, AvailableSpace) -> Option<f32>,
    tracks: &[GridTrack],
) -> f32 {
    if alignment.inner_gutter_weight() > 0 && available_space.is_definite() && tracks.len() > 1 {
        let inner_available_space = tracks
            .iter()
            .map(|track| get_track_size_estimate(track, available_space))
            .sum::<Option<f32>>()
            .map(|track_size_sum| f32_max(0.0, available_space.unwrap() - track_size_sum))
            .unwrap_or(0.0);

        let weighted_track_count = (((tracks.len() - 3) / 2) * alignment.inner_gutter_weight() as usize)
            + (2 * alignment.outer_gutter_weight() as usize);

        (inner_available_space / weighted_track_count as f32) * alignment.inner_gutter_weight() as f32
    } else {
        0.0
    }
}

/// Convert origin-zero coordinates track placement in grid track vector indexes
pub(in super::super) fn resolve_item_track_indexes(
    items: &mut Vec<GridItem>,
    column_counts: TrackCounts,
    row_counts: TrackCounts,
) {
    for item in items {
        item.column_indexes = item.column.map(|oz_index| column_counts.oz_line_to_grid_track_vec_index(oz_index));
        item.row_indexes = item.row.map(|oz_index| row_counts.oz_line_to_grid_track_vec_index(oz_index));
    }
}

/// Determine (in each axis) whether the item crosses any flexible tracks
pub(in super::super) fn determine_if_item_crosses_flexible_tracks(
    items: &mut Vec<GridItem>,
    columns: &Vec<GridTrack>,
    rows: &Vec<GridTrack>,
) {
    for item in items {
        item.crosses_flexible_column =
            (item.column_indexes.start..=item.column_indexes.end).any(|i| columns[i as usize].is_flexible());
        item.crosses_flexible_row =
            (item.row_indexes.start..=item.row_indexes.end).any(|i| rows[i as usize].is_flexible());
    }
}

pub(in super::super) fn track_sizing_algorithm(
    available_space: Size<AvailableSpace>,
    available_space_mode: AvailableSpaceMode,
    axis: GridAxis,
    columns: &mut [GridTrack],
    container_style: &Style,
    rows: &mut [GridTrack],
    items: &mut [GridItem],
    style: &Style,
    measure_node: impl Fn(Node, Size<Option<f32>>, Size<AvailableSpace>, SizingMode) -> Size<f32>,
) {
    let get_track_size_estimate = match available_space_mode {
        AvailableSpaceMode::Estimates => |track: &GridTrack, available_space: AvailableSpace| {
            track.max_track_sizing_function.definite_value(available_space)
        },
        AvailableSpaceMode::OtherAxisSizes => |track: &GridTrack, _| Some(track.base_size),
    };

    #[inline(always)]
    fn get_column_placement(item: &GridItem) -> Line<u16> {
        item.column_indexes
    }
    #[inline(always)]
    fn get_row_placement(item: &GridItem) -> Line<u16> {
        item.row_indexes
    }

    // The track sizing algorithm is generic over which axis it operates over, but it is performance sensitive
    // we don't want to perform a dynamic lookup every time we access a property, so we instead pass in getter functions
    // under the assumption that the inner function will be monomorphised, and they'll be inlined
    match axis {
        GridAxis::Inline => {
            #[inline(always)]
            fn get_column_cross_flex_track(item: &GridItem) -> bool {
                item.crosses_flexible_column
            }
            track_sizing_algorithm_inner(
                axis,
                available_space,
                container_style.min_size.width.get_absolute(),
                container_style.max_size.width.get_absolute(),
                columns,
                rows,
                items,
                style,
                get_track_size_estimate,
                get_column_placement,
                get_row_placement,
                get_column_cross_flex_track,
                measure_node,
            );
        }
        GridAxis::Block => {
            #[inline(always)]
            fn get_row_crosses_flex_track(item: &GridItem) -> bool {
                item.crosses_flexible_row
            }
            track_sizing_algorithm_inner(
                axis,
                available_space,
                container_style.min_size.height.get_absolute(),
                container_style.max_size.height.get_absolute(),
                columns,
                rows,
                items,
                style,
                get_track_size_estimate,
                get_row_placement,
                get_column_placement,
                get_row_crosses_flex_track,
                measure_node,
            );
        }
    }
}

/// Track sizing algorithm
/// Note: Gutters are treated as empty fixed-size tracks for the purpose of the track sizing algorithm.
pub(in super::super) fn track_sizing_algorithm_inner(
    axis: GridAxis,
    available_space: Size<AvailableSpace>,
    axis_min_size: Option<f32>,
    axis_max_size: Option<f32>,
    axis_tracks: &mut [GridTrack],
    other_axis_tracks: &mut [GridTrack],
    items: &mut [GridItem],
    style: &Style,
    get_track_size_estimate: impl Fn(&GridTrack, AvailableSpace) -> Option<f32>,
    get_item_placement: impl Fn(&GridItem) -> Line<u16>,
    get_other_axis_placement: impl Fn(&GridItem) -> Line<u16>,
    get_crosses_flex_track: impl Fn(&GridItem) -> bool,
    measure_node: impl Fn(Node, Size<Option<f32>>, Size<AvailableSpace>, SizingMode) -> Size<f32>,
) {
    // 11.4 Initialise Track sizes
    // Initialize each track’s base size and growth limit.
    for track in axis_tracks {
        // For each track, if the track’s min track sizing function is:
        // - A fixed sizing function
        //     Resolve to an absolute length and use that size as the track’s initial base size.
        //     Note: Indefinite lengths cannot occur, as they’re treated as auto.
        // - An intrinsic sizing function
        //     Use an initial base size of zero.
        track.base_size = track.min_track_sizing_function.definite_value(available_space.get(axis)).unwrap_or(0.0);

        // For each track, if the track’s max track sizing function is:
        // - A fixed sizing function
        //     Resolve to an absolute length and use that size as the track’s initial growth limit.
        // - An intrinsic sizing function
        //     Use an initial growth limit of infinity.
        // - A flexible sizing function
        //     Use an initial growth limit of infinity.
        track.growth_limit =
            track.max_track_sizing_function.definite_value(available_space.get(axis)).unwrap_or(f32::INFINITY);

        // In all cases, if the growth limit is less than the base size, increase the growth limit to match the base size.
        if track.growth_limit < track.base_size {
            track.growth_limit = track.base_size;
        }
    }

    // Pre-computations for 11.5 Resolve Intrinsic Track Sizes

    // The track sizing algorithm requires us to iterate through the items in ascendeding order of the number of
    // tracks they span (first items that span 1 track, then items that span 2 tracks, etc).
    // To avoid having to do multiple iterations of the items, we pre-sort them into this order.
    items.sort_by(cmp_by_span_then_start(get_item_placement));

    // Compute an additional amount to add to each spanned gutter when computing item's estimated size in the
    // in the opposite axis based on the alignment, container size, and estimated track sizes in that axis
    let gutter_alignment_adjustment = compute_alignment_gutter_adjustment(
        style.grid_align_content(axis.other()),
        available_space.get(axis.other()),
        &get_track_size_estimate,
        &other_axis_tracks,
    );

    // 11.5b Resolve Intrinsic Track Sizes

    // Step 1. Shim baseline-aligned items so their intrinsic size contributions reflect their baseline alignment.
    // TODO: we do not yet support baseline alignment for CSS Grid

    // Step 2. We skip Step 2 as it is noted that:
    //
    //    This step is a simplification of the steps below for handling spanning items, and should yield
    //    the same behavior as running those instructions on items with a span of 1.
    //
    // We choose this alternative of running Step 3 on items with a span of 1 as we need to write the code for this anyway.

    // Step 3.
    // Iterate over items that don't cross a flex track. Items should have already been sorted in ascending order
    // of the number of tracks they cross.
    for item in items.iter_mut().filter(|item| !get_crosses_flex_track(item)) {
        let item_other_axis_size: Option<f32> = {
            let available_space = available_space.get(axis.other());
            let placement = get_other_axis_placement(item);
            (&other_axis_tracks)[(placement.start as usize + 1)..(placement.end as usize)]
                .iter()
                .map(|track| get_track_size_estimate(track, available_space))
                .sum::<Option<f32>>()
        };

        // TODO: be smarter about only computing these when they are required
        let min_content_size = measure_node(
            item.node,
            Size { width: None, height: item_other_axis_size },
            Size::MIN_CONTENT,
            SizingMode::ContentSize,
        );

        // TODO: resolve styles here
        // let minimum_contributions =

        let max_content_size = measure_node(
            item.node,
            Size { width: None, height: item_other_axis_size },
            Size::MAX_CONTENT,
            SizingMode::ContentSize,
        );

        let placement = get_other_axis_placement(item);
        distribute_space_to_base_size(
            min_content_size.get(axis),
            &mut other_axis_tracks[(placement.start as usize + 1)..(placement.end as usize)],
            IntrinsicContributionType::Minimum,
        );
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
enum IntrinsicContributionType {
    Minimum,
    // MinContent,
    Maximum,
    // MaxContent,
}

/// 11.5.1. Distributing Extra Space Across Spanned Tracks
/// https://www.w3.org/TR/css-grid-1/#extra-space
// TODO: Actually add planned increase to base size
fn distribute_space_to_base_size(
    space: f32,
    tracks: &mut [GridTrack],
    intrinsic_contribution_type: IntrinsicContributionType,
) {
    // Define a small constant to avoid infinite loops due to rounding errors. Rather than stopping distributing
    // extra space when it gets to exactly zero, we will stop when it falls below this amount
    const THRESHOLD: f32 = 0.000001;

    // 1. Find the space to distribute
    let track_sizes: f32 = tracks.iter().map(|track| track.base_size).sum();
    let mut extra_space: f32 = f32_max(0.0, space - track_sizes);

    // 2. Distribute space up to limits:
    // Note: there are two exit conditions to this loop:
    //   - We run out of space to distribute (extra_space falls below THRESHOLD)
    //   - We run out of growable tracks to distribute to
    while extra_space > THRESHOLD {
        let number_of_growable_tracks = tracks
            .iter()
            .filter(|track| track.base_size /*+ track.base_size_planned_increase*/ < track.growth_limit)
            .count();
        if number_of_growable_tracks == 0 {
            break;
        }

        // Compute item-incurred increase for this iteration
        let min_increase_limit = tracks
            .iter()
            .map(|track| track.growth_limit - (track.base_size/*+ track.base_size_planned_increase*/))
            .min_by(|a, b| a.total_cmp(b))
            .unwrap(); // We will never pass an empty track list to this function
        let item_incurred_increase = f32_min(min_increase_limit, extra_space / number_of_growable_tracks as f32);

        // for track in tracks.iter().filter(|track| track.base_size + /*track.base_size_planned_increase*/ < track.growth_limit) {
        //     if item_incurred_increase > track.base_size_planned_increase {
        //         track.base_size_planned_increase += item_incurred_increase;
        //     }
        // }
        for track in tracks.iter_mut().filter(|track| track.base_size < track.growth_limit) {
            track.base_size += item_incurred_increase;
        }

        extra_space -= item_incurred_increase * number_of_growable_tracks as f32;
    }

    // 3. Distribute remaining span beyond limits (if any)
    if extra_space > THRESHOLD {
        // When accommodating minimum contributions or accommodating min-content contributions:
        //   - any affected track that happens to also have an intrinsic max track sizing function;
        // When accommodating max-content contributions:
        //   - any affected track that happens to also have a max-content max track sizing function
        let mut filter = match intrinsic_contribution_type {
            IntrinsicContributionType::Minimum => {
                (|track: &GridTrack| track.max_track_sizing_function.is_intrinsic()) as fn(&GridTrack) -> bool
            }
            IntrinsicContributionType::Maximum => {
                (|track: &GridTrack| track.max_track_sizing_function.is_max_content()) as fn(&GridTrack) -> bool
            }
        };

        // If there are no such tracks (matching filter above), then use all affected tracks.
        let mut number_of_tracks = tracks.iter().filter(|track| filter(track)).count();
        if number_of_tracks == 0 {
            filter = (|_| true) as fn(&GridTrack) -> bool;
            number_of_tracks = tracks.len();
        }

        // Distribute remaining space
        let item_incurred_increase = extra_space / number_of_tracks as f32;
        for track in tracks.iter_mut().filter(|track| filter(track)) {
            track.base_size += item_incurred_increase;
        }
    }
}

/// 11.5.1. Distributing Extra Space Across Spanned Tracks
/// This is simplified (and faster) version of the algorithm for growth limits
/// https://www.w3.org/TR/css-grid-1/#extra-space
// TODO: Actually add planned increase to growth limit
fn distribute_space_to_growth_limit(space: f32, tracks: &mut [GridTrack]) {
    // 1. Find the space to distribute
    let track_sizes: f32 = tracks
        .iter()
        .map(|track| if track.growth_limit == f32::INFINITY { track.base_size } else { track.growth_limit })
        .sum();
    let extra_space: f32 = f32_max(0.0, space - track_sizes);

    // 2. Distribute space up to limits:
    // 3. Distribute space beyond limits
    // If space remains after all tracks are frozen, unfreeze and continue to distribute space to the item-incurred increase
    // when handling any intrinsic growth limit: all affected tracks.
    let number_of_growable_tracks = tracks.iter().filter(|track| track.infinitely_growable).count();
    if number_of_growable_tracks > 0 {
        let item_incurred_increase = extra_space / number_of_growable_tracks as f32;
        for track in tracks.iter_mut().filter(|track| track.infinitely_growable) {
            track.growth_limit = if track.growth_limit == f32::INFINITY {
                track.base_size + item_incurred_increase
            } else {
                track.growth_limit + item_incurred_increase
            }
        }
    } else {
        let item_incurred_increase = extra_space / tracks.len() as f32;
        for track in tracks {
            track.growth_limit = if track.growth_limit == f32::INFINITY {
                track.base_size + item_incurred_increase
            } else {
                track.growth_limit + item_incurred_increase
            }
        }
    };
}
