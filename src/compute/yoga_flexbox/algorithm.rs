//! The Yoga flexbox algorithm, ported from `yoga/algorithm/CalculateLayout.cpp` and `FlexLine.cpp`
//! at Yoga commit `08250431e09bc4b864917e51f9e9d52998ee2dce`.
//!
//! Undefined values are `f32::NAN`, as in Yoga; min/max constraints are `Option<f32>`.

#![allow(dead_code)]

use crate::geometry::{AbsoluteAxis, Line, Point, Rect, Size};
use crate::style::{
    AlignItems, AvailableSpace, BoxSizing, CompactLength, Dimension, Direction, FlexDirection, JustifyContent,
    Overflow, Position,
};
use crate::tree::{LayoutInput, LayoutOutput, LayoutYogaFlexboxContainer, NodeId, RequestedAxis, RunMode, SizingMode};
use crate::util::sys::Vec;
use crate::util::MaybeMath;
use crate::{CoreStyle, FlexboxContainerStyle, FlexboxItemStyle};

use super::absolute::YogaDetailedInfo;
use super::axis::{
    bound_axis, bound_axis_within_min_and_max, constrain_max_size_for_mode, dimension, fallback_justify_content,
    flex_end_edge, flex_start_edge, inline_end_edge, inline_start_edge, is_row, resolve_cross_direction,
    resolve_direction, PhysicalEdge,
};
use super::axis::{
    bound_axis_within_min_and_max as bound_within, calculate_available_inner_dimension, needs_trailing_position,
    position_of_opposite_edge,
};
use super::config::YogaConfig;
use super::enums::YogaErrata;
use super::enums::YogaSizingMode;
use super::length::{YogaResolve, YogaResolveOrZero};
use super::numeric::min_or_defined;
use super::numeric::{inexact_equals, is_defined, is_undefined, max_or_defined};
use crate::style::{AlignContent, AlignContentKeyword, AlignItemsKeyword, FlexWrap};
use crate::tree::{
    AxisStaticAlign, AxisStaticEdge, AxisStaticPosition, Baselines, Layout, OofCandidate, OofCandidates,
    OofPositioningArea,
};

/// Yoga has no `calc()`; calc values resolve to zero in this mode
#[inline]
fn no_calc(_val: *const (), _basis: f32) -> f32 {
    0.0
}

/// Convert Yoga's NaN-as-undefined into Taffy's `Option`
#[inline]
pub(crate) fn defined(value: f32) -> Option<f32> {
    if is_defined(value) {
        Some(value)
    } else {
        None
    }
}

/// A child of a Yoga flex container with the style values the algorithm reads, resolved once.
///
/// Percentages are resolved against the container's available inner size, which is what Yoga passes
/// as `ownerWidth`/`ownerHeight` to every child computation.
pub(crate) struct YogaItem {
    /// The child node
    pub(crate) node: NodeId,
    /// `display: none` (laid out as zero, skipped by the algorithm)
    pub(crate) is_hidden: bool,
    /// `position: absolute` (hoisted to the containing block, skipped by the flex algorithm)
    pub(crate) is_absolute: bool,
    /// Resolved margins (auto resolves to zero)
    pub(crate) margin: Rect<f32>,
    /// Which margins are `auto`
    pub(crate) margin_is_auto: Rect<bool>,
    /// Resolved padding + border per edge
    pub(crate) padding_border: Rect<f32>,
    /// Style min size (resolved per call site, see [`Self::resolved_min`])
    pub(crate) min_style: Size<crate::style::LengthPercentageAuto>,
    /// Style max size (resolved per call site, see [`Self::resolved_max`])
    pub(crate) max_style: Size<crate::style::LengthPercentageAuto>,
    /// Style size after `processDimensions` (max == min → max), resolved per call site
    pub(crate) size_style: Size<Dimension>,
    /// Style `flex-basis`, resolved per call site
    pub(crate) flex_basis_style: Dimension,
    /// Padding + border added to content-box sizes (zero for border-box)
    pub(crate) box_sizing_adjustment: Size<f32>,
    /// `resolveFlexGrow`
    pub(crate) flex_grow: f32,
    /// `resolveFlexShrink`
    pub(crate) flex_shrink: f32,
    /// `aspect-ratio`
    pub(crate) aspect_ratio: Option<f32>,
    /// `resolveChildAlignment`
    pub(crate) align: AlignItems,
    /// `overflow`
    pub(crate) overflow: Overflow,
    /// Resolved insets for `position: relative` (`None` when auto); horizontal edges resolve against
    /// the container's available inner width, vertical edges against its height
    pub(crate) inset: Rect<Option<f32>>,
    /// Which insets are `auto`
    pub(crate) inset_is_auto: Rect<bool>,
    /// `position: static` (Yoga ignores insets on static children)
    pub(crate) is_static: bool,
    /// The child's own `flex-direction` (its `setPosition` uses its own axes)
    pub(crate) own_flex_direction: FlexDirection,
    /// Layout position per physical edge (`LayoutResults::position`)
    pub(crate) position: Rect<f32>,
    /// Baseline of the most recent layout (`calculateBaseline`), NaN before any
    pub(crate) baseline: f32,
    /// `LayoutResults::computedFlexBasis` (NaN when undefined)
    pub(crate) computed_flex_basis: f32,
    /// `LayoutResults::computedAutoMinMainSize`: the §4.5 automatic minimum, when it applies
    pub(crate) computed_auto_min_main_size: Option<f32>,
    /// Whether the style min size in each axis is set (Yoga's "defined" min; `auto` is unset)
    pub(crate) min_size_is_set: Size<bool>,
    /// Measured size from the most recent child layout call (NaN before any)
    pub(crate) measured: Size<f32>,
    /// Index of the flex line the item ended up on
    pub(crate) line_index: usize,
    /// Out-of-flow candidates surfaced by the item's final layout, in the item's coordinates
    pub(crate) surfaced: OofCandidates,
}

impl YogaItem {
    /// `isNodeFlexible`
    #[inline]
    pub(crate) fn is_flexible(&self) -> bool {
        !self.is_absolute && (self.flex_grow != 0.0 || self.flex_shrink != 0.0)
    }

    /// `computeMarginForAxis`
    #[inline]
    pub(crate) fn margin_for_axis(&self, axis: FlexDirection) -> f32 {
        if is_row(axis) {
            self.margin.left + self.margin.right
        } else {
            self.margin.top + self.margin.bottom
        }
    }

    /// `paddingAndBorderForAxis`
    #[inline]
    pub(crate) fn padding_border_for_axis(&self, axis: FlexDirection) -> f32 {
        if is_row(axis) {
            self.padding_border.left + self.padding_border.right
        } else {
            self.padding_border.top + self.padding_border.bottom
        }
    }

    /// `getResolvedDimension`: the style size resolved against `reference`, box-sizing adjusted;
    /// `None` when undefined or negative (so `is_some()` is `hasDefiniteLength`)
    #[inline]
    pub(crate) fn resolved_size(&self, axis: AbsoluteAxis, reference: f32) -> Option<f32> {
        self.size_style
            .get_abs(axis)
            .yoga_resolve(defined(reference), no_calc)
            .filter(|v| *v >= 0.0)
            .map(|v| v + self.box_sizing_adjustment.get_abs(axis))
    }

    /// `hasDefiniteLength`
    #[inline]
    pub(crate) fn has_definite_size(&self, axis: AbsoluteAxis, reference: f32) -> bool {
        self.resolved_size(axis, reference).is_some()
    }

    /// Whether the (processed) style size is a percentage
    #[inline]
    pub(crate) fn size_is_percent(&self, axis: AbsoluteAxis) -> bool {
        self.size_style.get_abs(axis).into_raw().tag() == CompactLength::PERCENT_TAG
    }

    /// `resolvedMinDimension`
    #[inline]
    pub(crate) fn resolved_min(&self, axis: AbsoluteAxis, reference: f32) -> Option<f32> {
        self.min_style
            .get_abs(axis)
            .yoga_resolve(defined(reference), no_calc)
            .map(|v| v + self.box_sizing_adjustment.get_abs(axis))
    }

    /// `resolvedMaxDimension`
    #[inline]
    pub(crate) fn resolved_max(&self, axis: AbsoluteAxis, reference: f32) -> Option<f32> {
        self.max_style
            .get_abs(axis)
            .yoga_resolve(defined(reference), no_calc)
            .map(|v| v + self.box_sizing_adjustment.get_abs(axis))
    }

    /// `resolveFlexBasis`
    #[inline]
    pub(crate) fn resolved_flex_basis(&self, main_axis: FlexDirection, reference: f32) -> Option<f32> {
        self.flex_basis_style
            .yoga_resolve(defined(reference), no_calc)
            .map(|v| v + self.box_sizing_adjustment.get_abs(dimension(main_axis)))
    }

    /// `boundAxisWithinMinAndMax` for this item, min/max resolved against `reference`
    #[inline]
    pub(crate) fn bound_within_min_and_max(&self, axis: FlexDirection, value: f32, reference: f32) -> f32 {
        let dim = dimension(axis);
        bound_axis_within_min_and_max(value, self.resolved_min(dim, reference), self.resolved_max(dim, reference))
    }
}

/// The container-level values every child computation needs
#[derive(Copy, Clone)]
pub(crate) struct YogaContainer {
    /// The container node
    pub(crate) node: NodeId,
    /// Resolved writing direction
    pub(crate) direction: crate::style::Direction,
    /// Resolved main axis (`resolveDirection(flexDirection, direction)`)
    pub(crate) main_axis: FlexDirection,
    /// Resolved cross axis
    pub(crate) cross_axis: FlexDirection,
    /// `flex-wrap` is not `nowrap`
    pub(crate) is_wrap: bool,
    /// Container `overflow` in the main axis direction (Yoga reads a single `overflow`)
    pub(crate) overflow: Overflow,
    /// `align-items`
    pub(crate) align_items: AlignItems,
    /// `availableInnerWidth`/`availableInnerHeight` (may be undefined); fixed for the whole layout
    pub(crate) available_inner: Size<f32>,
    /// `availableInnerMainDim`: the main-axis inner size, adjusted per flex line
    pub(crate) available_inner_main_dim: f32,
    /// The container's own `ownerWidth`/`ownerHeight`
    pub(crate) owner_size: Size<f32>,
    /// Width/height sizing modes the container was asked for
    pub(crate) sizing_mode: Size<YogaSizingMode>,
    /// Resolved padding + border per edge
    pub(crate) padding_border: Rect<f32>,
    /// Resolved min size, box-sizing adjusted
    pub(crate) min_size: Size<Option<f32>>,
    /// Whether the style min size is set per axis (Yoga's "defined")
    pub(crate) min_size_is_set: Size<bool>,
    /// Resolved gaps: `width` is the column gap (between row items), `height` the row gap
    pub(crate) gap: Size<f32>,
    /// `justify-content` (Yoga default flex-start)
    pub(crate) justify_content: JustifyContent,
    /// `isBaselineLayout`
    pub(crate) is_baseline_layout: bool,
    /// The Yoga configuration
    pub(crate) config: YogaConfig,
}

