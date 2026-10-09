//! Shared positioning pass for out-of-flow (`position: absolute` / `position: fixed`) boxes.
//!
//! Layout algorithms do not lay out their out-of-flow children directly (unless they are the
//! child's containing block). Instead they emit [`OofCandidate`] records which bubble up the tree
//! via [`LayoutOutput::oof_candidates`](crate::LayoutOutput) until they reach the box's containing
//! block, which lays the box out using the routine in this module.
use crate::geometry::{Line, Point, Rect, Size};
use crate::style::{
    AlignItemsKeyword, AlignSelf, AlignmentSafety, AvailableSpace, ContainingBlockClaims, CoreStyle, OofItemStyle,
    Overflow,
};
#[cfg(feature = "grid")]
use crate::tree::DetailedLayoutInfo;
use crate::tree::{
    AxisStaticPosition, Layout, LayoutContainingBlock, LayoutInput, LayoutOutput, LayoutPartialTreeExt, NodeId,
    OofCandidate, OofCandidates, OofPositioningArea, RequestedAxis, RunMode, SizingMode,
};
use crate::util::sys::{f32_max, Vec};
use crate::util::{MaybeMath, MaybeResolve, ResolveOrZero};
use crate::{AxisStaticEdge, BoxSizing, Direction};

#[cfg(feature = "content_size")]
use super::common::scrollable_overflow::compute_scrollable_overflow_contribution;
use super::common::sizing_keyword::resolve_absolute_sizing_keywords;

/// Resolve the final static offset of an out-of-flow box from its static position, given the
/// box's final size and resolved margins.
///
/// In each axis the box's margin box is aligned within the static-position area according to the
/// recorded alignment keyword. When the alignment is `safe` and the margin box overflows the
/// area, the fallback keyword is used instead (the `safe` overflow-position keyword from CSS
/// Box Alignment). The returned offset locates the box's border box and is relative to whatever
/// the static-position areas are relative to (the containing block's border box once a candidate
/// has bubbled to its containing block).
pub fn resolve_static_offset(
    static_position: Point<AxisStaticPosition>,
    final_size: Size<f32>,
    resolved_margin: Rect<f32>,
) -> Point<f32> {
    Point {
        x: resolve_static_offset_axis(
            static_position.x,
            final_size.width,
            Line { start: resolved_margin.left, end: resolved_margin.right },
        ),
        y: resolve_static_offset_axis(
            static_position.y,
            final_size.height,
            Line { start: resolved_margin.top, end: resolved_margin.bottom },
        ),
    }
}

/// Single-axis version of [`resolve_static_offset`]
fn resolve_static_offset_axis(sp: AxisStaticPosition, size: f32, margin: Line<f32>) -> f32 {
    let margin_box_size = size + margin.start + margin.end;
    let overflows = matches!(sp.align.safety, AlignmentSafety::Safe) && margin_box_size > sp.area.end - sp.area.start;
    let keyword = if overflows { sp.align.fallback } else { sp.align.keyword };
    align_in_line(sp.area, margin_box_size, keyword) + margin.start
}

/// Position a box of `size` within `container` so that it is aligned to the given physical edge
fn align_in_line(container: Line<f32>, size: f32, edge: AxisStaticEdge) -> f32 {
    match edge {
        AxisStaticEdge::Start => container.start,
        AxisStaticEdge::End => container.end - size,
        AxisStaticEdge::Center => (container.start + container.end - size) / 2.0,
    }
}

/// The self-alignment of an out-of-flow box in one axis, resolved relative to the writing mode
/// of its containing block (`Start` is the logical start edge of the containing block).
#[derive(Copy, Clone, PartialEq, Eq, Debug)]
enum OofAlignment {
    /// `normal` (or `auto`): stretch-fit sizing for non-replaced, non-table boxes; start alignment
    Normal,
    /// `stretch`: stretch-fit sizing; start alignment
    Stretch,
    /// Aligned to the logical start edge
    Start,
    /// Centered
    Center,
    /// Aligned to the logical end edge
    End,
}

