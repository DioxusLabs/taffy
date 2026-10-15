//! Absolute positioning with Yoga's semantics.
//!
//! Ported from `layoutAbsoluteChild`, `positionAbsoluteChild` and the `set*LayoutPosition` helpers in
//! `yoga/algorithm/AbsoluteLayout.cpp` at Yoga commit `08250431e09bc4b864917e51f9e9d52998ee2dce`.
//! The containing block is found by Taffy's out-of-flow hoisting; the parent's contribution
//! (`justifyAbsoluteChild` / `alignAbsoluteChild`, and `layoutAbsoluteDescendants`' offsets) arrives
//! as the candidate's static position, and the containing block's as [`YogaDetailedInfo`].

use crate::compute::oof::OofBoxLayout;
use crate::geometry::{Line, Point, Rect, Size};
use crate::style::{AvailableSpace, BoxSizing, Direction};
use crate::tree::{
    AxisStaticEdge, Layout, LayoutContainingBlock, LayoutInput, LayoutOutput, LayoutPartialTreeExt, OofCandidate,
    RequestedAxis, RunMode, SizingMode,
};
use crate::util::{MaybeMath, MaybeResolve, ResolveOrZero};
use crate::CoreStyle;

use super::algorithm::{defined, processed_dimension};
use super::axis::bound_axis;
use super::enums::YogaSizingMode;
use super::numeric::{is_defined, is_undefined};

/// What a Yoga containing block records for the layout of its absolute children: the arguments of
/// Yoga's `layoutAbsoluteDescendants` that Taffy's out-of-flow hoisting does not carry.
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct YogaDetailedInfo {
    /// The containing block's width sizing mode (`layoutAbsoluteChild`'s `widthMode`)
    pub width_sizing_mode: YogaSizingMode,
    /// `containingBlockWidth`/`containingBlockHeight`: the border box minus borders, or the available
    /// inner size under the `AbsolutePercentAgainstInnerSize` errata (undefined axes are NaN)
    pub containing_block_size: Size<f32>,
}

/// One axis of `positionAbsoluteChild`, in physical terms: the inset the child is anchored by, or
/// the static position when neither inset is set.
fn position_axis(
    inset: Line<Option<f32>>,
    inline_start_is_end: bool,
    area: Line<f32>,
    margin: Line<f32>,
    size: f32,
    static_area: Line<f32>,
    static_edge: AxisStaticEdge,
) -> f32 {
    let (inline_start, inline_end) =
        if inline_start_is_end { (inset.end, inset.start) } else { (inset.start, inset.end) };
    let anchor_start = |inset: f32| area.start + inset + margin.start;
    let anchor_end = |inset: f32| area.end - inset - margin.end - size;
    match (inline_start, inline_end) {
        (Some(inset), _) if !inline_start_is_end => anchor_start(inset),
        (Some(inset), _) => anchor_end(inset),
        (None, Some(inset)) if !inline_start_is_end => anchor_end(inset),
        (None, Some(inset)) => anchor_start(inset),
        (None, None) => match static_edge {
            AxisStaticEdge::Start => static_area.start + margin.start,
            AxisStaticEdge::End => static_area.end - margin.end - size,
            AxisStaticEdge::Center => {
                static_area.start
                    + (static_area.end - static_area.start - (size + margin.start + margin.end)) / 2.0
                    + margin.start
            }
        },
    }
}

