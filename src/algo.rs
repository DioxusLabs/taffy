#![allow(dead_code)]

use std::f32;

use crate::layout;
use crate::style;

use crate::number::Number::*;
use crate::number::*;

#[derive(Debug, Copy, Clone)]
struct FlexSize {
    width: f32,
    height: f32,
}

impl FlexSize {
    fn main(self, direction: style::FlexDirection) -> f32 {
        match direction {
            style::FlexDirection::Row | style::FlexDirection::RowReverse => self.width,
            style::FlexDirection::Column | style::FlexDirection::ColumnReverse => self.height,
        }
    }

    fn cross(self, direction: style::FlexDirection) -> f32 {
        match direction {
            style::FlexDirection::Row | style::FlexDirection::RowReverse => self.height,
            style::FlexDirection::Column | style::FlexDirection::ColumnReverse => self.width,
        }
    }
}

#[derive(Debug)]
struct ComputeResult {
    size: FlexSize,
    children: Vec<layout::Node>,
}

struct FlexItem<'a> {
    node: &'a style::Node,

    width: Number,
    min_width: Number,
    max_width: Number,

    height: Number,
    min_height: Number,
    max_height: Number,

    main: Number,
    min_main: Number,
    max_main: Number,

    cross: Number,
    min_cross: Number,
    max_cross: Number,

    main_start: Number,
    main_end: Number,
    cross_start: Number,
    cross_end: Number,

    margin_main_start: f32,
    margin_main_end: f32,
    margin_cross_start: f32,
    margin_cross_end: f32,

    padding_main_start: f32,
    padding_main_end: f32,
    padding_cross_start: f32,
    padding_cross_end: f32,

    border_main_start: f32,
    border_main_end: f32,
    border_cross_start: f32,
    border_cross_end: f32,

    // temporary values for main size determination
    flex_basis: f32,
    inner_flex_basis: f32,
    hypothetical_inner_main_size: f32,
    hypothetical_outer_main_size: f32,
    target_main_size: f32,
    outer_target_main_size: f32,
    violation: f32,
    frozen: bool,

    // temporary values for cross size determination
    hypothetical_inner_cross_size: f32,
    hypothetical_outer_cross_size: f32,
    target_cross_size: f32,
    outer_target_cross_size: f32,

    baseline: f32,

    // temporary values for holding offset in the main / cross direction.
    // offset is the relative position from the item's natural flow position based on
    // relative position values, alignment, and justification. Does not unclude margin/padding/border.
    offset_main: f32,
    offset_cross: f32,
}

struct FlexLine<'a> {
    items: Vec<FlexItem<'a>>,
    cross_size: f32,
    offset_cross: f32,
}

pub fn compute(root: &style::Node) -> layout::Node {
    // TODO - Don't do two passes here just to handle min/max.
    // Probably want to pass min/max down as top level paramerer instead.
    let first_pass = compute_internal(
        root,
        root.width.resolve(Undefined),
        root.height.resolve(Undefined),
        Undefined,
        Undefined,
        Undefined,
    );

    let result = compute_internal(
        root,
        first_pass
            .size
            .width
            .maybe_max(root.min_width.resolve(Undefined))
            .maybe_min(root.max_width.resolve(Undefined))
            .to_number(),
        first_pass
            .size
            .height
            .maybe_max(root.min_height.resolve(Undefined))
            .maybe_min(root.max_height.resolve(Undefined))
            .to_number(),
        Undefined,
        Undefined,
        Undefined,
    );

    let mut layout = layout::Node {
        width: result.size.width,
        height: result.size.height,
        x: 0.0,
        y: 0.0,
        children: result.children,
    };

    round_layout(&mut layout, 0.0, 0.0);
    layout
}

fn round_layout(layout: &mut layout::Node, abs_x: f32, abs_y: f32) {
    let abs_x = abs_x + layout.x;
    let abs_y = abs_y + layout.y;

    layout.x = layout.x.round();
    layout.y = layout.y.round();
    layout.width = (abs_x + layout.width).round() - abs_x.round();
    layout.height = (abs_y + layout.height).round() - abs_y.round();
    layout.children.iter_mut().for_each(|child| round_layout(child, abs_x, abs_y));
}