impl YogaContainer {
    /// `computeFlexStartPaddingAndBorder` for an axis
    #[inline]
    pub(crate) fn flex_start_padding_border(&self, axis: FlexDirection) -> f32 {
        edge_value(&self.padding_border, flex_start_edge(axis))
    }

    /// `computeFlexEndPaddingAndBorder` for an axis
    #[inline]
    pub(crate) fn flex_end_padding_border(&self, axis: FlexDirection) -> f32 {
        edge_value(&self.padding_border, flex_end_edge(axis))
    }

    /// `computeGapForAxis`
    #[inline]
    pub(crate) fn gap_for_axis(&self, axis: FlexDirection) -> f32 {
        if is_row(axis) {
            self.gap.width
        } else {
            self.gap.height
        }
    }
}

/// Write one axis of a `Size`
#[inline]
fn set_abs(size: &mut Size<f32>, axis: AbsoluteAxis, value: f32) {
    match axis {
        AbsoluteAxis::Horizontal => size.width = value,
        AbsoluteAxis::Vertical => size.height = value,
    }
}

/// Read one physical edge of a `Rect`
#[inline]
fn edge_value<T: Copy>(rect: &Rect<T>, edge: PhysicalEdge) -> T {
    match edge {
        PhysicalEdge::Left => rect.left,
        PhysicalEdge::Top => rect.top,
        PhysicalEdge::Right => rect.right,
        PhysicalEdge::Bottom => rect.bottom,
    }
}

/// Write one physical edge of a `Rect`
#[inline]
fn set_edge_value<T>(rect: &mut Rect<T>, edge: PhysicalEdge, value: T) {
    match edge {
        PhysicalEdge::Left => rect.left = value,
        PhysicalEdge::Top => rect.top = value,
        PhysicalEdge::Right => rect.right = value,
        PhysicalEdge::Bottom => rect.bottom = value,
    }
}

/// Resolve one child's styles into a [`YogaItem`] (Yoga's `processDimensions`, `resolvedMin/MaxDimension`,
/// `getResolvedDimension`, `resolveFlexBasis`, `resolveChildAlignment`).
pub(crate) fn gather_item(
    tree: &mut impl LayoutYogaFlexboxContainer,
    container: &YogaContainer,
    child: NodeId,
) -> YogaItem {
    let style = tree.get_flexbox_child_style(child);
    let owner_width = defined(container.available_inner.width);
    let owner_size = Size { width: owner_width, height: defined(container.available_inner.height) };
    let calc = |val, basis| tree.resolve_calc_value(val, basis);

    let margin_style = style.margin();
    let margin = margin_style.yoga_resolve_or_zero(owner_width, calc);
    let margin_is_auto = margin_style.map(|m| m.is_auto());
    let padding_border = style.padding().yoga_resolve_or_zero(owner_width, calc)
        + style.border().yoga_resolve_or_zero(owner_width, calc);
    let padding_border_sum = padding_border.sum_axes();
    let box_sizing_adjustment =
        if style.box_sizing() == BoxSizing::ContentBox { padding_border_sum } else { Size::ZERO };

    // processDimensions: a max size equal to the min size replaces the style size
    let style_size = style.size();
    let processed = Size {
        width: processed_dimension(style_size.width, style.min_size().width, style.max_size().width),
        height: processed_dimension(style_size.height, style.min_size().height, style.max_size().height),
    };
    let align = match style.align_self() {
        Some(align_self) => align_self,
        None => container.align_items,
    };
    // Yoga: baseline alignment in a column container falls back to flex-start
    let align =
        if align == AlignItems::BASELINE && !is_row(container.main_axis) { AlignItems::FLEX_START } else { align };

    let own_flex_direction = tree.get_flexbox_container_style(child).flex_direction();
    let state = tree.yoga_node_state(child);
    YogaItem {
        node: child,
        is_hidden: style.box_generation_mode() == crate::style::BoxGenerationMode::None,
        is_absolute: style.position() == Position::Absolute,
        margin,
        margin_is_auto,
        padding_border,
        min_style: style.min_size(),
        max_style: style.max_size(),
        size_style: processed,
        flex_basis_style: style.flex_basis(),
        box_sizing_adjustment,
        flex_grow: style.flex_grow(),
        flex_shrink: style.flex_shrink(),
        aspect_ratio: style.aspect_ratio(),
        align,
        overflow: style.overflow().x,
        inset: {
            let inset = style.inset();
            Rect {
                left: inset.left.yoga_resolve(owner_size.width, calc),
                right: inset.right.yoga_resolve(owner_size.width, calc),
                top: inset.top.yoga_resolve(owner_size.height, calc),
                bottom: inset.bottom.yoga_resolve(owner_size.height, calc),
            }
        },
        inset_is_auto: style.inset().map(|i| i.is_auto()),
        is_static: style.position() == Position::Static,
        own_flex_direction,
        position: Rect::ZERO,
        baseline: f32::NAN,
        computed_flex_basis: state.computed_flex_basis.unwrap_or(f32::NAN),
        computed_auto_min_main_size: None,
        min_size_is_set: style.min_size().map(|min| !min.is_auto()),
        measured: Size { width: f32::NAN, height: f32::NAN },
        line_index: 0,
        surfaced: OofCandidates::NONE,
    }
}

/// Yoga's `processDimensions` for one axis
#[inline]
pub(crate) fn processed_dimension(
    size: Dimension,
    min: crate::style::LengthPercentageAuto,
    max: crate::style::LengthPercentageAuto,
) -> Dimension {
    if !max.is_auto() && max == min {
        Dimension::from(max)
    } else {
        size
    }
}

/// Run a child layout with Yoga's sizing modes. `child_size` includes the child's margins, as Yoga's
/// `availableWidth`/`availableHeight` do; Taffy's constraints exclude them.
pub(crate) fn layout_yoga_child(
    tree: &mut impl LayoutYogaFlexboxContainer,
    item: &mut YogaItem,
    owner_size: Size<f32>,
    child_size: Size<f32>,
    modes: Size<YogaSizingMode>,
    run_mode: RunMode,
) -> LayoutOutput {
    let margin_row = item.margin.left + item.margin.right;
    let margin_column = item.margin.top + item.margin.bottom;
    let constraint = |mode: YogaSizingMode, size: f32, margin: f32| -> (Option<f32>, AvailableSpace) {
        match mode {
            YogaSizingMode::StretchFit if is_defined(size) => {
                (Some(size - margin), AvailableSpace::Definite(size - margin))
            }
            YogaSizingMode::FitContent if is_defined(size) => (None, AvailableSpace::Definite(size - margin)),
            _ => (None, AvailableSpace::MaxContent),
        }
    };
    let (known_width, available_width) = constraint(modes.width, child_size.width, margin_row);
    let (known_height, available_height) = constraint(modes.height, child_size.height, margin_column);
    // Yoga's MaxContent keeps the available size for the child's children
    let max_content_size = |mode: YogaSizingMode, size: f32, margin: f32| {
        (mode == YogaSizingMode::MaxContent && is_defined(size)).then_some(size - margin)
    };
    let yoga_available_size = Size {
        width: max_content_size(modes.width, child_size.width, margin_row),
        height: max_content_size(modes.height, child_size.height, margin_column),
    };

    let output = tree.compute_child_layout(
        item.node,
        LayoutInput {
            #[cfg(feature = "yoga_flexbox")]
            yoga_available_size,
            run_mode,
            sizing_mode: SizingMode::ContentSize,
            axis: RequestedAxis::Both,
            known_dimensions: Size { width: known_width, height: known_height },
            known_dimensions_are_definite: Size { width: true, height: true },
            parent_size: Size { width: defined(owner_size.width), height: defined(owner_size.height) },
            available_space: Size { width: available_width, height: available_height },
            vertical_margins_are_collapsible: Line { start: false, end: false },
        },
    );
    let mut output = output;
    item.measured = output.size;
    item.baseline = output.baselines.first.unwrap_or(output.size.height);
    if run_mode == RunMode::PerformLayout {
        item.surfaced = output.oof_candidates.take();
    }
    output
}