impl OofAlignment {
    /// Convert an `align-self`/`justify-self` value for an out-of-flow box into a logical
    /// alignment relative to the containing block's writing mode.
    ///
    /// `alignment` must already have had `self-start`/`self-end` resolved against the containing
    /// block's direction (see [`AlignSelf::resolve_self_relative`]).
    fn from_resolved(alignment: AlignSelf) -> Self {
        debug_assert!(
            !matches!(alignment.keyword, AlignItemsKeyword::SelfStart | AlignItemsKeyword::SelfEnd),
            "self-start/self-end must be resolved before computing out-of-flow alignment"
        );
        match alignment.keyword {
            AlignItemsKeyword::Normal => Self::Normal,
            AlignItemsKeyword::Stretch => Self::Stretch,
            AlignItemsKeyword::Center => Self::Center,
            AlignItemsKeyword::End | AlignItemsKeyword::FlexEnd | AlignItemsKeyword::SelfEnd => Self::End,
            // The fallback alignment of `baseline` is `start`
            AlignItemsKeyword::Start
            | AlignItemsKeyword::FlexStart
            | AlignItemsKeyword::SelfStart
            | AlignItemsKeyword::Baseline => Self::Start,
        }
    }

    /// The physical edge of the containing block the box's margin box is aligned to once its
    /// size has been determined. `Normal` and `Stretch` behave as `Start`.
    fn physical_edge(self, reversed: bool) -> AxisStaticEdge {
        match (self, reversed) {
            (Self::Center, _) => AxisStaticEdge::Center,
            (Self::End, false) | (Self::Normal | Self::Stretch | Self::Start, true) => AxisStaticEdge::End,
            (Self::End, true) | (Self::Normal | Self::Stretch | Self::Start, false) => AxisStaticEdge::Start,
        }
    }
}

/// The inputs to the absolute positioning layout model
/// (<https://www.w3.org/TR/css-position-3/#abspos-layout>) in a single axis. All coordinates
/// are physical (left-to-right / top-to-bottom) and relative to the start of the containing
/// block's inset-resolution area.
#[derive(Copy, Clone, Debug)]
struct OofAxis {
    /// The size of the containing block in this axis
    cb_size: f32,
    /// The box's resolved inset properties (`None` = `auto`)
    inset: Line<Option<f32>>,
    /// The box's resolved margins (`None` = `auto`)
    margin: Line<Option<f32>>,
    /// The box's static position (relative to the containing block's inset-resolution area)
    static_position: AxisStaticPosition,
    /// The box's self-alignment in this axis
    alignment: OofAlignment,
    /// The overflow-position modifier of the box's self-alignment in this axis
    safety: AlignmentSafety,
    /// Whether this is the inline axis of the containing block
    is_inline: bool,
    /// Whether the logical start edge of the containing block is the physical end edge of this
    /// axis (the inline axis of an RTL containing block)
    reversed: bool,
    /// Whether the containing block is a scroll container which scrolls in this axis
    is_scroll_axis: bool,
}

impl OofAxis {
    /// The physical edge corresponding to the logical start edge of the containing block
    fn start_edge(&self) -> AxisStaticEdge {
        if self.reversed {
            AxisStaticEdge::End
        } else {
            AxisStaticEdge::Start
        }
    }

    /// The physical edge corresponding to the logical end edge of the containing block
    fn end_edge(&self) -> AxisStaticEdge {
        if self.reversed {
            AxisStaticEdge::Start
        } else {
            AxisStaticEdge::End
        }
    }

    /// The inset-modified containing block (IMCB) in this axis
    /// <https://www.w3.org/TR/css-position-3/#resolving-insets>
    fn inset_modified_containing_block(&self) -> Line<f32> {
        let (mut imcb, weaker_edge) = match (self.inset.start, self.inset.end) {
            (Some(start), Some(end)) => (Line { start, end: self.cb_size - end }, self.end_edge()),
            // A lone auto inset resolves to zero, and is the weaker inset
            (Some(start), None) => (Line { start, end: self.cb_size }, AxisStaticEdge::End),
            (None, Some(end)) => (Line { start: 0.0, end: self.cb_size - end }, AxisStaticEdge::Start),
            // Both insets are auto: resolve them from the static position rectangle and the
            // alignment within it
            (None, None) => {
                let sp = self.static_position;
                let imcb = match sp.align.keyword {
                    AxisStaticEdge::Start => Line { start: sp.area.start, end: self.cb_size },
                    AxisStaticEdge::End => Line { start: 0.0, end: sp.area.end },
                    AxisStaticEdge::Center => {
                        // Center the IMCB on the static position rectangle's center, bounded by
                        // the nearer edge of the containing block
                        let center = (sp.area.start + sp.area.end) / 2.0;
                        let start_distance = center;
                        let end_distance = self.cb_size - center;
                        if start_distance.abs() <= end_distance.abs() {
                            Line { start: 0.0, end: 2.0 * start_distance }
                        } else {
                            Line { start: self.cb_size - 2.0 * end_distance, end: self.cb_size }
                        }
                    }
                };
                (imcb, self.end_edge())
            }
        };

        // Overconstrained insets: the weaker inset is reduced to bring the IMCB size up to zero
        // <https://www.w3.org/TR/css-position-3/#resolving-insets>
        if imcb.end < imcb.start {
            match weaker_edge {
                AxisStaticEdge::Start => imcb.start = imcb.end,
                _ => imcb.end = imcb.start,
            }
        }
        imcb
    }

