use std::f32;

use layout;
use style;

#[derive(Debug, Copy, Clone)]
struct FlexSize {
    width: f32,
    height: f32,
}

impl FlexSize {
    fn main(&self, direction: style::FlexDirection) -> f32 {
        match direction {
            style::FlexDirection::Row | style::FlexDirection::RowReverse => self.width,
            style::FlexDirection::Column | style::FlexDirection::ColumnReverse => self.height,
        }
    }

    fn cross(&self, direction: style::FlexDirection) -> f32 {
        match direction {
            style::FlexDirection::Row | style::FlexDirection::RowReverse => self.height,
            style::FlexDirection::Column | style::FlexDirection::ColumnReverse => self.width,
        }
    }
}

#[derive(Debug, Copy, Clone)]
struct SizeConstraint {
    min: f32,
    max: f32,
}

impl SizeConstraint {
    fn exactly(size: f32) -> SizeConstraint {
        SizeConstraint {
            min: size,
            max: size,
        }
    }

    fn undefined() -> SizeConstraint {
        SizeConstraint {
            min: f32::NAN,
            max: f32::NAN,
        }
    }

    fn at_least(size: f32) -> SizeConstraint {
        SizeConstraint {
            min: size,
            max: f32::NAN,
        }
    }

    fn at_most(size: f32) -> SizeConstraint {
        SizeConstraint {
            min: f32::NAN,
            max: size,
        }
    }

    fn between(min: f32, max: f32) -> SizeConstraint {
        SizeConstraint { min: min, max: max }
    }
}

struct FlexItem<'a> {
    node: &'a style::Node,

    // temporary values for main size determination
    flex_basis: f32,
    inner_flex_basis: f32,
    hypothetical_inner_main_size: f32,
    hypothetical_outer_main_size: f32,
    target_main_size: f32,
    violation: f32,
    frozen: bool,

    // temporary values for cross size determination
    hypothetical_inner_cross_size: f32,
    hypothetical_outer_cross_size: f32,
    target_cross_size: f32,

    main_margin_start: f32,
    main_margin_end: f32,
    cross_margin_start: f32,
    cross_margin_end: f32,

    // temporary values for holding offset in the main / cross direction.
    // offset is the relative position from the item's natural flow position based on
    // relative position values, alignment, and justification. Does not unclude margin/padding/border.
    offset_main: f32,
    offset_cross: f32,
}

struct ComputeResult {
    size: FlexSize,
    children: Vec<layout::Node>,
}

struct FlexLine<'a> {
    items: Vec<FlexItem<'a>>,
    cross_size: f32,
    offset_cross: f32,
}

pub fn compute(root: &style::Node) -> layout::Node {
    let result = compute_internal(
        root,
        SizeConstraint::undefined(),
        SizeConstraint::undefined(),
    );

    round_layout(&layout::Node {
        width: result.size.width,
        height: result.size.height,
        x: 0.0,
        y: 0.0,
        children: result.children,
    }, 0.0, 0.0)
}

fn round_layout(layout: &layout::Node, abs_x: f32, abs_y: f32) -> layout::Node {
    let abs_x = abs_x + layout.x;
    let abs_y = abs_y + layout.y;

    let rounded_x = layout.x.round();
    let rounded_y = layout.y.round();

    let rounded_width = (abs_x + layout.width).round() - abs_x.round();
    let rounded_height = (abs_y + layout.height).round() - abs_y.round();

    let rounded_children = layout.children.iter().map(|child| {
        round_layout(child, abs_x, abs_y)
    }).collect();

    layout::Node {
        width: rounded_width,
        height: rounded_height,
        x: rounded_x,
        y: rounded_y,
        children: rounded_children,
    }
}