fn compute_internal(
    node: &style::Node,

    // The width and the height of this node, if Some
    // the node should be layed out at exactly this size.
    // If None the node should at most be the size of
    // available_width / available_height if present.
    node_width: Number,
    node_height: Number,

    // This available width and height. This is the
    // the inner node_width / node_height of the parent and should
    // not be exceeded by this node unless node_width / node_height
    // parameters say otherwise.
    parent_inner_width: Number,
    parent_inner_height: Number,

    // This will always be the parent width except for the case when
    // the child is in an absolute context. This value should only
    // be used for calculated percentage values.
    percent_calc_base: Number,
) -> ComputeResult {
    // Define some general constants we will need for the remainder
    // of the algorithm.

    let is_row = node.flex_direction.is_row();
    let is_column = node.flex_direction.is_column();
    let is_wrap_reverse = node.flex_wrap == style::FlexWrap::WrapReverse;

    let parent_inner_main = if is_row { parent_inner_width } else { parent_inner_height };
    let parent_inner_cross = if is_row { parent_inner_height } else { parent_inner_width };

    let (margin_main_start, margin_cross_start, margin_main_end, margin_cross_end) = (
        node.main_margin_start(node.flex_direction).resolve(percent_calc_base).or_else(0.0),
        node.cross_margin_start(node.flex_direction).resolve(percent_calc_base).or_else(0.0),
        node.main_margin_end(node.flex_direction).resolve(percent_calc_base).or_else(0.0),
        node.cross_margin_end(node.flex_direction).resolve(percent_calc_base).or_else(0.0),
    );

    let margin_main = margin_main_start + margin_main_end;
    let margin_cross = margin_cross_start + margin_cross_end;

    let (padding_main_start, padding_cross_start, padding_main_end, padding_cross_end) = (
        node.main_padding_start(node.flex_direction).resolve(percent_calc_base).or_else(0.0),
        node.cross_padding_start(node.flex_direction).resolve(percent_calc_base).or_else(0.0),
        node.main_padding_end(node.flex_direction).resolve(percent_calc_base).or_else(0.0),
        node.cross_padding_end(node.flex_direction).resolve(percent_calc_base).or_else(0.0),
    );

    let padding_main = padding_main_start + padding_main_end;
    let padding_cross = padding_cross_start + padding_cross_end;

    let (border_main_start, border_cross_start, border_main_end, border_cross_end) = (
        node.main_border_start(node.flex_direction).resolve(percent_calc_base).or_else(0.0),
        node.cross_border_start(node.flex_direction).resolve(percent_calc_base).or_else(0.0),
        node.main_border_end(node.flex_direction).resolve(percent_calc_base).or_else(0.0),
        node.cross_border_end(node.flex_direction).resolve(percent_calc_base).or_else(0.0),
    );

    let border_main = border_main_start + border_main_end;
    let border_cross = border_cross_start + border_cross_end;

    let node_main = if is_row { node_width } else { node_height };
    let node_cross = if is_row { node_height } else { node_width };

    let node_inner_main = node_main - border_main - padding_main;
    let node_inner_cross = node_cross - border_cross - padding_cross;

    let percent_calc_base_child = if is_row { node_inner_main } else { node_inner_cross };

    // 9.2. Line Length Determination

    // 1. Generate anonymous flex items as described in §4 Flex Items.

    let mut flex_items: Vec<FlexItem> = node
        .children
        .iter()
        .filter(|child| child.position != style::Position::Absolute)
        .map(|child| FlexItem {
            node: child,

            width: child.width.resolve(percent_calc_base_child),
            min_width: child.min_width.resolve(percent_calc_base_child),
            max_width: child.max_width.resolve(percent_calc_base_child),

            height: child.height.resolve(percent_calc_base_child),
            min_height: child.min_height.resolve(percent_calc_base_child),
            max_height: child.max_height.resolve(percent_calc_base_child),

            main: child.main_size(node.flex_direction).resolve(percent_calc_base_child),
            min_main: child.min_main_size(node.flex_direction).resolve(percent_calc_base_child),
            max_main: child.max_main_size(node.flex_direction).resolve(percent_calc_base_child),

            cross: child.cross_size(node.flex_direction).resolve(percent_calc_base_child),
            min_cross: child.min_cross_size(node.flex_direction).resolve(percent_calc_base_child),
            max_cross: child.max_cross_size(node.flex_direction).resolve(percent_calc_base_child),

            main_start: child.main_start(node.flex_direction).resolve(percent_calc_base_child),
            main_end: child.main_end(node.flex_direction).resolve(percent_calc_base_child),
            cross_start: child.cross_start(node.flex_direction).resolve(percent_calc_base_child),
            cross_end: child.cross_end(node.flex_direction).resolve(percent_calc_base_child),

            margin_main_start: child
                .main_margin_start(node.flex_direction)
                .resolve(percent_calc_base_child)
                .or_else(0.0),

            margin_main_end: child.main_margin_end(node.flex_direction).resolve(percent_calc_base_child).or_else(0.0),

            margin_cross_start: child
                .cross_margin_start(node.flex_direction)
                .resolve(percent_calc_base_child)
                .or_else(0.0),

            margin_cross_end: child.cross_margin_end(node.flex_direction).resolve(percent_calc_base_child).or_else(0.0),

            padding_main_start: child
                .main_padding_start(node.flex_direction)
                .resolve(percent_calc_base_child)
                .or_else(0.0),

            padding_main_end: child.main_padding_end(node.flex_direction).resolve(percent_calc_base_child).or_else(0.0),

            padding_cross_start: child
                .cross_padding_start(node.flex_direction)
                .resolve(percent_calc_base_child)
                .or_else(0.0),

            padding_cross_end: child
                .cross_padding_end(node.flex_direction)
                .resolve(percent_calc_base_child)
                .or_else(0.0),

            border_main_start: child
                .main_border_start(node.flex_direction)
                .resolve(percent_calc_base_child)
                .or_else(0.0),

            border_main_end: child.main_border_end(node.flex_direction).resolve(percent_calc_base_child).or_else(0.0),

            border_cross_start: child
                .cross_border_start(node.flex_direction)
                .resolve(percent_calc_base_child)
                .or_else(0.0),

            border_cross_end: child.cross_border_end(node.flex_direction).resolve(percent_calc_base_child).or_else(0.0),

            flex_basis: 0.0,
            inner_flex_basis: 0.0,
            hypothetical_inner_main_size: 0.0,
            hypothetical_outer_main_size: 0.0,
            target_main_size: 0.0,
            outer_target_main_size: 0.0,
            violation: 0.0,
            frozen: false,

            hypothetical_inner_cross_size: 0.0,
            hypothetical_outer_cross_size: 0.0,
            target_cross_size: 0.0,
            outer_target_cross_size: 0.0,

            baseline: 0.0,

            offset_main: 0.0,
            offset_cross: 0.0,
        })
        .collect();

    // 2. Determine the available main and cross space for the flex items.
    //    For each dimension, if that dimension of the flex container’s content box
    //    is a definite size, use that; if that dimension of the flex container is
    //    being sized under a min or max-content constraint, the available space in
    //    that dimension is that constraint; otherwise, subtract the flex container’s
    //    margin, border, and padding from the space available to the flex container
    //    in that dimension and use that value. This might result in an infinite value.

    let available_main = node_main.or_else(parent_inner_main - margin_main) - padding_main - border_main;
    let available_cross = node_cross.or_else(parent_inner_cross - margin_cross) - padding_cross - border_cross;

    // TODO - this does not follow spec. See commented out code below
    // 3. Determine the flex base size and hypothetical main size of each item:
    flex_items.iter_mut().for_each(|child| {
        // A. If the item has a definite used flex basis, that’s the flex base size.

        let flex_basis = child.node.flex_basis.resolve(percent_calc_base_child);
        if flex_basis.is_defined() {
            child.flex_basis = flex_basis.or_else(0.0);
            return;
        };

        // B. If the flex item has an intrinsic aspect ratio,
        //    a used flex basis of content, and a definite cross size,
        //    then the flex base size is calculated from its inner
        //    cross size and the flex item’s intrinsic aspect ratio.

        if let Defined(ratio) = child.node.aspect_ratio {
            if node_cross.is_defined() && child.node.flex_basis == style::Dimension::Auto {
                child.flex_basis = (node_cross * ratio).or_else(0.0);
                return;
            }
        }

        // C. If the used flex basis is content or depends on its available space,
        //    and the flex container is being sized under a min-content or max-content
        //    constraint (e.g. when performing automatic table layout [CSS21]),
        //    size the item under that constraint. The flex base size is the item’s
        //    resulting main size.

        // TODO - Probably need to cover this case in future

        // D. Otherwise, if the used flex basis is content or depends on its
        //    available space, the available main size is infinite, and the flex item’s
        //    inline axis is parallel to the main axis, lay the item out using the rules
        //    for a box in an orthogonal flow [CSS3-WRITING-MODES]. The flex base size
        //    is the item’s max-content main size.

        // TODO - Probably need to cover this case in future

        // E. Otherwise, size the item into the available space using its used flex basis
        //    in place of its main size, treating a value of content as max-content.
        //    If a cross size is needed to determine the main size (e.g. when the
        //    flex item’s main size is in its block axis) and the flex item’s cross size
        //    is auto and not definite, in this calculation use fit-content as the
        //    flex item’s cross size. The flex base size is the item’s resulting main size.

        child.flex_basis = compute_internal(
            child.node,
            child.width.maybe_max(child.min_width).maybe_min(child.max_width),
            child.height.maybe_max(child.min_height).maybe_min(child.max_height),
            if is_row { available_main } else { available_cross },
            if is_row { available_cross } else { available_main },
            percent_calc_base_child,
        )
        .size
        .main(node.flex_direction)
        .maybe_max(child.min_main)
        .maybe_min(child.max_main);
    });

    // The hypothetical main size is the item’s flex base size clamped according to its
    // used min and max main sizes (and flooring the content box size at zero).

    flex_items.iter_mut().for_each(|child| {
        child.inner_flex_basis = child.flex_basis
            - (child.padding_main_start + child.padding_main_end + child.border_main_start + child.border_main_end);

        // TODO - not really spec abiding but needs to be done somewhere. probably somewhere else though.
        // The following logic was developed not from the spec but by trail and error looking into how
        // webkit handled various scenarios. Can probably be solved better by passing in
        // min-content max-content constraints fromt the top
        let min_main = if is_row {
            compute_internal(
                child.node,
                Undefined,
                Undefined,
                if is_row { available_main } else { available_cross },
                if is_row { available_cross } else { available_main },
                percent_calc_base_child,
            )
            .size
            .width
            .maybe_min(child.width)
            .maybe_max(child.min_width)
            .to_number()
        } else {
            child.min_main
        };

        child.hypothetical_inner_main_size = child.flex_basis.maybe_max(min_main).maybe_min(child.max_main);

        child.hypothetical_outer_main_size =
            child.hypothetical_inner_main_size + child.margin_main_start + child.margin_main_end;
    });

    // 9.3. Main Size Determination

    // 5. Collect flex items into flex lines:
    //    - If the flex container is single-line, collect all the flex items into
    //      a single flex line.
    //    - Otherwise, starting from the first uncollected item, collect consecutive
    //      items one by one until the first time that the next collected item would
    //      not fit into the flex container’s inner main size (or until a forced break
    //      is encountered, see §10 Fragmenting Flex Layout). If the very first
    //      uncollected item wouldn’t fit, collect just it into the line.
    //
    //      For this step, the size of a flex item is its outer hypothetical main size. (Note: This can be negative.)
    //      Repeat until all flex items have been collected into flex lines
    //
    //      Note that the "collect as many" line will collect zero-sized flex items onto
    //      the end of the previous line even if the last non-zero item exactly "filled up" the line.

    let mut flex_lines = {
        let mut lines: Vec<FlexLine> = vec![];
        let mut line_length = 0.0;

        if node.flex_wrap == style::FlexWrap::NoWrap {
            lines.push(FlexLine { items: flex_items, cross_size: 0.0, offset_cross: 0.0 });
        } else {
            let mut line = FlexLine { items: vec![], cross_size: 0.0, offset_cross: 0.0 };

            for child in flex_items {
                line_length += child.hypothetical_outer_main_size;

                if let Defined(main) = available_main {
                    if line_length > main && !line.items.is_empty() {
                        line_length = child.hypothetical_outer_main_size;
                        lines.push(line);
                        line = FlexLine { items: vec![], cross_size: 0.0, offset_cross: 0.0 };
                    }
                }

                line.items.push(child);
            }

            lines.push(line);
        }

        lines
    };

    // 6. Resolve the flexible lengths of all the flex items to find their used main size.
    //    See §9.7 Resolving Flexible Lengths.
    //
    // 9.7. Resolving Flexible Lengths

    flex_lines.iter_mut().for_each(|line| {
        // 1. Determine the used flex factor. Sum the outer hypothetical main sizes of all
        //    items on the line. If the sum is less than the flex container’s inner main size,
        //    use the flex grow factor for the rest of this algorithm; otherwise, use the
        //    flex shrink factor.

        let used_flex_factor: f32 = line.items.iter().map(|child| child.hypothetical_outer_main_size).sum();
        let growing = used_flex_factor < node_inner_main.or_else(0.0);
        let shrinking = !growing;

        // 2. Size inflexible items. Freeze, setting its target main size to its hypothetical main size
        //    - Any item that has a flex factor of zero
        //    - If using the flex grow factor: any item that has a flex base size
        //      greater than its hypothetical main size
        //    - If using the flex shrink factor: any item that has a flex base size
        //      smaller than its hypothetical main size

        line.items.iter_mut().for_each(|child| {
            // TODO - This is not found by reading the spec. Maybe this can be done in some other place
            // instead. This was found by trail and error fixing tests to align with webkit output.
            if node_inner_main.is_undefined() && is_row {
                child.target_main_size = compute_internal(
                    child.node,
                    child.width.maybe_max(child.min_width).maybe_min(child.max_width),
                    child.height.maybe_max(child.min_height).maybe_min(child.max_height),
                    if is_row { available_main } else { available_cross },
                    if is_row { available_cross } else { available_main },
                    percent_calc_base_child,
                )
                .size
                .main(node.flex_direction)
                .maybe_max(child.min_main)
                .maybe_min(child.max_main);
            } else {
                child.target_main_size = child.hypothetical_inner_main_size;
            }

            // TODO this should really only be set inside the if-statement below but
            // that casues the target_main_size to never be set for some items

            child.outer_target_main_size = child.target_main_size + child.margin_main_start + child.margin_main_end;

            if (child.node.flex_grow == 0.0 && child.node.flex_shrink == 0.0)
                || (growing && child.flex_basis > child.hypothetical_inner_main_size)
                || (shrinking && child.flex_basis < child.hypothetical_inner_main_size)
            {
                child.frozen = true;
            }
        });

        // 3. Calculate initial free space. Sum the outer sizes of all items on the line,
        //    and subtract this from the flex container’s inner main size. For frozen items,
        //    use their outer target main size; for other items, use their outer flex base size.

        let used_space: f32 = line
            .items
            .iter()
            .map(|child| {
                child.margin_main_start
                    + child.margin_main_end
                    + if child.frozen { child.target_main_size } else { child.flex_basis }
            })
            .sum();

        let initial_free_space = (node_inner_main - used_space).or_else(0.0);

        // 4. Loop

        loop {
            // a. Check for flexible items. If all the flex items on the line are frozen,
            //    free space has been distributed; exit this loop.

            let mut frozen: Vec<&mut FlexItem> = vec![];
            let mut unfrozen: Vec<&mut FlexItem> = vec![];

            line.items.iter_mut().for_each(|child| {
                if child.frozen {
                    frozen.push(child);
                } else {
                    unfrozen.push(child);
                }
            });

            if unfrozen.is_empty() {
                break;
            }

            // b. Calculate the remaining free space as for initial free space, above.
            //    If the sum of the unfrozen flex items’ flex factors is less than one,
            //    multiply the initial free space by this sum. If the magnitude of this
            //    value is less than the magnitude of the remaining free space, use this
            //    as the remaining free space.

            let used_space: f32 = Iterator::chain(frozen.iter(), unfrozen.iter())
                .map(|child| {
                    child.margin_main_start
                        + child.margin_main_end
                        + if child.frozen { child.target_main_size } else { child.flex_basis }
                })
                .sum();

            let sum_flex_grow: f32 = unfrozen.iter().map(|item| item.node.flex_grow).sum();
            let sum_flex_shrink: f32 = unfrozen.iter().map(|item| item.node.flex_shrink).sum();

            let free_space = if growing && sum_flex_grow < 1.0 {
                (initial_free_space * sum_flex_grow).maybe_min(node_inner_main - used_space)
            } else if shrinking && sum_flex_shrink < 1.0 {
                (initial_free_space * sum_flex_shrink).maybe_max(node_inner_main - used_space)
            } else {
                (node_inner_main - used_space).or_else(0.0)
            };

            // c. Distribute free space proportional to the flex factors.
            //    - If the remaining free space is zero
            //        Do Nothing
            //    - If using the flex grow factor
            //        Find the ratio of the item’s flex grow factor to the sum of the
            //        flex grow factors of all unfrozen items on the line. Set the item’s
            //        target main size to its flex base size plus a fraction of the remaining
            //        free space proportional to the ratio.
            //    - If using the flex shrink factor
            //        For every unfrozen item on the line, multiply its flex shrink factor by
            //        its inner flex base size, and note this as its scaled flex shrink factor.
            //        Find the ratio of the item’s scaled flex shrink factor to the sum of the
            //        scaled flex shrink factors of all unfrozen items on the line. Set the item’s
            //        target main size to its flex base size minus a fraction of the absolute value
            //        of the remaining free space proportional to the ratio. Note this may result
            //        in a negative inner main size; it will be corrected in the next step.
            //    - Otherwise
            //        Do Nothing

            if free_space.is_normal() {
                if growing && sum_flex_grow > 0.0 {
                    unfrozen.iter_mut().for_each(|child| {
                        child.target_main_size = child.flex_basis + free_space * (child.node.flex_grow / sum_flex_grow);
                    });
                } else if shrinking && sum_flex_shrink > 0.0 {
                    let sum_scaled_shrink_factor: f32 =
                        unfrozen.iter().map(|child| child.inner_flex_basis * child.node.flex_shrink).sum();

                    if sum_scaled_shrink_factor > 0.0 {
                        unfrozen.iter_mut().for_each(|child| {
                            let scaled_shrink_factor = child.inner_flex_basis * child.node.flex_shrink;
                            child.target_main_size =
                                child.flex_basis + free_space * (scaled_shrink_factor / sum_scaled_shrink_factor);
                        });
                    }
                }
            }

            // d. Fix min/max violations. Clamp each non-frozen item’s target main size by its
            //    used min and max main sizes and floor its content-box size at zero. If the
            //    item’s target main size was made smaller by this, it’s a max violation.
            //    If the item’s target main size was made larger by this, it’s a min violation.

            let total_violation = unfrozen.iter_mut().fold(0.0, |acc, child| {
                // TODO - not really spec abiding but needs to be done somewhere. probably somewhere else though.
                // The following logic was developed not from the spec but by trail and error looking into how
                // webkit handled various scenarios. Can probably be solved better by passing in
                // min-content max-content constraints fromt the top
                let min_main = if is_row {
                    compute_internal(
                        child.node,
                        Undefined,
                        Undefined,
                        if is_row { available_main } else { available_cross },
                        if is_row { available_cross } else { available_main },
                        percent_calc_base_child,
                    )
                    .size
                    .width
                    .maybe_min(child.width)
                    .maybe_max(child.min_width)
                    .to_number()
                } else {
                    child.min_main
                };

                let max_main = child.max_main;
                let clamped = child.target_main_size.maybe_min(max_main).maybe_max(min_main).max(0.0);
                child.violation = clamped - child.target_main_size;
                child.target_main_size = clamped;
                child.outer_target_main_size = child.target_main_size + child.margin_main_start + child.margin_main_end;

                acc + child.violation
            });

            // e. Freeze over-flexed items. The total violation is the sum of the adjustments
            //    from the previous step ∑(clamped size - unclamped size). If the total violation is:
            //    - Zero
            //        Freeze all items.
            //    - Positive
            //        Freeze all the items with min violations.
            //    - Negative
            //        Freeze all the items with max violations.

            unfrozen.iter_mut().for_each(|child| match total_violation {
                v if v > 0.0 => child.frozen = child.violation > 0.0,
                v if v < 0.0 => child.frozen = child.violation < 0.0,
                _ => child.frozen = true,
            })

            // f. Return to the start of this loop.
        }
    });

    // Not part of the spec from what i can see but seems correct
    let container_main_size = node_main.or_else({
        let longest_line = flex_lines.iter().fold(f32::MIN, |acc, line| {
            let length: f32 = line.items.iter().map(|item| item.outer_target_main_size).sum();
            acc.max(length)
        });

        let size = longest_line + padding_main + border_main;
        match available_main {
            Defined(val) if flex_lines.len() > 1 && size < val => val,
            _ => size,
        }
    });

    let inner_container_main_size = container_main_size - padding_main - border_main;

    // 9.4. Cross Size Determination

    // 7. Determine the hypothetical cross size of each item by performing layout with the
    //    used main size and the available space, treating auto as fit-content.

    flex_lines.iter_mut().for_each(|line| {
        line.items.iter_mut().for_each(|child| {
            let child_cross = child.cross.maybe_max(child.min_cross).maybe_min(child.max_cross);

            child.hypothetical_inner_cross_size = compute_internal(
                child.node,
                if is_row { child.target_main_size.to_number() } else { child_cross },
                if is_row { child_cross } else { child.target_main_size.to_number() },
                if is_row { container_main_size.to_number() } else { available_cross },
                if is_row { available_cross } else { container_main_size.to_number() },
                percent_calc_base_child,
            )
            .size
            .cross(node.flex_direction)
            .maybe_max(child.min_cross)
            .maybe_min(child.max_cross);

            child.hypothetical_outer_cross_size =
                child.hypothetical_inner_cross_size + child.margin_cross_start + child.margin_cross_end;
        });
    });

    // TODO - probably should move this somewhere else as it doesn't make a ton of sense here but we need it below
    // TODO - This is expensive and should only be done if we really require a baseline. aka, make it lazy

    fn calc_baseline(layout: &layout::Node) -> f32 {
        if layout.children.is_empty() {
            layout.height
        } else {
            calc_baseline(&layout.children[0])
        }
    };

    flex_lines.iter_mut().for_each(|line| {
        line.items.iter_mut().for_each(|child| {
            let result = compute_internal(
                child.node,
                if is_row {
                    child.target_main_size.to_number()
                } else {
                    child.hypothetical_inner_cross_size.to_number()
                },
                if is_row {
                    child.hypothetical_inner_cross_size.to_number()
                } else {
                    child.target_main_size.to_number()
                },
                if is_row { container_main_size.to_number() } else { node_cross },
                if is_row { node_cross } else { container_main_size.to_number() },
                percent_calc_base_child,
            );
            child.baseline = calc_baseline(&layout::Node {
                width: result.size.width,
                height: result.size.height,
                x: 0.0,
                y: 0.0,
                children: result.children,
            });
        });
    });

    // 8. Calculate the cross size of each flex line.
    //    If the flex container is single-line and has a definite cross size, the cross size
    //    of the flex line is the flex container’s inner cross size. Otherwise, for each flex line:
    //
    //    If the flex container is single-line, then clamp the line’s cross-size to be within
    //    the container’s computed min and max cross sizes. Note that if CSS 2.1’s definition
    //    of min/max-width/height applied more generally, this behavior would fall out automatically.

    if flex_lines.len() == 1 && node_cross.is_defined() {
        flex_lines[0].cross_size = (node_cross - padding_cross - border_cross).or_else(0.0);
    } else {
        flex_lines.iter_mut().for_each(|line| {
            //    1. Collect all the flex items whose inline-axis is parallel to the main-axis, whose
            //       align-self is baseline, and whose cross-axis margins are both non-auto. Find the
            //       largest of the distances between each item’s baseline and its hypothetical outer
            //       cross-start edge, and the largest of the distances between each item’s baseline
            //       and its hypothetical outer cross-end edge, and sum these two values.

            //    2. Among all the items not collected by the previous step, find the largest
            //       outer hypothetical cross size.

            //    3. The used cross-size of the flex line is the largest of the numbers found in the
            //       previous two steps and zero.

            let max_baseline: f32 = line.items.iter().map(|child| child.baseline).fold(0.0, |acc, x| acc.max(x));
            line.cross_size = line
                .items
                .iter()
                .map(|child| {
                    if child.node.align_self(node) == style::AlignSelf::Baseline
                        && child.node.cross_margin_start(node.flex_direction) != style::Dimension::Auto
                        && child.node.cross_margin_end(node.flex_direction) != style::Dimension::Auto
                        && child.node.cross_size(node.flex_direction) == style::Dimension::Auto
                    {
                        max_baseline - child.baseline + child.hypothetical_outer_cross_size
                    } else {
                        child.hypothetical_outer_cross_size
                    }
                })
                .fold(0.0, |acc, x| acc.max(x));
        });
    }

    // 9. Handle 'align-content: stretch'. If the flex container has a definite cross size,
    //    align-content is stretch, and the sum of the flex lines' cross sizes is less than
    //    the flex container’s inner cross size, increase the cross size of each flex line
    //    by equal amounts such that the sum of their cross sizes exactly equals the
    //    flex container’s inner cross size.

    if node.align_content == style::AlignContent::Stretch && node_cross.is_defined() {
        let total_cross: f32 = flex_lines.iter().map(|line| line.cross_size).sum();
        let inner_cross = (node_cross - padding_cross - border_cross).or_else(0.0);

        if total_cross < inner_cross {
            let remaining = inner_cross - total_cross;
            let addition = remaining / flex_lines.len() as f32;
            flex_lines.iter_mut().for_each(|line| line.cross_size += addition);
        }
    }

    // 10. Collapse visibility:collapse items. If any flex items have visibility: collapse,
    //     note the cross size of the line they’re in as the item’s strut size, and restart
    //     layout from the beginning.
    //
    //     In this second layout round, when collecting items into lines, treat the collapsed
    //     items as having zero main size. For the rest of the algorithm following that step,
    //     ignore the collapsed items entirely (as if they were display:none) except that after
    //     calculating the cross size of the lines, if any line’s cross size is less than the
    //     largest strut size among all the collapsed items in the line, set its cross size to
    //     that strut size.
    //
    //     Skip this step in the second layout round.

    // TODO implement once (if ever) we support visibility:collapse

    // 11. Determine the used cross size of each flex item. If a flex item has align-self: stretch,
    //     its computed cross size property is auto, and neither of its cross-axis margins are auto,
    //     the used outer cross size is the used cross size of its flex line, clamped according to
    //     the item’s used min and max cross sizes. Otherwise, the used cross size is the item’s
    //     hypothetical cross size.
    //
    //     If the flex item has align-self: stretch, redo layout for its contents, treating this
    //     used size as its definite cross size so that percentage-sized children can be resolved.
    //
    //     Note that this step does not affect the main size of the flex item, even if it has an
    //     intrinsic aspect ratio.

    flex_lines.iter_mut().for_each(|line| {
        let line_cross_size = line.cross_size;

        line.items.iter_mut().for_each(|child| {
            let is_stretch = child.node.align_self(node) == style::AlignSelf::Stretch;

            child.target_cross_size = if is_stretch
                && child.node.cross_margin_start(node.flex_direction) != style::Dimension::Auto
                && child.node.cross_margin_end(node.flex_direction) != style::Dimension::Auto
                && child.node.cross_size(node.flex_direction) == style::Dimension::Auto
            {
                (line_cross_size - child.margin_cross_start - child.margin_cross_end)
                    .maybe_max(child.min_cross)
                    .maybe_min(child.max_cross)
            } else {
                child.hypothetical_inner_cross_size
            };

            child.outer_target_cross_size = child.target_cross_size + child.margin_cross_start + child.margin_cross_end;
        });
    });

    // 9.5. Main-Axis Alignment

    // 12. Distribute any remaining free space. For each flex line:
    //     1. If the remaining free space is positive and at least one main-axis margin on this
    //        line is auto, distribute the free space equally among these margins. Otherwise,
    //        set all auto margins to zero.
    //     2. Align the items along the main-axis per justify-content.

    flex_lines.iter_mut().for_each(|line| {
        let used_space: f32 = line.items.iter().map(|child| child.outer_target_main_size).sum();
        let free_space = inner_container_main_size - used_space;
        let mut num_auto_margins = 0;

        line.items.iter_mut().for_each(|child| {
            if child.node.main_margin_start(node.flex_direction) == style::Dimension::Auto {
                num_auto_margins += 1;
            }
            if child.node.main_margin_end(node.flex_direction) == style::Dimension::Auto {
                num_auto_margins += 1;
            }
        });

        if free_space > 0.0 && num_auto_margins > 0 {
            let margin = free_space / num_auto_margins as f32;

            line.items.iter_mut().for_each(|child| {
                if child.node.main_margin_start(node.flex_direction) == style::Dimension::Auto {
                    child.margin_main_start = margin;
                }
                if child.node.main_margin_end(node.flex_direction) == style::Dimension::Auto {
                    child.margin_main_end = margin;
                }
            });
        } else {
            let num_items = line.items.len();
            let layout_reverse = node.flex_direction.is_reverse();

            let justify_item = |(i, child): (usize, &mut FlexItem)| {
                let is_first = i == 0;

                child.offset_main = match node.justify_content {
                    style::JustifyContent::FlexStart => {
                        if layout_reverse && is_first {
                            free_space
                        } else {
                            0.0
                        }
                    }
                    style::JustifyContent::Center => {
                        if is_first {
                            free_space / 2.0
                        } else {
                            0.0
                        }
                    }
                    style::JustifyContent::FlexEnd => {
                        if is_first && !layout_reverse {
                            free_space
                        } else {
                            0.0
                        }
                    }
                    style::JustifyContent::SpaceBetween => {
                        if is_first {
                            0.0
                        } else {
                            free_space / (num_items - 1) as f32
                        }
                    }
                    style::JustifyContent::SpaceAround => {
                        if is_first {
                            (free_space / num_items as f32) / 2.0
                        } else {
                            free_space / num_items as f32
                        }
                    }
                    style::JustifyContent::SpaceEvenly => free_space / (num_items + 1) as f32,
                };
            };

            if layout_reverse {
                line.items.iter_mut().rev().enumerate().for_each(justify_item);
            } else {
                line.items.iter_mut().enumerate().for_each(justify_item);
            }
        }
    });

    // 9.6. Cross-Axis Alignment

    // 13. Resolve cross-axis auto margins. If a flex item has auto cross-axis margins:
    //     - If its outer cross size (treating those auto margins as zero) is less than the
    //       cross size of its flex line, distribute the difference in those sizes equally
    //       to the auto margins.
    //     - Otherwise, if the block-start or inline-start margin (whichever is in the cross axis)
    //       is auto, set it to zero. Set the opposite margin so that the outer cross size of the
    //       item equals the cross size of its flex line.

    flex_lines.iter_mut().for_each(|line| {
        let line_cross_size = line.cross_size;
        let max_baseline: f32 = line.items.iter_mut().map(|child| child.baseline).fold(0.0, |acc, x| acc.max(x));

        line.items.iter_mut().for_each(|child| {
            let free_space = line_cross_size - child.outer_target_cross_size;

            if child.node.cross_margin_start(node.flex_direction) == style::Dimension::Auto
                && child.node.cross_margin_end(node.flex_direction) == style::Dimension::Auto
            {
                child.margin_cross_start = free_space / 2.0;
                child.margin_cross_end = free_space / 2.0;
            } else if child.node.cross_margin_start(node.flex_direction) == style::Dimension::Auto {
                child.margin_cross_start = free_space;
            } else if child.node.cross_margin_end(node.flex_direction) == style::Dimension::Auto {
                child.margin_cross_end = free_space;
            } else {
                // 14. Align all flex items along the cross-axis per align-self, if neither of the item’s
                //     cross-axis margins are auto.

                child.offset_cross = match child.node.align_self(node) {
                    style::AlignSelf::Auto => 0.0, // Should never happen
                    style::AlignSelf::FlexStart => {
                        if is_wrap_reverse {
                            free_space
                        } else {
                            0.0
                        }
                    }
                    style::AlignSelf::FlexEnd => {
                        if is_wrap_reverse {
                            0.0
                        } else {
                            free_space
                        }
                    }
                    style::AlignSelf::Center => free_space / 2.0,
                    style::AlignSelf::Baseline => {
                        if is_row {
                            max_baseline - child.baseline
                        } else {
                            // basline alignment only makes sense if the direction is row
                            // we treat it as flex-start alignment in columns.
                            if is_wrap_reverse {
                                free_space
                            } else {
                                0.0
                            }
                        }
                    }
                    style::AlignSelf::Stretch => {
                        if is_wrap_reverse {
                            free_space
                        } else {
                            0.0
                        }
                    }
                };
            }
        });
    });

    // 15. Determine the flex container’s used cross size:
    //     - If the cross size property is a definite size, use that, clamped by the used
    //       min and max cross sizes of the flex container.
    //     - Otherwise, use the sum of the flex lines' cross sizes, clamped by the used
    //       min and max cross sizes of the flex container.

    let total_cross_size: f32 = flex_lines.iter().map(|line| line.cross_size).sum();
    let container_cross_size = node_cross.or_else(total_cross_size + padding_cross + border_cross);
    let inner_container_cross_size = container_cross_size - padding_cross - border_cross;

    // 16. Align all flex lines per align-content.

    let free_space = inner_container_cross_size - total_cross_size;
    let num_lines = flex_lines.len();

    let align_line = |(i, line): (usize, &mut FlexLine)| {
        let is_first = i == 0;

        line.offset_cross = match node.align_content {
            style::AlignContent::FlexStart => {
                if is_first && is_wrap_reverse {
                    free_space
                } else {
                    0.0
                }
            }
            style::AlignContent::FlexEnd => {
                if is_first && !is_wrap_reverse {
                    free_space
                } else {
                    0.0
                }
            }
            style::AlignContent::Center => {
                if is_first {
                    free_space / 2.0
                } else {
                    0.0
                }
            }
            style::AlignContent::Stretch => 0.0,
            style::AlignContent::SpaceBetween => {
                if is_first {
                    0.0
                } else {
                    free_space / (num_lines - 1) as f32
                }
            }
            style::AlignContent::SpaceAround => {
                if is_first {
                    (free_space / num_lines as f32) / 2.0
                } else {
                    free_space / num_lines as f32
                }
            }
        };
    };

    if is_wrap_reverse {
        flex_lines.iter_mut().rev().enumerate().for_each(align_line);
    } else {
        flex_lines.iter_mut().enumerate().for_each(align_line);
    }

    let container_width = if is_row { container_main_size } else { container_cross_size };
    let container_height = if is_column { container_main_size } else { container_cross_size };

    // Do a final layout pass and gather the resulting layouts
    let mut children: Vec<layout::Node> = {
        let mut lines: Vec<Vec<layout::Node>> = vec![];
        let mut total_offset_cross = padding_cross_start + border_cross_start;

        {
            let layout_line = |line: &mut FlexLine| {
                let mut children: Vec<layout::Node> = vec![];
                let mut total_offset_main = padding_main_start + border_main_start;
                let line_offset_cross = line.offset_cross;

                {
                    let layout_item = |child: &mut FlexItem| {
                        let result = compute_internal(
                            child.node,
                            if is_row {
                                child.target_main_size.to_number()
                            } else {
                                child.target_cross_size.to_number()
                            },
                            if is_row {
                                child.target_cross_size.to_number()
                            } else {
                                child.target_main_size.to_number()
                            },
                            container_width.to_number(),
                            container_height.to_number(),
                            percent_calc_base_child,
                        );

                        let offset_main = total_offset_main
                            + child.offset_main
                            + child.margin_main_start
                            + (child.main_start.or_else(0.0) - child.main_end.or_else(0.0));

                        let offset_cross = total_offset_cross
                            + child.offset_cross
                            + line_offset_cross
                            + child.margin_cross_start
                            + (child.cross_start.or_else(0.0) - child.cross_end.or_else(0.0));

                        children.push(layout::Node {
                            width: result.size.width,
                            height: result.size.height,
                            x: if is_row { offset_main } else { offset_cross },
                            y: if is_column { offset_main } else { offset_cross },
                            children: result.children,
                        });

                        total_offset_main += child.offset_main
                            + child.margin_main_start
                            + result.size.main(node.flex_direction)
                            + child.margin_main_end;
                    };

                    if node.flex_direction.is_reverse() {
                        line.items.iter_mut().rev().for_each(layout_item);
                    } else {
                        line.items.iter_mut().for_each(layout_item);
                    }
                }

                total_offset_cross += line_offset_cross + line.cross_size;

                if node.flex_direction.is_reverse() {
                    children.reverse();
                }

                lines.push(children);
            };

            if is_wrap_reverse {
                flex_lines.iter_mut().rev().for_each(layout_line);
            } else {
                flex_lines.iter_mut().for_each(layout_line);
            }
        }

        if is_wrap_reverse {
            lines.into_iter().rev().flat_map(|x| x).collect()
        } else {
            lines.into_iter().flat_map(|x| x).collect()
        }
    };

    // Before returning we perform absolute layout on all absolutely positioned children
    let mut absolute_children: Vec<layout::Node> = node
        .children
        .iter()
        .filter(|child| child.position == style::Position::Absolute)
        .map(|child| {
            let container_width = container_width.to_number();
            let container_height = container_height.to_number();

            let start = child.start.resolve(container_width) + child.margin.start.resolve(container_width);
            let end = child.end.resolve(container_width) + child.margin.end.resolve(container_width);
            let top = child.top.resolve(container_height) + child.margin.top.resolve(container_height);
            let bottom = child.bottom.resolve(container_height) + child.margin.bottom.resolve(container_height);

            let (start_main, end_main) = if is_row { (start, end) } else { (top, bottom) };
            let (start_cross, end_cross) = if is_row { (top, bottom) } else { (start, end) };

            let child_width = child
                .width
                .resolve(container_width)
                .maybe_max(child.min_width.resolve(container_width))
                .maybe_min(child.max_width.resolve(container_width));

            let child_height = child
                .height
                .resolve(container_height)
                .maybe_max(child.min_height.resolve(container_height))
                .maybe_min(child.max_height.resolve(container_height));

            let width = child_width.or_else(container_width - start - end);
            let height = child_height.or_else(container_height - top - bottom);

            let result = compute_internal(child, width, height, container_width, container_height, container_width);

            let free_main_space = container_main_size
                - result
                    .size
                    .main(node.flex_direction)
                    .maybe_max(child.min_main_size(node.flex_direction).resolve(percent_calc_base_child))
                    .maybe_min(child.max_main_size(node.flex_direction).resolve(percent_calc_base_child));

            let free_cross_space = container_cross_size
                - result
                    .size
                    .cross(node.flex_direction)
                    .maybe_max(child.min_cross_size(node.flex_direction).resolve(percent_calc_base_child))
                    .maybe_min(child.max_cross_size(node.flex_direction).resolve(percent_calc_base_child));

            let offset_main = if start_main.is_defined() {
                start_main.or_else(0.0) + border_main_start
            } else if end_main.is_defined() {
                free_main_space - end_main.or_else(0.0) - border_main_end
            } else {
                match node.justify_content {
                    style::JustifyContent::SpaceBetween | style::JustifyContent::FlexStart => {
                        padding_main_start + border_main_start
                    }
                    style::JustifyContent::FlexEnd => free_main_space - padding_main_end - border_main_end,
                    style::JustifyContent::SpaceEvenly
                    | style::JustifyContent::SpaceAround
                    | style::JustifyContent::Center => free_main_space / 2.0,
                }
            };

            let offset_cross = if start_cross.is_defined() {
                start_cross.or_else(0.0) + border_cross_start
            } else if end_cross.is_defined() {
                free_cross_space - end_cross.or_else(0.0) - border_cross_end
            } else {
                match child.align_self(node) {
                    style::AlignSelf::Auto => 0.0, // Should never happen
                    style::AlignSelf::FlexStart => {
                        if is_wrap_reverse {
                            free_cross_space - padding_cross_end - border_cross_end
                        } else {
                            padding_cross_start + border_cross_start
                        }
                    }
                    style::AlignSelf::FlexEnd => {
                        if is_wrap_reverse {
                            padding_cross_start + border_cross_start
                        } else {
                            free_cross_space - padding_cross_end - border_cross_end
                        }
                    }
                    style::AlignSelf::Center => free_cross_space / 2.0,
                    style::AlignSelf::Baseline => free_cross_space / 2.0, // Treat as center for now until we have baseline support
                    style::AlignSelf::Stretch => {
                        if is_wrap_reverse {
                            free_cross_space - padding_cross_end - border_cross_end
                        } else {
                            padding_cross_start + border_cross_start
                        }
                    }
                }
            };

            layout::Node {
                width: result.size.width,
                height: result.size.height,
                x: if is_row { offset_main } else { offset_cross },
                y: if is_column { offset_main } else { offset_cross },
                children: result.children,
            }
        })
        .collect();

    // TODO - This adds all the absolute children to the end of the list instead of adding them in
    // the child order they were defined. This has to be fixed.
    children.append(&mut absolute_children);

    ComputeResult { size: FlexSize { width: container_width, height: container_height }, children }
}
