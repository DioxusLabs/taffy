//! A dedicated implementation of the stack layout algorithm (`Display::Stack`), which shares the container-level
//! setup of the flexbox algorithm but sizes and positions its items in its own passes.
use super::*;

/// The intermediate results of a stack calculation for a single item
struct StackItem {
    /// The identifier for the associated node
    node: NodeId,
    /// The order of the node relative to it's siblings
    order: u32,

    /// The size of this item
    size: Size<Option<f32>>,
    /// The raw size style of this item
    size_style: Size<Dimension>,
    /// The minimum allowable size of this item
    min_size: Size<Option<f32>>,
    /// The maximum allowable size of this item
    max_size: Size<Option<f32>>,
    /// The aspect ratio of this item
    aspect_ratio: Option<f32>,
    /// The cross-alignment of this item
    align_self: AlignSelf,

    /// The overflow style of the item
    overflow: Point<Overflow>,
    /// The contain style of the item
    #[cfg_attr(not(feature = "content_size"), allow(dead_code))]
    contain: Contain,
    /// The width of the scrollbars (if it has any)
    scrollbar_width: f32,

    /// The `position: relative` offset of this item
    relative_inset: Size<f32>,
    /// The margin of this item
    margin: Rect<f32>,
    /// Whether each margin is an auto margin or not
    margin_is_auto: Rect<bool>,
    /// The padding of this item
    padding: Rect<f32>,
    /// The border of this item
    border: Rect<f32>,

    /// The main size of the item before it is clamped by its minimum and maximum main sizes
    base_main_size: f32,
    /// Whether `base_main_size` is definite (rather than derived from the item's content)
    main_size_is_definite: bool,
    /// `base_main_size` clamped by the item's minimum and maximum main sizes. For an item with an `fl` main
    /// size this is its minimum main size.
    hypothetical_main_size: f32,
    /// The cross size of the item if it is not stretched
    hypothetical_cross_size: f32,
    /// The used size of the item
    target_size: Size<f32>,

    /// Is the `fl` main size of this item resolved
    frozen: bool,
    /// The amount by which the `fl` main size of this item was clamped
    violation: f32,

    /// The position of the baseline of this item
    baseline: f32,
    /// The main-axis offset of this item from its natural flow position
    offset_main: f32,
    /// The cross-axis offset of this item from its natural flow position
    offset_cross: f32,

    /// Whether the max size must be resolved again once the width of the container is known
    max_size_is_stale: bool,
    /// Out-of-flow candidates bubbled out of this item's subtree
    oof_candidates: OofCandidates,
}

impl StackItem {
    /// The factor of the item's main size if that size is an `fl` value
    #[inline(always)]
    fn fill_factor(&self, dir: FlexDirection) -> Option<f32> {
        let main_size = self.size_style.main(dir);
        main_size.is_fill().then(|| main_size.value())
    }

    /// Whether the used cross size of the item is the cross size of the container's line
    #[inline(always)]
    fn is_stretched(&self, dir: FlexDirection) -> bool {
        let cross_size = self.size_style.cross(dir);
        !self.margin_is_auto.cross_start(dir)
            && !self.margin_is_auto.cross_end(dir)
            && (cross_size.is_stretch_or_fill() || (self.align_self == AlignSelf::STRETCH && cross_size.is_auto()))
    }

    /// Returns true if the item participates in baseline alignment
    #[inline(always)]
    fn participates_in_baseline_alignment(&self, dir: FlexDirection) -> bool {
        self.align_self == AlignSelf::BASELINE
            && !self.margin_is_auto.cross_start(dir)
            && !self.margin_is_auto.cross_end(dir)
    }

    /// Whether the dimensions that the item is laid out with are definite
    #[inline(always)]
    fn known_dimension_definiteness(&self, constants: &AlgoConstants) -> Size<bool> {
        let dir = constants.dir;
        let main_is_definite = constants.has_definite_main_size || self.main_size_is_definite;
        let cross_is_definite = self.is_stretched(dir)
            || self.size.cross(dir).is_some()
            || (!dir.is_row() && constants.cross_axis_available_space_is_definite);
        Size { width: true, height: true }.with_main(dir, main_is_definite).with_cross(dir, cross_is_definite)
    }
}

