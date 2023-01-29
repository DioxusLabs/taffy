//! Alignment of tracks and final positioning of items
use super::types::GridTrack;
use crate::axis::InBothAbsAxis;
use crate::compute::common::alignment::compute_alignment_offset;
use crate::geometry::{Line, Point, Rect, Size};
use crate::layout::{Layout, SizingMode};
use crate::math::MaybeMath;
use crate::resolve::MaybeResolve;
use crate::style::{AlignContent, AlignItems, AlignSelf, AvailableSpace, Position};
use crate::sys::{f32_max, f32_min};
use crate::tree::LayoutTree;

/// Align the grid tracks within the grid according to the align-content (rows) or
/// justify-content (columns) property. This only does anything if the size of the
/// grid is not equal to the size of the grid container in the axis being aligned.
pub(super) fn align_tracks(
    grid_container_content_box_size: f32,
    padding: Line<f32>,
    border: Line<f32>,
    tracks: &mut [GridTrack],
    track_alignment_style: AlignContent,
) {
    let used_size: f32 = tracks.iter().map(|track| track.base_size).sum();
    let size_diff = grid_container_content_box_size - used_size;
    let free_space = f32_max(size_diff, 0.0);
    let overflow = f32_min(size_diff, 0.0);

    // If the used_size > grid_container_size then the tracks must overflow their container
    // The direction in which they do so is determined by the alignment style
    let origin = padding.start
        + border.start
        + match track_alignment_style {
            AlignContent::Start => 0.0,
            AlignContent::FlexStart => 0.0,
            AlignContent::End => overflow,
            AlignContent::FlexEnd => overflow,
            AlignContent::Center => overflow / 2.0,
            AlignContent::Stretch => 0.0,
            AlignContent::SpaceBetween => 0.0,
            AlignContent::SpaceEvenly => 0.0,
            AlignContent::SpaceAround => 0.0,
        };

    // Count the number of non-collapsed tracks (not counting gutters)
    let num_tracks = tracks.iter().skip(1).step_by(2).filter(|track| !track.is_collapsed).count();

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
pub(super) fn align_and_position_item<Tree: LayoutTree>(
    tree: &mut Tree,
    node: Tree::ChildId,
    order: u32,
    grid_area: Rect<f32>,
    container_alignment_styles: InBothAbsAxis<Option<AlignItems>>,
    baseline_shim: f32,
) {
    let grid_area_size = Size { width: grid_area.right - grid_area.left, height: grid_area.bottom - grid_area.top };

    let style = tree.child_style(node);
    let aspect_ratio = style.aspect_ratio;
    let justify_self = style.justify_self;
    let align_self = style.align_self;

    let position = style.position;
    let inset_horizontal = style.inset.horizontal_components().map(|size| size.resolve_to_option(grid_area_size.width));
    let inset_vertical = style.inset.vertical_components().map(|size| size.resolve_to_option(grid_area_size.height));
    let inherent_size = style.size.maybe_resolve(grid_area_size).maybe_apply_aspect_ratio(aspect_ratio);
    let min_size = style.min_size.maybe_resolve(grid_area_size).maybe_apply_aspect_ratio(aspect_ratio);
    let max_size = style.max_size.maybe_resolve(grid_area_size).maybe_apply_aspect_ratio(aspect_ratio);

    // Resolve default alignment styles if they are set on neither the parent or the node itself
    // Note: if the child has a preferred aspect ratio but neither width or height are set, then the width is stretched
    // and the then height is calculated from the width according the aspect ratio
    // See: https://www.w3.org/TR/css-grid-1/#grid-item-sizing
    let alignment_styles = InBothAbsAxis {
        horizontal: justify_self.or(container_alignment_styles.horizontal).unwrap_or_else(|| {
            if inherent_size.width.is_some() {
                AlignSelf::Start
            } else {
                AlignSelf::Stretch
            }
        }),
        vertical: align_self.or(container_alignment_styles.vertical).unwrap_or_else(|| {
            if inherent_size.height.is_some() || aspect_ratio.is_some() {
                AlignSelf::Start
            } else {
                AlignSelf::Stretch
            }
        }),
    };

    // Note: This is not a bug. It is part of the CSS spec that both horizontal and vertical margins
    // resolve against the WIDTH of the grid area.
    let margin = style.margin.map(|margin| margin.resolve_to_option(grid_area_size.width));

    let grid_area_minus_item_margins_size = Size {
        width: grid_area_size.width.maybe_sub(margin.left).maybe_sub(margin.right),
        height: grid_area_size.height.maybe_sub(margin.top).maybe_sub(margin.bottom) - baseline_shim,
    };

    // If node is absolutely positioned and width is not set explicitly, then deduce it
    // from left, right and container_content_box if both are set.
    let width = inherent_size.width.or_else(|| {
        // Apply width derived from both the left and right properties of an absolutely
        // positioned element being set
        if position == Position::Absolute {
            if let (Some(left), Some(right)) = (inset_horizontal.start, inset_horizontal.end) {
                return Some(f32_max(grid_area_minus_item_margins_size.width - left - right, 0.0));
            }
        }

        // Apply width based on stretch alignment if:
        //  - Alignment style is "stretch"
        //  - The node is not absolutely positioned
        //  - The node does not have auto margins in this axis.
        if margin.left.is_some()
            && margin.right.is_some()
            && alignment_styles.horizontal == AlignSelf::Stretch
            && position != Position::Absolute
        {
            return Some(grid_area_minus_item_margins_size.width);
        }

        None
    });
    // Reapply aspect ratio after stretch and absolute position width adjustments
    let Size { width, height } = Size { width, height: inherent_size.height }.maybe_apply_aspect_ratio(aspect_ratio);

    let height = height.or_else(|| {
        if position == Position::Absolute {
            if let (Some(top), Some(bottom)) = (inset_vertical.start, inset_vertical.end) {
                return Some(f32_max(grid_area_minus_item_margins_size.height - top - bottom, 0.0));
            }
        }

        // Apply height based on stretch alignment if:
        //  - Alignment style is "stretch"
        //  - The node is not absolutely positioned
        //  - The node does not have auto margins in this axis.
        if margin.top.is_some()
            && margin.bottom.is_some()
            && alignment_styles.vertical == AlignSelf::Stretch
            && position != Position::Absolute
        {
            return Some(grid_area_minus_item_margins_size.height);
        }

        None
    });
    // Reapply aspect ratio after stretch and absolute position height adjustments
    let Size { width, height } = Size { width, height }.maybe_apply_aspect_ratio(aspect_ratio);

    // Clamp size by min and max width/height
    let Size { width, height } = Size { width, height }.maybe_clamp(min_size, max_size);

    // Layout node
    let measured_size_and_baselines = tree.perform_child_layout(
        node,
        Size { width, height },
        grid_area_size.map(Option::Some),
        grid_area_minus_item_margins_size.map(AvailableSpace::Definite),
        SizingMode::InherentSize,
    );

    // Resolve final size
    let Size { width, height } =
        Size { width, height }.unwrap_or(measured_size_and_baselines.size).maybe_clamp(min_size, max_size);

    let x = align_item_within_area(
        Line { start: grid_area.left, end: grid_area.right },
        justify_self.unwrap_or(alignment_styles.horizontal),
        width,
        position,
        inset_horizontal,
        margin.horizontal_components(),
        0.0,
    );
    let y = align_item_within_area(
        Line { start: grid_area.top, end: grid_area.bottom },
        align_self.unwrap_or(alignment_styles.vertical),
        height,
        position,
        inset_vertical,
        margin.vertical_components(),
        baseline_shim,
    );

    *tree.child_layout_mut(node) = Layout { order, size: Size { width, height }, location: Point { x, y } };
}

/// Align and size a grid item along a single axis
pub(super) fn align_item_within_area(
    grid_area: Line<f32>,
    alignment_style: AlignSelf,
    resolved_size: f32,
    position: Position,
    inset: Line<Option<f32>>,
    margin: Line<Option<f32>>,
    baseline_shim: f32,
) -> f32 {
    // Calculate grid area dimension in the axis
    let non_auto_margin = Line { start: margin.start.unwrap_or(0.0) + baseline_shim, end: margin.end.unwrap_or(0.0) };
    let grid_area_size = f32_max(grid_area.end - grid_area.start, 0.0);
    let free_space = f32_max(grid_area_size - resolved_size - non_auto_margin.sum(), 0.0);

    // Expand auto margins to fill available space
    let auto_margin_count = margin.start.is_none() as u8 + margin.end.is_none() as u8;
    let auto_margin_size = if auto_margin_count > 0 { free_space / auto_margin_count as f32 } else { 0.0 };
    let resolved_margin = Line {
        start: margin.start.unwrap_or(auto_margin_size) + baseline_shim,
        end: margin.end.unwrap_or(auto_margin_size),
    };

    // Compute offset in the axis
    let alignment_based_offset = match alignment_style {
        AlignSelf::Start | AlignSelf::FlexStart => resolved_margin.start,
        AlignSelf::End | AlignSelf::FlexEnd => grid_area_size - resolved_size - resolved_margin.end,
        AlignSelf::Center => (grid_area_size - resolved_size + resolved_margin.start - resolved_margin.end) / 2.0,
        // TODO: Add support for baseline alignment. For now we treat it as "start".
        AlignSelf::Baseline => resolved_margin.start,
        AlignSelf::Stretch => resolved_margin.start,
    };

    let offset_within_area = if position == Position::Absolute {
        if let Some(start) = inset.start {
            start + non_auto_margin.start
        } else if let Some(end) = inset.end {
            grid_area_size - end - resolved_size - non_auto_margin.end
        } else {
            alignment_based_offset
        }
    } else {
        alignment_based_offset
    };

    let mut start = grid_area.start + offset_within_area;
    if position == Position::Relative {
        start += inset.start.or(inset.end.map(|pos| -pos)).unwrap_or(0.0);
    }

    start
}