/// Yoga's `computeFlexBasisForChild`
pub(crate) fn compute_flex_basis_for_child(
    tree: &mut impl LayoutYogaFlexboxContainer,
    container: &YogaContainer,
    item: &mut YogaItem,
) {
    let main_axis = container.main_axis;
    let is_main_axis_row = is_row(main_axis);
    let available = container.available_inner;
    let main_axis_size = if is_main_axis_row { available.width } else { available.height };
    let owner_size = available;

    let is_row_style_dim_defined = item.has_definite_size(AbsoluteAxis::Horizontal, available.width);
    let is_column_style_dim_defined = item.has_definite_size(AbsoluteAxis::Vertical, available.height);

    let resolved_flex_basis = item.resolved_flex_basis(main_axis, main_axis_size);
    let use_resolved_flex_basis = resolved_flex_basis.is_some() && is_defined(main_axis_size);
    if use_resolved_flex_basis {
        if is_undefined(item.computed_flex_basis) {
            let padding_border = item.padding_border_for_axis(main_axis);
            item.computed_flex_basis = max_or_defined(resolved_flex_basis.unwrap_or(f32::NAN), padding_border);
        }
    } else if is_main_axis_row && is_row_style_dim_defined {
        let padding_border = item.padding_border_for_axis(FlexDirection::Row);
        item.computed_flex_basis = max_or_defined(
            item.resolved_size(AbsoluteAxis::Horizontal, available.width).unwrap_or(f32::NAN),
            padding_border,
        );
    } else if !is_main_axis_row && is_column_style_dim_defined {
        let padding_border = item.padding_border_for_axis(FlexDirection::Column);
        item.computed_flex_basis = max_or_defined(
            item.resolved_size(AbsoluteAxis::Vertical, available.height).unwrap_or(f32::NAN),
            padding_border,
        );
    } else {
        let mut child_width = f32::NAN;
        let mut child_height = f32::NAN;
        let mut child_width_mode = YogaSizingMode::MaxContent;
        let mut child_height_mode = YogaSizingMode::MaxContent;
        let margin_row = item.margin_for_axis(FlexDirection::Row);
        let margin_column = item.margin_for_axis(FlexDirection::Column);

        if let Some(width) = item.resolved_size(AbsoluteAxis::Horizontal, available.width) {
            child_width = width + margin_row;
            child_width_mode = YogaSizingMode::StretchFit;
        }
        if let Some(height) = item.resolved_size(AbsoluteAxis::Vertical, available.height) {
            child_height = height + margin_column;
            child_height_mode = YogaSizingMode::StretchFit;
        }

        let parent_scrolls = container.overflow == Overflow::Scroll;
        if ((!is_main_axis_row && parent_scrolls) || !parent_scrolls)
            && is_undefined(child_width)
            && is_defined(available.width)
        {
            child_width = available.width;
            child_width_mode = YogaSizingMode::FitContent;
        }

        // TODO(yoga_flexbox): FixFlexBasisFitContent's height-independent-subtree check
        // (isInColumnStretchScrollSubtree walks the ancestors, which needs a host hook).
        let apply_height_fit_content = is_main_axis_row || !parent_scrolls;
        if apply_height_fit_content && is_undefined(child_height) && is_defined(available.height) {
            child_height = available.height;
            child_height_mode = YogaSizingMode::FitContent;
        }

        if let Some(aspect_ratio) = item.aspect_ratio {
            if !is_main_axis_row && child_width_mode == YogaSizingMode::StretchFit {
                child_height = margin_column + (child_width - margin_row) / aspect_ratio;
                child_height_mode = YogaSizingMode::StretchFit;
            } else if is_main_axis_row && child_height_mode == YogaSizingMode::StretchFit {
                child_width = margin_row + (child_height - margin_column) * aspect_ratio;
                child_width_mode = YogaSizingMode::StretchFit;
            }
        }

        let has_exact_width = is_defined(available.width) && container.sizing_mode.width == YogaSizingMode::StretchFit;
        let child_width_stretch = item.align == AlignItems::STRETCH && child_width_mode != YogaSizingMode::StretchFit;
        if !is_main_axis_row && !is_row_style_dim_defined && has_exact_width && child_width_stretch {
            child_width = available.width;
            child_width_mode = YogaSizingMode::StretchFit;
            if let Some(aspect_ratio) = item.aspect_ratio {
                child_height = (child_width - margin_row) / aspect_ratio;
                child_height_mode = YogaSizingMode::StretchFit;
            }
        }

        let has_exact_height =
            is_defined(available.height) && container.sizing_mode.height == YogaSizingMode::StretchFit;
        let child_height_stretch = item.align == AlignItems::STRETCH && child_height_mode != YogaSizingMode::StretchFit;
        if is_main_axis_row && !is_column_style_dim_defined && has_exact_height && child_height_stretch {
            child_height = available.height;
            child_height_mode = YogaSizingMode::StretchFit;
            if let Some(aspect_ratio) = item.aspect_ratio {
                child_width = (child_height - margin_column) * aspect_ratio;
                child_width_mode = YogaSizingMode::StretchFit;
            }
        }

        constrain_max_size_for_mode(
            item.resolved_max(AbsoluteAxis::Horizontal, available.width).map(|max| max + margin_row),
            &mut child_width_mode,
            &mut child_width,
        );
        constrain_max_size_for_mode(
            item.resolved_max(AbsoluteAxis::Vertical, available.height).map(|max| max + margin_column),
            &mut child_height_mode,
            &mut child_height,
        );

        layout_yoga_child(
            tree,
            item,
            owner_size,
            Size { width: child_width, height: child_height },
            Size { width: child_width_mode, height: child_height_mode },
            RunMode::ComputeSize,
        );

        item.computed_flex_basis =
            max_or_defined(item.measured.get_abs(dimension(main_axis)), item.padding_border_for_axis(main_axis));
    }
}

/// Yoga's `computeFlexBasisForChildren`: returns the total outer flex basis of the in-flow children
pub(crate) fn compute_flex_basis_for_children(
    tree: &mut impl LayoutYogaFlexboxContainer,
    container: &YogaContainer,
    items: &mut [YogaItem],
    perform_layout: bool,
) -> f32 {
    let main_axis = container.main_axis;
    let sizing_mode_main = if is_row(main_axis) { container.sizing_mode.width } else { container.sizing_mode.height };

    // A single flexible child with non-zero grow and shrink gets a zero basis without measuring
    let mut single_flex_child: Option<usize> = None;
    if sizing_mode_main == YogaSizingMode::StretchFit {
        for (index, item) in items.iter().enumerate() {
            if item.is_flexible() {
                if single_flex_child.is_some()
                    || inexact_equals(item.flex_grow, 0.0)
                    || inexact_equals(item.flex_shrink, 0.0)
                {
                    single_flex_child = None;
                    break;
                } else {
                    single_flex_child = Some(index);
                }
            }
        }
    }

    let mut total_outer_flex_basis = 0.0;
    for (index, item) in items.iter_mut().enumerate() {
        if item.is_hidden {
            if perform_layout {
                tree.compute_child_layout(item.node, LayoutInput::HIDDEN);
            }
            continue;
        }
        if perform_layout {
            let child_main_axis = resolve_direction(item.own_flex_direction, container.direction);
            let child_cross_axis = resolve_cross_direction(child_main_axis, container.direction);
            item.set_position(child_main_axis, child_cross_axis, container.direction);
        }
        if item.is_absolute {
            continue;
        }
        if single_flex_child == Some(index) {
            item.computed_flex_basis = 0.0;
        } else {
            compute_flex_basis_for_child(tree, container, item);
        }
        total_outer_flex_basis += item.computed_flex_basis + item.margin_for_axis(main_axis);
    }
    total_outer_flex_basis
}

/// Yoga's `FlexLineRunningLayout`
#[derive(Copy, Clone, Default, Debug)]
pub(crate) struct FlexLineRunningLayout {
    /// Sum of the flex grow factors of the flexible items on the line
    pub(crate) total_flex_grow_factors: f32,
    /// Sum of `-flex_shrink * computed_flex_basis` of the flexible items on the line
    pub(crate) total_flex_shrink_scaled_factors: f32,
    /// Free space left after flexing
    pub(crate) remaining_free_space: f32,
    /// Main-axis size of the line
    pub(crate) main_dim: f32,
    /// Cross-axis size of the line
    pub(crate) cross_dim: f32,
}

/// Yoga's `FlexLine`: the in-flow items are a range of `items`
#[derive(Debug)]
pub(crate) struct FlexLine {
    /// Indices into the item list of the in-flow items on this line
    pub(crate) items_in_flow: Vec<usize>,
    /// Main-axis size consumed by the items' hypothetical sizes, margins and gaps
    pub(crate) size_consumed: f32,
    /// Number of `auto` main-axis margins on the line
    pub(crate) number_of_auto_margins: usize,
    /// Running layout state
    pub(crate) layout: FlexLineRunningLayout,
}

/// Yoga's `calculateFlexLine`: collect the next line of items starting at `*next`
pub(crate) fn calculate_flex_line(
    container: &YogaContainer,
    items: &mut [YogaItem],
    gap: f32,
    main_axis_owner_size: f32,
    available_inner_main_dim: f32,
    next: &mut usize,
    line_count: usize,
) -> FlexLine {
    let main_axis = container.main_axis;
    let (start_edge, end_edge) = (flex_start_edge(main_axis), flex_end_edge(main_axis));
    let mut items_in_flow = Vec::with_capacity(items.len() - *next);
    let mut size_consumed = 0.0;
    let mut total_flex_grow_factors = 0.0;
    let mut total_flex_shrink_scaled_factors = 0.0;
    let mut number_of_auto_margins = 0;
    let mut first_in_line: Option<usize> = None;
    let mut size_consumed_including_min_constraint = 0.0;

    while *next < items.len() {
        let index = *next;
        let item = &mut items[index];
        if item.is_hidden || item.is_absolute {
            *next += 1;
            continue;
        }
        if first_in_line.is_none() {
            first_in_line = Some(index);
        }
        if margin_is_auto_at(&item.margin_is_auto, start_edge) {
            number_of_auto_margins += 1;
        }
        if margin_is_auto_at(&item.margin_is_auto, end_edge) {
            number_of_auto_margins += 1;
        }
        item.line_index = line_count;
        let child_margin_main = item.margin_for_axis(main_axis);
        let leading_gap = if first_in_line == Some(index) { 0.0 } else { gap };
        let flex_basis_with_min_and_max =
            item.bound_within_min_and_max(main_axis, item.computed_flex_basis, main_axis_owner_size);
        let _ = main_axis_owner_size;

        if size_consumed_including_min_constraint + flex_basis_with_min_and_max + child_margin_main + leading_gap
            > available_inner_main_dim
            && container.is_wrap
            && !items_in_flow.is_empty()
        {
            break;
        }
        size_consumed_including_min_constraint += flex_basis_with_min_and_max + child_margin_main + leading_gap;
        size_consumed += flex_basis_with_min_and_max + child_margin_main + leading_gap;
        if item.is_flexible() {
            total_flex_grow_factors += item.flex_grow;
            total_flex_shrink_scaled_factors += -item.flex_shrink * item.computed_flex_basis;
        }
        items_in_flow.push(index);
        *next += 1;
    }

    if total_flex_grow_factors > 0.0 && total_flex_grow_factors < 1.0 {
        total_flex_grow_factors = 1.0;
    }
    if total_flex_shrink_scaled_factors > 0.0 && total_flex_shrink_scaled_factors < 1.0 {
        total_flex_shrink_scaled_factors = 1.0;
    }

    FlexLine {
        items_in_flow,
        size_consumed,
        number_of_auto_margins,
        layout: FlexLineRunningLayout {
            total_flex_grow_factors,
            total_flex_shrink_scaled_factors,
            ..FlexLineRunningLayout::default()
        },
    }
}

/// Yoga's `flexStartMarginIsAuto`/`flexEndMarginIsAuto` on already-physical margin flags
#[inline]
fn margin_is_auto_at(margin_is_auto: &Rect<bool>, edge: PhysicalEdge) -> bool {
    edge_value(margin_is_auto, edge)
}

impl YogaItem {
    /// Yoga's `boundAxis` for this item, min/max resolved against `reference`
    #[inline]
    pub(crate) fn bound_axis(&self, axis: FlexDirection, value: f32, reference: f32) -> f32 {
        let dim = dimension(axis);
        bound_axis(
            value,
            self.resolved_min(dim, reference),
            self.resolved_max(dim, reference),
            self.padding_border_for_axis(axis),
        )
    }

    /// Yoga's `boundAxisWithAutoMin`
    #[inline]
    pub(crate) fn bound_axis_with_auto_min(&self, axis: FlexDirection, value: f32, reference: f32) -> f32 {
        let bounded = self.bound_axis(axis, value, reference);
        match self.computed_auto_min_main_size {
            Some(auto_min) if bounded < auto_min => auto_min,
            _ => bounded,
        }
    }
}

