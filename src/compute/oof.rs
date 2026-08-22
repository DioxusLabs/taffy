//! Shared positioning pass for out-of-flow (`position: absolute` / `position: fixed`) boxes.
//!
//! Layout algorithms do not lay out their out-of-flow children directly (unless they are the
//! child's containing block). Instead they emit [`OofCandidate`] records which bubble up the tree
//! via [`LayoutOutput::oof_candidates`](crate::LayoutOutput) until they reach the box's containing
//! block, which lays the box out using the routine in this module.
use crate::geometry::{Line, Point, Rect, Size};
use crate::style::{AvailableSpace, CoreStyle, Position};
#[cfg(feature = "grid")]
use crate::tree::DetailedLayoutInfo;
use crate::tree::{Layout, LayoutPartialTree, LayoutPartialTreeExt, NodeId, OofCandidate, OofCandidates, SizingMode};
use crate::util::sys::{f32_max, Vec};
use crate::util::{MaybeMath, MaybeResolve, ResolveOrZero};
use crate::{BoxSizing, Direction, StaticEdge};

#[cfg(feature = "content_size")]
use super::common::scrollable_overflow::compute_scrollable_overflow_contribution;
use super::common::sizing_keyword::resolve_absolute_sizing_keywords;

/// Perform final layout on the out-of-flow candidates for which the current node is the
/// containing block, and collect the remainder into `unclaimed` for further bubbling.
///
/// - `node_id` is the current node. If its detailed layout info (as returned by
///   [`LayoutPartialTree::get_detailed_layout_info`]) indicates that it is a grid container,
///   each claimed box is positioned relative to the grid area determined by its grid-placement
///   properties rather than the passed inset-resolution area.
/// - `candidates` is the merged, document-ordered list of candidates held by the current node
///   (direct out-of-flow children plus candidates bubbled from in-flow children). Anchors must be
///   relative to the current node's border box.
/// - `area_size`/`area_offset` describe the inset-resolution area of the current node
///   (border box minus borders and scrollbar gutters).
/// - `claim_absolute` should be `true` if the current node acts as a containing block for
///   `position: absolute` boxes (i.e. if its own position style is not `static`).
/// - `claim_fixed` should be `true` only for the final root positioning pass.
///
/// Laying out a claimed box may surface further candidates from within its subtree (e.g. a
/// `position: fixed` descendant of a `position: absolute` box). These are re-swept: claimed ones
/// are appended to the work list, unclaimed ones are added to `unclaimed`.
///
/// The ids of claimed boxes are appended to `hoisted` in order. Returns the scrollable overflow
/// contribution of the claimed boxes.
#[allow(clippy::too_many_arguments)]
pub(crate) fn perform_oof_layout(
    tree: &mut impl LayoutPartialTree,
    node_id: NodeId,
    candidates: OofCandidates,
    area_size: Size<f32>,
    area_offset: Point<f32>,
    direction: Direction,
    claim_absolute: bool,
    claim_fixed: bool,
    #[cfg(feature = "content_size")] is_scroll_container: bool,
    hoisted: &mut Vec<NodeId>,
    unclaimed: &mut OofCandidates,
) -> Rect<f32> {
    #[cfg_attr(not(feature = "content_size"), allow(unused_mut))]
    let mut absolute_overflow_rect = Rect::ZERO;

    if candidates.is_empty() {
        return absolute_overflow_rect;
    }

    let claims = |position: Position| match position {
        Position::Absolute => claim_absolute,
        Position::Fixed => claim_fixed,
        _ => false,
    };

    // Split the candidate list into an initial work list of claimed candidates and the unclaimed
    // remainder. Further claimed candidates surfaced while laying out a claimed box are appended
    // to the work list (Blink-style re-sweep).
    let mut worklist: Vec<OofCandidate> = Vec::new();
    for candidate in candidates.iter() {
        if claims(candidate.position) {
            worklist.push(*candidate);
        } else {
            unclaimed.push(*candidate);
        }
    }

    #[cfg(feature = "grid")]
    let area_rect = Rect {
        left: area_offset.x,
        right: area_offset.x + area_size.width,
        top: area_offset.y,
        bottom: area_offset.y + area_size.height,
    };
    #[cfg(not(feature = "grid"))]
    let _ = node_id;

    let mut index = 0;
    while index < worklist.len() {
        let candidate = worklist[index];
        index += 1;
        hoisted.push(candidate.node);

        let child_style = tree.get_core_container_style(candidate.node);

        // If the current node is a grid container then the box is positioned relative to the
        // grid area determined by its grid-placement properties (which falls back to the passed
        // area for `auto` placement)
        #[cfg(feature = "grid")]
        let (area_size, area_offset) = match tree.get_detailed_layout_info(node_id) {
            DetailedLayoutInfo::Grid(grid_info) => {
                let grid_area = grid_info.resolve_absolute_grid_area(
                    child_style.grid_row(),
                    child_style.grid_column(),
                    direction,
                    area_rect,
                );
                (
                    Size { width: grid_area.right - grid_area.left, height: grid_area.bottom - grid_area.top },
                    Point { x: grid_area.left, y: grid_area.top },
                )
            }
            _ => (area_size, area_offset),
        };
        let area_width = area_size.width;
        let area_height = area_size.height;

        let aspect_ratio = child_style.aspect_ratio();
        let margin =
            child_style.margin().map(|margin| margin.resolve_to_option(area_width, |val, basis| tree.calc(val, basis)));
        let padding = child_style.padding().resolve_or_zero(Some(area_width), |val, basis| tree.calc(val, basis));
        let border = child_style.border().resolve_or_zero(Some(area_width), |val, basis| tree.calc(val, basis));
        let padding_border_sum = (padding + border).sum_axes();
        let box_sizing_adjustment =
            if child_style.box_sizing() == BoxSizing::ContentBox { padding_border_sum } else { Size::ZERO };

        // Resolve inset
        let left = child_style.inset().left.maybe_resolve(area_width, |val, basis| tree.calc(val, basis));
        let right = child_style.inset().right.maybe_resolve(area_width, |val, basis| tree.calc(val, basis));
        let top = child_style.inset().top.maybe_resolve(area_height, |val, basis| tree.calc(val, basis));
        let bottom = child_style.inset().bottom.maybe_resolve(area_height, |val, basis| tree.calc(val, basis));

        // Compute known dimensions from min/max/inherent size styles
        let size_style = child_style.size();
        let style_size = size_style
            .maybe_resolve(area_size, |val, basis| tree.calc(val, basis))
            .maybe_apply_aspect_ratio(aspect_ratio)
            .maybe_add(box_sizing_adjustment);
        let min_size = child_style
            .min_size()
            .maybe_resolve(area_size, |val, basis| tree.calc(val, basis))
            .maybe_apply_aspect_ratio(aspect_ratio)
            .maybe_add(box_sizing_adjustment)
            .or(padding_border_sum.map(Some))
            .maybe_max(padding_border_sum);
        let max_size = child_style
            .max_size()
            .maybe_resolve(area_size, |val, basis| tree.calc(val, basis))
            .maybe_apply_aspect_ratio(aspect_ratio)
            .maybe_add(box_sizing_adjustment);
        let mut known_dimensions = style_size.maybe_clamp(min_size, max_size);

        let overflow = child_style.overflow();
        let scrollbar_width = child_style.scrollbar_width();
        #[cfg(feature = "content_size")]
        let contain = child_style.contain();

        drop(child_style);

        // Resolve any sizing keywords (min-content, max-content, fit-content, fit-content(...),
        // stretch) in the size styles. An explicitly sized axis takes precedence over the
        // inset-derived size below.
        if size_style.width.is_sizing_keyword() || size_style.height.is_sizing_keyword() {
            resolve_absolute_sizing_keywords(
                tree,
                candidate.node,
                &mut known_dimensions,
                size_style,
                area_size,
                Rect { left, right, top, bottom },
                margin,
                SizingMode::ContentSize,
            );
            known_dimensions = known_dimensions.maybe_apply_aspect_ratio(aspect_ratio).maybe_clamp(min_size, max_size);
        }

        // Fill in width from left/right and reapply aspect ratio if:
        //   - Width is not already known
        //   - Item has both left and right inset properties set
        if let (None, Some(left), Some(right)) = (known_dimensions.width, left, right) {
            let new_width_raw = area_width.maybe_sub(margin.left).maybe_sub(margin.right) - left - right;
            known_dimensions.width = Some(f32_max(new_width_raw, 0.0));
            known_dimensions = known_dimensions.maybe_apply_aspect_ratio(aspect_ratio).maybe_clamp(min_size, max_size);
        }

        // Fill in height from top/bottom and reapply aspect ratio if:
        //   - Height is not already known
        //   - Item has both top and bottom inset properties set
        if let (None, Some(top), Some(bottom)) = (known_dimensions.height, top, bottom) {
            let new_height_raw = area_height.maybe_sub(margin.top).maybe_sub(margin.bottom) - top - bottom;
            known_dimensions.height = Some(f32_max(new_height_raw, 0.0));
            known_dimensions = known_dimensions.maybe_apply_aspect_ratio(aspect_ratio).maybe_clamp(min_size, max_size);
        }

        let final_size = match (known_dimensions.width, known_dimensions.height) {
            (Some(width), Some(height)) => Size { width, height },
            _ => {
                let measured_size = tree.measure_child_size_both(
                    candidate.node,
                    known_dimensions,
                    area_size.map(Some),
                    Size {
                        width: AvailableSpace::Definite(area_width.maybe_clamp(min_size.width, max_size.width)),
                        height: AvailableSpace::Definite(area_height.maybe_clamp(min_size.height, max_size.height)),
                    },
                    SizingMode::ContentSize,
                    Line::FALSE,
                );
                known_dimensions.unwrap_or(measured_size)
            }
        }
        .maybe_clamp(min_size, max_size);

        let mut layout_output = tree.perform_child_layout(
            candidate.node,
            final_size.map(Some),
            area_size.map(Some),
            Size {
                width: AvailableSpace::Definite(area_width.maybe_clamp(min_size.width, max_size.width)),
                height: AvailableSpace::Definite(area_height.maybe_clamp(min_size.height, max_size.height)),
            },
            SizingMode::ContentSize,
            Line::FALSE,
        );

        let non_auto_margin = Rect {
            left: if left.is_some() { margin.left.unwrap_or(0.0) } else { 0.0 },
            right: if right.is_some() { margin.right.unwrap_or(0.0) } else { 0.0 },
            top: if top.is_some() { margin.top.unwrap_or(0.0) } else { 0.0 },
            bottom: if bottom.is_some() { margin.bottom.unwrap_or(0.0) } else { 0.0 },
        };

        // Expand auto margins to fill available space
        // https://www.w3.org/TR/CSS21/visudet.html#abs-non-replaced-width
        let auto_margin = {
            // Auto margins for absolutely positioned elements in block containers only resolve
            // if inset is set. Otherwise they resolve to 0.
            let absolute_auto_margin_space = Point {
                x: right.map(|right| area_size.width - right - left.unwrap_or(0.0)).unwrap_or(final_size.width),
                y: bottom.map(|bottom| area_size.height - bottom - top.unwrap_or(0.0)).unwrap_or(final_size.height),
            };
            let free_space = Size {
                width: absolute_auto_margin_space.x - final_size.width - non_auto_margin.horizontal_axis_sum(),
                height: absolute_auto_margin_space.y - final_size.height - non_auto_margin.vertical_axis_sum(),
            };

            let auto_margin_size = Size {
                // If all three of 'left', 'width', and 'right' are 'auto': First set any 'auto' values for 'margin-left' and 'margin-right' to 0.
                // Then, if the 'direction' property of the element establishing the static-position containing block is 'ltr' set 'left' to the
                // static position and apply rule number three below; otherwise, set 'right' to the static position and apply rule number one below.
                //
                // If none of the three is 'auto': If both 'margin-left' and 'margin-right' are 'auto', solve the equation under the extra constraint
                // that the two margins get equal values, unless this would make them negative, in which case when direction of the containing block is
                // 'ltr' ('rtl'), set 'margin-left' ('margin-right') to zero and solve for 'margin-right' ('margin-left'). If one of 'margin-left' or
                // 'margin-right' is 'auto', solve the equation for that value. If the values are over-constrained, ignore the value for 'left' (in case
                // the 'direction' property of the containing block is 'rtl') or 'right' (in case 'direction' is 'ltr') and solve for that value.
                width: {
                    let auto_margin_count = margin.left.is_none() as u8 + margin.right.is_none() as u8;
                    if auto_margin_count == 2 && free_space.width <= 0.0 {
                        0.0
                    } else if auto_margin_count > 0 {
                        free_space.width / auto_margin_count as f32
                    } else {
                        0.0
                    }
                },
                height: {
                    let auto_margin_count = margin.top.is_none() as u8 + margin.bottom.is_none() as u8;
                    if auto_margin_count == 2 && free_space.height <= 0.0 {
                        0.0
                    } else if auto_margin_count > 0 {
                        free_space.height / auto_margin_count as f32
                    } else {
                        0.0
                    }
                },
            };

            Rect {
                left: margin.left.map(|_| 0.0).unwrap_or(auto_margin_size.width),
                right: margin.right.map(|_| 0.0).unwrap_or(auto_margin_size.width),
                top: margin.top.map(|_| 0.0).unwrap_or(auto_margin_size.height),
                bottom: margin.bottom.map(|_| 0.0).unwrap_or(auto_margin_size.height),
            }
        };

        let resolved_margin = Rect {
            left: margin.left.unwrap_or(auto_margin.left),
            right: margin.right.unwrap_or(auto_margin.right),
            top: margin.top.unwrap_or(auto_margin.top),
            bottom: margin.bottom.unwrap_or(auto_margin.bottom),
        };

        // Static position fallback in each axis: place the edge of the box's margin box indicated
        // by the static position's edge at the recorded anchor (which is relative to this node's
        // border box), applying the `safe` overflow-position fallback where recorded
        let resolve_static = |sp: crate::tree::StaticPosition, size: f32, margin_sum: f32| {
            let overflows = sp.safe_extent.map(|extent| size + margin_sum > extent).unwrap_or(false);
            if overflows {
                (sp.fallback_anchor, sp.fallback_edge)
            } else {
                (sp.anchor, sp.edge)
            }
        };
        let (anchor_x, edge_x) =
            resolve_static(candidate.static_position.x, final_size.width, resolved_margin.horizontal_axis_sum());
        let (anchor_y, edge_y) =
            resolve_static(candidate.static_position.y, final_size.height, resolved_margin.vertical_axis_sum());
        let static_x = match edge_x {
            StaticEdge::Start => anchor_x + resolved_margin.left,
            StaticEdge::End => anchor_x - final_size.width - resolved_margin.right,
            StaticEdge::Center => {
                anchor_x - final_size.width / 2.0 + (resolved_margin.left - resolved_margin.right) / 2.0
            }
        };
        let static_y = match edge_y {
            StaticEdge::Start => anchor_y + resolved_margin.top,
            StaticEdge::End => anchor_y - final_size.height - resolved_margin.bottom,
            StaticEdge::Center => {
                anchor_y - final_size.height / 2.0 + (resolved_margin.top - resolved_margin.bottom) / 2.0
            }
        };

        let x_offset = match (left, right) {
            (Some(left), Some(right)) => {
                if direction.is_rtl() {
                    area_size.width - final_size.width - right - resolved_margin.right
                } else {
                    left + resolved_margin.left
                }
            }
            (Some(left), None) => left + resolved_margin.left,
            (None, Some(right)) => area_size.width - final_size.width - right - resolved_margin.right,
            (None, None) => static_x - area_offset.x,
        };
        let location = Point {
            x: x_offset + area_offset.x,
            y: top
                .map(|top| top + resolved_margin.top)
                .or(bottom.map(|bottom| area_size.height - final_size.height - bottom - resolved_margin.bottom))
                .maybe_add(area_offset.y)
                .unwrap_or(static_y),
        };
        // Note: axis intentionally switched here as scrollbars take up space in the opposite axis
        // to the axis in which scrolling is enabled.
        let scrollbar_size = Size {
            width: if overflow.y == crate::Overflow::Scroll { scrollbar_width } else { 0.0 },
            height: if overflow.x == crate::Overflow::Scroll { scrollbar_width } else { 0.0 },
        };

        tree.set_unrounded_layout(
            candidate.node,
            &Layout {
                order: candidate.order,
                size: final_size,
                #[cfg(feature = "content_size")]
                scrollable_overflow_rect: layout_output.scrollable_overflow_rect,
                scrollbar_size,
                location,
                padding,
                border,
                margin: resolved_margin,
            },
        );

        // Re-sweep any candidates surfaced from within the box's subtree, translating their
        // anchors so they are relative to this node's border box
        let mut surfaced = layout_output.oof_candidates.take();
        if !surfaced.is_empty() {
            surfaced.translate(location);
            for surfaced_candidate in surfaced.iter() {
                if claims(surfaced_candidate.position) {
                    worklist.push(*surfaced_candidate);
                } else {
                    unclaimed.push(*surfaced_candidate);
                }
            }
        }

        #[cfg(feature = "content_size")]
        {
            // Location is measured from the scroll origin (the inline-start edge: right side in RTL)
            let relative_location = if direction.is_rtl() {
                Point {
                    x: area_size.width - (location.x - area_offset.x) - final_size.width,
                    y: location.y - area_offset.y,
                }
            } else {
                Point { x: location.x - area_offset.x, y: location.y - area_offset.y }
            };
            absolute_overflow_rect = absolute_overflow_rect.union(compute_scrollable_overflow_contribution(
                relative_location,
                final_size,
                layout_output.scrollable_overflow_rect,
                overflow,
                contain,
                is_scroll_container,
            ));
        }
    }

    absolute_overflow_rect
}