    /// The size of the IMCB in this axis: the space available to the box's margin box
    fn inset_modified_containing_block_size(&self) -> f32 {
        let imcb = self.inset_modified_containing_block();
        imcb.end - imcb.start
    }

    /// The stretch-fit size of the box's border box: the IMCB minus the non-auto margins (auto
    /// margins are treated as zero) <https://www.w3.org/TR/css-position-3/#abspos-auto-size>
    fn stretch_fit_size(&self) -> f32 {
        let non_auto_margin_sum = self.margin.start.unwrap_or(0.0) + self.margin.end.unwrap_or(0.0);
        f32_max(self.inset_modified_containing_block_size() - non_auto_margin_sum, 0.0)
    }

    /// Whether an `auto` size in this axis is the stretch-fit size (rather than fit-content)
    /// <https://www.w3.org/TR/css-position-3/#abspos-auto-size>
    fn is_stretch_sized(&self, is_replaced: bool, is_table: bool) -> bool {
        let no_auto_inset = self.inset.start.is_some() && self.inset.end.is_some();
        no_auto_inset
            && match self.alignment {
                OofAlignment::Stretch => true,
                OofAlignment::Normal => !is_replaced && !is_table,
                _ => false,
            }
    }

    /// Resolve the box's auto margins given its final size
    /// <https://www.w3.org/TR/css-position-3/#abspos-margins>
    fn resolve_margins(&self, size: f32) -> Line<f32> {
        let both_insets = self.inset.start.is_some() && self.inset.end.is_some();
        let imcb = self.inset_modified_containing_block();
        let free_space = imcb.end - imcb.start - size;
        match (self.margin.start, self.margin.end) {
            (Some(start), Some(end)) => Line { start, end },
            // If either inset is auto then auto margins resolve to zero
            _ if !both_insets => Line { start: self.margin.start.unwrap_or(0.0), end: self.margin.end.unwrap_or(0.0) },
            (Some(start), None) => Line { start, end: free_space - start },
            (None, Some(end)) => Line { start: free_space - end, end },
            (None, None) => {
                // In the inline axis, negative free space is absorbed entirely by the end margin
                if free_space < 0.0 && self.is_inline {
                    if self.reversed {
                        Line { start: free_space, end: 0.0 }
                    } else {
                        Line { start: 0.0, end: free_space }
                    }
                } else {
                    Line { start: free_space / 2.0, end: free_space / 2.0 }
                }
            }
        }
    }

    /// Compute the physical start of the box's border box given its final size and margins
    /// <https://www.w3.org/TR/css-position-3/#abspos-alignment>
    fn position(&self, size: f32, margin: Line<f32>) -> f32 {
        let imcb = self.inset_modified_containing_block();
        match (self.inset.start, self.inset.end) {
            // Both insets are auto: static alignment, with safe overflow relative to the IMCB
            (None, None) => {
                let sp = self.static_position;
                let margin_box_size = size + margin.start + margin.end;
                let offset = if sp.align.safety == AlignmentSafety::Safe && margin_box_size > imcb.end - imcb.start {
                    align_in_line(imcb, margin_box_size, self.start_edge())
                } else {
                    align_in_line(sp.area, margin_box_size, sp.align.keyword)
                };
                offset + margin.start
            }
            // One auto inset: the margin box is aligned to the edge of the stronger inset
            (Some(_), None) => imcb.start + margin.start,
            (None, Some(_)) => imcb.end - size - margin.end,
            (Some(_), Some(_)) => {
                // Auto margins have already absorbed all of the free space
                if self.margin.start.is_none() || self.margin.end.is_none() {
                    return imcb.start + margin.start;
                }

                let margin_box_size = size + margin.start + margin.end;
                let imcb_size = imcb.end - imcb.start;
                let edge = self.alignment.physical_edge(self.reversed);
                let margin_box_start = match self.safety {
                    AlignmentSafety::Unsafe => align_in_line(imcb, margin_box_size, edge),
                    AlignmentSafety::Safe => {
                        let edge = if margin_box_size > imcb_size { self.start_edge() } else { edge };
                        align_in_line(imcb, margin_box_size, edge)
                    }
                    // Default overflow alignment for absolutely positioned boxes:
                    // <https://www.w3.org/TR/css-align-3/#auto-safety-position>
                    AlignmentSafety::Default => {
                        if margin_box_size <= imcb_size || self.alignment == OofAlignment::Normal {
                            // 1. Fits within the IMCB: align as specified
                            align_in_line(imcb, margin_box_size, edge)
                        } else {
                            // The overflow limit rect is the bounding rect of the IMCB and the
                            // containing block, extended to infinity in the scrollable direction
                            // of a scroll container
                            let mut limit = Line { start: imcb.start.min(0.0), end: f32_max(imcb.end, self.cb_size) };
                            if self.is_scroll_axis {
                                if self.reversed {
                                    limit.start = f32::NEG_INFINITY;
                                } else {
                                    limit.end = f32::INFINITY;
                                }
                            }
                            if margin_box_size <= limit.end - limit.start {
                                // 2. Fits within the overflow limit rect: cover the IMCB and align
                                // as specified as far as possible without overflowing the limit
                                let start = align_in_line(imcb, margin_box_size, edge);
                                f32_max(start.min(limit.end - margin_box_size), limit.start)
                            } else {
                                // 3. Start-align within the overflow limit rect
                                align_in_line(limit, margin_box_size, self.start_edge())
                            }
                        }
                    }
                };
                margin_box_start + margin.start
            }
        }
    }
}