/// Compute a stack layout: `inputs.known_dimensions` have already had the container's size styles applied
pub(super) fn compute_preliminary(
    tree: &mut impl LayoutFlexboxContainer,
    node: NodeId,
    inputs: LayoutInput,
) -> LayoutOutput {
    let LayoutInput { known_dimensions, parent_size, available_space, run_mode, .. } = inputs;

    let mut constants = compute_constants(
        tree,
        tree.get_flexbox_container_style(node),
        known_dimensions,
        inputs.known_dimensions_are_definite,
        parent_size,
        available_space,
    );
    let dir = constants.dir;
    let available_space = determine_available_space(known_dimensions, available_space, &constants);

    // Generate the items and determine the base and hypothetical main size of each one
    let mut items = generate_items(tree, node, &constants, available_space);

    // Determine the main size of the container
    if let Some(inner_main_size) = constants.node_inner_size.main(dir) {
        let outer_main_size = inner_main_size + constants.content_box_inset.main_axis_sum(dir);
        constants.inner_container_size.set_main(dir, inner_main_size);
        constants.container_size.set_main(dir, outer_main_size);
    } else {
        determine_container_main_size(available_space, &items, &mut constants);
        constants.node_inner_size.set_main(dir, Some(constants.inner_container_size.main(dir)));
        constants.node_outer_size.set_main(dir, Some(constants.container_size.main(dir)));

        // Re-resolve percentage gaps
        let style = tree.get_flexbox_container_style(node);
        let inner_container_size = constants.inner_container_size.main(dir);
        let new_gap = style
            .gap()
            .main(dir)
            .maybe_resolve(inner_container_size, |val, basis| tree.calc(val, basis))
            .unwrap_or(0.0);
        constants.gap.set_main(dir, new_gap);
    }

    // If only the container's main size has been requested then we are done, as nothing below changes it
    if run_mode == RunMode::ComputeSize {
        let main_axis_is_requested_axis = match inputs.axis {
            RequestedAxis::Horizontal => dir.is_row(),
            RequestedAxis::Vertical => dir.is_column(),
            RequestedAxis::Both => false,
        };
        if main_axis_is_requested_axis {
            let size = Size::ZERO.with_main(dir, constants.container_size.main(dir));
            return LayoutOutput::from_outer_size(size);
        }
    }

    // Determine the used main size of each item
    resolve_main_sizes(&mut items, &constants);

    // Determine the cross size of the line and the used cross size of each item
    determine_hypothetical_cross_sizes(tree, &mut items, &constants, known_dimensions, available_space);
    calculate_baselines(tree, known_dimensions, available_space, &mut items, &constants);
    let line_cross_size = calculate_line_cross_size(&items, known_dimensions, &constants);
    for item in items.iter_mut() {
        let cross_size = if item.is_stretched(dir) {
            let max_cross_size = if item.max_size_is_stale {
                // Percentage padding and border resolve against the width of the container, which was not
                // known when the item was generated
                let child_style = tree.get_flexbox_child_style(item.node);
                let inner_width = constants.node_inner_size.width;
                let padding = child_style.padding().resolve_or_zero(inner_width, |val, basis| tree.calc(val, basis));
                let border = child_style.border().resolve_or_zero(inner_width, |val, basis| tree.calc(val, basis));
                child_style
                    .max_size()
                    .maybe_resolve(constants.node_inner_size, |val, basis| tree.calc(val, basis))
                    .maybe_add((padding + border).sum_axes())
                    .cross(dir)
            } else {
                item.max_size.cross(dir)
            };
            (line_cross_size - item.margin.cross_axis_sum(dir))
                .max(0.0)
                .maybe_clamp(item.min_size.cross(dir), max_cross_size)
        } else {
            item.hypothetical_cross_size
        };
        item.target_size.set_cross(dir, cross_size);
    }

    // Align the items in each axis
    distribute_remaining_free_space(&mut items, &constants);
    resolve_cross_axis_alignment(&mut items, line_cross_size, &constants);

    // Determine the container’s used cross size
    let padding_border_sum = constants.content_box_inset.cross_axis_sum(dir);
    let outer_container_size = known_dimensions
        .cross(dir)
        .unwrap_or(line_cross_size + padding_border_sum)
        .maybe_clamp(constants.min_size.cross(dir), constants.max_size.cross(dir))
        .max(padding_border_sum - constants.scrollbar_gutter.cross(dir));
    let inner_container_size = f32_max(outer_container_size - padding_border_sum, 0.0);
    constants.container_size.set_cross(dir, outer_container_size);
    constants.inner_container_size.set_cross(dir, inner_container_size);

    if run_mode == RunMode::ComputeSize {
        return LayoutOutput::from_outer_size(constants.container_size);
    }

    // Align the line per align-content
    let line_offset_cross = {
        let free_space = constants.inner_container_size.cross(dir) - line_cross_size;
        let align_content_mode = apply_alignment_fallback(free_space, 1, constants.align_content);
        compute_alignment_offset(free_space, 1, constants.gap.cross(dir), align_content_mode, false, true)
    };

    // Do a final layout pass and gather the resulting layouts
    let inflow_overflow_rect = final_layout_pass(tree, &mut items, line_cross_size, line_offset_cross, &constants);

    let mut candidates = OofCandidates::new();
    collect_oof_candidates(
        tree,
        node,
        &constants,
        items.iter_mut().map(|item| (item.order, &mut item.oof_candidates)),
        &mut candidates,
    );

    let absolute_position_inset = constants.border
        + Rect {
            left: if constants.layout_direction.is_rtl() { constants.scrollbar_gutter.x } else { 0.0 },
            right: if constants.layout_direction.is_rtl() { 0.0 } else { constants.scrollbar_gutter.x },
            top: 0.0,
            bottom: constants.scrollbar_gutter.y,
        };
    let absolute_position_area = constants.container_size - absolute_position_inset.sum_axes();
    let absolute_position_offset = Point { x: absolute_position_inset.left, y: absolute_position_inset.top };

    let len = tree.child_count(node);
    for order in 0..len {
        let child = tree.get_child_id(node, order);
        if tree.get_flexbox_child_style(child).box_generation_mode() == BoxGenerationMode::None {
            tree.set_unrounded_layout(child, &Layout::with_order(order as u32));
            tree.perform_child_layout(
                child,
                Size::NONE,
                Size::NONE,
                Size::MAX_CONTENT,
                SizingMode::ContentSize,
                Line::FALSE,
            );
        }
    }

    // The container's first baseline, as for a single-line flex container
    let first_vertical_baseline = if constants.is_column {
        let item = if dir.is_reverse() { items.last() } else { items.first() };
        item.map(|child| child.baseline)
    } else {
        items
            .iter()
            .find(|item| item.participates_in_baseline_alignment(dir))
            .or_else(|| items.first())
            .map(|child| child.baseline)
    };

    let mut output = LayoutOutput::from_sizes_and_baselines(
        constants.container_size,
        inflow_overflow_rect,
        Baselines::from_first(first_vertical_baseline),
    );
    output.oof_candidates = candidates;
    output.oof_positioning_area =
        Some(OofPositioningArea { size: absolute_position_area, offset: absolute_position_offset });
    output
}