fn compute_internal(
    node: &style::Node,
    width_constraint: SizeConstraint,
    height_constraint: SizeConstraint,
) -> ComputeResult {
    // Define some general constants we will need for the remainder
    // of the algorithm.

    let (main_constraint, cross_constraint) = if node.flex_direction.is_row() {
        (width_constraint, height_constraint)
    } else {
        (height_constraint, width_constraint)
    };

    let (intrinsic_main_size, intrinsic_cross_size) = (
        node.main_size(node.flex_direction)
            .resolve(main_constraint.max, f32::NAN),
        node.cross_size(node.flex_direction)
            .resolve(cross_constraint.max, f32::NAN),
    );

    let (margin_main_start, margin_cross_start) = (
        node.main_margin_start(node.flex_direction)
            .resolve(main_constraint.max, 0.0),
        node.cross_margin_start(node.flex_direction)
            .resolve(main_constraint.max, 0.0),
    );

    let (margin_main_end, margin_cross_end) = (
        node.main_margin_end(node.flex_direction)
            .resolve(main_constraint.max, 0.0),
        node.cross_margin_end(node.flex_direction)
            .resolve(main_constraint.max, 0.0),
    );

    let (margin_main, margin_cross) = (
        margin_main_start + margin_main_end,
        margin_cross_start + margin_cross_end,
    );

    let (padding_main_start, padding_cross_start) = (
        node.main_padding_start(node.flex_direction)
            .resolve(main_constraint.max, 0.0),
        node.cross_padding_start(node.flex_direction)
            .resolve(main_constraint.max, 0.0),
    );

    let (padding_main_end, padding_cross_end) = (
        node.main_padding_end(node.flex_direction)
            .resolve(main_constraint.max, 0.0),
        node.cross_padding_end(node.flex_direction)
            .resolve(main_constraint.max, 0.0),
    );

    let (padding_main, padding_cross) = (
        padding_main_start + padding_main_end,
        padding_cross_start + padding_cross_end,
    );

    let (border_main_start, border_cross_start) = (
        node.main_border_start(node.flex_direction)
            .resolve(main_constraint.max, 0.0),
        node.cross_border_start(node.flex_direction)
            .resolve(main_constraint.max, 0.0),
    );

    let (border_main_end, border_cross_end) = (
        node.main_border_end(node.flex_direction)
            .resolve(main_constraint.max, 0.0),
        node.cross_border_end(node.flex_direction)
            .resolve(main_constraint.max, 0.0),
    );

    let (border_main, border_cross) = (
        border_main_start + border_main_end,
        border_cross_start + border_cross_end,
    );

    let (inner_intrinsic_main_size, inner_intrinsic_cross_size) = (
        intrinsic_main_size - padding_main - border_main,
        intrinsic_cross_size - padding_cross - border_cross,
    );

    // 9.2. Line Length Determination

    // 1. Generate anonymous flex items as described in §4 Flex Items.

    let mut flex_items: Vec<FlexItem> = node
        .children
        .iter()
        .map(|child| FlexItem {
            node: child,

            flex_basis: 0.0,
            inner_flex_basis: 0.0,
            hypothetical_inner_main_size: 0.0,
            hypothetical_outer_main_size: 0.0,
            target_main_size: 0.0,
            violation: 0.0,
            frozen: false,

            hypothetical_inner_cross_size: 0.0,
            hypothetical_outer_cross_size: 0.0,
            target_cross_size: 0.0,

            main_margin_start: 0.0,
            main_margin_end: 0.0,
            cross_margin_start: 0.0,
            cross_margin_end: 0.0,

            offset_main: 0.0,
            offset_cross: 0.0,
        }).collect();

    // 2. Determine the available main and cross space for the flex items.
    //    For each dimension, if that dimension of the flex container’s content box
    //    is a definite size, use that; if that dimension of the flex container is
    //    being sized under a min or max-content constraint, the available space in
    //    that dimension is that constraint; otherwise, subtract the flex container’s
    //    margin, border, and padding from the space available to the flex container
    //    in that dimension and use that value. This might result in an infinite value.

    let avaliable_main = node
        .main_size(node.flex_direction)
        .resolve(main_constraint.max, main_constraint.max - margin_main)
        - padding_main
        - border_main;

    let avaliable_cross = node
        .cross_size(node.flex_direction)
        .resolve(cross_constraint.max, cross_constraint.max - margin_cross)
        - padding_cross
        - border_cross;

    // 3. Determine the flex base size and hypothetical main size of each item:
    for child in &mut flex_items {
        // A. If the item has a definite used flex basis, that’s the flex base size.

        let flex_basis = child.node.flex_basis.resolve(avaliable_main, f32::NAN);
        if flex_basis.is_finite() {
            child.flex_basis = flex_basis;
            continue;
        }

        // B. If the flex item has an intrinsic aspect ratio,
        //    a used flex basis of content, and a definite cross size,
        //    then the flex base size is calculated from its inner
        //    cross size and the flex item’s intrinsic aspect ratio.

        if let Some(ratio) = child.node.aspect_ratio {
            let cross_size = child
                .node
                .cross_size(node.flex_direction)
                .resolve(avaliable_cross, f32::NAN);

            if intrinsic_cross_size.is_finite() && child.node.flex_basis == style::Dimension::Auto {
                child.flex_basis = ratio * cross_size;
                continue;
            }
        }

        // C. If the used flex basis is content or depends on its available space,
        //    and the flex container is being sized under a min-content or max-content
        //    constraint (e.g. when performing automatic table layout [CSS21]),
        //    size the item under that constraint. The flex base size is the item’s
        //    resulting main size.

        if main_constraint.min.is_finite() || main_constraint.max.is_finite() {
            let size = compute_internal(
                child.node,
                if node.flex_direction.is_row() {
                    main_constraint
                } else {
                    SizeConstraint::undefined()
                },
                if node.flex_direction.is_column() {
                    main_constraint
                } else {
                    SizeConstraint::undefined()
                },
            ).size;
            child.flex_basis = size.main(node.flex_direction);
            continue;
        }

        // D. Otherwise, if the used flex basis is content or depends on its
        //    available space, the available main size is infinite, and the flex item’s
        //    inline axis is parallel to the main axis, lay the item out using the rules
        //    for a box in an orthogonal flow [CSS3-WRITING-MODES]. The flex base size
        //    is the item’s max-content main size.

        if avaliable_main.is_nan() {
            let size = compute_internal(
                child.node,
                SizeConstraint::undefined(),
                SizeConstraint::undefined(),
            ).size;
            child.flex_basis = size.main(node.flex_direction);
            continue;
        }

        // E. Otherwise, size the item into the available space using its used flex basis
        //    in place of its main size, treating a value of content as max-content.
        //    If a cross size is needed to determine the main size (e.g. when the
        //    flex item’s main size is in its block axis) and the flex item’s cross size
        //    is auto and not definite, in this calculation use fit-content as the
        //    flex item’s cross size. The flex base size is the item’s resulting main size.

        let size = compute_internal(
            child.node,
            if node.flex_direction.is_row() {
                SizeConstraint::at_most(avaliable_main)
            } else {
                SizeConstraint::at_most(avaliable_cross)
            },
            if node.flex_direction.is_column() {
                SizeConstraint::at_most(avaliable_main)
            } else {
                SizeConstraint::at_most(avaliable_cross)
            },
        ).size;
        child.flex_basis = size.main(node.flex_direction);
    }

    // The hypothetical main size is the item’s flex base size clamped according to its
    // used min and max main sizes (and flooring the content box size at zero).

    for child in &mut flex_items {
        let padding_start = child
            .node
            .main_padding_start(node.flex_direction)
            .resolve(avaliable_main, 0.0);
        let padding_end = child
            .node
            .main_padding_end(node.flex_direction)
            .resolve(avaliable_main, 0.0);
        let border_start = child
            .node
            .main_border_start(node.flex_direction)
            .resolve(avaliable_main, 0.0);
        let border_end = child
            .node
            .main_border_end(node.flex_direction)
            .resolve(avaliable_main, 0.0);
        child.inner_flex_basis =
            child.flex_basis - (padding_start + padding_end + border_start + border_end);

        child.hypothetical_inner_main_size = child
            .flex_basis
            .max(
                child
                    .node
                    .min_main_size(node.flex_direction)
                    .resolve(avaliable_main, f32::MIN),
            ).min(
                child
                    .node
                    .max_main_size(node.flex_direction)
                    .resolve(avaliable_main, f32::MAX),
            ).max(0.0);

        child.hypothetical_outer_main_size = child.hypothetical_inner_main_size
            + child
                .node
                .main_margin_start(node.flex_direction)
                .resolve(avaliable_main, 0.0)
            + child
                .node
                .main_margin_end(node.flex_direction)
                .resolve(avaliable_main, 0.0);
    }

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

        if node.wrap == style::Wrap::NoWrap {
            lines.push(FlexLine {
                items: flex_items,
                cross_size: 0.0,
                offset_cross: 0.0,
            });
        } else {
            let mut line = FlexLine {
                items: vec![],
                cross_size: 0.0,
                offset_cross: 0.0,
            };

            for child in flex_items {
                line_length += child.hypothetical_outer_main_size;

                if line_length > avaliable_main && line.items.len() > 0 {
                    line_length = child.hypothetical_outer_main_size;
                    lines.push(line);
                    line = FlexLine {
                        items: vec![],
                        cross_size: 0.0,
                        offset_cross: 0.0,
                    };
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

    for line in &mut flex_lines {
        // 1. Determine the used flex factor. Sum the outer hypothetical main sizes of all
        //    items on the line. If the sum is less than the flex container’s inner main size,
        //    use the flex grow factor for the rest of this algorithm; otherwise, use the
        //    flex shrink factor.

        let used_flex_factor: f32 = line
            .items
            .iter()
            .map(|child| child.hypothetical_outer_main_size)
            .sum();
        let growing = used_flex_factor < avaliable_main;
        let shrinking = !growing;

        // 2. Size inflexible items. Freeze, setting its target main size to its hypothetical main size
        //    - Any item that has a flex factor of zero
        //    - If using the flex grow factor: any item that has a flex base size
        //      greater than its hypothetical main size
        //    - If using the flex shrink factor: any item that has a flex base size
        //      smaller than its hypothetical main size

        for child in line.items.iter_mut() {
            child.target_main_size = child.hypothetical_inner_main_size;

            if (child.node.flex_grow == 0.0 && child.node.flex_shrink == 0.0)
                || (growing && child.flex_basis > child.hypothetical_inner_main_size)
                || (shrinking && child.flex_basis < child.hypothetical_inner_main_size)
            {
                child.frozen = true;
            }
        }

        // 3. Calculate initial free space. Sum the outer sizes of all items on the line,
        //    and subtract this from the flex container’s inner main size. For frozen items,
        //    use their outer target main size; for other items, use their outer flex base size.

        let used_space: f32 = line
            .items
            .iter()
            .map(|child| {
                child
                    .node
                    .main_margin_start(node.flex_direction)
                    .resolve(avaliable_main, 0.0)
                    + child
                        .node
                        .main_margin_end(node.flex_direction)
                        .resolve(avaliable_main, 0.0)
                    + if child.frozen {
                        child.target_main_size
                    } else {
                        child.flex_basis
                    }
            }).sum();

        let initial_free_space = avaliable_main - used_space;

        // 4. Loop

        loop {
            // a. Check for flexible items. If all the flex items on the line are frozen,
            //    free space has been distributed; exit this loop.

            let mut frozen: Vec<&mut FlexItem> = vec![];
            let mut unfrozen: Vec<&mut FlexItem> = vec![];

            for child in line.items.iter_mut() {
                if child.frozen {
                    frozen.push(child);
                } else {
                    unfrozen.push(child);
                }
            }

            if unfrozen.len() == 0 {
                break;
            }

            // b. Calculate the remaining free space as for initial free space, above.
            //    If the sum of the unfrozen flex items’ flex factors is less than one,
            //    multiply the initial free space by this sum. If the magnitude of this
            //    value is less than the magnitude of the remaining free space, use this
            //    as the remaining free space.

            let used_space: f32 = Iterator::chain(frozen.iter(), unfrozen.iter())
                .map(|child| {
                    child
                        .node
                        .main_margin_start(node.flex_direction)
                        .resolve(avaliable_main, 0.0)
                        + child
                            .node
                            .main_margin_end(node.flex_direction)
                            .resolve(avaliable_main, 0.0)
                        + if child.frozen {
                            child.target_main_size
                        } else {
                            child.flex_basis
                        }
                }).sum();

            let sum_flex_grow: f32 = unfrozen.iter().map(|item| item.node.flex_grow).sum();
            let sum_flex_shrink: f32 = unfrozen.iter().map(|item| item.node.flex_shrink).sum();

            let free_space = if growing && sum_flex_grow < 1.0 {
                (initial_free_space * sum_flex_grow).min(avaliable_main - used_space)
            } else if shrinking && sum_flex_shrink < 1.0 {
                (initial_free_space * sum_flex_shrink).max(avaliable_main - used_space)
            } else {
                avaliable_main - used_space
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

            if free_space > 0.0 && growing && sum_flex_grow > 0.0 {
                for child in &mut unfrozen {
                    child.target_main_size =
                        child.flex_basis + free_space * (child.node.flex_grow / sum_flex_grow);
                }
            } else if free_space < 0.0 && shrinking && sum_flex_shrink > 0.0 {
                let sum_scaled_shrink_factor: f32 = unfrozen
                    .iter()
                    .map(|child| child.inner_flex_basis * child.node.flex_shrink)
                    .sum();

                for child in &mut unfrozen {
                    let scaled_shrink_factor = child.inner_flex_basis * child.node.flex_shrink;
                    child.target_main_size = child.flex_basis
                        + free_space * (scaled_shrink_factor / sum_scaled_shrink_factor);

                    println!("child.flex_basis: {}, scaled_shrink_factor: {}, sum_scaled_shrink_factor: {}, child.inner_flex_basis: {}", child.flex_basis, scaled_shrink_factor, sum_scaled_shrink_factor, child.inner_flex_basis);
                }
            }

            // d. Fix min/max violations. Clamp each non-frozen item’s target main size by its
            //    used min and max main sizes and floor its content-box size at zero. If the
            //    item’s target main size was made smaller by this, it’s a max violation.
            //    If the item’s target main size was made larger by this, it’s a min violation.

            let mut total_violation = 0.0;
            for child in &mut unfrozen {
                let max = child
                    .node
                    .max_main_size(node.flex_direction)
                    .resolve(avaliable_main, f32::MAX);
                let min = child
                    .node
                    .min_main_size(node.flex_direction)
                    .resolve(avaliable_main, f32::MIN)
                    .max(0.0);

                let clamped = if child.target_main_size > max {
                    max
                } else if child.target_main_size < min {
                    min
                } else {
                    child.target_main_size
                };

                child.violation = clamped - child.target_main_size;
                total_violation += child.violation;
                child.target_main_size = clamped;
            }

            // e. Freeze over-flexed items. The total violation is the sum of the adjustments
            //    from the previous step ∑(clamped size - unclamped size). If the total violation is:
            //    - Zero
            //        Freeze all items.
            //    - Positive
            //        Freeze all the items with min violations.
            //    - Negative
            //        Freeze all the items with max violations.

            if total_violation > 0.0 {
                for child in &mut unfrozen {
                    child.frozen = child.violation > 0.0;
                }
            } else if total_violation < 0.0 {
                for child in &mut unfrozen {
                    child.frozen = child.violation < 0.0;
                }
            } else {
                for child in &mut unfrozen {
                    child.frozen = true;
                }
            }

            // f. Return to the start of this loop.
        }
    }

    // Not part of the spec from what i can see but seems correct
    let container_main_size = if intrinsic_main_size.is_finite() {
        intrinsic_main_size
            .min(main_constraint.max)
            .max(main_constraint.min)
    } else {
        let mut longest_line = f32::MIN;

        for line in &flex_lines {
            let length: f32 = line.items.iter().map(|item| item.target_main_size).sum();

            longest_line = if length > longest_line {
                length
            } else {
                longest_line
            };
        }

        (longest_line + padding_main + border_main)
            .min(main_constraint.max)
            .max(main_constraint.min)
    };

    // 9.4. Cross Size Determination

    // 7. Determine the hypothetical cross size of each item by performing layout with the
    //    used main size and the available space, treating auto as fit-content.

    for line in &mut flex_lines {
        for child in &mut line.items {
            let size = compute_internal(
                child.node,
                if node.flex_direction.is_row() {
                    SizeConstraint::exactly(child.target_main_size)
                } else {
                    SizeConstraint::between(cross_constraint.min, avaliable_cross)
                },
                if node.flex_direction.is_column() {
                    SizeConstraint::exactly(child.target_main_size)
                } else {
                    SizeConstraint::between(cross_constraint.min, avaliable_cross)
                },
            ).size;

            child.hypothetical_inner_cross_size = size
                .cross(node.flex_direction)
                .max(
                    child
                        .node
                        .min_cross_size(node.flex_direction)
                        .resolve(avaliable_cross, f32::MIN),
                ).min(
                    child
                        .node
                        .max_cross_size(node.flex_direction)
                        .resolve(avaliable_cross, f32::MAX),
                ).max(0.0);

            child.hypothetical_outer_cross_size = child.hypothetical_inner_cross_size
                + child
                    .node
                    .cross_margin_start(node.flex_direction)
                    .resolve(avaliable_cross, 0.0)
                + child
                    .node
                    .cross_margin_end(node.flex_direction)
                    .resolve(avaliable_cross, 0.0);
        }
    }

    // 8. Calculate the cross size of each flex line.
    //    If the flex container is single-line and has a definite cross size, the cross size
    //    of the flex line is the flex container’s inner cross size. Otherwise, for each flex line:
    //
    //    1. Collect all the flex items whose inline-axis is parallel to the main-axis, whose
    //       align-self is baseline, and whose cross-axis margins are both non-auto. Find the
    //       largest of the distances between each item’s baseline and its hypothetical outer
    //       cross-start edge, and the largest of the distances between each item’s baseline
    //       and its hypothetical outer cross-end edge, and sum these two values.
    //    2. Among all the items not collected by the previous step, find the largest
    //       outer hypothetical cross size.
    //    3. The used cross-size of the flex line is the largest of the numbers found in the
    //       previous two steps and zero.
    //
    //    If the flex container is single-line, then clamp the line’s cross-size to be within
    //    the container’s computed min and max cross sizes. Note that if CSS 2.1’s definition
    //    of min/max-width/height applied more generally, this behavior would fall out automatically.

    if flex_lines.len() == 1 && intrinsic_cross_size.is_finite() {
        flex_lines[0].cross_size = inner_intrinsic_cross_size
            .min(cross_constraint.max)
            .max(cross_constraint.min);
    } else {
        for line in &mut flex_lines {
            // TODO handle baseline (1)

            line.cross_size = line
                .items
                .iter()
                .map(|child| child.hypothetical_outer_cross_size)
                .fold(0.0, |acc, x| acc.max(x));
        }
    }

    // 9. Handle 'align-content: stretch'. If the flex container has a definite cross size,
    //    align-content is stretch, and the sum of the flex lines' cross sizes is less than
    //    the flex container’s inner cross size, increase the cross size of each flex line
    //    by equal amounts such that the sum of their cross sizes exactly equals the
    //    flex container’s inner cross size.

    if node.align_content == style::AlignContent::Stretch && intrinsic_cross_size.is_finite() {
        let total_cross: f32 = flex_lines.iter().map(|line| line.cross_size).sum();

        if total_cross < inner_intrinsic_cross_size {
            let remaining = inner_intrinsic_cross_size - total_cross;
            let addition = remaining / flex_lines.len() as f32;

            for line in &mut flex_lines {
                line.cross_size += addition;
            }
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

    for line in &mut flex_lines {
        for child in &mut line.items {
            let is_stretch = child.node.align_self(node) == style::AlignSelf::Stretch;

            if is_stretch
                && child.node.cross_margin_start(node.flex_direction) != style::Dimension::Auto
                && child.node.cross_margin_end(node.flex_direction) != style::Dimension::Auto
                && child.node.cross_size(node.flex_direction) == style::Dimension::Auto
            {
                child.target_cross_size = line
                    .cross_size
                    .min(
                        child
                            .node
                            .max_cross_size(node.flex_direction)
                            .resolve(avaliable_cross, f32::MAX),
                    ).max(
                        child
                            .node
                            .min_cross_size(node.flex_direction)
                            .resolve(avaliable_cross, f32::MIN),
                    );
            } else {
                child.target_cross_size = child.hypothetical_inner_cross_size;
            }

            if is_stretch {
                // TODO use result somehow
                compute_internal(
                    child.node,
                    if node.flex_direction.is_row() {
                        SizeConstraint::exactly(child.target_main_size)
                    } else {
                        SizeConstraint::exactly(child.target_cross_size)
                    },
                    if node.flex_direction.is_column() {
                        SizeConstraint::exactly(child.target_main_size)
                    } else {
                        SizeConstraint::exactly(child.target_cross_size)
                    },
                );
            }
        }
    }

    // 9.5. Main-Axis Alignment

    // 12. Distribute any remaining free space. For each flex line:
    //     1. If the remaining free space is positive and at least one main-axis margin on this
    //        line is auto, distribute the free space equally among these margins. Otherwise,
    //        set all auto margins to zero.
    //     2. Align the items along the main-axis per justify-content.

    for line in &mut flex_lines {
        let used_space: f32 = line.items.iter().map(|child| child.target_main_size).sum();
        let free_space = avaliable_main - used_space;

        if free_space > 0.0 {
            let mut num_auto_margins = 0;

            for child in &mut line.items {
                if child.node.main_margin_start(node.flex_direction) == style::Dimension::Auto {
                    num_auto_margins += 1;
                }
                if child.node.main_margin_end(node.flex_direction) == style::Dimension::Auto {
                    num_auto_margins += 1;
                }
            }

            if num_auto_margins > 0 {
                let margin = free_space / num_auto_margins as f32;

                for child in &mut line.items {
                    if child.node.main_margin_start(node.flex_direction) == style::Dimension::Auto {
                        child.main_margin_start = margin;
                    }
                    if child.node.main_margin_end(node.flex_direction) == style::Dimension::Auto {
                        child.main_margin_end = margin;
                    }
                }
            } else {
                let num_items = line.items.len();
                let mut is_first = true;

                for child in &mut line.items {
                    child.main_margin_start = child
                        .node
                        .main_margin_start(node.flex_direction)
                        .resolve(container_main_size, 0.0);
                    child.main_margin_end = child
                        .node
                        .main_margin_end(node.flex_direction)
                        .resolve(container_main_size, 0.0);

                    child.offset_main = match node.justify_content {
                        style::JustifyContent::FlexStart => 0.0,
                        style::JustifyContent::Center => if is_first {
                            free_space / 2.0
                        } else {
                            0.0
                        },
                        style::JustifyContent::FlexEnd => if is_first {
                            free_space
                        } else {
                            0.0
                        },
                        style::JustifyContent::SpaceBetween => if is_first {
                            0.0
                        } else {
                            free_space / (num_items - 1) as f32
                        },
                        style::JustifyContent::SpaceAround => if is_first {
                            (free_space / num_items as f32) / 2.0
                        } else {
                            free_space / num_items as f32
                        },
                        style::JustifyContent::SpaceEvenly => free_space / (num_items + 1) as f32,
                    };

                    is_first = false;
                }
            }
        }
    }

    // 9.6. Cross-Axis Alignment

    // 13. Resolve cross-axis auto margins. If a flex item has auto cross-axis margins:
    //     - If its outer cross size (treating those auto margins as zero) is less than the
    //       cross size of its flex line, distribute the difference in those sizes equally
    //       to the auto margins.
    //     - Otherwise, if the block-start or inline-start margin (whichever is in the cross axis)
    //       is auto, set it to zero. Set the opposite margin so that the outer cross size of the
    //       item equals the cross size of its flex line.

    for line in &mut flex_lines {
        for child in &mut line.items {
            if child.target_cross_size < line.cross_size {
                let free_space = line.cross_size - child.target_cross_size;

                if child.node.cross_margin_start(node.flex_direction) == style::Dimension::Auto
                    && child.node.cross_margin_end(node.flex_direction) == style::Dimension::Auto
                {
                    child.cross_margin_start = free_space / 2.0;
                    child.cross_margin_end = free_space / 2.0;
                } else if child.node.cross_margin_start(node.flex_direction)
                    == style::Dimension::Auto
                {
                    child.cross_margin_start = free_space;
                } else if child.node.cross_margin_end(node.flex_direction) == style::Dimension::Auto
                {
                    child.cross_margin_end = free_space;
                } else {
                    child.cross_margin_start = child
                        .node
                        .cross_margin_start(node.flex_direction)
                        .resolve(line.cross_size, 0.0);
                    child.cross_margin_end = child
                        .node
                        .cross_margin_end(node.flex_direction)
                        .resolve(line.cross_size, 0.0);

                    // 14. Align all flex items along the cross-axis per align-self, if neither of the item’s
                    //     cross-axis margins are auto.

                    child.offset_cross = match child.node.align_self(node) {
                        style::AlignSelf::Auto => 0.0, // Should never happen
                        style::AlignSelf::FlexStart => 0.0,
                        style::AlignSelf::FlexEnd => free_space,
                        style::AlignSelf::Center => free_space / 2.0,
                        style::AlignSelf::Baseline => free_space / 2.0, // Treat as center for now until we have baseline support
                        style::AlignSelf::Stretch => 0.0,
                    };
                }
            }
        }
    }

    // 15. Determine the flex container’s used cross size:
    //     - If the cross size property is a definite size, use that, clamped by the used
    //       min and max cross sizes of the flex container.
    //     - Otherwise, use the sum of the flex lines' cross sizes, clamped by the used
    //       min and max cross sizes of the flex container.

    let total_cross_size: f32 = flex_lines.iter().map(|line| line.cross_size).sum();
    let container_cross_size = if intrinsic_cross_size.is_finite() {
        intrinsic_cross_size
            .min(cross_constraint.max)
            .max(cross_constraint.min)
    } else {
        (total_cross_size + padding_cross + border_cross)
            .min(cross_constraint.max)
            .max(cross_constraint.min)
    };

    // 16. Align all flex lines per align-content.

    let free_space = container_cross_size - total_cross_size;
    let num_lines = flex_lines.len();
    let mut is_first = true;

    for line in &mut flex_lines {
        line.offset_cross = match node.align_content {
            style::AlignContent::FlexStart => 0.0,
            style::AlignContent::FlexEnd => if is_first {
                free_space
            } else {
                0.0
            },
            style::AlignContent::Center => if is_first {
                free_space / 2.0
            } else {
                0.0
            },
            style::AlignContent::Stretch => 0.0,
            style::AlignContent::SpaceBetween => if is_first {
                0.0
            } else {
                free_space / (num_lines - 1) as f32
            },
            style::AlignContent::SpaceAround => if is_first {
                (free_space / num_lines as f32) / 2.0
            } else {
                free_space / num_lines as f32
            },
        };
        is_first = false;
    }

    let mut children: Vec<layout::Node> = vec![];

    let mut total_offset_cross = padding_cross_start + border_cross_start;

    for line in &mut flex_lines {
        let mut total_offset_main = padding_main_start + border_main_start;

        for child in &mut line.items {
            let result = compute_internal(
                child.node,
                if node.flex_direction.is_row() {
                    SizeConstraint::exactly(child.target_main_size)
                } else {
                    SizeConstraint::exactly(child.target_cross_size)
                },
                if node.flex_direction.is_column() {
                    SizeConstraint::exactly(child.target_main_size)
                } else {
                    SizeConstraint::exactly(child.target_cross_size)
                },
            );

            let offset_main = {
                total_offset_main
                    + child.offset_main
                    + child.main_margin_start
                    + (child
                        .node
                        .main_start(node.flex_direction)
                        .resolve(container_main_size, 0.0)
                        - child
                            .node
                            .main_end(node.flex_direction)
                            .resolve(container_main_size, 0.0))
            };

            let offset_cross = {
                total_offset_cross
                    + child.offset_cross
                    + line.offset_cross
                    + child.cross_margin_start
                    + (child
                        .node
                        .cross_start(node.flex_direction)
                        .resolve(container_main_size, 0.0)
                        - child
                            .node
                            .cross_end(node.flex_direction)
                            .resolve(container_main_size, 0.0))
            };

            children.push(layout::Node {
                width: result.size.width,
                height: result.size.height,
                x: if node.flex_direction.is_row() {
                    offset_main
                } else {
                    offset_cross
                },
                y: if node.flex_direction.is_column() {
                    offset_main
                } else {
                    offset_cross
                },
                children: result.children,
            });

            total_offset_main =
                offset_main + result.size.main(node.flex_direction) + child.main_margin_end;
        }

        total_offset_cross += line.offset_cross + line.cross_size;
    }

    ComputeResult {
        size: FlexSize {
            width: if node.flex_direction.is_row() {
                container_main_size
            } else {
                container_cross_size
            },
            height: if node.flex_direction.is_column() {
                container_main_size
            } else {
                container_cross_size
            },
        },
        children: children,
    }
}
