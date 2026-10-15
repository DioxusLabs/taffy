//! The Yoga flexbox algorithm, ported from `yoga/algorithm/CalculateLayout.cpp` and `FlexLine.cpp`
//! at Yoga commit `08250431e09bc4b864917e51f9e9d52998ee2dce`.
//!
//! Undefined values are `f32::NAN`, as in Yoga; min/max constraints are `Option<f32>`.

#![allow(dead_code)]

use crate::geometry::{AbsoluteAxis, Line, Rect, Size};
use crate::style::{AlignItems, AvailableSpace, BoxSizing, Dimension, FlexDirection, Overflow, Position};
use crate::tree::{LayoutInput, LayoutOutput, LayoutYogaFlexboxContainer, NodeId, RequestedAxis, RunMode, SizingMode};
use crate::util::sys::Vec;
use crate::util::{MaybeMath, MaybeResolve, ResolveOrZero};
use crate::{CoreStyle, FlexboxItemStyle};

use super::axis::{bound_axis_within_min_and_max, constrain_max_size_for_mode, dimension, is_row};
use super::enums::YogaSizingMode;
use super::numeric::{inexact_equals, is_defined, is_undefined, max_or_defined};

/// Convert Yoga's NaN-as-undefined into Taffy's `Option`
#[inline]
fn defined(value: f32) -> Option<f32> {
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
    /// Resolved min size, box-sizing adjusted (`resolvedMinDimension`)
    pub(crate) min_size: Size<Option<f32>>,
    /// Resolved max size, box-sizing adjusted (`resolvedMaxDimension`)
    pub(crate) max_size: Size<Option<f32>>,
    /// Resolved style size after `processDimensions` (max == min → max) and box-sizing adjustment,
    /// `None` when not definite (`getResolvedDimension` + `hasDefiniteLength`)
    pub(crate) size: Size<Option<f32>>,
    /// Resolved `flex-basis` in the main axis, box-sizing adjusted (`resolveFlexBasis`)
    pub(crate) flex_basis: Option<f32>,
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
    /// `LayoutResults::computedFlexBasis` (NaN when undefined)
    pub(crate) computed_flex_basis: f32,
    /// Measured size from the most recent child layout call (NaN before any)
    pub(crate) measured: Size<f32>,
    /// Index of the flex line the item ended up on
    pub(crate) line_index: usize,
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

    /// `hasDefiniteLength`
    #[inline]
    pub(crate) fn has_definite_size(&self, axis: AbsoluteAxis) -> bool {
        self.size.get_abs(axis).is_some()
    }

    /// `boundAxisWithinMinAndMax` for this item
    #[inline]
    pub(crate) fn bound_within_min_and_max(&self, axis: FlexDirection, value: f32) -> f32 {
        let dim = dimension(axis);
        bound_axis_within_min_and_max(value, self.min_size.get_abs(dim), self.max_size.get_abs(dim))
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
    /// `availableInnerWidth`/`availableInnerHeight` (may be undefined)
    pub(crate) available_inner: Size<f32>,
    /// The container's own `ownerWidth`/`ownerHeight`
    pub(crate) owner_size: Size<f32>,
    /// Width/height sizing modes the container was asked for
    pub(crate) sizing_mode: Size<YogaSizingMode>,
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
    let margin = margin_style.resolve_or_zero(owner_width, calc);
    let margin_is_auto = margin_style.map(|m| m.is_auto());
    let padding_border =
        style.padding().resolve_or_zero(owner_width, calc) + style.border().resolve_or_zero(owner_width, calc);
    let padding_border_sum = padding_border.sum_axes();
    let box_sizing_adjustment =
        if style.box_sizing() == BoxSizing::ContentBox { padding_border_sum } else { Size::ZERO };

    let min_size = style.min_size().maybe_resolve(owner_size, calc).maybe_add(box_sizing_adjustment);
    let max_size = style.max_size().maybe_resolve(owner_size, calc).maybe_add(box_sizing_adjustment);

    // processDimensions: a max size equal to the min size replaces the style size
    let style_size = style.size();
    let processed = Size {
        width: processed_dimension(style_size.width, style.min_size().width, style.max_size().width),
        height: processed_dimension(style_size.height, style.min_size().height, style.max_size().height),
    };
    let size =
        processed.maybe_resolve(owner_size, calc).map(|v| v.filter(|v| *v >= 0.0)).maybe_add(box_sizing_adjustment);

    let main_axis_owner_size = if is_row(container.main_axis) { owner_size.width } else { owner_size.height };
    let flex_basis = style
        .flex_basis()
        .maybe_resolve(main_axis_owner_size, calc)
        .map(|basis| basis + box_sizing_adjustment.get_abs(dimension(container.main_axis)));

    let align = match style.align_self() {
        Some(align_self) => align_self,
        None => container.align_items,
    };
    // Yoga: baseline alignment in a column container falls back to flex-start
    let align =
        if align == AlignItems::BASELINE && !is_row(container.main_axis) { AlignItems::FLEX_START } else { align };

    let state = tree.yoga_node_state(child);
    YogaItem {
        node: child,
        is_hidden: style.box_generation_mode() == crate::style::BoxGenerationMode::None,
        is_absolute: style.position() == Position::Absolute,
        margin,
        margin_is_auto,
        padding_border,
        min_size,
        max_size,
        size,
        flex_basis,
        flex_grow: style.flex_grow(),
        flex_shrink: style.flex_shrink(),
        aspect_ratio: style.aspect_ratio(),
        align,
        overflow: style.overflow().x,
        computed_flex_basis: state.computed_flex_basis.unwrap_or(f32::NAN),
        measured: Size { width: f32::NAN, height: f32::NAN },
        line_index: 0,
    }
}

/// Yoga's `processDimensions` for one axis
#[inline]
fn processed_dimension(
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

    let output = tree.compute_child_layout(
        item.node,
        LayoutInput {
            run_mode,
            sizing_mode: SizingMode::InherentSize,
            axis: RequestedAxis::Both,
            known_dimensions: Size { width: known_width, height: known_height },
            known_dimensions_are_definite: Size { width: true, height: true },
            parent_size: Size { width: defined(owner_size.width), height: defined(owner_size.height) },
            available_space: Size { width: available_width, height: available_height },
            vertical_margins_are_collapsible: Line { start: false, end: false },
        },
    );
    item.measured = output.size;
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

    let is_row_style_dim_defined = item.has_definite_size(AbsoluteAxis::Horizontal);
    let is_column_style_dim_defined = item.has_definite_size(AbsoluteAxis::Vertical);

    let use_resolved_flex_basis = item.flex_basis.is_some() && is_defined(main_axis_size);
    if use_resolved_flex_basis {
        if is_undefined(item.computed_flex_basis) {
            let padding_border = item.padding_border_for_axis(main_axis);
            item.computed_flex_basis = max_or_defined(item.flex_basis.unwrap_or(f32::NAN), padding_border);
        }
    } else if is_main_axis_row && is_row_style_dim_defined {
        let padding_border = item.padding_border_for_axis(FlexDirection::Row);
        item.computed_flex_basis = max_or_defined(item.size.width.unwrap_or(f32::NAN), padding_border);
    } else if !is_main_axis_row && is_column_style_dim_defined {
        let padding_border = item.padding_border_for_axis(FlexDirection::Column);
        item.computed_flex_basis = max_or_defined(item.size.height.unwrap_or(f32::NAN), padding_border);
    } else {
        let mut child_width = f32::NAN;
        let mut child_height = f32::NAN;
        let mut child_width_mode = YogaSizingMode::MaxContent;
        let mut child_height_mode = YogaSizingMode::MaxContent;
        let margin_row = item.margin_for_axis(FlexDirection::Row);
        let margin_column = item.margin_for_axis(FlexDirection::Column);

        if let Some(width) = item.size.width {
            child_width = width + margin_row;
            child_width_mode = YogaSizingMode::StretchFit;
        }
        if let Some(height) = item.size.height {
            child_height = height + margin_column;
            child_height_mode = YogaSizingMode::StretchFit;
        }

        let parent_scrolls = container.overflow == Overflow::Scroll;
        if (!is_main_axis_row && parent_scrolls) || !parent_scrolls {
            if is_undefined(child_width) && is_defined(available.width) {
                child_width = available.width;
                child_width_mode = YogaSizingMode::FitContent;
            }
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
            item.max_size.width.map(|max| max + margin_row),
            &mut child_width_mode,
            &mut child_width,
        );
        constrain_max_size_for_mode(
            item.max_size.height.map(|max| max + margin_column),
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
    let (start_is_auto, end_is_auto) = main_axis_auto_margin_edges(main_axis);
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
        if start_is_auto(&item.margin_is_auto) {
            number_of_auto_margins += 1;
        }
        if end_is_auto(&item.margin_is_auto) {
            number_of_auto_margins += 1;
        }
        item.line_index = line_count;
        let child_margin_main = item.margin_for_axis(main_axis);
        let leading_gap = if first_in_line == Some(index) { 0.0 } else { gap };
        let flex_basis_with_min_and_max = item.bound_within_min_and_max(main_axis, item.computed_flex_basis);
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

/// Accessors for the flex-start and flex-end margin flags of a main axis
/// (`flexStartMarginIsAuto`/`flexEndMarginIsAuto`; the physical edges follow the resolved axis)
fn main_axis_auto_margin_edges(main_axis: FlexDirection) -> (fn(&Rect<bool>) -> bool, fn(&Rect<bool>) -> bool) {
    match main_axis {
        FlexDirection::Row => (|m| m.left, |m| m.right),
        FlexDirection::RowReverse => (|m| m.right, |m| m.left),
        FlexDirection::Column => (|m| m.top, |m| m.bottom),
        FlexDirection::ColumnReverse => (|m| m.bottom, |m| m.top),
    }
}
