use super::types::{GridAxis, GridItem, GridTrack, TrackCounts};
use crate::compute::compute_node_layout;
use crate::geometry::{Line, Size};
use crate::layout::{AvailableSpace, RunMode, SizingMode};
use crate::math::MaybeMath;
use crate::node::Node;
use crate::prelude::LayoutTree;
use crate::resolve::MaybeResolve;
use crate::style::{AlignContent, MaxTrackSizingFunction, MinTrackSizingFunction, Style};
use crate::sys::{f32_max, f32_min};
use core::cmp::Ordering;

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub(super) enum AvailableSpaceMode {
    Estimates,
    OtherAxisSizes,
}

/// Takes an axis, and a list of grid items sorted firstly by whether they cross a flex track
/// in the specified axis (items that don't cross a flex track first) and then by the number
/// of tracks they cross in specified axis (ascending order).
struct ItemBatcher {
    index_offset: usize,
    axis: GridAxis,
    done: bool,
    current_span: u16,
    current_is_flex: bool,
}

impl ItemBatcher {
    fn new(axis: GridAxis) -> Self {
        ItemBatcher { index_offset: 0, axis, current_span: 1, current_is_flex: false, done: false }
    }

    // This is basically a manual version of Iterator::next which passes `items`
    // in as a parameter on each iteration to work around borrow checker rules
    fn next<'items>(&mut self, items: &'items mut [GridItem]) -> Option<(&'items mut [GridItem], bool)> {
        if self.current_is_flex || self.index_offset >= items.len() {
            return None;
        }

        let item = &items[self.index_offset];
        self.current_span = item.span(self.axis);
        self.current_is_flex = item.crosses_flexible_track(self.axis);

        let next_index_offset = if self.current_is_flex {
            items.len()
        } else {
            items
                .iter()
                // .skip(self.index_offset)
                .position(|item: &GridItem| {
                    item.crosses_flexible_track(self.axis) || item.span(self.axis) > self.current_span
                })
                // .map(|next_index_local_offset| self.index_offset + next_index_local_offset)
                .map(|next_index_local_offset| next_index_local_offset)
                .unwrap_or_else(|| items.len())
        };

        let batch_range = self.index_offset..next_index_offset;
        self.index_offset = next_index_offset;
        if next_index_offset >= items.len() {
            self.done = true;
        }

        let batch = &mut items[batch_range];
        Some((batch, self.current_is_flex))
    }
}

/// To make track sizing efficient we want to order tracks
/// Here a placement is either a Line<i16> representing a row-start/row-end or a column-start/column-end
#[inline(always)]
pub(super) fn cmp_by_cross_flex_then_span_then_start(axis: GridAxis) -> impl FnMut(&GridItem, &GridItem) -> Ordering {
    move |item_a: &GridItem, item_b: &GridItem| -> Ordering {
        match (item_a.crosses_flexible_track(axis), item_b.crosses_flexible_track(axis)) {
            (false, true) => Ordering::Less,
            (true, false) => Ordering::Greater,
            _ => {
                let placement_a = item_a.placement(axis);
                let placement_b = item_b.placement(axis);
                let a_span = placement_a.end - placement_a.start;
                let b_span = placement_b.end - placement_b.start;
                match a_span.cmp(&b_span) {
                    Ordering::Less => Ordering::Less,
                    Ordering::Greater => Ordering::Greater,
                    Ordering::Equal => placement_a.start.cmp(&placement_b.start),
                }
            }
        }
    }
}