/// Run the out-of-flow positioning pass for `node_id` after its layout algorithm has produced
/// `output`: lay out the out-of-flow candidates in `output.oof_candidates` for which the node is
/// the containing block, record them as the node's hoisted children, and replace
/// `output.oof_candidates` with the unclaimed remainder (which bubble further up the tree).
/// The scrollable overflow contributed by the claimed boxes is merged into
/// `output.scrollable_overflow_rect`.
///
/// This should be called once per `RunMode::PerformLayout` container layout, after the layout
/// algorithm (block/flexbox/grid) has run, and *inside* any layout caching wrapper (such as
/// [`compute_cached_layout`](crate::compute_cached_layout)) so that cache hits do not re-run the
/// pass. It is a no-op when `output.oof_positioning_area` is `None` (leaf and size-only outputs).
///
/// Which candidates the node claims is determined by the node's style via
/// [`CoreStyle::is_containing_block`]. `position: fixed` boxes unclaimed by every ancestor are
/// claimed by the final root positioning pass in [`compute_root_layout`](crate::compute_root_layout).
pub fn compute_oof_layout(tree: &mut impl LayoutContainingBlock, node_id: NodeId, output: &mut LayoutOutput) {
    let Some(area) = output.oof_positioning_area else { return };

    let style = tree.get_core_container_style(node_id);
    let direction = style.direction();
    let claims = style.is_containing_block();
    drop(style);

    let candidates = output.oof_candidates.take();
    let result = compute_oof_layout_for_area(tree, node_id, candidates.as_slice(), area, direction, claims);
    // Always record the hoisted child list (even when empty) so that lists recorded by previous
    // layout runs do not persist
    tree.clear_hoisted_children(node_id);
    tree.add_hoisted_children(node_id, &result.hoisted);
    output.oof_candidates = result.unclaimed;
    #[cfg(feature = "content_size")]
    {
        output.scrollable_overflow_rect = output.scrollable_overflow_rect.union(result.scrollable_overflow_rect);
    }
}

/// The result of laying out out-of-flow candidates against an explicit positioning area.
pub struct OofLayoutResult {
    /// Candidates claimed and laid out by this pass, in document order.
    pub hoisted: Vec<NodeId>,
    /// Candidates not claimed by this pass, which must continue bubbling.
    pub unclaimed: OofCandidates,
    /// Scrollable overflow contributed by the claimed candidates, relative to the positioning
    /// area's origin.
    pub scrollable_overflow_rect: Rect<f32>,
}

/// Lay out out-of-flow candidates against an explicit positioning area.
///
/// Unlike [`compute_oof_layout`], this function does not update the geometry owner's hoisted
/// children or merge overflow into a [`LayoutOutput`]. This is useful when the CSS containing
/// block does not have its own layout node and another node owns the resulting geometry.
///
/// `geometry_owner` supplies the layout tree operations, detailed layout information, and
/// scroll-container state used by the positioning pass. `direction` and `claims` describe the
/// actual CSS containing block.
pub fn compute_oof_layout_for_area(
    tree: &mut impl LayoutContainingBlock,
    geometry_owner: NodeId,
    candidates: &[OofCandidate],
    area: OofPositioningArea,
    direction: Direction,
    claims: ContainingBlockClaims,
) -> OofLayoutResult {
    let overflow = tree.get_core_container_style(geometry_owner).overflow();

    let mut hoisted = Vec::new();
    let mut unclaimed = OofCandidates::new();
    let scrollable_overflow_rect = perform_oof_layout(
        tree,
        geometry_owner,
        candidates,
        area.size,
        area.offset,
        direction,
        claims,
        overflow,
        &mut hoisted,
        &mut unclaimed,
    );

    OofLayoutResult { hoisted, unclaimed, scrollable_overflow_rect }
}