/// Generate the stack items, and determine the base and hypothetical main size of each one. An item whose
/// main size is neither definite nor `fl` is measured.
#[inline]
fn generate_items(
    tree: &mut impl LayoutFlexboxContainer,
    node: NodeId,
    constants: &AlgoConstants,
    available_space: Size<AvailableSpace>,
) -> Vec<StackItem> {
    let dir = constants.dir;
    let node_inner_size = constants.node_inner_size;
    // Percentage sizes of items resolve against the container's inner size, but only if that size is definite
    let percent_resolution_size =
        if constants.known_main_size_is_definite { node_inner_size } else { node_inner_size.with_main(dir, None) };
    let percent_resolution_main_size = percent_resolution_size.main(dir);

    let mut items: Vec<StackItem> = Vec::new();
    for order in 0..tree.child_count(node) {
        let child = tree.get_child_id(node, order);
        let child_style = tree.get_flexbox_child_style(child);
        if child_style.position().is_out_of_flow() || child_style.box_generation_mode() == BoxGenerationMode::None {
            continue;
        }

        let aspect_ratio = child_style.aspect_ratio();
        let padding = child_style.padding().resolve_or_zero(node_inner_size.width, |val, basis| tree.calc(val, basis));
        let border = child_style.border().resolve_or_zero(node_inner_size.width, |val, basis| tree.calc(val, basis));
        let pb_sum = (padding + border).sum_axes();
        let is_content_box = child_style.box_sizing() == BoxSizing::ContentBox;
        let box_sizing_adjustment = if is_content_box { pb_sum } else { Size::ZERO };
        let mut item = StackItem {
            node: child,
            order: order as u32,
            size: child_style
                .size()
                .maybe_resolve(percent_resolution_size, |val, basis| tree.calc(val, basis))
                .maybe_apply_aspect_ratio(aspect_ratio)
                .maybe_add(box_sizing_adjustment),
            size_style: child_style.size(),
            min_size: child_style
                .min_size()
                .maybe_resolve(percent_resolution_size, |val, basis| tree.calc(val, basis))
                .maybe_add(box_sizing_adjustment),
            max_size: child_style
                .max_size()
                .maybe_resolve(percent_resolution_size, |val, basis| tree.calc(val, basis))
                .maybe_add(box_sizing_adjustment),
            aspect_ratio,
            relative_inset: if child_style.position() == Position::Relative {
                let inset = child_style
                    .inset()
                    .zip_size(node_inner_size, |p, s| p.maybe_resolve(s, |val, basis| tree.calc(val, basis)));
                resolve_relative_inset(inset, constants)
            } else {
                Size::ZERO
            },
            margin: child_style.margin().resolve_or_zero(node_inner_size.width, |val, basis| tree.calc(val, basis)),
            margin_is_auto: child_style.margin().map(LengthPercentageAuto::is_auto),
            padding,
            border,
            align_self: resolve_normal_alignment(child_style.align_self().unwrap_or(constants.align_items))
                .resolve_self_relative(child_style.direction(), constants.layout_direction, constants.is_column),
            overflow: child_style.overflow(),
            contain: child_style.contain(),
            scrollbar_width: child_style.scrollbar_width(),
            base_main_size: 0.0,
            main_size_is_definite: true,
            hypothetical_main_size: 0.0,
            hypothetical_cross_size: 0.0,
            target_size: Size::zero(),
            frozen: false,
            violation: 0.0,
            baseline: 0.0,
            offset_main: 0.0,
            offset_cross: 0.0,
            max_size_is_stale: is_content_box && node_inner_size.width.is_none(),
            oof_candidates: OofCandidates::NONE,
        };
        drop(child_style);

        let transferred_min_size = item.min_size.maybe_apply_aspect_ratio(aspect_ratio);
        let transferred_max_size = item.max_size.maybe_apply_aspect_ratio(aspect_ratio);

        let base_main_size = 'base_main_size: {
            // An item with an `fl` main size has a base size of zero
            if item.size_style.main(dir).is_fill() {
                break 'base_main_size 0.0;
            }
            if let Some(main_size) = item.size.main(dir) {
                break 'base_main_size main_size;
            }

            // A main size that is a sizing keyword either resolves to an exact size or determines the
            // available space constraint the item is measured under
            let main_stretch_size =
                percent_resolution_main_size.maybe_sub(item.margin.main_axis_sum(dir)).maybe_max(0.0);
            let keyword_main_available_space = match resolve_sizing_keyword(
                item.size_style.main(dir),
                main_stretch_size,
                percent_resolution_main_size,
                |val, basis| tree.calc(val, basis),
            ) {
                Some(SizingKeywordResolution::Exact(size)) => break 'base_main_size size,
                Some(SizingKeywordResolution::Measure(available)) => Some(available),
                None => None,
            };

            // Available space and known dimensions for sizing the item (as for a flex item)
            let cross_axis_parent_size = node_inner_size.cross(dir);
            let child_min_cross = transferred_min_size.cross(dir);
            let child_max_cross = transferred_max_size.cross(dir);
            let cross_axis_available_space: AvailableSpace = match available_space.cross(dir) {
                AvailableSpace::Definite(val) => AvailableSpace::Definite(
                    (cross_axis_parent_size.unwrap_or(val) - item.margin.cross_axis_sum(dir))
                        .max(0.0)
                        .maybe_clamp(child_min_cross, child_max_cross),
                ),
                AvailableSpace::MinContent => match child_min_cross {
                    Some(min) => AvailableSpace::Definite(min),
                    None => AvailableSpace::MinContent,
                },
                AvailableSpace::MaxContent => match child_max_cross {
                    Some(max) => AvailableSpace::Definite(max),
                    None => AvailableSpace::MaxContent,
                },
            };
            let mut child_cross_size_is_definite = item.size.cross(dir).is_some();
            let child_known_dimensions = {
                let mut ckd = item.size.with_main(dir, None);
                ckd.set_cross(dir, ckd.cross(dir).maybe_clamp(child_min_cross, child_max_cross));
                if item.align_self == AlignSelf::STRETCH
                    && !item.margin_is_auto.cross_start(dir)
                    && !item.margin_is_auto.cross_end(dir)
                    && ckd.cross(dir).is_none()
                {
                    ckd.set_cross(dir, cross_axis_available_space.into_option());
                    child_cross_size_is_definite =
                        constants.has_definite_cross_size && cross_axis_parent_size.is_some();
                }
                ckd
            };

            // A definite cross size is transferred through the aspect ratio
            if child_cross_size_is_definite {
                if let (Some(ratio), Some(cross)) = (aspect_ratio, child_known_dimensions.cross(dir)) {
                    break 'base_main_size if dir.is_row() { cross * ratio } else { cross / ratio };
                }
            }

            item.main_size_is_definite = false;
            let child_available_space = Size::MAX_CONTENT
                .with_main(
                    dir,
                    keyword_main_available_space.unwrap_or(
                        if available_space.main(dir) == AvailableSpace::MinContent {
                            AvailableSpace::MinContent
                        } else {
                            AvailableSpace::MaxContent
                        },
                    ),
                )
                .with_cross(dir, cross_axis_available_space);
            tree.measure_child_size(
                child,
                child_known_dimensions,
                Size::from_cross(dir, cross_axis_parent_size),
                child_available_space,
                SizingMode::ContentSize,
                dir.main_axis(),
                Line::FALSE,
            )
        };

        let padding_border_main = pb_sum.main(dir);
        item.base_main_size = base_main_size.max(padding.main_axis_sum(dir) + border.main_axis_sum(dir));

        // Items have no automatic minimum size
        let min_main_size =
            item.min_size.main(dir).unwrap_or(0.0).maybe_max(transferred_min_size.main(dir)).max(padding_border_main);
        item.hypothetical_main_size =
            item.base_main_size.maybe_clamp(Some(min_main_size), transferred_max_size.main(dir));

        items.push(item);
    }
    items
}