pub(super) fn compute_alignment_gutter_adjustment(
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
pub(super) fn resolve_item_track_indexes(
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
pub(super) fn determine_if_item_crosses_flexible_tracks(
    items: &mut Vec<GridItem>,
    columns: &Vec<GridTrack>,
    rows: &Vec<GridTrack>,
) {
    for item in items {
        item.crosses_flexible_column =
            item.track_range_excluding_lines(GridAxis::Inline).any(|i| columns[i as usize].is_flexible());
        item.crosses_flexible_row =
            item.track_range_excluding_lines(GridAxis::Block).any(|i| rows[i as usize].is_flexible());
    }
}

/// Track sizing algorithm
/// Note: Gutters are treated as empty fixed-size tracks for the purpose of the track sizing algorithm.
pub(super) fn track_sizing_algorithm<Tree, MeasureFunc>(
    tree: &mut Tree,
    axis: GridAxis,
    available_space: Size<AvailableSpace>,
    available_grid_space: Size<AvailableSpace>,
    container_style: &Style,
    axis_tracks: &mut [GridTrack],
    other_axis_tracks: &mut [GridTrack],
    items: &mut [GridItem],
    get_track_size_estimate: impl Fn(&GridTrack, AvailableSpace) -> Option<f32>,
    measure_node: MeasureFunc,
) where
    Tree: LayoutTree,
    MeasureFunc: Fn(&mut Tree, Node, Size<Option<f32>>, Size<AvailableSpace>, RunMode, SizingMode) -> Size<f32>,
{
    // 11.4 Initialise Track sizes
    // Initialize each track’s base size and growth limit.
    initialize_track_sizes(axis_tracks, available_space.get(axis));

    // If all tracks have base_size = growth_limit, then skip the rest of this function.
    // Note: this can only happen both track sizing function have the same fixed track sizing function
    if axis_tracks.iter().all(|track| track.base_size == track.growth_limit) {
        return;
    }

    // Pre-computations for 11.5 Resolve Intrinsic Track Sizes

    // The track sizing algorithm requires us to iterate through the items in ascendeding order of the number of
    // tracks they span (first items that span 1 track, then items that span 2 tracks, etc).
    // To avoid having to do multiple iterations of the items, we pre-sort them into this order.
    items.sort_by(cmp_by_cross_flex_then_span_then_start(axis));

    // Compute an additional amount to add to each spanned gutter when computing item's estimated size in the
    // in the opposite axis based on the alignment, container size, and estimated track sizes in that axis
    let gutter_alignment_adjustment = compute_alignment_gutter_adjustment(
        container_style.grid_align_content(axis.other()),
        available_space.get(axis.other()),
        &get_track_size_estimate,
        &other_axis_tracks,
    );

    // 11.5 Resolve Intrinsic Track Sizes
    resolve_intrinsic_track_sizes(
        tree,
        axis,
        axis_tracks,
        other_axis_tracks,
        items,
        available_space,
        get_track_size_estimate,
    );

    // 11.6. Maximise Tracks
    // Distributes free space (if any) to tracks with FINITE growth limits, up to their limits.
    maximise_tracks(axis, axis_tracks, available_grid_space);

    // 11.7. Expand Flexible Tracks
    // This step sizes flexible tracks using the largest value it can assign to an fr without exceeding the available space.
    let axis_min_size = container_style.min_size.get(axis).get_absolute();
    let axis_max_size = container_style.max_size.get(axis).get_absolute();
    expand_flexible_tracks(tree, axis, axis_tracks, items, axis_min_size, axis_max_size, available_grid_space);

    // 11.8. Stretch auto Tracks
    // This step expands tracks that have an auto max track sizing function by dividing any remaining positive, definite free space equally amongst them.
    stretch_auto_tracks(axis, axis_tracks, container_style, available_space, available_grid_space);
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
enum IntrinsicContributionType {
    Minimum,
    // MinContent,
    Maximum,
    // MaxContent,
}

fn flush_planned_base_size_increases(tracks: &mut [GridTrack]) {
    for track in tracks {
        track.base_size += track.base_size_planned_increase;
        track.base_size_planned_increase = 0.0;
    }
}

fn flush_planned_growth_limit_increases(tracks: &mut [GridTrack], set_infinitely_growable: bool) {
    for track in tracks {
        if track.growth_limit_planned_increase > 0.0 {
            track.growth_limit = if track.growth_limit == f32::INFINITY {
                track.base_size + track.growth_limit_planned_increase
            } else {
                track.growth_limit + track.growth_limit_planned_increase
            };
            // track.growth_limit = track.base_size + track.growth_limit_planned_increase;
            track.infinitely_growable = set_infinitely_growable;
        } else {
            track.infinitely_growable = false;
        }
        track.growth_limit_planned_increase = 0.0
    }
}

// 11.4 Initialise Track sizes
// Initialize each track’s base size and growth limit.
fn initialize_track_sizes(axis_tracks: &mut [GridTrack], axis_available_space: AvailableSpace) {
    let last_track_idx = axis_tracks.len() - 1;

    // First and last grid lines are always zero-sized.
    axis_tracks[0].base_size = 0.0;
    axis_tracks[0].growth_limit = 0.0;
    axis_tracks[last_track_idx].base_size = 0.0;
    axis_tracks[last_track_idx].growth_limit = 0.0;

    let all_but_first_and_last = 1..last_track_idx;
    for track in axis_tracks[all_but_first_and_last].iter_mut() {
        // For each track, if the track’s min track sizing function is:
        // - A fixed sizing function
        //     Resolve to an absolute length and use that size as the track’s initial base size.
        //     Note: Indefinite lengths cannot occur, as they’re treated as auto.
        // - An intrinsic sizing function
        //     Use an initial base size of zero.
        track.base_size = track.min_track_sizing_function.definite_value(axis_available_space).unwrap_or(0.0);

        // For each track, if the track’s max track sizing function is:
        // - A fixed sizing function
        //     Resolve to an absolute length and use that size as the track’s initial growth limit.
        // - An intrinsic sizing function
        //     Use an initial growth limit of infinity.
        // - A flexible sizing function
        //     Use an initial growth limit of infinity.
        track.growth_limit =
            track.max_track_sizing_function.definite_value(axis_available_space).unwrap_or(f32::INFINITY);

        // In all cases, if the growth limit is less than the base size, increase the growth limit to match the base size.
        if track.growth_limit < track.base_size {
            track.growth_limit = track.base_size;
        }
    }
}

// 11.5 Resolve Intrinsic Track Sizes
fn resolve_intrinsic_track_sizes(
    tree: &mut impl LayoutTree,
    axis: GridAxis,
    axis_tracks: &mut [GridTrack],
    other_axis_tracks: &mut [GridTrack],
    items: &mut [GridItem],
    available_space: Size<AvailableSpace>,
    get_track_size_estimate: impl Fn(&GridTrack, AvailableSpace) -> Option<f32>,
) {
    // Step 1. Shim baseline-aligned items so their intrinsic size contributions reflect their baseline alignment.
    // TODO: we do not yet support baseline alignment for CSS Grid

    // Step 2. We skip Step 2 as it is noted that:
    //
    //    This step is a simplification of the steps below for handling spanning items, and should yield
    //    the same behavior as running those instructions on items with a span of 1.
    //
    // We choose this alternative of running Step 3 on items with a span of 1 as we need to write the code for this anyway.

    // Step 3 and Step 4
    // 3. Iterate over items that don't cross a flex track. Items should have already been sorted in ascending order
    // of the number of tracks they cross.
    // 4. Next, repeat the previous step instead considering (together, rather than grouped by span size) all items
    // that do span a track with a flexible sizing function while

    // Compute item's intrinsic (content-based) sizes
    // Note: For items with a specified minimum size of auto (the initial value), the minimum contribution is usually equivalent
    // to the min-content contribution—but can differ in some cases, see §6.6 Automatic Minimum Size of Grid Items.
    // Also, minimum contribution <= min-content contribution <= max-content contribution.
    // TODO: be smarter about only computing these when they are required
    let mut compute_item_sizes = |item: &mut GridItem, axis_tracks: &[GridTrack]| {
        let known_dimensions = item.known_dimensions_cached(
            axis,
            other_axis_tracks,
            available_space.get(axis.other()),
            &get_track_size_estimate,
        );

        let min_content_size = item.min_content_contribution_cached(tree, known_dimensions);
        let max_content_size = item.max_content_contribution_cached(tree, known_dimensions);
        let axis_minimum_size =
            item.minimum_contribution_cached(tree, axis, axis_tracks, available_space, known_dimensions);

        (axis_minimum_size, min_content_size.get(axis), max_content_size.get(axis))
    };

    let axis_available_space = available_space.get(axis);

    let mut batched_item_iterator = ItemBatcher::new(axis);
    while let Some((batch, is_flex)) = batched_item_iterator.next(items) {
        // 1. For intrinsic minimums:
        // First increase the base size of tracks with an intrinsic min track sizing function
        let has_intrinsic_min_track_sizing_function = move |track: &GridTrack| {
            track.min_track_sizing_function.definite_value(available_space.get(axis)).is_none()
        };
        for (i, mut item) in batch.iter_mut().enumerate() {
            let (axis_minimum_size, axis_min_content_size, axis_max_content_size) =
                compute_item_sizes(&mut item, &axis_tracks);

            // ...by distributing extra space as needed to accommodate these items’ minimum contributions.
            // If the grid container is being sized under a min- or max-content constraint, use the items’ limited min-content contributions
            // in place of their minimum contributions here.
            let space = match axis_available_space {
                AvailableSpace::MinContent | AvailableSpace::MaxContent => {
                    let limit = item.spanned_fixed_track_limit(axis, axis_tracks, axis_available_space);
                    axis_min_content_size.maybe_min(limit).max(axis_minimum_size)
                }
                AvailableSpace::Definite(_) => axis_minimum_size,
            };
            let tracks = &mut axis_tracks[item.track_range_excluding_lines(axis)];
            if space > 0.0 {
                distribute_item_space_to_base_size(
                    is_flex,
                    space,
                    tracks,
                    has_intrinsic_min_track_sizing_function,
                    IntrinsicContributionType::Minimum,
                );
            }
        }
        flush_planned_base_size_increases(axis_tracks);

        // 2. For content-based minimums:
        // Next continue to increase the base size of tracks with a min track sizing function of min-content or max-content
        // by distributing extra space as needed to account for these items' min-content contributions.
        let has_min_or_max_content_min_track_sizing_function = move |track: &GridTrack| {
            use MinTrackSizingFunction::{MaxContent, MinContent};
            matches!(track.min_track_sizing_function, MinContent | MaxContent)
        };
        for (i, mut item) in batch.iter_mut().enumerate() {
            let (axis_minimum_size, axis_min_content_size, axis_max_content_size) =
                compute_item_sizes(&mut item, &axis_tracks);
            let tracks = &mut axis_tracks[item.track_range_excluding_lines(axis)];
            let space = axis_min_content_size;
            if space > 0.0 {
                distribute_item_space_to_base_size(
                    is_flex,
                    space,
                    tracks,
                    has_min_or_max_content_min_track_sizing_function,
                    IntrinsicContributionType::Minimum,
                );
            }
        }
        flush_planned_base_size_increases(axis_tracks);

        // 3. For max-content minimums:

        // If the grid container is being sized under a max-content constraint, continue to increase the base size of tracks with
        // a min track sizing function of auto or max-content by distributing extra space as needed to account for these items'
        // limited max-content contributions.
        if axis_available_space == AvailableSpace::MaxContent {
            let has_auto_or_max_content_min_track_sizing_function = move |track: &GridTrack| {
                use MinTrackSizingFunction::{Auto, MaxContent};
                matches!(track.min_track_sizing_function, Auto | MaxContent)
            };
            for (i, mut item) in batch.iter_mut().enumerate() {
                let (axis_minimum_size, axis_min_content_size, axis_max_content_size) =
                    compute_item_sizes(&mut item, &axis_tracks);
                let limit = item.spanned_fixed_track_limit(axis, axis_tracks, axis_available_space);
                let space = axis_max_content_size.maybe_min(limit);
                let tracks = &mut axis_tracks[item.track_range_excluding_lines(axis)];
                if space > 0.0 {
                    distribute_item_space_to_base_size(
                        is_flex,
                        space,
                        tracks,
                        has_auto_or_max_content_min_track_sizing_function,
                        IntrinsicContributionType::Minimum,
                    );
                }
            }
            flush_planned_base_size_increases(axis_tracks);
        }

        // In all cases, continue to increase the base size of tracks with a min track sizing function of max-content by distributing
        // extra space as needed to account for these items' max-content contributions.
        let has_max_content_min_track_sizing_function =
            move |track: &GridTrack| matches!(track.min_track_sizing_function, MinTrackSizingFunction::MaxContent);
        for (i, mut item) in batch.iter_mut().enumerate() {
            let (axis_minimum_size, axis_min_content_size, axis_max_content_size) =
                compute_item_sizes(&mut item, &axis_tracks);
            let space = axis_max_content_size;
            let tracks = &mut axis_tracks[item.track_range_excluding_lines(axis)];
            if space > 0.0 {
                distribute_item_space_to_base_size(
                    is_flex,
                    space,
                    tracks,
                    has_max_content_min_track_sizing_function,
                    IntrinsicContributionType::Maximum,
                );
            }
        }
        flush_planned_base_size_increases(axis_tracks);

        // 4. If at this point any track’s growth limit is now less than its base size, increase its growth limit to match its base size.
        for track in axis_tracks.iter_mut() {
            if track.growth_limit < track.base_size {
                track.growth_limit = track.base_size;
            }
        }

        // 5. For intrinsic maximums: Next increase the growth limit of tracks with an intrinsic max track sizing function by
        // distributing extra space as needed to account for these items' min-content contributions.
        let has_intrinsic_max_track_sizing_function = move |track: &GridTrack| {
            track.max_track_sizing_function.definite_value(available_space.get(axis)).is_none()
        };
        for (i, mut item) in batch.iter_mut().enumerate() {
            let (axis_minimum_size, axis_min_content_size, axis_max_content_size) =
                compute_item_sizes(&mut item, &axis_tracks);
            let space = axis_min_content_size;
            let tracks = &mut axis_tracks[item.track_range_excluding_lines(axis)];
            if space > 0.0 {
                distribute_item_space_to_growth_limit(is_flex, space, tracks, has_intrinsic_max_track_sizing_function);
            }
        }
        // Mark any tracks whose growth limit changed from infinite to finite in this step as infinitely growable for the next step.
        flush_planned_growth_limit_increases(axis_tracks, true);

        // 6. For max-content maximums: Lastly continue to increase the growth limit of tracks with a max track sizing function of max-content
        // by distributing extra space as needed to account for these items' max-content contributions. However, limit the growth of any
        // fit-content() tracks by their fit-content() argument.
        let has_max_content_max_track_sizing_function =
            move |track: &GridTrack| track.max_track_sizing_function == MaxTrackSizingFunction::MaxContent;
        for (i, mut item) in batch.iter_mut().enumerate() {
            let (axis_minimum_size, axis_min_content_size, axis_max_content_size) =
                compute_item_sizes(&mut item, &axis_tracks);
            let space = axis_max_content_size;
            let tracks = &mut axis_tracks[item.track_range_excluding_lines(axis)];
            if space > 0.0 {
                distribute_item_space_to_growth_limit(
                    is_flex,
                    space,
                    tracks,
                    has_max_content_max_track_sizing_function,
                );
            }
        }
        // Mark any tracks whose growth limit changed from infinite to finite in this step as infinitely growable for the next step.
        flush_planned_growth_limit_increases(axis_tracks, false);
    }

    // Step 5. If any track still has an infinite growth limit (because, for example, it had no items placed
    // in it or it is a flexible track), set its growth limit to its base size.
    // NOTE: this step is super-important to ensure that the "Maximise Tracks" step doesn't affect flexible tracks
    axis_tracks
        .iter_mut()
        .filter(|track| track.growth_limit == f32::INFINITY)
        .for_each(|track| track.growth_limit = track.base_size);
}

/// 11.5.1. Distributing Extra Space Across Spanned Tracks
/// https://www.w3.org/TR/css-grid-1/#extra-space
fn distribute_item_space_to_base_size(
    is_flex: bool,
    space: f32,
    tracks: &mut [GridTrack],
    track_is_affected: impl Fn(&GridTrack) -> bool,
    intrinsic_contribution_type: IntrinsicContributionType,
) {
    if is_flex {
        let filter = |track: &GridTrack| track.is_flexible() && track_is_affected(track);
        distribute_item_space_to_base_size_inner(space, tracks, filter, intrinsic_contribution_type)
    } else {
        distribute_item_space_to_base_size_inner(space, tracks, track_is_affected, intrinsic_contribution_type)
    }

    fn distribute_item_space_to_base_size_inner(
        space: f32,
        tracks: &mut [GridTrack],
        track_is_affected: impl Fn(&GridTrack) -> bool,
        intrinsic_contribution_type: IntrinsicContributionType,
    ) {
        // Skip this distribution if there is either
        //   - no space to distribute
        //   - no affected tracks to distribute space to
        if space == 0.0 || tracks.iter().filter(|track| track_is_affected(track)).count() == 0 {
            return;
        }

        // 1. Find the space to distribute
        let track_sizes: f32 = tracks.iter().map(|track| track.base_size).sum();
        let extra_space: f32 = f32_max(0.0, space - track_sizes);

        // 2. Distribute space up to limits:
        // Note: there are two exit conditions to this loop:
        //   - We run out of space to distribute (extra_space falls below THRESHOLD)
        //   - We run out of growable tracks to distribute to

        // Define a small constant to avoid infinite loops due to rounding errors. Rather than stopping distributing
        // extra space when it gets to exactly zero, we will stop when it falls below this amount
        const THRESHOLD: f32 = 0.000001;

        let extra_space = distribute_space_up_to_limits(extra_space, tracks, &track_is_affected);

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
            let mut number_of_tracks =
                tracks.iter().filter(|track| track_is_affected(track)).filter(|track| filter(track)).count();
            if number_of_tracks == 0 {
                filter = (|_| true) as fn(&GridTrack) -> bool;
                number_of_tracks = tracks.len();
            }

            // Distribute remaining space
            let additional_item_incurred_increase = extra_space / number_of_tracks as f32;
            for track in tracks.iter_mut().filter(|track| track_is_affected(track)).filter(|track| filter(track)) {
                track.item_incurred_increase += additional_item_incurred_increase;
            }
        }

        // 4. For each affected track, if the track’s item-incurred increase is larger than the track’s planned increase
        // set the track’s planned increase to that value.
        for track in tracks.iter_mut() {
            if track.item_incurred_increase > track.base_size_planned_increase {
                track.base_size_planned_increase = track.item_incurred_increase;
            }

            // Reset the item_incurresed increase ready for the next space distribution
            track.item_incurred_increase = 0.0;
        }
    }
}

/// 11.5.1. Distributing Extra Space Across Spanned Tracks
/// This is simplified (and faster) version of the algorithm for growth limits
/// https://www.w3.org/TR/css-grid-1/#extra-space
fn distribute_item_space_to_growth_limit(
    is_flex: bool,
    space: f32,
    tracks: &mut [GridTrack],
    track_is_affected: impl Fn(&GridTrack) -> bool,
) {
    if is_flex {
        let filter = |track: &GridTrack| track.is_flexible() && track_is_affected(track);
        distribute_item_space_to_growth_limit_inner(space, tracks, filter)
    } else {
        distribute_item_space_to_growth_limit_inner(space, tracks, track_is_affected)
    }

    fn distribute_item_space_to_growth_limit_inner(
        space: f32,
        tracks: &mut [GridTrack],
        track_is_affected: impl Fn(&GridTrack) -> bool,
    ) {
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
        let number_of_growable_tracks =
            tracks.iter().filter(|track| track_is_affected(track)).filter(|track| track.infinitely_growable).count();
        if number_of_growable_tracks > 0 {
            let item_incurred_increase = extra_space / number_of_growable_tracks as f32;
            for track in
                tracks.iter_mut().filter(|track| track_is_affected(track)).filter(|track| track.infinitely_growable)
            {
                if item_incurred_increase > track.growth_limit_planned_increase {
                    track.growth_limit_planned_increase = item_incurred_increase;
                }
            }
        } else {
            let number_of_affected_tracks = tracks.iter().filter(|track| track_is_affected(track)).count();
            if number_of_affected_tracks > 0 {
                let item_incurred_increase = extra_space / number_of_affected_tracks as f32;
                for track in tracks.iter_mut().filter(|track| track_is_affected(track)) {
                    if item_incurred_increase > track.growth_limit_planned_increase {
                        track.growth_limit_planned_increase = item_incurred_increase;
                    }
                }
            }
        };
    }
}

// 11.6 Maximise Tracks
// Distributes free space (if any) to tracks with FINITE growth limits, up to their limits.
fn maximise_tracks(axis: GridAxis, axis_tracks: &mut [GridTrack], available_grid_space: Size<AvailableSpace>) {
    let used_space: f32 = axis_tracks.iter().map(|track| track.base_size).sum();
    let free_space = available_grid_space.get(axis).compute_free_space(used_space);
    if free_space == f32::INFINITY {
        axis_tracks.iter_mut().for_each(|track| track.base_size = track.growth_limit);
    } else if free_space > 0.0 {
        distribute_space_up_to_limits(free_space, axis_tracks, |_| true);
        for track in axis_tracks.iter_mut() {
            track.base_size += track.item_incurred_increase;
            track.item_incurred_increase = 0.0;
        }
    }
}

// 11.7. Expand Flexible Tracks
// This step sizes flexible tracks using the largest value it can assign to an fr without exceeding the available space.
fn expand_flexible_tracks(
    tree: &mut impl LayoutTree,
    axis: GridAxis,
    axis_tracks: &mut [GridTrack],
    items: &mut [GridItem],
    axis_min_size: Option<f32>,
    axis_max_size: Option<f32>,
    available_grid_space: Size<AvailableSpace>,
) {
    // First, find the grid’s used flex fraction:
    let flex_fraction = match available_grid_space.get(axis) {
        // If the free space is zero:
        //    The used flex fraction is zero.
        // Otherwise, if the free space is a definite length:
        //   The used flex fraction is the result of finding the size of an fr using all of the grid tracks and
        //   a space to fill of the available grid space.
        AvailableSpace::Definite(available_space) => {
            let used_space: f32 = axis_tracks.iter().map(|track| track.base_size).sum();
            let free_space = available_space - used_space;
            if free_space == 0.0 {
                0.0
            } else {
                find_size_of_fr(axis_tracks, available_space)
            }
        }
        // If ... sizing the grid container under a min-content constraint the used flex fraction is zero.
        AvailableSpace::MinContent => 0.0,
        // Otherwise, if the free space is an indefinite length:
        AvailableSpace::MaxContent => {
            // The used flex fraction is the maximum of:
            let flex_fraction = f32_max(
                // For each flexible track, if the flexible track’s flex factor is greater than one,
                // the result of dividing the track’s base size by its flex factor; otherwise, the track’s base size.
                axis_tracks
                    .iter()
                    .filter(|track| track.max_track_sizing_function.is_flexible())
                    .map(|track| {
                        let flex_factor = track.flex_factor();
                        if flex_factor > 1.0 {
                            track.base_size * flex_factor
                        } else {
                            track.base_size
                        }
                    })
                    .max_by(|a, b| a.total_cmp(b))
                    .unwrap_or(0.0),
                // For each grid item that crosses a flexible track, the result of finding the size of an fr using all the grid tracks
                // that the item crosses and a space to fill of the item’s max-content contribution.
                items
                    .iter_mut()
                    .filter(|item| item.crosses_flexible_track(axis))
                    .map(|item| {
                        let tracks = &axis_tracks[item.track_range_excluding_lines(axis)];
                        // TODO: plumb estimate of other axis size (known_dimensions) in here rather than just passing Size::NONE?
                        let max_content_contribution = item.max_content_contribution_cached(tree, Size::NONE);
                        find_size_of_fr(tracks, max_content_contribution.get(axis))
                    })
                    .max_by(|a, b| a.total_cmp(b))
                    .unwrap_or(0.0),
            );

            // If using this flex fraction would cause the grid to be smaller than the grid container’s min-width/height (or larger than the
            // grid container’s max-width/height), then redo this step, treating the free space as definite and the available grid space as equal
            // to the grid container’s inner size when it’s sized to its min-width/height (max-width/height).
            // (Note: min_size takes precedence over max_size)
            let hypothetical_grid_size: f32 = axis_tracks
                .iter()
                .map(|track| match track.max_track_sizing_function {
                    MaxTrackSizingFunction::Flex(track_flex_factor) => {
                        f32_max(track.base_size, track_flex_factor * flex_fraction)
                    }
                    _ => track.base_size,
                })
                .sum();
            let axis_min_size = axis_min_size.unwrap_or(0.0);
            let axis_max_size = axis_max_size.unwrap_or(f32::INFINITY);
            if hypothetical_grid_size < axis_min_size {
                find_size_of_fr(axis_tracks, axis_min_size)
            } else if hypothetical_grid_size > axis_max_size {
                find_size_of_fr(axis_tracks, axis_max_size)
            } else {
                flex_fraction
            }
        }
    };

    // For each flexible track, if the product of the used flex fraction and the track’s flex factor is greater
    // than the track’s base size, set its base size to that product.
    for track in axis_tracks.iter_mut() {
        if let MaxTrackSizingFunction::Flex(track_flex_factor) = track.max_track_sizing_function {
            track.base_size = f32_max(track.base_size, track_flex_factor * flex_fraction);
        }
    }
}

// 11.7.1. Find the Size of an fr
// This algorithm finds the largest size that an fr unit can be without exceeding the target size.
// It must be called with a set of grid tracks and some quantity of space to fill.
fn find_size_of_fr(tracks: &[GridTrack], space_to_fill: f32) -> f32 {
    // Handle the trivial case where there is no space to fill
    // Do not remove as otherwise the loop below will loop infinitely
    if space_to_fill == 0.0 {
        return 0.0;
    }

    // If the product of the hypothetical fr size (computed below) and any flexible track’s flex factor
    // is less than the track’s base size, then we must restart this algorithm treating all such tracks as inflexible.
    // We therefore wrap the entire algorithm in a loop, with an hypotherical_fr_size of INFINITY such that the above
    // condition can never be true for the first iteration.
    let mut hypothetical_fr_size = f32::INFINITY;
    let mut previous_iter_hypothetical_fr_size = f32::INFINITY;
    loop {
        // Let leftover space be the space to fill minus the base sizes of the non-flexible grid tracks.
        // Let flex factor sum be the sum of the flex factors of the flexible tracks. If this value is less than 1, set it to 1 instead.
        // We compute both of these in a single loop to avoid iterating over the data twice
        let mut used_space = 0.0;
        let mut naive_flex_factor_sum = 0.0;
        for track in tracks.iter() {
            match track.max_track_sizing_function {
                // Tracks for which flex_factor * hypothetical_fr_size < track.base_size are treated as inflexible
                MaxTrackSizingFunction::Flex(flex_factor) if flex_factor * hypothetical_fr_size >= track.base_size => {
                    naive_flex_factor_sum += flex_factor;
                }
                _ => used_space += track.base_size,
            };
        }
        let leftover_space = space_to_fill - used_space;
        let flex_factor = f32_max(naive_flex_factor_sum, 1.0);

        // Let the hypothetical fr size be the leftover space divided by the flex factor sum.
        previous_iter_hypothetical_fr_size = hypothetical_fr_size;
        hypothetical_fr_size = leftover_space / flex_factor;

        // If the product of the hypothetical fr size and a flexible track’s flex factor is less than the track’s base size,
        // restart this algorithm treating all such tracks as inflexible.
        // We keep track of the hypothetical_fr_size
        let hypotherical_fr_size_is_valid = tracks.iter().all(|track| match track.max_track_sizing_function {
            MaxTrackSizingFunction::Flex(flex_factor) => {
                flex_factor * hypothetical_fr_size >= track.base_size
                    || flex_factor * previous_iter_hypothetical_fr_size < track.base_size
            }
            _ => true,
        });
        if hypotherical_fr_size_is_valid {
            break;
        }
    }

    // Return the hypothetical fr size.
    hypothetical_fr_size
}

// 11.8. Stretch auto Tracks
// This step expands tracks that have an auto max track sizing function by dividing any remaining positive, definite free space equally amongst them.
fn stretch_auto_tracks(
    axis: GridAxis,
    axis_tracks: &mut [GridTrack],
    container_style: &Style,
    available_space: Size<AvailableSpace>,
    available_grid_space: Size<AvailableSpace>,
) {
    let num_auto_tracks =
        axis_tracks.iter().filter(|track| track.max_track_sizing_function == MaxTrackSizingFunction::Auto).count();
    if num_auto_tracks > 0 {
        let used_space: f32 = axis_tracks.iter().map(|track| track.base_size).sum();

        // If the free space is indefinite, but the grid container has a definite min-width/height
        // use that size to calculate the free space for this step instead.
        let free_space = if available_grid_space.get(axis).is_definite() {
            available_grid_space.get(axis).compute_free_space(used_space)
        } else {
            match container_style.min_size.maybe_resolve(available_space.as_options()).get(axis) {
                Some(size) => size - used_space,
                None => 0.0,
            }
        };
        if free_space > 0.0 {
            let extra_space_per_auto_track = free_space / num_auto_tracks as f32;
            axis_tracks
                .iter_mut()
                .filter(|track| track.max_track_sizing_function == MaxTrackSizingFunction::Auto)
                .for_each(|track| track.base_size += extra_space_per_auto_track);
        }
    }
}

/// Helper function for distributing space to tracks evenly
/// Used by both distribute_item_space_to_base_size and maximise_tracks steps
fn distribute_space_up_to_limits(
    space_to_distribute: f32,
    tracks: &mut [GridTrack],
    track_is_affected: impl Fn(&GridTrack) -> bool,
) -> f32 {
    // Define a small constant to avoid infinite loops due to rounding errors. Rather than stopping distributing
    // extra space when it gets to exactly zero, we will stop when it falls below this amount
    const THRESHOLD: f32 = 0.000001;

    let mut space_to_distribute = space_to_distribute;
    while space_to_distribute > THRESHOLD {
        let number_of_growable_tracks = tracks
            .iter()
            .filter(|track| track.base_size + track.item_incurred_increase < track.growth_limit)
            .filter(|track| track_is_affected(track))
            .count();

        if number_of_growable_tracks == 0 {
            break;
        }

        // Compute item-incurred increase for this iteration
        let min_increase_limit = tracks
            .iter()
            .filter(|track| track.base_size + track.item_incurred_increase < track.growth_limit)
            .filter(|track| track_is_affected(track))
            .map(|track| track.growth_limit - track.base_size)
            .min_by(|a, b| a.total_cmp(b))
            .unwrap(); // We will never pass an empty track list to this function
        let iteration_item_incurred_increase =
            f32_min(min_increase_limit, space_to_distribute / number_of_growable_tracks as f32);

        for track in tracks
            .iter_mut()
            .filter(|track| track_is_affected(track))
            .filter(|track| track.base_size + track.item_incurred_increase < track.growth_limit)
        {
            track.item_incurred_increase += iteration_item_incurred_increase;
        }

        space_to_distribute -= iteration_item_incurred_increase * number_of_growable_tracks as f32;
    }

    space_to_distribute
}