/// Yoga's `layoutAbsoluteChild` + `positionAbsoluteChild` for one hoisted candidate.
///
/// `area_size`/`area_offset` are the containing block's padding box (Taffy's out-of-flow area);
/// `direction` is the containing block's resolved direction.
pub(crate) fn layout_absolute_child<Tree: LayoutContainingBlock>(
    tree: &mut Tree,
    candidate: OofCandidate,
    area_size: Size<f32>,
    area_offset: Point<f32>,
    direction: Direction,
    info: &YogaDetailedInfo,
) -> OofBoxLayout {
    let node = candidate.node;
    let cb = info.containing_block_size;
    let cb_opt = Size { width: defined(cb.width), height: defined(cb.height) };

    let child_style = tree.get_oof_item_style(node);
    let calc = |val, basis| tree.calc(val, basis);
    // Yoga resolves margins, padding and border against the containing block width
    let margin = child_style.margin().map(|m| m.resolve_or_zero(cb_opt.width, calc));
    let padding = child_style.padding().resolve_or_zero(cb_opt.width, calc);
    let border = child_style.border().resolve_or_zero(cb_opt.width, calc);
    let padding_border = (padding + border).sum_axes();
    let box_sizing_adjustment =
        if child_style.box_sizing() == BoxSizing::ContentBox { padding_border } else { Size::ZERO };
    let min_size = child_style.min_size().maybe_resolve(cb_opt, calc).maybe_add(box_sizing_adjustment);
    let max_size = child_style.max_size().maybe_resolve(cb_opt, calc).maybe_add(box_sizing_adjustment);
    let size_style = child_style.size();
    let processed = Size {
        width: processed_dimension(size_style.width, child_style.min_size().width, child_style.max_size().width),
        height: processed_dimension(size_style.height, child_style.min_size().height, child_style.max_size().height),
    };
    // `hasDefiniteLength` / `getResolvedDimension`
    let resolved_size =
        processed.maybe_resolve(cb_opt, calc).map(|v| v.filter(|v| *v >= 0.0)).maybe_add(box_sizing_adjustment);
    let inset_style = child_style.inset();
    let inset = Rect {
        left: inset_style.left.maybe_resolve(cb_opt.width, calc),
        right: inset_style.right.maybe_resolve(cb_opt.width, calc),
        top: inset_style.top.maybe_resolve(cb_opt.height, calc),
        bottom: inset_style.bottom.maybe_resolve(cb_opt.height, calc),
    };
    let aspect_ratio = child_style.aspect_ratio();
    #[cfg(feature = "content_size")]
    let overflow = child_style.overflow();
    #[cfg(feature = "content_size")]
    let contain = child_style.contain();
    drop(child_style);

    let margin_row = margin.left + margin.right;
    let margin_column = margin.top + margin.bottom;

    // Sizes include the margins, as Yoga's `childWidth`/`childHeight` do
    let mut child_width = f32::NAN;
    let mut child_height = f32::NAN;
    if let Some(width) = resolved_size.width {
        child_width = width + margin_row;
    } else if let (Some(left), Some(right)) = (inset.left, inset.right) {
        // The containing block's border box minus its borders is the out-of-flow area
        child_width =
            bound_axis(area_size.width - (left + right), min_size.width, max_size.width, padding_border.width);
    }
    if let Some(height) = resolved_size.height {
        child_height = height + margin_column;
    } else if let (Some(top), Some(bottom)) = (inset.top, inset.bottom) {
        child_height =
            bound_axis(area_size.height - (top + bottom), min_size.height, max_size.height, padding_border.height);
    }
    if is_undefined(child_width) != is_undefined(child_height) {
        if let Some(aspect_ratio) = aspect_ratio {
            if is_undefined(child_width) {
                child_width = margin_row + (child_height - margin_column) * aspect_ratio;
            } else {
                child_height = margin_column + (child_width - margin_row) / aspect_ratio;
            }
        }
    }

    let constraint = |mode: YogaSizingMode, size: f32, margin: f32| -> (Option<f32>, AvailableSpace) {
        match mode {
            YogaSizingMode::StretchFit if is_defined(size) => {
                (Some(size - margin), AvailableSpace::Definite(size - margin))
            }
            YogaSizingMode::FitContent if is_defined(size) => (None, AvailableSpace::Definite(size - margin)),
            _ => (None, AvailableSpace::MaxContent),
        }
    };
    let layout_input = |run_mode: RunMode, known: Size<Option<f32>>, available: Size<AvailableSpace>| LayoutInput {
        #[cfg(feature = "yoga_flexbox")]
        yoga_available_size: Size::NONE,
        run_mode,
        sizing_mode: SizingMode::ContentSize,
        axis: RequestedAxis::Both,
        known_dimensions: known,
        known_dimensions_are_definite: Size { width: true, height: true },
        parent_size: cb_opt,
        available_space: available,
        vertical_margins_are_collapsible: Line { start: false, end: false },
    };

    if is_undefined(child_width) || is_undefined(child_height) {
        let mut width_mode =
            if is_undefined(child_width) { YogaSizingMode::MaxContent } else { YogaSizingMode::StretchFit };
        let height_mode =
            if is_undefined(child_height) { YogaSizingMode::MaxContent } else { YogaSizingMode::StretchFit };
        // Absolute children of column containers fit the containing block width
        if !candidate.yoga_parent_is_row
            && is_undefined(child_width)
            && info.width_sizing_mode != YogaSizingMode::MaxContent
            && is_defined(cb.width)
            && cb.width > 0.0
        {
            child_width = cb.width;
            width_mode = YogaSizingMode::FitContent;
        }
        let (known_width, available_width) = constraint(width_mode, child_width, margin_row);
        let (known_height, available_height) = constraint(height_mode, child_height, margin_column);
        let measured: LayoutOutput = tree.compute_child_layout(
            node,
            layout_input(
                RunMode::ComputeSize,
                Size { width: known_width, height: known_height },
                Size { width: available_width, height: available_height },
            ),
        );
        child_width = measured.size.width + margin_row;
        child_height = measured.size.height + margin_column;
    }

    let size = Size { width: child_width - margin_row, height: child_height - margin_column };
    let mut output = tree.compute_child_layout(
        node,
        layout_input(RunMode::PerformLayout, size.map(Some), size.map(AvailableSpace::Definite)),
    );
    // The child's own layout bounds the size (`boundAxis`), and positioning uses that measured size
    let size = output.size;

    let area = Point {
        x: Line { start: area_offset.x, end: area_offset.x + area_size.width },
        y: Line { start: area_offset.y, end: area_offset.y + area_size.height },
    };
    let location = Point {
        x: position_axis(
            Line { start: inset.left, end: inset.right },
            direction.is_rtl(),
            area.x,
            Line { start: margin.left, end: margin.right },
            size.width,
            candidate.static_position.x.area,
            candidate.static_position.x.align.keyword,
        ),
        y: position_axis(
            Line { start: inset.top, end: inset.bottom },
            false,
            area.y,
            Line { start: margin.top, end: margin.bottom },
            size.height,
            candidate.static_position.y.area,
            candidate.static_position.y.align.keyword,
        ),
    };

    let layout = Layout {
        order: candidate.order,
        size,
        #[cfg(feature = "content_size")]
        scrollable_overflow_rect: output.scrollable_overflow_rect,
        scrollbar_size: Size::ZERO,
        location,
        padding,
        border,
        margin,
    };
    tree.set_unrounded_layout(node, &layout);
    OofBoxLayout {
        layout,
        surfaced: output.oof_candidates.take(),
        #[cfg(feature = "content_size")]
        overflow,
        #[cfg(feature = "content_size")]
        contain,
    }
}