/// Determine the main size of a container whose main size is not known: its items are not flexed, so it
/// is the sum of their main sizes
#[inline]
fn determine_container_main_size(
    available_space: Size<AvailableSpace>,
    items: &[StackItem],
    constants: &mut AlgoConstants,
) {
    let dir = constants.dir;
    let main_content_box_inset = constants.content_box_inset.main_axis_sum(dir);
    let gap_sum = sum_axis_gaps(constants.gap.main(dir), items.len());

    let items_main_size: f32 = match available_space.main(dir) {
        AvailableSpace::Definite(_) => items
            .iter()
            .map(|item| {
                let padding_border_sum = (item.padding + item.border).main_axis_sum(dir);
                (item.base_main_size.maybe_max(item.min_size.main(dir)) + item.margin.main_axis_sum(dir))
                    .max(padding_border_sum)
            })
            .sum(),
        AvailableSpace::MinContent | AvailableSpace::MaxContent => items
            .iter()
            .map(|item| {
                let margin = item.margin.main_axis_sum(dir);
                let content_contribution = if item.size_style.main(dir).is_fill() {
                    item.hypothetical_main_size + margin
                } else {
                    let basis = if constants.is_row {
                        item.base_main_size.maybe_max(item.size.main(dir))
                    } else {
                        item.base_main_size
                    };
                    basis.maybe_max(item.min_size.main(dir)).max(0.0) + margin
                };
                // This is computed as for a flex item that neither grows nor shrinks, so that the result
                // is exactly the same
                let diff = content_contribution - item.base_main_size;
                if diff < 0.0 {
                    let inner_base_main_size =
                        item.base_main_size - item.padding.main_axis_sum(dir) - item.border.main_axis_sum(dir);
                    if inner_base_main_size == 0.0 {
                        item.base_main_size
                    } else {
                        item.base_main_size + inner_base_main_size * (diff / inner_base_main_size)
                    }
                } else {
                    item.base_main_size + diff
                }
            })
            .sum::<f32>(),
    };
    // The content size is floored at zero when sizing under a min-content or max-content constraint
    let content_main_size = match available_space.main(dir) {
        AvailableSpace::Definite(_) => items_main_size + gap_sum,
        AvailableSpace::MinContent | AvailableSpace::MaxContent => f32_max(items_main_size + gap_sum, 0.0),
    };

    let outer_main_size = (content_main_size + main_content_box_inset)
        .maybe_clamp(constants.min_size.main(dir), constants.max_size.main(dir))
        .max(main_content_box_inset - constants.scrollbar_gutter.main(dir));
    let inner_main_size = f32_max(outer_main_size - main_content_box_inset, 0.0);
    constants.container_size.set_main(dir, outer_main_size);
    constants.inner_container_size.set_main(dir, inner_main_size);
    constants.node_inner_size.set_main(dir, Some(inner_main_size));
}