/// The result of laying out a single out-of-flow box with [`layout_oof_box`].
pub(crate) struct OofBoxLayout {
    /// The box's final layout (its `location` is relative to the containing block's border box)
    pub layout: Layout,
    /// Out-of-flow candidates surfaced from within the box's subtree, with anchors relative to
    /// the box's own border box
    pub surfaced: OofCandidates,
    /// The box's `overflow` style
    #[cfg(feature = "content_size")]
    pub overflow: Point<Overflow>,
    /// The box's `contain` style
    #[cfg(feature = "content_size")]
    pub contain: crate::style::Contain,
}

/// Size and position a single out-of-flow box against its containing block, and store its layout.
///
/// - `area_size`/`area_offset` describe the inset-resolution area of the containing block
///   (border box minus borders and scrollbar gutters), relative to the containing block's border box.
/// - `container_overflow` is the `overflow` style of the containing block, which determines
///   whether overflowing boxes may be aligned into its scrollable overflow area.
/// - `grid_owner` is the node whose detailed layout info is consulted for grid placement: if it is
///   a grid container, the box is positioned relative to the grid area determined by its
///   grid-placement properties rather than the passed area.
/// - `before_layout` is called with the inputs of the final layout pass just before it runs, which
///   allows the caller to inspect the layout cache for the box.
#[inline(always)]
#[allow(clippy::too_many_arguments)]
pub(crate) fn layout_oof_box<Tree: LayoutContainingBlock>(
    tree: &mut Tree,
    candidate: OofCandidate,
    area_size: Size<f32>,
    area_offset: Point<f32>,
    direction: Direction,
    container_overflow: Point<Overflow>,
    #[cfg(feature = "grid")] grid_owner: Option<NodeId>,
    before_layout: impl FnOnce(&mut Tree, &LayoutInput),
) -> OofBoxLayout {
    let child_style = tree.get_oof_item_style(candidate.node);

    // If the current node is a grid container then the box is positioned relative to the
    // grid area determined by its grid-placement properties (which falls back to the passed
    // area for `auto` placement)
    #[cfg(feature = "grid")]
    let (area_size, area_offset) = match grid_owner.map(|owner| tree.get_detailed_layout_info(owner)) {
        Some(DetailedLayoutInfo::Grid(grid_info)) => {
            let area_rect = Rect {
                left: area_offset.x,
                right: area_offset.x + area_size.width,
                top: area_offset.y,
                bottom: area_offset.y + area_size.height,
            };
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
    let mut min_size = child_style
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

    let is_replaced = child_style.is_replaced();
    let is_table = child_style.is_table();
    let item_direction = child_style.direction();
    // Taffy only supports the `horizontal-tb` writing mode, so only the inline axis can be reversed.
    // `auto` (`None`) behaves as `normal` for absolutely positioned boxes: it does not defer to the
    // `*-items` properties of the containing block.
    let justify_self =
        child_style.justify_self().unwrap_or(AlignSelf::NORMAL).resolve_self_relative(item_direction, direction, true);
    let align_self =
        child_style.align_self().unwrap_or(AlignSelf::NORMAL).resolve_self_relative(item_direction, direction, false);
    let overflow = child_style.overflow();
    let scrollbar_width = child_style.scrollbar_width();
    #[cfg(feature = "content_size")]
    let contain = child_style.contain();

    drop(child_style);

    // Static positions are relative to the containing block's border box: make them relative to
    // the inset-resolution area
    let static_position = Point {
        x: AxisStaticPosition {
            area: candidate.static_position.x.area.map(|v| v - area_offset.x),
            align: candidate.static_position.x.align,
        },
        y: AxisStaticPosition {
            area: candidate.static_position.y.area.map(|v| v - area_offset.y),
            align: candidate.static_position.y.align,
        },
    };

    let horizontal = OofAxis {
        cb_size: area_width,
        inset: Line { start: left, end: right },
        margin: Line { start: margin.left, end: margin.right },
        static_position: static_position.x,
        alignment: OofAlignment::from_resolved(justify_self),
        safety: justify_self.safety,
        is_inline: true,
        reversed: direction.is_rtl(),
        is_scroll_axis: container_overflow.x.is_scroll_container(),
    };
    let vertical = OofAxis {
        cb_size: area_height,
        inset: Line { start: top, end: bottom },
        margin: Line { start: margin.top, end: margin.bottom },
        static_position: static_position.y,
        alignment: OofAlignment::from_resolved(align_self),
        safety: align_self.safety,
        is_inline: false,
        reversed: false,
        is_scroll_axis: container_overflow.y.is_scroll_container(),
    };
    let stretch_fit_size = Size { width: horizontal.stretch_fit_size(), height: vertical.stretch_fit_size() };
    // The space available to the box's border box is its stretch-fit size (the IMCB minus its
    // non-auto margins). The available space for a table never exceeds that of the containing
    // block (minus the margins) <https://www.w3.org/TR/css-tables-3/#abspos>
    let mut available_space = stretch_fit_size;
    if is_table {
        let non_auto_margin = margin.map(|m| m.unwrap_or(0.0));
        available_space = available_space.f32_min(Size {
            width: f32_max(area_width - non_auto_margin.horizontal_axis_sum(), 0.0),
            height: f32_max(area_height - non_auto_margin.vertical_axis_sum(), 0.0),
        });
    }
    let available_space = available_space.maybe_clamp(min_size, max_size).map(AvailableSpace::Definite);

    // Resolve any sizing keywords (min-content, max-content, fit-content, fit-content(...),
    // stretch) in the size styles. An explicitly sized axis takes precedence over the
    // automatic size below.
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

    // Resolve automatic sizes <https://www.w3.org/TR/css-position-3/#abspos-auto-size>
    //
    // An auto size is the stretch-fit size (the IMCB minus margins) if the self-alignment in
    // that axis is `stretch` (or `normal` for a non-replaced, non-table box) and neither inset is
    // auto. Otherwise it is the fit-content size, which is measured below. Stretch-fit sizes are
    // resolved first so that a size in the other axis can be transferred through the aspect ratio.
    // The block axis is the ratio-dependent axis, so a stretched inline size is transferred
    // through the aspect ratio rather than also stretching the block size. An explicit
    // `stretch` in both axes stretches both and ignores the aspect ratio.
    let stretch_width = known_dimensions.width.is_none() && horizontal.is_stretch_sized(is_replaced, is_table);
    let stretch_height = known_dimensions.height.is_none() && vertical.is_stretch_sized(is_replaced, is_table);
    let both_explicit_stretch =
        horizontal.alignment == OofAlignment::Stretch && vertical.alignment == OofAlignment::Stretch;
    let stretch_aspect_ratio =
        if stretch_width && stretch_height && both_explicit_stretch { None } else { aspect_ratio };
    if stretch_width {
        if is_table {
            // A table cannot be stretched smaller than its content: stretch-fit is a minimum
            min_size.width = Some(f32_max(min_size.width.unwrap_or(0.0), stretch_fit_size.width));
        } else {
            known_dimensions.width = Some(stretch_fit_size.width);
            known_dimensions =
                known_dimensions.maybe_apply_aspect_ratio(stretch_aspect_ratio).maybe_clamp(min_size, max_size);
        }
    }
    if known_dimensions.height.is_none() && stretch_height {
        if is_table {
            min_size.height = Some(f32_max(min_size.height.unwrap_or(0.0), stretch_fit_size.height));
        } else {
            known_dimensions.height = Some(stretch_fit_size.height);
            known_dimensions =
                known_dimensions.maybe_apply_aspect_ratio(stretch_aspect_ratio).maybe_clamp(min_size, max_size);
        }
    }

    let final_size = match (known_dimensions.width, known_dimensions.height) {
        (Some(width), Some(height)) => Size { width, height },
        _ => {
            let measured_size = tree.measure_child_size_both(
                candidate.node,
                known_dimensions,
                area_size.map(Some),
                available_space,
                SizingMode::ContentSize,
                Line::FALSE,
            );
            known_dimensions.unwrap_or(measured_size)
        }
    }
    .maybe_clamp(min_size, max_size);

    let layout_input = LayoutInput {
        known_dimensions: final_size.map(Some),
        known_dimensions_are_definite: Size { width: true, height: true },
        parent_size: area_size.map(Some),
        available_space,
        sizing_mode: SizingMode::ContentSize,
        axis: RequestedAxis::Both,
        run_mode: RunMode::PerformLayout,
        vertical_margins_are_collapsible: Line::FALSE,
    };
    before_layout(tree, &layout_input);
    let mut layout_output = tree.compute_child_layout(candidate.node, layout_input);

    // Resolve auto margins <https://www.w3.org/TR/css-position-3/#abspos-margins>
    let horizontal_margin = horizontal.resolve_margins(final_size.width);
    let vertical_margin = vertical.resolve_margins(final_size.height);
    let resolved_margin = Rect {
        left: horizontal_margin.start,
        right: horizontal_margin.end,
        top: vertical_margin.start,
        bottom: vertical_margin.end,
    };

    // Align the margin box within the IMCB <https://www.w3.org/TR/css-position-3/#abspos-alignment>
    let location = Point {
        x: horizontal.position(final_size.width, horizontal_margin) + area_offset.x,
        y: vertical.position(final_size.height, vertical_margin) + area_offset.y,
    };

    // Note: axis intentionally switched here as scrollbars take up space in the opposite axis
    // to the axis in which scrolling is enabled.
    let scrollbar_size = Size {
        width: if overflow.y == Overflow::Scroll { scrollbar_width } else { 0.0 },
        height: if overflow.x == Overflow::Scroll { scrollbar_width } else { 0.0 },
    };

    let layout = Layout {
        order: candidate.order,
        size: final_size,
        #[cfg(feature = "content_size")]
        scrollable_overflow_rect: layout_output.scrollable_overflow_rect,
        scrollbar_size,
        location,
        padding,
        border,
        margin: resolved_margin,
    };
    tree.set_unrounded_layout(candidate.node, &layout);

    OofBoxLayout {
        layout,
        surfaced: layout_output.oof_candidates.take(),
        #[cfg(feature = "content_size")]
        overflow,
        #[cfg(feature = "content_size")]
        contain,
    }
}

/// Perform final layout on the out-of-flow candidates for which the current node is the
/// containing block, and collect the remainder into `unclaimed` for further bubbling.
///
/// - `node_id` is the current node. If its detailed layout info (as returned by
///   [`LayoutContainingBlock::get_detailed_layout_info`]) indicates that it is a grid container,
///   each claimed box is positioned relative to the grid area determined by its grid-placement
///   properties rather than the passed inset-resolution area.
/// - `candidates` is the merged, document-ordered list of candidates held by the current node
///   (direct out-of-flow children plus candidates bubbled from in-flow children). Anchors must be
///   relative to the current node's border box.
/// - `area_size`/`area_offset` describe the inset-resolution area of the current node
///   (border box minus borders and scrollbar gutters).
/// - `claims` describes which out-of-flow positions the current node acts as a containing block
///   for (see [`CoreStyle::is_containing_block`]). The final root positioning pass claims all.
/// - `container_overflow` is the `overflow` style of the containing block.
///
/// Laying out a claimed box may surface further candidates from within its subtree (e.g. a
/// `position: fixed` descendant of a `position: absolute` box). These are re-swept: claimed ones
/// are appended to the work list, unclaimed ones are added to `unclaimed`.
///
/// The ids of claimed boxes are appended to `hoisted` in order. Returns the scrollable overflow
/// contribution of the claimed boxes.
#[allow(clippy::too_many_arguments)]
pub(crate) fn perform_oof_layout(
    tree: &mut impl LayoutContainingBlock,
    node_id: NodeId,
    candidates: &[OofCandidate],
    area_size: Size<f32>,
    area_offset: Point<f32>,
    direction: Direction,
    claims: ContainingBlockClaims,
    container_overflow: Point<Overflow>,
    hoisted: &mut Vec<NodeId>,
    unclaimed: &mut OofCandidates,
) -> Rect<f32> {
    #[cfg_attr(not(feature = "content_size"), allow(unused_mut))]
    let mut absolute_overflow_rect = Rect::ZERO;

    if candidates.is_empty() {
        return absolute_overflow_rect;
    }

    #[cfg(feature = "content_size")]
    let is_scroll_container = container_overflow.x.is_scroll_container() || container_overflow.y.is_scroll_container();

    // Split the candidate list into an initial work list of claimed candidates and the unclaimed
    // remainder. Further claimed candidates surfaced while laying out a claimed box are appended
    // to the work list (Blink-style re-sweep).
    let mut worklist: Vec<OofCandidate> = Vec::new();
    for candidate in candidates {
        if claims.for_position(candidate.position) {
            worklist.push(*candidate);
        } else {
            unclaimed.push(*candidate);
        }
    }

    #[cfg(not(feature = "grid"))]
    let _ = node_id;

    let mut index = 0;
    while index < worklist.len() {
        let candidate = worklist[index];
        index += 1;
        hoisted.push(candidate.node);

        let OofBoxLayout {
            layout,
            surfaced,
            #[cfg(feature = "content_size")]
            overflow,
            #[cfg(feature = "content_size")]
            contain,
        } = layout_oof_box(
            tree,
            candidate,
            area_size,
            area_offset,
            direction,
            container_overflow,
            #[cfg(feature = "grid")]
            Some(node_id),
            |_, _| {},
        );
        let location = layout.location;
        let final_size = layout.size;

        // Re-sweep any candidates surfaced from within the box's subtree, translating their
        // anchors so they are relative to this node's border box
        let mut surfaced = surfaced;
        if !surfaced.is_empty() {
            surfaced.translate(location);
            for surfaced_candidate in surfaced.iter() {
                if claims.for_position(surfaced_candidate.position) {
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
                layout.scrollable_overflow_rect,
                overflow,
                contain,
                is_scroll_container,
            ));
        }
    }

    absolute_overflow_rect
}

#[cfg(test)]
mod tests {
    use super::resolve_static_offset;
    use crate::geometry::{Line, Point, Rect, Size};
    use crate::style::AlignmentSafety;
    use crate::tree::{AxisStaticAlign, AxisStaticEdge, AxisStaticPosition};

    fn sp(
        area: Line<f32>,
        keyword: AxisStaticEdge,
        safety: AlignmentSafety,
        fallback: AxisStaticEdge,
    ) -> AxisStaticPosition {
        AxisStaticPosition { area, align: AxisStaticAlign { keyword, safety, fallback } }
    }

    fn resolve_x(sp_x: AxisStaticPosition, width: f32, margin: Rect<f32>) -> f32 {
        let position = Point { x: sp_x, y: AxisStaticPosition::from_edge(0.0, AxisStaticEdge::Start) };
        resolve_static_offset(position, Size { width, height: 10.0 }, margin).x
    }

    const AREA: Line<f32> = Line { start: 10.0, end: 110.0 };
    const NO_MARGIN: Rect<f32> = Rect { left: 0.0, right: 0.0, top: 0.0, bottom: 0.0 };
    const MARGIN: Rect<f32> = Rect { left: 5.0, right: 15.0, top: 0.0, bottom: 0.0 };

    #[test]
    fn start_alignment() {
        let sp_x = sp(AREA, AxisStaticEdge::Start, AlignmentSafety::Unsafe, AxisStaticEdge::Start);
        assert_eq!(resolve_x(sp_x, 20.0, NO_MARGIN), 10.0);
        assert_eq!(resolve_x(sp_x, 20.0, MARGIN), 15.0);
    }

    #[test]
    fn end_alignment() {
        let sp_x = sp(AREA, AxisStaticEdge::End, AlignmentSafety::Unsafe, AxisStaticEdge::End);
        assert_eq!(resolve_x(sp_x, 20.0, NO_MARGIN), 90.0);
        assert_eq!(resolve_x(sp_x, 20.0, MARGIN), 75.0);
    }

    #[test]
    fn center_alignment() {
        let sp_x = sp(AREA, AxisStaticEdge::Center, AlignmentSafety::Unsafe, AxisStaticEdge::Center);
        assert_eq!(resolve_x(sp_x, 20.0, NO_MARGIN), 50.0);
        // Center offsets by half the margin difference
        assert_eq!(resolve_x(sp_x, 20.0, MARGIN), 45.0);
    }

    #[test]
    fn safe_alignment_falls_back_only_on_overflow() {
        let sp_x = sp(AREA, AxisStaticEdge::End, AlignmentSafety::Safe, AxisStaticEdge::Start);
        // Fits: aligned to the end
        assert_eq!(resolve_x(sp_x, 20.0, NO_MARGIN), 90.0);
        // Margin box exactly fills the area: no fallback
        assert_eq!(resolve_x(sp_x, 100.0, NO_MARGIN), 10.0);
        // Margin box overflows the area: falls back to start
        assert_eq!(resolve_x(sp_x, 120.0, NO_MARGIN), 10.0);
        assert_eq!(resolve_x(sp_x, 90.0, MARGIN), 15.0);
    }

    #[test]
    fn unsafe_alignment_never_falls_back() {
        let sp_x = sp(AREA, AxisStaticEdge::End, AlignmentSafety::Unsafe, AxisStaticEdge::Start);
        assert_eq!(resolve_x(sp_x, 120.0, NO_MARGIN), -10.0);
    }

    #[test]
    fn degenerate_area() {
        // A zero-extent area (static positions emitted by block layout) behaves as an anchor point
        let anchor = AxisStaticPosition::from_edge(40.0, AxisStaticEdge::Start);
        assert_eq!(resolve_x(anchor, 20.0, NO_MARGIN), 40.0);
        let anchor_end = AxisStaticPosition::from_edge(40.0, AxisStaticEdge::End);
        assert_eq!(resolve_x(anchor_end, 20.0, NO_MARGIN), 20.0);
    }
}