/// Yoga's `computeMinContentMainSize`: the min-content size of `node` in `requested_axis`.
///
/// Leaves with a measure function are measured with zero available space in that axis (Yoga's
/// `AtMost 0` probe); containers sum/max their children recursively. Percentages resolve against
/// `owner_size`, as in Yoga. The static min-content overrides and min-content measure callback of
/// Yoga's API are not supported.
pub(crate) fn compute_min_content_main_size(
    tree: &mut impl LayoutYogaFlexboxContainer,
    node: NodeId,
    requested_axis: FlexDirection,
    owner_size: Size<f32>,
) -> f32 {
    let want_row = is_row(requested_axis);
    if tree.has_measure_func(node) {
        let output = tree.compute_child_layout(
            node,
            LayoutInput {
                #[cfg(feature = "yoga_flexbox")]
                yoga_available_size: Size::NONE,
                run_mode: RunMode::ComputeSize,
                sizing_mode: SizingMode::ContentSize,
                axis: RequestedAxis::Both,
                known_dimensions: Size::NONE,
                known_dimensions_are_definite: Size { width: true, height: true },
                parent_size: Size { width: defined(owner_size.width), height: defined(owner_size.height) },
                available_space: Size {
                    width: if want_row { AvailableSpace::Definite(0.0) } else { AvailableSpace::MaxContent },
                    height: if want_row { AvailableSpace::MaxContent } else { AvailableSpace::Definite(0.0) },
                },
                vertical_margins_are_collapsible: Line { start: false, end: false },
            },
        );
        return if want_row { output.size.width } else { output.size.height };
    }
    if tree.child_count(node) == 0 {
        return 0.0;
    }

    let (node_main_axis, node_cross_axis, padding_border) = {
        let style = tree.get_flexbox_container_style(node);
        let direction = style.direction();
        let node_main_axis = resolve_direction(style.flex_direction(), direction);
        let node_cross_axis = resolve_cross_direction(node_main_axis, direction);
        let owner_width = defined(owner_size.width);
        let calc = |val, basis| tree.resolve_calc_value(val, basis);
        let padding_border = style.padding().yoga_resolve_or_zero(owner_width, calc)
            + style.border().yoga_resolve_or_zero(owner_width, calc);
        (node_main_axis, node_cross_axis, padding_border.sum_axes())
    };

    let mut main_total = 0.0;
    let mut cross_max: f32 = 0.0;
    let child_count = tree.child_count(node);
    for index in 0..child_count {
        let child = tree.get_child_id(node, index);
        let margin = {
            let child_style = tree.get_flexbox_child_style(child);
            if child_style.box_generation_mode() == crate::style::BoxGenerationMode::None
                || child_style.position() == Position::Absolute
            {
                continue;
            }
            let owner_width = defined(owner_size.width);
            child_style
                .margin()
                .yoga_resolve_or_zero(owner_width, |val, basis| tree.resolve_calc_value(val, basis))
                .sum_axes()
        };
        let child_main = compute_min_content_main_size(tree, child, node_main_axis, owner_size)
            + margin.get_abs(dimension(node_main_axis));
        let child_cross = compute_min_content_main_size(tree, child, node_cross_axis, owner_size)
            + margin.get_abs(dimension(node_cross_axis));
        main_total += child_main;
        cross_max = cross_max.max(child_cross);
    }
    main_total += padding_border.get_abs(dimension(node_main_axis));
    cross_max += padding_border.get_abs(dimension(node_cross_axis));

    let node_main_is_row = is_row(node_main_axis);
    let width_min = if node_main_is_row { main_total } else { cross_max };
    let height_min = if node_main_is_row { cross_max } else { main_total };
    if want_row {
        width_min
    } else {
        height_min
    }
}

/// Yoga's `computeAutoMinMainSize`: the automatic minimum size of a flex item (CSS Flexbox §4.5)
pub(crate) fn compute_auto_min_main_size(
    tree: &mut impl LayoutYogaFlexboxContainer,
    container: &YogaContainer,
    item: &YogaItem,
    min_size_undefined_instead_of_auto: bool,
) -> Option<f32> {
    let main_axis = container.main_axis;
    let main_dim = dimension(main_axis);
    let is_main_axis_row = is_row(main_axis);
    if min_size_undefined_instead_of_auto || item.is_hidden || item.min_size_is_set.get_abs(main_dim) {
        return None;
    }
    if item.overflow != Overflow::Visible {
        return Some(0.0);
    }

    let specified_main = item.resolved_size(main_dim, container.owner_size.get_abs(main_dim));
    let transferred_main = item.aspect_ratio.and_then(|ratio| {
        let cross_dim = if is_main_axis_row { AbsoluteAxis::Vertical } else { AbsoluteAxis::Horizontal };
        item.resolved_size(cross_dim, container.available_inner.get_abs(cross_dim)).map(|cross| {
            if is_main_axis_row {
                cross * ratio
            } else {
                cross / ratio
            }
        })
    });

    let content_main = compute_min_content_main_size(tree, item.node, main_axis, container.available_inner);
    let mut floor = content_main;
    if let Some(specified) = specified_main {
        if specified < floor {
            floor = specified;
        }
    } else if let Some(transferred) = transferred_main {
        if transferred < floor {
            floor = transferred;
        }
    }
    if let Some(max_main) = item.resolved_max(main_dim, container.owner_size.get_abs(main_dim)) {
        if floor > max_main {
            floor = max_main;
        }
    }
    if is_undefined(floor) || floor < 0.0 {
        floor = 0.0;
    }
    Some(floor)
}

impl YogaItem {
    /// Yoga's `dimensionWithMargin`
    #[inline]
    pub(crate) fn dimension_with_margin(&self, axis: FlexDirection) -> f32 {
        self.measured.get_abs(dimension(axis)) + self.margin_for_axis(axis)
    }

    /// Yoga's `Node::relativePosition`: the offset a relatively positioned child's insets apply
    fn relative_position(&self, axis: FlexDirection, direction: Direction) -> f32 {
        if self.is_static {
            return 0.0;
        }
        let start = inline_start_edge(axis, direction);
        if !edge_value(&self.inset_is_auto, start) {
            return edge_value(&self.inset, start).unwrap_or(0.0);
        }
        -edge_value(&self.inset, inline_end_edge(axis, direction)).unwrap_or(0.0)
    }

    /// Yoga's `Node::setPosition`: seed the layout position with margins and relative insets.
    ///
    /// `main_axis`/`cross_axis` are the *child's* resolved axes, `direction` the child's direction.
    pub(crate) fn set_position(&mut self, main_axis: FlexDirection, cross_axis: FlexDirection, direction: Direction) {
        let relative_main = self.relative_position(main_axis, direction);
        let relative_cross = self.relative_position(cross_axis, direction);
        for (axis, relative) in [(main_axis, relative_main), (cross_axis, relative_cross)] {
            let leading = inline_start_edge(axis, direction);
            let trailing = inline_end_edge(axis, direction);
            set_edge_value(&mut self.position, leading, edge_value(&self.margin, leading) + relative);
            set_edge_value(&mut self.position, trailing, edge_value(&self.margin, trailing) + relative);
        }
    }
}

/// Yoga's `distributeFreeSpaceFirstPass`: shrink/grow items that hit a min/max and take their delta
/// out of the line's free space and flex factor totals.
pub(crate) fn distribute_free_space_first_pass(container: &YogaContainer, items: &mut [YogaItem], line: &mut FlexLine) {
    let main_axis = container.main_axis;
    let use_running_totals = container.config.has_errata(YogaErrata::FLEX_FIRST_PASS_USES_RUNNING_TOTALS);
    let original_total_grow = line.layout.total_flex_grow_factors;
    let original_total_shrink_scaled = line.layout.total_flex_shrink_scaled_factors;
    let mut delta_free_space = 0.0;

    for &index in &line.items_in_flow {
        let item = &items[index];
        let child_flex_basis = item.bound_within_min_and_max(
            main_axis,
            item.computed_flex_basis,
            container.owner_size.get_abs(dimension(main_axis)),
        );
        if line.layout.remaining_free_space < 0.0 {
            let flex_shrink_scaled_factor = -item.flex_shrink * child_flex_basis;
            if is_defined(flex_shrink_scaled_factor) && flex_shrink_scaled_factor != 0.0 {
                let total = if use_running_totals {
                    line.layout.total_flex_shrink_scaled_factors
                } else {
                    original_total_shrink_scaled
                };
                let base_main_size =
                    child_flex_basis + line.layout.remaining_free_space / total * flex_shrink_scaled_factor;
                let bound_main_size =
                    item.bound_axis_with_auto_min(main_axis, base_main_size, container.available_inner_main_dim);
                if is_defined(base_main_size) && is_defined(bound_main_size) && base_main_size != bound_main_size {
                    delta_free_space += bound_main_size - child_flex_basis;
                    line.layout.total_flex_shrink_scaled_factors -= -item.flex_shrink * item.computed_flex_basis;
                }
            }
        } else if is_defined(line.layout.remaining_free_space) && line.layout.remaining_free_space > 0.0 {
            let flex_grow_factor = item.flex_grow;
            if is_defined(flex_grow_factor) && flex_grow_factor != 0.0 {
                let total = if use_running_totals { line.layout.total_flex_grow_factors } else { original_total_grow };
                let base_main_size = child_flex_basis + line.layout.remaining_free_space / total * flex_grow_factor;
                let bound_main_size = item.bound_axis(main_axis, base_main_size, container.available_inner_main_dim);
                if is_defined(base_main_size) && is_defined(bound_main_size) && base_main_size != bound_main_size {
                    delta_free_space += bound_main_size - child_flex_basis;
                    line.layout.total_flex_grow_factors -= flex_grow_factor;
                }
            }
        }
    }
    line.layout.remaining_free_space -= delta_free_space;
}