/// Determine the used main size of each item: items with an `fl` main size share the space that the other
/// items leave free, and those take their hypothetical main size
#[inline]
fn resolve_main_sizes(items: &mut [StackItem], constants: &AlgoConstants) {
    let dir = constants.dir;
    let mut used_space = 0.0;
    let mut fill_items_margin: Option<f32> = None;
    for item in items.iter_mut() {
        if item.size_style.main(dir).is_fill() {
            *fill_items_margin.get_or_insert(0.0) += item.margin.main_axis_sum(dir);
        } else {
            item.target_size.set_main(dir, item.hypothetical_main_size);
            used_space += item.hypothetical_main_size + item.margin.main_axis_sum(dir);
        }
    }
    let Some(fill_items_margin) = fill_items_margin else { return };

    let used_space = sum_axis_gaps(constants.gap.main(dir), items.len()) + used_space + fill_items_margin;
    let mut free_space = constants.node_inner_size.main(dir).unwrap_or(0.0) - used_space;

    // Each iteration freezes at least one item: all of them if none violates its min or max size, else those that
    // violate in the direction of the total violation (as for flexible lengths).
    loop {
        let factor_sum: f32 = items.iter().filter(|item| !item.frozen).filter_map(|item| item.fill_factor(dir)).sum();
        let fill_size = f32_max(free_space, 0.0) / f32_max(factor_sum, 1.0);

        let mut total_violation = 0.0;
        for item in items.iter_mut().filter(|item| !item.frozen) {
            let Some(fill_factor) = item.fill_factor(dir) else { continue };
            let min_size = item.hypothetical_main_size;
            let max_size = item.max_size.main(dir).unwrap_or(f32::INFINITY);
            let unclamped_size = fill_factor * fill_size;
            let size = f32_max(f32_min(unclamped_size, max_size), min_size);
            item.violation = size - unclamped_size;
            item.target_size.set_main(dir, size);
            total_violation += item.violation;
        }

        let mut any_unfrozen = false;
        for item in items.iter_mut().filter(|item| !item.frozen && item.fill_factor(dir).is_some()) {
            item.frozen = match total_violation {
                v if v > 0.0 => item.violation > 0.0,
                v if v < 0.0 => item.violation < 0.0,
                _ => true,
            };
            if item.frozen {
                free_space -= item.target_size.main(dir);
            } else {
                any_unfrozen = true;
            }
        }
        if !any_unfrozen {
            break;
        }
    }
}

