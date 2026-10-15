//! The Yoga flexbox algorithm, ported from `yoga/algorithm/CalculateLayout.cpp` and `FlexLine.cpp`
//! at Yoga commit `08250431e09bc4b864917e51f9e9d52998ee2dce`.
//!
//! Undefined values are `f32::NAN`, as in Yoga; min/max constraints are `Option<f32>`.

#![allow(dead_code)]

use crate::geometry::{AbsoluteAxis, Line, Rect, Size};
use crate::style::{
    AlignItems, AvailableSpace, BoxSizing, CompactLength, Dimension, Direction, FlexDirection, JustifyContent,
    Overflow, Position,
};
use crate::tree::{LayoutInput, LayoutOutput, LayoutYogaFlexboxContainer, NodeId, RequestedAxis, RunMode, SizingMode};
use crate::util::sys::Vec;
use crate::util::{MaybeMath, MaybeResolve, ResolveOrZero};
use crate::{CoreStyle, FlexboxContainerStyle, FlexboxItemStyle};

use super::axis::{
    bound_axis, bound_axis_within_min_and_max, constrain_max_size_for_mode, dimension, fallback_justify_content,
    flex_end_edge, flex_start_edge, inline_end_edge, inline_start_edge, is_row, resolve_cross_direction,
    resolve_direction, PhysicalEdge,
};
use super::config::YogaConfig;
use super::enums::YogaErrata;
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
    /// Whether the (processed) style size is a percentage, per axis
    pub(crate) size_is_percent: Size<bool>,
    /// Resolved insets for `position: relative` (`None` when auto); horizontal edges resolve against
    /// the container's available inner width, vertical edges against its height
    pub(crate) inset: Rect<Option<f32>>,
    /// Which insets are `auto`
    pub(crate) inset_is_auto: Rect<bool>,
    /// `position: static` (Yoga ignores insets on static children)
    pub(crate) is_static: bool,
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
        size_is_percent: processed.map(|d| d.into_raw().tag() == CompactLength::PERCENT_TAG),
        inset: {
            let inset = style.inset();
            Rect {
                left: inset.left.maybe_resolve(owner_size.width, calc),
                right: inset.right.maybe_resolve(owner_size.width, calc),
                top: inset.top.maybe_resolve(owner_size.height, calc),
                bottom: inset.bottom.maybe_resolve(owner_size.height, calc),
            }
        },
        inset_is_auto: style.inset().map(|i| i.is_auto()),
        is_static: style.position() == Position::Static,
        position: Rect::ZERO,
        baseline: f32::NAN,
        computed_flex_basis: state.computed_flex_basis.unwrap_or(f32::NAN),
        computed_auto_min_main_size: None,
        min_size_is_set: style.min_size().map(|min| !min.is_auto()),
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
    item.baseline = output.baselines.first.unwrap_or(output.size.height);
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

/// Yoga's `flexStartMarginIsAuto`/`flexEndMarginIsAuto` on already-physical margin flags
#[inline]
fn margin_is_auto_at(margin_is_auto: &Rect<bool>, edge: PhysicalEdge) -> bool {
    edge_value(margin_is_auto, edge)
}

impl YogaItem {
    /// Yoga's `boundAxis` for this item
    #[inline]
    pub(crate) fn bound_axis(&self, axis: FlexDirection, value: f32) -> f32 {
        let dim = dimension(axis);
        bound_axis(value, self.min_size.get_abs(dim), self.max_size.get_abs(dim), self.padding_border_for_axis(axis))
    }

    /// Yoga's `boundAxisWithAutoMin`
    #[inline]
    pub(crate) fn bound_axis_with_auto_min(&self, axis: FlexDirection, value: f32) -> f32 {
        let bounded = self.bound_axis(axis, value);
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
                run_mode: RunMode::ComputeSize,
                sizing_mode: SizingMode::InherentSize,
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
        let padding_border =
            style.padding().resolve_or_zero(owner_width, calc) + style.border().resolve_or_zero(owner_width, calc);
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
                .resolve_or_zero(owner_width, |val, basis| tree.resolve_calc_value(val, basis))
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

    let specified_main = item.size.get_abs(main_dim);
    let transferred_main = item.aspect_ratio.and_then(|ratio| {
        let cross_dim = if is_main_axis_row { AbsoluteAxis::Vertical } else { AbsoluteAxis::Horizontal };
        item.size.get_abs(cross_dim).map(|cross| if is_main_axis_row { cross * ratio } else { cross / ratio })
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
    if let Some(max_main) = item.max_size.get_abs(main_dim) {
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
        let child_flex_basis = item.bound_within_min_and_max(main_axis, item.computed_flex_basis);
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
                let bound_main_size = item.bound_axis_with_auto_min(main_axis, base_main_size);
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
                let bound_main_size = item.bound_axis(main_axis, base_main_size);
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
    let available_inner_main = available.get_abs(dimension(main_axis));
    let available_inner_cross = available.get_abs(dimension(cross_axis));
    let sizing_mode_cross = container.sizing_mode.get_abs(dimension(cross_axis));
    let cross_dim = dimension(cross_axis);
    let mut delta_free_space = 0.0;

    for &index in &line.items_in_flow {
        let item = &mut items[index];
        let child_flex_basis = item.bound_within_min_and_max(main_axis, item.computed_flex_basis);
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
                updated_main_size = item.bound_axis_with_auto_min(main_axis, child_size);
            }
        } else if is_defined(line.layout.remaining_free_space) && line.layout.remaining_free_space > 0.0 {
            let flex_grow_factor = item.flex_grow;
            if !flex_grow_factor.is_nan() && flex_grow_factor != 0.0 {
                updated_main_size = item.bound_axis_with_auto_min(
                    main_axis,
                    child_flex_basis
                        + line.layout.remaining_free_space / line.layout.total_flex_grow_factors * flex_grow_factor,
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

        let cross_is_definite = item.has_definite_size(cross_dim);
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
            child_cross_size = item.size.get_abs(cross_dim).unwrap_or(f32::NAN) + margin_cross;
            let is_loose_percentage_measurement =
                item.size_is_percent.get_abs(cross_dim) && sizing_mode_cross != YogaSizingMode::StretchFit;
            child_cross_mode = if is_undefined(child_cross_size) || is_loose_percentage_measurement {
                YogaSizingMode::MaxContent
            } else {
                YogaSizingMode::StretchFit
            };
        }
        let mut child_cross_mode = child_cross_mode;

        constrain_max_size_for_mode(
            item.max_size.get_abs(dimension(main_axis)).map(|max| max + margin_main),
            &mut child_main_mode,
            &mut child_main_size,
        );
        constrain_max_size_for_mode(
            item.max_size.get_abs(cross_dim).map(|max| max + margin_cross),
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
        let _ = available_inner_main;
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
            line.layout.main_dim +=
                item.margin_for_axis(main_axis) + item.bound_within_min_and_max(main_axis, item.computed_flex_basis);
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