/// Yoga's `distributeFreeSpaceSecondPass`: size every item on the line and lay it out (or measure
/// it). Returns the free space that was distributed.
pub(crate) fn distribute_free_space_second_pass(
    tree: &mut impl LayoutYogaFlexboxContainer,
    container: &YogaContainer,
    items: &mut [YogaItem],
    line: &mut FlexLine,
    main_axis_overflows: bool,
    perform_layout: bool,
    had_overflow: &mut bool,
) -> f32 {
    let main_axis = container.main_axis;
    let cross_axis = container.cross_axis;
    let is_main_axis_row = is_row(main_axis);
    let available = container.available_inner;
    let available_inner_cross = available.get_abs(dimension(cross_axis));
    let sizing_mode_cross = container.sizing_mode.get_abs(dimension(cross_axis));
    let cross_dim = dimension(cross_axis);
    let mut delta_free_space = 0.0;

    for &index in &line.items_in_flow {
        let item = &mut items[index];
        let child_flex_basis = item.bound_within_min_and_max(
            main_axis,
            item.computed_flex_basis,
            container.owner_size.get_abs(dimension(main_axis)),
        );
        let mut updated_main_size = child_flex_basis;
        if is_defined(line.layout.remaining_free_space) && line.layout.remaining_free_space < 0.0 {
            let flex_shrink_scaled_factor = -item.flex_shrink * child_flex_basis;
            if flex_shrink_scaled_factor != 0.0 {
                let total = line.layout.total_flex_shrink_scaled_factors;
                let child_size = if is_defined(total) && total.abs() < 1e-6 {
                    child_flex_basis + flex_shrink_scaled_factor
                } else {
                    child_flex_basis + (line.layout.remaining_free_space / total) * flex_shrink_scaled_factor
                };
                updated_main_size =
                    item.bound_axis_with_auto_min(main_axis, child_size, container.available_inner_main_dim);
            }
        } else if is_defined(line.layout.remaining_free_space) && line.layout.remaining_free_space > 0.0 {
            let flex_grow_factor = item.flex_grow;
            if !flex_grow_factor.is_nan() && flex_grow_factor != 0.0 {
                updated_main_size = item.bound_axis_with_auto_min(
                    main_axis,
                    child_flex_basis
                        + line.layout.remaining_free_space / line.layout.total_flex_grow_factors * flex_grow_factor,
                    container.available_inner_main_dim,
                );
            }
        }
        delta_free_space += updated_main_size - child_flex_basis;

        let margin_main = item.margin_for_axis(main_axis);
        let margin_cross = item.margin_for_axis(cross_axis);
        let mut child_main_size = updated_main_size + margin_main;
        let mut child_main_mode = YogaSizingMode::StretchFit;
        let mut child_cross_size;
        let child_cross_mode;

        let cross_is_definite = item.has_definite_size(cross_dim, available_inner_cross);
        let cross_margins_not_auto = !margin_is_auto_at(&item.margin_is_auto, flex_start_edge(cross_axis))
            && !margin_is_auto_at(&item.margin_is_auto, flex_end_edge(cross_axis));
        if let Some(aspect_ratio) = item.aspect_ratio {
            child_cross_size = if is_main_axis_row {
                (child_main_size - margin_main) / aspect_ratio
            } else {
                (child_main_size - margin_main) * aspect_ratio
            };
            child_cross_mode = YogaSizingMode::StretchFit;
            child_cross_size += margin_cross;
        } else if !available_inner_cross.is_nan()
            && !cross_is_definite
            && sizing_mode_cross == YogaSizingMode::StretchFit
            && !(container.is_wrap && main_axis_overflows)
            && item.align == AlignItems::STRETCH
            && cross_margins_not_auto
        {
            child_cross_size = available_inner_cross;
            child_cross_mode = YogaSizingMode::StretchFit;
        } else if !cross_is_definite {
            child_cross_size = available_inner_cross;
            child_cross_mode =
                if is_undefined(child_cross_size) { YogaSizingMode::MaxContent } else { YogaSizingMode::FitContent };
        } else {
            child_cross_size = item.resolved_size(cross_dim, available_inner_cross).unwrap_or(f32::NAN) + margin_cross;
            let is_loose_percentage_measurement =
                item.size_is_percent(cross_dim) && sizing_mode_cross != YogaSizingMode::StretchFit;
            child_cross_mode = if is_undefined(child_cross_size) || is_loose_percentage_measurement {
                YogaSizingMode::MaxContent
            } else {
                YogaSizingMode::StretchFit
            };
        }
        let mut child_cross_mode = child_cross_mode;

        constrain_max_size_for_mode(
            item.resolved_max(dimension(main_axis), container.available_inner_main_dim).map(|max| max + margin_main),
            &mut child_main_mode,
            &mut child_main_size,
        );
        constrain_max_size_for_mode(
            item.resolved_max(cross_dim, available_inner_cross).map(|max| max + margin_cross),
            &mut child_cross_mode,
            &mut child_cross_size,
        );

        let requires_stretch_layout = !cross_is_definite && item.align == AlignItems::STRETCH && cross_margins_not_auto;
        let (child_width, child_height) =
            if is_main_axis_row { (child_main_size, child_cross_size) } else { (child_cross_size, child_main_size) };
        let (child_width_mode, child_height_mode) =
            if is_main_axis_row { (child_main_mode, child_cross_mode) } else { (child_cross_mode, child_main_mode) };
        let is_layout_pass = perform_layout && !requires_stretch_layout;
        layout_yoga_child(
            tree,
            item,
            available,
            Size { width: child_width, height: child_height },
            Size { width: child_width_mode, height: child_height_mode },
            if is_layout_pass { RunMode::PerformLayout } else { RunMode::ComputeSize },
        );
        *had_overflow = *had_overflow || tree.yoga_node_state(item.node).had_overflow;
    }
    delta_free_space
}

/// Yoga's `resolveFlexibleLength`
pub(crate) fn resolve_flexible_length(
    tree: &mut impl LayoutYogaFlexboxContainer,
    container: &YogaContainer,
    items: &mut [YogaItem],
    line: &mut FlexLine,
    main_axis_overflows: bool,
    perform_layout: bool,
    had_overflow: &mut bool,
) {
    let original_free_space = line.layout.remaining_free_space;
    let min_size_undefined_instead_of_auto =
        container.config.has_errata(YogaErrata::MIN_SIZE_UNDEFINED_INSTEAD_OF_AUTO);
    for position in 0..line.items_in_flow.len() {
        let index = line.items_in_flow[position];
        let auto_min = if min_size_undefined_instead_of_auto {
            None
        } else {
            compute_auto_min_main_size(tree, container, &items[index], false)
        };
        items[index].computed_auto_min_main_size = auto_min;
    }
    distribute_free_space_first_pass(container, items, line);
    let distributed_free_space = distribute_free_space_second_pass(
        tree,
        container,
        items,
        line,
        main_axis_overflows,
        perform_layout,
        had_overflow,
    );
    line.layout.remaining_free_space = original_free_space - distributed_free_space;
}

/// Yoga's `justifyMainAxis`: position the line's items along the main axis and compute the line's
/// main and cross size.
pub(crate) fn justify_main_axis(
    container: &YogaContainer,
    items: &mut [YogaItem],
    line: &mut FlexLine,
    perform_layout: bool,
) {
    let main_axis = container.main_axis;
    let cross_axis = container.cross_axis;
    let main_dim = dimension(main_axis);
    let sizing_mode_main = container.sizing_mode.get_abs(main_dim);
    let sizing_mode_cross = container.sizing_mode.get_abs(dimension(cross_axis));
    let available_inner_main = container.available_inner.get_abs(main_dim);
    let available_inner_cross = container.available_inner.get_abs(dimension(cross_axis));
    let leading_padding_border_main = container.flex_start_padding_border(main_axis);
    let trailing_padding_border_main = container.flex_end_padding_border(main_axis);
    let gap = container.gap_for_axis(main_axis);

    if sizing_mode_main == YogaSizingMode::FitContent && line.layout.remaining_free_space > 0.0 {
        match container.min_size.get_abs(main_dim) {
            Some(min) if container.min_size_is_set.get_abs(main_dim) => {
                let min_available_main = min - leading_padding_border_main - trailing_padding_border_main;
                let occupied_space_by_child_nodes = available_inner_main - line.layout.remaining_free_space;
                line.layout.remaining_free_space =
                    max_or_defined(0.0, min_available_main - occupied_space_by_child_nodes);
            }
            _ => line.layout.remaining_free_space = 0.0,
        }
    }

    let mut leading_main_dim = 0.0;
    let mut between_main_dim = gap;
    let justify_content = if line.layout.remaining_free_space >= 0.0 {
        container.justify_content
    } else {
        fallback_justify_content(container.justify_content)
    };
    let item_count = line.items_in_flow.len() as f32;
    if line.number_of_auto_margins == 0 {
        if justify_content == JustifyContent::CENTER {
            leading_main_dim = line.layout.remaining_free_space / 2.0;
        } else if justify_content == JustifyContent::FLEX_END {
            leading_main_dim = line.layout.remaining_free_space;
        } else if justify_content == JustifyContent::SPACE_BETWEEN {
            if line.items_in_flow.len() > 1 {
                between_main_dim += line.layout.remaining_free_space / (item_count - 1.0);
            }
        } else if justify_content == JustifyContent::SPACE_EVENLY {
            leading_main_dim = line.layout.remaining_free_space / (item_count + 1.0);
            between_main_dim += leading_main_dim;
        } else if justify_content == JustifyContent::SPACE_AROUND {
            leading_main_dim = 0.5 * line.layout.remaining_free_space / item_count;
            between_main_dim += leading_main_dim * 2.0;
        }
    }

    line.layout.main_dim = leading_padding_border_main + leading_main_dim;
    line.layout.cross_dim = 0.0;
    let mut max_ascent_for_current_line: f32 = 0.0;
    let mut max_descent_for_current_line: f32 = 0.0;
    let start_edge = flex_start_edge(main_axis);
    let end_edge = flex_end_edge(main_axis);
    let last = line.items_in_flow.last().copied();

    for &index in &line.items_in_flow {
        let item = &mut items[index];
        if margin_is_auto_at(&item.margin_is_auto, start_edge) && line.layout.remaining_free_space > 0.0 {
            line.layout.main_dim += line.layout.remaining_free_space / line.number_of_auto_margins as f32;
        }
        if perform_layout {
            let position = edge_value(&item.position, start_edge) + line.layout.main_dim;
            set_edge_value(&mut item.position, start_edge, position);
        }
        if last != Some(index) {
            line.layout.main_dim += between_main_dim;
        }
        if margin_is_auto_at(&item.margin_is_auto, end_edge) && line.layout.remaining_free_space > 0.0 {
            line.layout.main_dim += line.layout.remaining_free_space / line.number_of_auto_margins as f32;
        }
        let can_skip_flex = !perform_layout && sizing_mode_cross == YogaSizingMode::StretchFit;
        if can_skip_flex {
            line.layout.main_dim += item.margin_for_axis(main_axis)
                + item.bound_within_min_and_max(
                    main_axis,
                    item.computed_flex_basis,
                    container.owner_size.get_abs(dimension(main_axis)),
                );
            line.layout.cross_dim = available_inner_cross;
        } else {
            line.layout.main_dim += item.dimension_with_margin(main_axis);
            if container.is_baseline_layout {
                let ascent = item.baseline + item.margin.top;
                let descent = item.measured.height + item.margin_for_axis(FlexDirection::Column) - ascent;
                max_ascent_for_current_line = max_or_defined(max_ascent_for_current_line, ascent);
                max_descent_for_current_line = max_or_defined(max_descent_for_current_line, descent);
            } else {
                line.layout.cross_dim = max_or_defined(line.layout.cross_dim, item.dimension_with_margin(cross_axis));
            }
        }
    }
    line.layout.main_dim += trailing_padding_border_main;
    if container.is_baseline_layout {
        line.layout.cross_dim = max_ascent_for_current_line + max_descent_for_current_line;
    }
}

/// Translate Taffy's constraints for one axis into Yoga's `availableWidth`/`availableHeight` and
/// sizing mode (Taffy constraints exclude the node's margin, as Yoga's do after subtracting it).
#[inline]
fn axis_constraint(known: Option<f32>, available: AvailableSpace) -> (f32, YogaSizingMode) {
    match (known, available) {
        (Some(size), _) => (size, YogaSizingMode::StretchFit),
        (None, AvailableSpace::Definite(size)) => (size, YogaSizingMode::FitContent),
        (None, AvailableSpace::MaxContent) => (f32::NAN, YogaSizingMode::MaxContent),
        (None, AvailableSpace::MinContent) => (0.0, YogaSizingMode::FitContent),
    }
}