/// Determine the cross size that each item has if it is not stretched. An item whose cross size is neither
/// definite nor that of a line of known size is measured.
#[inline]
fn determine_hypothetical_cross_sizes(
    tree: &mut impl LayoutFlexboxContainer,
    items: &mut [StackItem],
    constants: &AlgoConstants,
    node_size: Size<Option<f32>>,
    available_space: Size<AvailableSpace>,
) {
    let dir = constants.dir;
    let line_cross_size_is_known = node_size.cross(dir).is_some();

    for child in items.iter_mut() {
        if line_cross_size_is_known && child.is_stretched(dir) && !child.participates_in_baseline_alignment(dir) {
            continue;
        }

        let padding_border_sum = (child.padding + child.border).cross_axis_sum(dir);
        let transferred_min_cross = child.min_size.maybe_apply_aspect_ratio(child.aspect_ratio).cross(dir);
        let transferred_max_cross = child.max_size.maybe_apply_aspect_ratio(child.aspect_ratio).cross(dir);
        let child_cross = child
            .size
            .cross(dir)
            .maybe_clamp(transferred_min_cross, transferred_max_cross)
            .maybe_max(padding_border_sum);

        child.hypothetical_cross_size = child_cross.unwrap_or_else(|| {
            let child_available_main: AvailableSpace =
                (constants.container_size.main(dir) - child.margin.main_axis_sum(dir)).max(0.0).into();
            let child_available_cross = available_space
                .cross(dir)
                .map_definite_value(|val| (val - child.margin.cross_axis_sum(dir)).max(0.0))
                .maybe_clamp(transferred_min_cross, transferred_max_cross)
                .maybe_max(padding_border_sum);
            let cross_stretch_size =
                constants.node_inner_size.cross(dir).maybe_sub(child.margin.cross_axis_sum(dir)).maybe_max(0.0);
            let child_available_cross = match resolve_sizing_keyword(
                child.size_style.cross(dir),
                cross_stretch_size,
                constants.node_inner_size.cross(dir),
                |val, basis| tree.calc(val, basis),
            ) {
                Some(SizingKeywordResolution::Measure(available)) => available,
                _ => child_available_cross,
            };

            tree.compute_child_layout(
                child.node,
                LayoutInput {
                    run_mode: RunMode::ComputeSize,
                    sizing_mode: SizingMode::ContentSize,
                    axis: dir.cross_axis().into(),
                    known_dimensions: Size::NONE.with_main(dir, Some(child.target_size.main(dir))),
                    known_dimensions_are_definite: child.known_dimension_definiteness(constants),
                    parent_size: constants.node_inner_size,
                    available_space: Size::MAX_CONTENT
                        .with_main(dir, child_available_main)
                        .with_cross(dir, child_available_cross),
                    vertical_margins_are_collapsible: Line::FALSE,
                },
            )
            .size
            .get_abs(dir.cross_axis())
            .maybe_clamp(transferred_min_cross, transferred_max_cross)
            .max(padding_border_sum)
        });
    }
}

/// Calculate the baselines of the items of a row container that participate in baseline alignment
#[inline]
fn calculate_baselines(
    tree: &mut impl LayoutFlexboxContainer,
    node_size: Size<Option<f32>>,
    available_space: Size<AvailableSpace>,
    items: &mut [StackItem],
    constants: &AlgoConstants,
) {
    let dir = constants.dir;
    if !constants.is_row {
        return;
    }
    // Only compute baselines if there is more than one item to align
    if items.iter().filter(|child| child.participates_in_baseline_alignment(dir)).count() <= 1 {
        return;
    }

    for child in items.iter_mut() {
        if !child.participates_in_baseline_alignment(dir) {
            continue;
        }
        let output = tree.compute_child_layout(
            child.node,
            LayoutInput {
                run_mode: RunMode::PerformLayout,
                sizing_mode: SizingMode::ContentSize,
                axis: RequestedAxis::Both,
                known_dimensions: Size {
                    width: Some(child.target_size.width),
                    height: Some(child.hypothetical_cross_size),
                },
                known_dimensions_are_definite: child.known_dimension_definiteness(constants),
                parent_size: constants.node_inner_size,
                available_space: Size {
                    width: constants.container_size.width.into(),
                    height: available_space.height.maybe_set(node_size.height),
                }
                .maybe_sub(child.margin.sum_axes())
                .maybe_max(Size::ZERO),
                vertical_margins_are_collapsible: Line::FALSE,
            },
        );
        let height = output.size.height;
        let baseline = child_first_baseline(output.baselines.first, height, child.overflow.y).unwrap_or(height);
        child.baseline = baseline + child.margin.top;
    }
}

/// Calculate the cross size of the container's single line
#[inline]
fn calculate_line_cross_size(items: &[StackItem], node_size: Size<Option<f32>>, constants: &AlgoConstants) -> f32 {
    let dir = constants.dir;
    let cross_axis_padding_border = constants.content_box_inset.cross_axis_sum(dir);
    let cross_min_size = constants.min_size.cross(dir);
    let cross_max_size = constants.max_size.cross(dir);

    let mut line_cross_size = if node_size.cross(dir).is_some() {
        node_size
            .cross(dir)
            .maybe_clamp(cross_min_size, cross_max_size)
            .maybe_sub(cross_axis_padding_border)
            .maybe_max(0.0)
            .unwrap_or(0.0)
    } else {
        let max_baseline: f32 = items.iter().map(|child| child.baseline).fold(0.0, |acc, x| acc.max(x));
        items
            .iter()
            .map(|child| {
                let outer_cross_size = child.hypothetical_cross_size + child.margin.cross_axis_sum(dir);
                if child.participates_in_baseline_alignment(dir) {
                    max_baseline - child.baseline + outer_cross_size
                } else {
                    outer_cross_size
                }
            })
            .fold(0.0_f32, |acc, x| acc.max(x))
            .maybe_clamp(
                cross_min_size.maybe_sub(cross_axis_padding_border),
                cross_max_size.maybe_sub(cross_axis_padding_border),
            )
    };

    // `align-content: stretch` grows the line to the container's minimum cross size
    if constants.align_content == AlignContent::STRETCH {
        let container_min_inner_cross = node_size
            .cross(dir)
            .or(cross_min_size)
            .maybe_clamp(cross_min_size, cross_max_size)
            .maybe_sub(cross_axis_padding_border)
            .maybe_max(0.0)
            .unwrap_or(0.0);
        if line_cross_size < container_min_inner_cross {
            line_cross_size += container_min_inner_cross - line_cross_size;
        }
    }

    line_cross_size
}