/// Yoga's `calculateLayoutImpl` for a container (the leaf paths live in [`compute_yoga_leaf_layout`]).
pub fn compute_yoga_flexbox_layout(
    tree: &mut impl LayoutYogaFlexboxContainer,
    node: NodeId,
    inputs: LayoutInput,
) -> LayoutOutput {
    let perform_layout = inputs.run_mode == RunMode::PerformLayout;
    let config = *tree.yoga_config();
    let owner_size = Size {
        width: inputs.parent_size.width.unwrap_or(f32::NAN),
        height: inputs.parent_size.height.unwrap_or(f32::NAN),
    };
    let (mut available_width, mut width_mode) =
        axis_constraint(inputs.known_dimensions.width, inputs.available_space.width);
    let (mut available_height, mut height_mode) =
        axis_constraint(inputs.known_dimensions.height, inputs.available_space.height);
    // A Yoga parent's MaxContent still hands down the available size (see LayoutInput::yoga_available_size)
    for (mode, available, size) in [
        (width_mode, &mut available_width, inputs.yoga_available_size.width),
        (height_mode, &mut available_height, inputs.yoga_available_size.height),
    ] {
        if mode == YogaSizingMode::MaxContent {
            if let Some(size) = size {
                *available = size;
            }
        }
    }

    // Container styles (percentages resolve against the owner, as Yoga's `ownerWidth`)
    let style = tree.get_flexbox_container_style(node);
    let calc = |val, basis| tree.resolve_calc_value(val, basis);
    let direction = style.direction();
    let main_axis = resolve_direction(style.flex_direction(), direction);
    let cross_axis = resolve_cross_direction(main_axis, direction);
    let is_main_axis_row = is_row(main_axis);
    let flex_wrap = style.flex_wrap();
    let is_wrap = flex_wrap != FlexWrap::NoWrap;
    let owner_width = defined(owner_size.width);
    let owner_opt = Size { width: owner_width, height: defined(owner_size.height) };
    let padding = style.padding().yoga_resolve_or_zero(owner_width, calc);
    let border = style.border().yoga_resolve_or_zero(owner_width, calc);
    let padding_border = padding + border;
    let padding_border_sum = padding_border.sum_axes();
    let box_sizing_adjustment =
        if style.box_sizing() == BoxSizing::ContentBox { padding_border_sum } else { Size::ZERO };
    let min_size = style.min_size().yoga_resolve(owner_opt, calc).maybe_add(box_sizing_adjustment);
    let max_size = style.max_size().yoga_resolve(owner_opt, calc).maybe_add(box_sizing_adjustment);
    let min_size_is_set = style.min_size().map(|min| !min.is_auto());
    let size_style = style.size();
    let overflow = style.overflow().x;
    // Taffy's `normal` keyword has no Yoga equivalent and takes Yoga's defaults
    let align_items = style.align_items();
    let align_items = if align_items == AlignItems::NORMAL { AlignItems::STRETCH } else { align_items };
    let align_content = style.align_content();
    let align_content = if align_content == AlignContent::NORMAL { AlignContent::FLEX_START } else { align_content };
    let justify_content = style.justify_content();
    let justify_content =
        if justify_content == JustifyContent::NORMAL { JustifyContent::FLEX_START } else { justify_content };
    let gap_style = style.gap();
    let style_min_size = style.min_size();
    let style_max_size = style.max_size();
    let own_margin = style.margin().yoga_resolve_or_zero(owner_width, calc).sum_axes();
    drop(style);

    // A Yoga parent folds this node's style size into the known dimensions; at the root or under a
    // non-Yoga parent Yoga's `calculateLayout` applies the style size (or max size) itself.
    if inputs.sizing_mode == SizingMode::InherentSize {
        let processed = Size {
            width: processed_dimension(size_style.width, style_min_size.width, style_max_size.width),
            height: processed_dimension(size_style.height, style_min_size.height, style_max_size.height),
        };
        let resolved =
            processed.yoga_resolve(owner_opt, calc).map(|v| v.filter(|v| *v >= 0.0)).maybe_add(box_sizing_adjustment);
        // Yoga's calculateLayout adds the margin to a definite size but not to the max size; the
        // margin comes off again inside, so only the max branch subtracts it here
        let margin_sums = own_margin;
        for (known, resolved, max, margin, available, mode) in [
            (
                inputs.known_dimensions.width,
                resolved.width,
                max_size.width,
                margin_sums.width,
                &mut available_width,
                &mut width_mode,
            ),
            (
                inputs.known_dimensions.height,
                resolved.height,
                max_size.height,
                margin_sums.height,
                &mut available_height,
                &mut height_mode,
            ),
        ] {
            if known.is_some() {
                continue;
            }
            if let Some(size) = resolved {
                *available = size;
                *mode = YogaSizingMode::StretchFit;
            } else if let Some(max) = max {
                *available = max - margin;
                *mode = YogaSizingMode::FitContent;
            }
        }
    }
    let available = Size { width: available_width, height: available_height };
    let sizing_mode = Size { width: width_mode, height: height_mode };

    let bound_node_axis = |axis: FlexDirection, value: f32| -> f32 {
        let dim = dimension(axis);
        bound_axis(value, min_size.get_abs(dim), max_size.get_abs(dim), padding_border_sum.get_abs(dim))
    };

    // measureNodeWithFixedSize
    if !perform_layout {
        let is_fixed = |size: f32, mode: YogaSizingMode| {
            mode == YogaSizingMode::StretchFit
                || (is_defined(size) && mode == YogaSizingMode::FitContent && size <= 0.0)
        };
        if is_fixed(available.width, width_mode) && is_fixed(available.height, height_mode) {
            let fixed = |size: f32, mode: YogaSizingMode| {
                if is_undefined(size) || (mode == YogaSizingMode::FitContent && size < 0.0) {
                    0.0
                } else {
                    size
                }
            };
            let size = Size {
                width: bound_node_axis(FlexDirection::Row, fixed(available.width, width_mode)),
                height: bound_node_axis(FlexDirection::Column, fixed(available.height, height_mode)),
            };
            return LayoutOutput::from_outer_size(size);
        }
    }

    let mut had_overflow = false;
    let padding_border_axis_main = padding_border_sum.get_abs(dimension(main_axis));
    let padding_border_axis_cross = padding_border_sum.get_abs(dimension(cross_axis));
    let leading_padding_border_cross = edge_value(&padding_border, flex_start_edge(cross_axis));
    let mut sizing_mode_main = sizing_mode.get_abs(dimension(main_axis));
    let sizing_mode_cross = sizing_mode.get_abs(dimension(cross_axis));
    let main_axis_owner_size = owner_size.get_abs(dimension(main_axis));
    let cross_axis_owner_size = owner_size.get_abs(dimension(cross_axis));

    let available_inner = Size {
        width: calculate_available_inner_dimension(
            available.width,
            padding_border_sum.width,
            min_size.width,
            max_size.width,
        ),
        height: calculate_available_inner_dimension(
            available.height,
            padding_border_sum.height,
            min_size.height,
            max_size.height,
        ),
    };
    let mut available_inner_main = available_inner.get_abs(dimension(main_axis));
    let available_inner_cross = available_inner.get_abs(dimension(cross_axis));
    let gap = Size {
        width: gap_style.width.yoga_resolve(defined(available_inner.width), calc).unwrap_or(0.0),
        height: gap_style.height.yoga_resolve(defined(available_inner.height), calc).unwrap_or(0.0),
    };

    // The container's own definite cross size (`hasDefiniteLength` + `getResolvedDimension`), used
    // by the multi-line alignment pass
    let node_cross_size = size_style
        .get_abs(dimension(cross_axis))
        .yoga_resolve(defined(cross_axis_owner_size), calc)
        .filter(|v| *v >= 0.0)
        .map(|v| v + box_sizing_adjustment.get_abs(dimension(cross_axis)));

    let child_count = tree.child_count(node);
    let mut items: Vec<YogaItem> = Vec::with_capacity(child_count);
    let mut container = YogaContainer {
        node,
        direction,
        main_axis,
        cross_axis,
        is_wrap,
        overflow,
        align_items,
        available_inner,
        available_inner_main_dim: available_inner_main,
        owner_size,
        sizing_mode,
        padding_border,
        min_size,
        min_size_is_set,
        gap,
        justify_content,
        is_baseline_layout: false,
        config,
    };
    for order in 0..child_count {
        let child = tree.get_child_id(node, order);
        items.push(gather_item(tree, &container, child));
    }
    // isBaselineLayout
    container.is_baseline_layout = is_main_axis_row
        && (align_items == AlignItems::BASELINE
            || items.iter().any(|item| !item.is_absolute && item.align == AlignItems::BASELINE));

    // STEP 3: DETERMINE FLEX BASIS FOR EACH ITEM
    let mut total_main_dim = compute_flex_basis_for_children(tree, &container, &mut items, perform_layout);
    if child_count > 1 {
        total_main_dim += container.gap_for_axis(main_axis) * (child_count - 1) as f32;
    }
    let main_axis_overflows = sizing_mode_main != YogaSizingMode::MaxContent && total_main_dim > available_inner_main;
    if is_wrap && main_axis_overflows && sizing_mode_main == YogaSizingMode::FitContent {
        sizing_mode_main = YogaSizingMode::StretchFit;
        container.sizing_mode = if is_main_axis_row {
            Size { width: sizing_mode_main, height: sizing_mode_cross }
        } else {
            Size { width: sizing_mode_cross, height: sizing_mode_main }
        };
    }

    // STEP 4: COLLECT FLEX ITEMS INTO FLEX LINES
    let mut next = 0;
    let mut line_count = 0;
    let mut total_line_cross_dim = 0.0;
    let cross_axis_gap = container.gap_for_axis(cross_axis);
    let mut max_line_main_dim: f32 = 0.0;
    let mut lines: Vec<FlexLine> = Vec::new();
    let node_flex_grow_is_zero = true; // the container's own flex-grow is irrelevant to its children
    while next < items.len() {
        let mut line = calculate_flex_line(
            &container,
            &mut items,
            container.gap_for_axis(main_axis),
            main_axis_owner_size,
            available_inner_main,
            &mut next,
            line_count,
        );
        let can_skip_flex = !perform_layout && sizing_mode_cross == YogaSizingMode::StretchFit;
        let mut size_based_on_content = false;

        // STEP 5: RESOLVING FLEXIBLE LENGTHS ON MAIN AXIS
        if sizing_mode_main != YogaSizingMode::StretchFit {
            let min_inner_main = min_size.get_abs(dimension(main_axis)).unwrap_or(f32::NAN) - padding_border_axis_main;
            let max_inner_main = max_size.get_abs(dimension(main_axis)).unwrap_or(f32::NAN) - padding_border_axis_main;
            if is_defined(min_inner_main) && line.size_consumed < min_inner_main {
                available_inner_main = min_inner_main;
            } else if is_defined(max_inner_main) && line.size_consumed > max_inner_main {
                available_inner_main = max_inner_main;
            } else {
                let use_legacy_stretch_behaviour = config.has_errata(YogaErrata::STRETCH_FLEX_BASIS);
                if !use_legacy_stretch_behaviour
                    && ((is_defined(line.layout.total_flex_grow_factors) && line.layout.total_flex_grow_factors == 0.0)
                        || node_flex_grow_is_zero)
                {
                    available_inner_main = line.size_consumed;
                }
                size_based_on_content = !use_legacy_stretch_behaviour;
            }
        }
        container.available_inner_main_dim = available_inner_main;

        if !size_based_on_content && is_defined(available_inner_main) {
            line.layout.remaining_free_space = available_inner_main - line.size_consumed;
        } else if line.size_consumed < 0.0 {
            line.layout.remaining_free_space = -line.size_consumed;
        }

        if !can_skip_flex {
            resolve_flexible_length(
                tree,
                &container,
                &mut items,
                &mut line,
                main_axis_overflows,
                perform_layout,
                &mut had_overflow,
            );
        }
        had_overflow = had_overflow || line.layout.remaining_free_space < 0.0;

        // STEP 6: MAIN-AXIS JUSTIFICATION & CROSS-AXIS SIZE DETERMINATION
        justify_main_axis(&container, &mut items, &mut line, perform_layout);

        let mut container_cross_axis = available_inner_cross;
        if sizing_mode_cross == YogaSizingMode::MaxContent || sizing_mode_cross == YogaSizingMode::FitContent {
            container_cross_axis = bound_node_axis(cross_axis, line.layout.cross_dim + padding_border_axis_cross)
                - padding_border_axis_cross;
        }
        if !is_wrap && sizing_mode_cross == YogaSizingMode::StretchFit {
            line.layout.cross_dim = available_inner_cross;
        }
        if !is_wrap {
            line.layout.cross_dim = bound_node_axis(cross_axis, line.layout.cross_dim + padding_border_axis_cross)
                - padding_border_axis_cross;
        }

        // STEP 7: CROSS-AXIS ALIGNMENT
        if perform_layout {
            for &index in &line.items_in_flow {
                let item = &mut items[index];
                let mut leading_cross_dim = leading_padding_border_cross;
                let start_auto = margin_is_auto_at(&item.margin_is_auto, flex_start_edge(cross_axis));
                let end_auto = margin_is_auto_at(&item.margin_is_auto, flex_end_edge(cross_axis));
                if item.align == AlignItems::STRETCH && !start_auto && !end_auto {
                    if !item.has_definite_size(dimension(cross_axis), available_inner_cross) {
                        let mut child_main_size = item.measured.get_abs(dimension(main_axis));
                        let mut child_cross_size = match item.aspect_ratio {
                            Some(ratio) => {
                                item.margin_for_axis(cross_axis)
                                    + if is_main_axis_row { child_main_size / ratio } else { child_main_size * ratio }
                            }
                            None => line.layout.cross_dim,
                        };
                        child_main_size += item.margin_for_axis(main_axis);
                        let mut child_main_mode = YogaSizingMode::StretchFit;
                        let mut child_cross_mode = YogaSizingMode::StretchFit;
                        constrain_max_size_for_mode(
                            item.resolved_max(dimension(main_axis), container.available_inner_main_dim)
                                .map(|max| max + item.margin_for_axis(main_axis)),
                            &mut child_main_mode,
                            &mut child_main_size,
                        );
                        constrain_max_size_for_mode(
                            item.resolved_max(dimension(cross_axis), available_inner_cross)
                                .map(|max| max + item.margin_for_axis(cross_axis)),
                            &mut child_cross_mode,
                            &mut child_cross_size,
                        );
                        let (child_width, child_height) = if is_main_axis_row {
                            (child_main_size, child_cross_size)
                        } else {
                            (child_cross_size, child_main_size)
                        };
                        let cross_axis_does_not_grow = align_content != AlignContent::STRETCH && is_wrap;
                        let child_width_mode =
                            if is_undefined(child_width) || (!is_main_axis_row && cross_axis_does_not_grow) {
                                YogaSizingMode::MaxContent
                            } else {
                                YogaSizingMode::StretchFit
                            };
                        let child_height_mode =
                            if is_undefined(child_height) || (is_main_axis_row && cross_axis_does_not_grow) {
                                YogaSizingMode::MaxContent
                            } else {
                                YogaSizingMode::StretchFit
                            };
                        layout_yoga_child(
                            tree,
                            item,
                            container.available_inner,
                            Size { width: child_width, height: child_height },
                            Size { width: child_width_mode, height: child_height_mode },
                            RunMode::PerformLayout,
                        );
                    }
                } else {
                    let remaining_cross_dim = container_cross_axis - item.dimension_with_margin(cross_axis);
                    if start_auto && end_auto {
                        leading_cross_dim += max_or_defined(0.0, remaining_cross_dim / 2.0);
                    } else if end_auto {
                        // trailing auto margin: no offset
                    } else if start_auto {
                        leading_cross_dim += max_or_defined(0.0, remaining_cross_dim);
                    } else if item.align == AlignItems::FLEX_START {
                        // no offset
                    } else if item.align == AlignItems::CENTER {
                        leading_cross_dim += remaining_cross_dim / 2.0;
                    } else {
                        leading_cross_dim += remaining_cross_dim;
                    }
                }
                let edge = flex_start_edge(cross_axis);
                let position = edge_value(&item.position, edge) + total_line_cross_dim + leading_cross_dim;
                set_edge_value(&mut item.position, edge, position);
            }
        }

        let applied_cross_gap = if line_count != 0 { cross_axis_gap } else { 0.0 };
        total_line_cross_dim += line.layout.cross_dim + applied_cross_gap;
        max_line_main_dim = max_or_defined(max_line_main_dim, line.layout.main_dim);
        lines.push(line);
        line_count += 1;
    }

    // STEP 8: MULTI-LINE CONTENT ALIGNMENT
    if perform_layout && (is_wrap || container.is_baseline_layout) {
        let mut lead_per_line = 0.0;
        let mut current_lead = leading_padding_border_cross;
        let mut extra_space_per_line = 0.0;
        let cross_dim = dimension(cross_axis);
        let unclamped_cross_dim = if sizing_mode_cross == YogaSizingMode::StretchFit {
            available_inner_cross + padding_border_axis_cross
        } else if let Some(cross) = node_cross_size {
            cross
        } else {
            total_line_cross_dim + padding_border_axis_cross
        };
        let inner_cross_dim = bound_node_axis(cross_axis, unclamped_cross_dim) - padding_border_axis_cross;
        let remaining_align_content_dim = inner_cross_dim - total_line_cross_dim;
        let align_content = if remaining_align_content_dim >= 0.0 {
            align_content
        } else {
            super::axis::fallback_align_content(align_content)
        };
        let line_count_f = line_count as f32;
        if align_content == AlignContent::FLEX_END {
            current_lead += remaining_align_content_dim;
        } else if align_content == AlignContent::CENTER {
            current_lead += remaining_align_content_dim / 2.0;
        } else if align_content == AlignContent::STRETCH {
            extra_space_per_line = remaining_align_content_dim / line_count_f;
        } else if align_content == AlignContent::SPACE_AROUND {
            current_lead += remaining_align_content_dim / (2.0 * line_count_f);
            lead_per_line = remaining_align_content_dim / line_count_f;
        } else if align_content == AlignContent::SPACE_EVENLY {
            current_lead += remaining_align_content_dim / (line_count_f + 1.0);
            lead_per_line = remaining_align_content_dim / (line_count_f + 1.0);
        } else if align_content == AlignContent::SPACE_BETWEEN && line_count > 1 {
            lead_per_line = remaining_align_content_dim / (line_count_f - 1.0);
        }

        let mut end_index = 0;
        for i in 0..line_count {
            let start_index = end_index;
            let mut index = start_index;
            let mut line_height: f32 = 0.0;
            let mut max_ascent_for_current_line: f32 = 0.0;
            let mut max_descent_for_current_line: f32 = 0.0;
            while index < items.len() {
                let item = &items[index];
                if item.is_hidden {
                    index += 1;
                    continue;
                }
                if !item.is_absolute {
                    if item.line_index != i {
                        break;
                    }
                    if is_defined(item.measured.get_abs(cross_dim)) {
                        line_height = max_or_defined(line_height, item.dimension_with_margin(cross_axis));
                    }
                    if item.align == AlignItems::BASELINE {
                        let ascent = item.baseline + item.margin.top;
                        let descent = item.measured.height + item.margin_for_axis(FlexDirection::Column) - ascent;
                        max_ascent_for_current_line = max_or_defined(max_ascent_for_current_line, ascent);
                        max_descent_for_current_line = max_or_defined(max_descent_for_current_line, descent);
                        line_height =
                            max_or_defined(line_height, max_ascent_for_current_line + max_descent_for_current_line);
                    }
                }
                index += 1;
            }
            end_index = index;
            current_lead += if i != 0 { cross_axis_gap } else { 0.0 };
            line_height += extra_space_per_line;
            for item in items[start_index..end_index].iter_mut() {
                if item.is_hidden || item.is_absolute {
                    continue;
                }
                let cross_start = flex_start_edge(cross_axis);
                let cross_end = flex_end_edge(cross_axis);
                if item.align == AlignItems::FLEX_START {
                    let inset_start =
                        if item.is_static { 0.0 } else { edge_value(&item.inset, cross_start).unwrap_or(0.0) };
                    set_edge_value(&mut item.position, cross_start, current_lead + inset_start);
                } else if item.align == AlignItems::FLEX_END {
                    set_edge_value(
                        &mut item.position,
                        cross_start,
                        current_lead + line_height
                            - edge_value(&item.margin, cross_end)
                            - item.measured.get_abs(cross_dim),
                    );
                } else if item.align == AlignItems::CENTER {
                    let child_height = item.measured.get_abs(cross_dim);
                    set_edge_value(&mut item.position, cross_start, current_lead + (line_height - child_height) / 2.0);
                } else if item.align == AlignItems::STRETCH {
                    set_edge_value(
                        &mut item.position,
                        cross_start,
                        current_lead + edge_value(&item.margin, cross_start),
                    );
                    if !item.has_definite_size(cross_dim, available_inner_cross) {
                        let child_width = if is_main_axis_row {
                            item.measured.width + item.margin_for_axis(main_axis)
                        } else {
                            lead_per_line + line_height
                        };
                        let child_height = if !is_main_axis_row {
                            item.measured.height + item.margin_for_axis(cross_axis)
                        } else {
                            lead_per_line + line_height
                        };
                        if !(inexact_equals(child_width, item.measured.width)
                            && inexact_equals(child_height, item.measured.height))
                        {
                            layout_yoga_child(
                                tree,
                                item,
                                container.available_inner,
                                Size { width: child_width, height: child_height },
                                Size { width: YogaSizingMode::StretchFit, height: YogaSizingMode::StretchFit },
                                RunMode::PerformLayout,
                            );
                        }
                    }
                } else if item.align == AlignItems::BASELINE {
                    let inset_top = if item.is_static { 0.0 } else { item.inset.top.unwrap_or(0.0) };
                    item.position.top = current_lead + max_ascent_for_current_line - item.baseline + inset_top;
                }
            }
            current_lead = current_lead + lead_per_line + line_height;
        }
    }

    // STEP 9: COMPUTING FINAL DIMENSIONS
    let mut measured = Size {
        width: bound_node_axis(FlexDirection::Row, available.width),
        height: bound_node_axis(FlexDirection::Column, available.height),
    };
    let main_dim = dimension(main_axis);
    let cross_dim = dimension(cross_axis);
    if sizing_mode_main == YogaSizingMode::MaxContent
        || (overflow != Overflow::Scroll && sizing_mode_main == YogaSizingMode::FitContent)
    {
        set_abs(&mut measured, main_dim, bound_node_axis(main_axis, max_line_main_dim));
    } else if sizing_mode_main == YogaSizingMode::FitContent && overflow == Overflow::Scroll {
        set_abs(
            &mut measured,
            main_dim,
            max_or_defined(
                min_or_defined(
                    available_inner_main + padding_border_axis_main,
                    bound_within(max_line_main_dim, min_size.get_abs(main_dim), max_size.get_abs(main_dim)),
                ),
                padding_border_axis_main,
            ),
        );
    }
    if sizing_mode_cross == YogaSizingMode::MaxContent
        || (overflow != Overflow::Scroll && sizing_mode_cross == YogaSizingMode::FitContent)
    {
        set_abs(
            &mut measured,
            cross_dim,
            bound_node_axis(cross_axis, total_line_cross_dim + padding_border_axis_cross),
        );
    } else if sizing_mode_cross == YogaSizingMode::FitContent && overflow == Overflow::Scroll {
        set_abs(
            &mut measured,
            cross_dim,
            max_or_defined(
                min_or_defined(
                    available_inner_cross + padding_border_axis_cross,
                    bound_within(
                        total_line_cross_dim + padding_border_axis_cross,
                        min_size.get_abs(cross_dim),
                        max_size.get_abs(cross_dim),
                    ),
                ),
                padding_border_axis_cross,
            ),
        );
    }

    if perform_layout {
        // As we only wrapped in normal direction yet, we need to reverse the positions on wrap-reverse
        if flex_wrap == FlexWrap::WrapReverse {
            let edge = flex_start_edge(cross_axis);
            for item in items.iter_mut().filter(|item| !item.is_absolute) {
                let position =
                    measured.get_abs(cross_dim) - edge_value(&item.position, edge) - item.measured.get_abs(cross_dim);
                set_edge_value(&mut item.position, edge, position);
            }
        }
        // STEP 10: SIZING AND POSITIONING ABSOLUTE CHILDREN happens via Taffy's out-of-flow hoisting
        // STEP 11: SETTING TRAILING POSITIONS FOR CHILDREN
        let needs_main_trailing = needs_trailing_position(main_axis);
        let needs_cross_trailing = needs_trailing_position(cross_axis);
        if needs_main_trailing || needs_cross_trailing {
            for item in items.iter_mut().filter(|item| !item.is_hidden && !item.is_absolute) {
                for (axis, needed) in [(main_axis, needs_main_trailing), (cross_axis, needs_cross_trailing)] {
                    if needed {
                        let dim = dimension(axis);
                        let position = position_of_opposite_edge(
                            edge_value(&item.position, flex_start_edge(axis)),
                            measured.get_abs(dim),
                            item.measured.get_abs(dim),
                        );
                        set_edge_value(&mut item.position, flex_end_edge(axis), position);
                    }
                }
            }
        }
    }

    // Persist per-node Yoga state and write the children's layouts
    let mut candidates = OofCandidates::new();
    let excludes_padding = config.has_errata(YogaErrata::ABSOLUTE_POSITION_WITHOUT_INSETS_EXCLUDES_PADDING);
    let static_area = |axis: FlexDirection| -> Line<f32> {
        let (border, padding, size) = if is_row(axis) {
            (border.horizontal_components(), padding.horizontal_components(), measured.width)
        } else {
            (border.vertical_components(), padding.vertical_components(), measured.height)
        };
        let padding = if excludes_padding { Line { start: 0.0, end: 0.0 } } else { padding };
        Line { start: border.start + padding.start, end: size - border.end - padding.end }
    };
    let wrap_reverse = flex_wrap == FlexWrap::WrapReverse;
    for (order, item) in items.iter_mut().enumerate() {
        let mut state = tree.yoga_node_state(item.node);
        state.computed_flex_basis = defined(item.computed_flex_basis);
        tree.set_yoga_node_state(item.node, state);
        if !perform_layout {
            continue;
        }
        if item.is_hidden {
            tree.set_unrounded_layout(item.node, &Layout::with_order(order as u32));
            continue;
        }
        if item.is_absolute {
            // justifyAbsoluteChild / alignAbsoluteChild: the static position within the parent
            let main = absolute_static_position(main_axis, justify_placement(justify_content), static_area(main_axis));
            let cross = absolute_static_position(
                cross_axis,
                align_placement(item.align, wrap_reverse),
                static_area(cross_axis),
            );
            candidates.push(OofCandidate {
                node: item.node,
                order: order as u32,
                position: Position::Absolute,
                static_position: if is_main_axis_row {
                    Point { x: main, y: cross }
                } else {
                    Point { x: cross, y: main }
                },
                yoga_parent_is_row: is_main_axis_row,
            });
            continue;
        }
        // layoutAbsoluteDescendants: absolute descendants of static children position against this node
        if !item.surfaced.is_empty() {
            let mut surfaced = core::mem::take(&mut item.surfaced);
            surfaced.translate(Point { x: item.position.left, y: item.position.top });
            candidates.append(&mut surfaced);
        }
        tree.set_unrounded_layout(
            item.node,
            &Layout {
                order: order as u32,
                size: item.measured,
                #[cfg(feature = "content_size")]
                scrollable_overflow_rect: Rect {
                    left: 0.0,
                    top: 0.0,
                    right: item.measured.width,
                    bottom: item.measured.height,
                },
                scrollbar_size: Size::ZERO,
                location: Point { x: item.position.left, y: item.position.top },
                padding: Rect {
                    left: item.padding_border.left,
                    right: item.padding_border.right,
                    top: item.padding_border.top,
                    bottom: item.padding_border.bottom,
                },
                border: Rect::ZERO,
                margin: item.margin,
            },
        );
    }
    let mut node_state = tree.yoga_node_state(node);
    node_state.had_overflow = had_overflow;
    tree.set_yoga_node_state(node, node_state);

    // calculateBaseline for the container
    let baseline = {
        let mut baseline_item: Option<&YogaItem> = None;
        for item in items.iter().filter(|item| !item.is_hidden) {
            if item.line_index > 0 {
                break;
            }
            if item.is_absolute {
                continue;
            }
            if item.align == AlignItems::BASELINE {
                baseline_item = Some(item);
                break;
            }
            if baseline_item.is_none() {
                baseline_item = Some(item);
            }
        }
        match baseline_item {
            Some(item) => item.baseline + item.position.top,
            None => measured.height,
        }
    };

    let mut output = LayoutOutput::from_sizes_and_baselines(
        measured,
        Rect { left: 0.0, top: 0.0, right: measured.width, bottom: measured.height },
        Baselines::from_first(Some(baseline)),
    );
    output.oof_candidates = candidates;
    output.oof_positioning_area = Some(OofPositioningArea {
        size: measured - border.sum_axes(),
        offset: Point { x: border.left, y: border.top },
    });
    if perform_layout {
        let containing_block_size = if config.has_errata(YogaErrata::ABSOLUTE_PERCENT_AGAINST_INNER_SIZE) {
            container.available_inner
        } else {
            measured - border.sum_axes()
        };
        tree.set_detailed_yoga_info(
            node,
            YogaDetailedInfo { width_sizing_mode: sizing_mode.width, containing_block_size },
        );
    }
    output
}