/// Distribute the free space in the main axis: to `auto` margins if there are any, else per `justify-content`
#[inline]
fn distribute_remaining_free_space(items: &mut [StackItem], constants: &AlgoConstants) {
    let dir = constants.dir;
    let gap = constants.gap.main(dir);
    let mut used_space = sum_axis_gaps(gap, items.len());
    let mut num_auto_margins = 0;
    for child in items.iter() {
        used_space += child.target_size.main(dir) + child.margin.main_axis_sum(dir);
        num_auto_margins += child.margin_is_auto.main_start(dir) as u32 + child.margin_is_auto.main_end(dir) as u32;
    }
    let mut free_space = constants.inner_container_size.main(dir) - used_space;

    if free_space > 0.0 && num_auto_margins > 0 {
        let margin = free_space / num_auto_margins as f32;
        for child in items.iter_mut() {
            if child.margin_is_auto.main_start(dir) {
                if constants.is_row {
                    child.margin.left = margin;
                } else {
                    child.margin.top = margin;
                }
            }
            if child.margin_is_auto.main_end(dir) {
                if constants.is_row {
                    child.margin.right = margin;
                } else {
                    child.margin.bottom = margin;
                }
            }
        }
        free_space = 0.0;
    }

    let num_items = items.len();
    let layout_reverse = dir.is_reverse();
    let justify_content_mode = apply_alignment_fallback(free_space, num_items, constants.justify_content);
    let justify_item = |(i, child): (usize, &mut StackItem)| {
        child.offset_main =
            compute_alignment_offset(free_space, num_items, gap, justify_content_mode, layout_reverse, i == 0);
    };
    if layout_reverse {
        items.iter_mut().rev().enumerate().for_each(justify_item);
    } else {
        items.iter_mut().enumerate().for_each(justify_item);
    }
}

/// Resolve the `auto` cross-axis margins of each item, or else align it within the line per `align-self`
#[inline]
fn resolve_cross_axis_alignment(items: &mut [StackItem], line_cross_size: f32, constants: &AlgoConstants) {
    let dir = constants.dir;
    let cross_axis_should_reverse = constants.is_column && constants.layout_direction.is_rtl();
    let max_baseline: f32 = items.iter().map(|child| child.baseline).fold(0.0, |acc, x| acc.max(x));

    for child in items.iter_mut() {
        let free_space = line_cross_size - (child.target_size.cross(dir) + child.margin.cross_axis_sum(dir));
        let start_is_auto = child.margin_is_auto.cross_start(dir);
        let end_is_auto = child.margin_is_auto.cross_end(dir);

        if start_is_auto || end_is_auto {
            let (start, end) = match (start_is_auto, end_is_auto) {
                (true, true) => (Some(free_space / 2.0), Some(free_space / 2.0)),
                (true, false) => (Some(free_space), None),
                _ => (None, Some(free_space)),
            };
            if constants.is_row {
                child.margin.top = start.unwrap_or(child.margin.top);
                child.margin.bottom = end.unwrap_or(child.margin.bottom);
            } else {
                child.margin.left = start.unwrap_or(child.margin.left);
                child.margin.right = end.unwrap_or(child.margin.right);
            }
            continue;
        }

        let align_keyword = if child.align_self.is_safe() && free_space < 0.0 {
            AlignItemsKeyword::Start
        } else {
            child.align_self.keyword
        };
        let align_to_end = match align_keyword {
            AlignItemsKeyword::Start | AlignItemsKeyword::FlexStart | AlignItemsKeyword::Stretch => {
                cross_axis_should_reverse
            }
            AlignItemsKeyword::End | AlignItemsKeyword::FlexEnd => !cross_axis_should_reverse,
            AlignItemsKeyword::Center => {
                child.offset_cross = free_space / 2.0;
                continue;
            }
            AlignItemsKeyword::Baseline => {
                if constants.is_row {
                    child.offset_cross = max_baseline - child.baseline;
                    continue;
                }
                cross_axis_should_reverse
            }
            AlignItemsKeyword::Normal | AlignItemsKeyword::SelfStart | AlignItemsKeyword::SelfEnd => unreachable!(),
        };
        child.offset_cross = if align_to_end { free_space } else { 0.0 };
    }
}

/// Perform the final layout of each item and position it
#[inline]
fn final_layout_pass(
    tree: &mut impl LayoutFlexboxContainer,
    items: &mut [StackItem],
    line_cross_size: f32,
    line_offset_cross: f32,
    constants: &AlgoConstants,
) -> Rect<f32> {
    let dir = constants.dir;
    let container_size = constants.container_size;
    let content_box_inset = constants.content_box_inset;
    let is_rtl_row = dir.is_row() && constants.layout_direction.is_rtl();
    let is_rtl_column = dir.is_column() && constants.layout_direction.is_rtl();

    let mut total_offset_main = if is_rtl_row {
        container_size.width - content_box_inset.main_end(dir)
    } else {
        content_box_inset.main_start(dir)
    };
    let total_offset_cross = if is_rtl_column {
        container_size.width - content_box_inset.cross_end(dir) - (line_offset_cross + line_cross_size)
    } else {
        content_box_inset.cross_start(dir)
    };
    let effective_line_offset_cross = if is_rtl_column { 0.0 } else { line_offset_cross };

    #[cfg_attr(not(feature = "content_size"), allow(unused_mut))]
    let mut overflow_rect = Rect::ZERO;

    let mut layout_item = |item: &mut StackItem| {
        let mut layout_output = tree.compute_child_layout(
            item.node,
            LayoutInput {
                run_mode: RunMode::PerformLayout,
                sizing_mode: SizingMode::ContentSize,
                axis: RequestedAxis::Both,
                known_dimensions: item.target_size.map(|s| s.into()),
                known_dimensions_are_definite: item.known_dimension_definiteness(constants),
                parent_size: constants.node_inner_size,
                available_space: (container_size - item.margin.sum_axes()).map(|s| s.max(0.0).into()),
                vertical_margins_are_collapsible: Line::FALSE,
            },
        );
        let LayoutOutput {
            size,
            #[cfg(feature = "content_size")]
            scrollable_overflow_rect,
            ..
        } = layout_output;

        let main_relative_inset = item.relative_inset.main(dir);
        let offset_main = if is_rtl_row {
            total_offset_main - item.offset_main - item.margin.main_end(dir) - main_relative_inset - size.width
        } else {
            total_offset_main + item.offset_main + item.margin.main_start(dir) + main_relative_inset
        };
        let offset_cross = total_offset_cross
            + item.offset_cross
            + effective_line_offset_cross
            + item.margin.cross_start(dir)
            + item.relative_inset.cross(dir);

        let inner_baseline =
            child_first_baseline(layout_output.baselines.first, size.height, item.overflow.y).unwrap_or(size.height);
        item.baseline = inner_baseline
            + if dir.is_row() {
                total_offset_cross + item.offset_cross + effective_line_offset_cross + item.margin.cross_start(dir)
            } else {
                total_offset_main + item.offset_main + item.margin.main_start(dir)
            };

        let location = if dir.is_row() {
            Point { x: offset_main, y: offset_cross }
        } else {
            Point { x: offset_cross, y: offset_main }
        };
        let scrollbar_size = Size {
            width: if item.overflow.y == Overflow::Scroll { item.scrollbar_width } else { 0.0 },
            height: if item.overflow.x == Overflow::Scroll { item.scrollbar_width } else { 0.0 },
        };

        tree.set_unrounded_layout(
            item.node,
            &Layout {
                order: item.order,
                size,
                #[cfg(feature = "content_size")]
                scrollable_overflow_rect,
                scrollbar_size,
                location,
                padding: item.padding,
                border: item.border,
                margin: item.margin,
            },
        );

        item.oof_candidates = layout_output.oof_candidates.take();
        if !item.oof_candidates.is_empty() {
            item.oof_candidates.translate(location);
        }

        let advance = item.offset_main + item.margin.main_axis_sum(dir) + size.main(dir);
        if is_rtl_row {
            total_offset_main -= advance;
        } else {
            total_offset_main += advance;
        }

        #[cfg(feature = "content_size")]
        {
            let border = constants.border;
            let contribution_location = if constants.layout_direction.is_rtl() {
                Point { x: container_size.width - (location.x + size.width) - border.right, y: location.y - border.top }
            } else {
                Point { x: location.x - border.left, y: location.y - border.top }
            };
            overflow_rect = overflow_rect.union(compute_scrollable_overflow_contribution(
                contribution_location,
                size,
                scrollable_overflow_rect,
                item.overflow,
                item.contain,
                constants.is_scroll_container,
            ));
        }
    };

    if dir.is_reverse() {
        items.iter_mut().rev().for_each(&mut layout_item);
    } else {
        items.iter_mut().for_each(&mut layout_item);
    }

    #[cfg(feature = "content_size")]
    if constants.is_scroll_container {
        overflow_rect.right += if constants.layout_direction.is_rtl() {
            content_box_inset.left - constants.border.left - constants.scrollbar_gutter.x
        } else {
            content_box_inset.right - constants.border.right - constants.scrollbar_gutter.x
        };
        overflow_rect.bottom += content_box_inset.bottom - constants.border.bottom - constants.scrollbar_gutter.y;
    }

    overflow_rect
}