/// Where an absolute child without insets sits along an axis of its parent
#[derive(Copy, Clone, PartialEq, Eq, Debug)]
enum FlexPlacement {
    /// Against the parent's flex-start edge
    FlexStart,
    /// Against the parent's flex-end edge
    FlexEnd,
    /// Centred in the parent
    Center,
}

/// `justifyAbsoluteChild`'s mapping of the parent's `justify-content`
fn justify_placement(justify: JustifyContent) -> FlexPlacement {
    match justify.keyword {
        AlignContentKeyword::End | AlignContentKeyword::FlexEnd => FlexPlacement::FlexEnd,
        AlignContentKeyword::Center | AlignContentKeyword::SpaceAround | AlignContentKeyword::SpaceEvenly => {
            FlexPlacement::Center
        }
        _ => FlexPlacement::FlexStart,
    }
}

/// `alignAbsoluteChild`'s mapping of the child's resolved alignment, with `wrap-reverse` flipping
fn align_placement(align: AlignItems, wrap_reverse: bool) -> FlexPlacement {
    let placement = match align.keyword {
        AlignItemsKeyword::End | AlignItemsKeyword::FlexEnd => FlexPlacement::FlexEnd,
        AlignItemsKeyword::Center => FlexPlacement::Center,
        _ => FlexPlacement::FlexStart,
    };
    match (wrap_reverse, placement) {
        (true, FlexPlacement::FlexEnd) => FlexPlacement::FlexStart,
        (true, FlexPlacement::FlexStart) => FlexPlacement::FlexEnd,
        _ => placement,
    }
}

/// The static position of an absolute child along one of the parent's axes, in physical terms:
/// the parent's content box (padding box under `AbsolutePositionWithoutInsetsExcludesPadding`) and
/// the edge the child hugs, with reversed axes swapping start and end.
fn absolute_static_position(axis: FlexDirection, placement: FlexPlacement, area: Line<f32>) -> AxisStaticPosition {
    let reversed = matches!(flex_start_edge(axis), PhysicalEdge::Right | PhysicalEdge::Bottom);
    let edge = match (placement, reversed) {
        (FlexPlacement::Center, _) => AxisStaticEdge::Center,
        (FlexPlacement::FlexStart, false) | (FlexPlacement::FlexEnd, true) => AxisStaticEdge::Start,
        (FlexPlacement::FlexEnd, false) | (FlexPlacement::FlexStart, true) => AxisStaticEdge::End,
    };
    AxisStaticPosition { area, align: AxisStaticAlign::from_keyword(edge) }
}
