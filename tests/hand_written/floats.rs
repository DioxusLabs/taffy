#![cfg(feature = "float_layout")]
use taffy::geometry::Point;
use taffy::prelude::*;
use taffy::style::{Clear, Direction, Float, Overflow};
use taffy_test_helpers::new_test_tree;

/// Regression test for <https://wpt.live/css/CSS2/floats-clear/floats-146.xht>
///
/// A right float placed after two left floats must not be placed higher than the
/// outer top of the earlier floats (CSS2 float rule 5). This scenario previously
/// panicked in `FloatContext::subdivide_segment` when the float's end coincided
/// with an existing segment boundary due to floating point imprecision.
#[test]
fn float_no_higher_than_earlier_floats() {
    let mut taffy = new_test_tree();

    let block = |width: f32, height: f32, float: Float| Style {
        display: Display::Block,
        float,
        size: Size { width: length(width), height: length(height) },
        ..Default::default()
    };

    // Spacer block advancing the flow position to a fractional y offset (43.2)
    let spacer = taffy.new_leaf(block(400.0, 43.2, Float::None)).unwrap();
    let float_a = taffy.new_leaf(block(258.0, 42.0, Float::Left)).unwrap();
    let float_b = taffy.new_leaf(block(298.0, 42.0, Float::Left)).unwrap();
    let float_c = taffy.new_leaf(block(98.0, 42.0, Float::Right)).unwrap();

    let root = taffy
        .new_with_children(
            Style {
                display: Display::Block,
                size: Size { width: length(400.0), height: auto() },
                ..Default::default()
            },
            &[spacer, float_a, float_b, float_c],
        )
        .unwrap();

    taffy.compute_layout(root, Size::MAX_CONTENT).unwrap();

    let layout_a = taffy.layout(float_a).unwrap();
    let layout_b = taffy.layout(float_b).unwrap();
    let layout_c = taffy.layout(float_c).unwrap();

    // A is placed below the spacer
    assert_eq!(layout_a.location, Point { x: 0.0, y: 43.0 });
    // B does not fit next to A, so is placed below it
    assert_eq!(layout_b.location, Point { x: 0.0, y: 85.0 });
    // C must not be placed higher than B (it fits next to B on the right)
    assert_eq!(layout_c.location, Point { x: 302.0, y: 85.0 });
}

/// Regression test for <https://wpt.live/css/CSS2/floats-clear/margin-collapse-033.xht>
///
/// Floats that occupy no horizontal space (e.g. a zero-width float) still affect
/// `clear`: a cleared block must be placed past the bottom of all floats on the
/// relevant side, regardless of their width.
#[test]
fn clear_past_zero_width_float() {
    let mut taffy = new_test_tree();

    // float: left with auto width resolving to 0
    let floated = taffy
        .new_leaf(Style {
            display: Display::Block,
            float: Float::Left,
            size: Size { width: auto(), height: length(1.0) },
            ..Default::default()
        })
        .unwrap();
    let cleared = taffy.new_leaf(Style { display: Display::Block, clear: Clear::Left, ..Default::default() }).unwrap();
    let sibling = taffy
        .new_leaf(Style {
            display: Display::Block,
            margin: Rect { top: length(99.0), ..Rect::zero() },
            ..Default::default()
        })
        .unwrap();

    let container = taffy
        .new_with_children(
            Style {
                display: Display::Block,
                size: Size { width: length(100.0), height: auto() },
                ..Default::default()
            },
            &[floated, cleared, sibling],
        )
        .unwrap();
    let root = taffy.new_with_children(Style { display: Display::Block, ..Default::default() }, &[container]).unwrap();

    taffy.compute_layout(root, Size::MAX_CONTENT).unwrap();

    // The cleared (self-collapsing) block is placed below the float (y=1). Its clearance
    // prevents the sibling's 99px margin from collapsing with preceding margins, so the
    // container's height is 1 + 99 = 100.
    assert_eq!(taffy.layout(container).unwrap().size.height, 100.0);
}

fn float_block(width: f32, height: f32, float: Float) -> Style {
    Style {
        display: Display::Block,
        float,
        size: Size { width: length(width), height: length(height) },
        ..Default::default()
    }
}

fn root_style(width: f32) -> Style {
    Style { display: Display::Block, size: Size { width: length(width), height: auto() }, ..Default::default() }
}

/// Regression test for <https://wpt.live/css/CSS2/floats/floats-rule3-outside-left-001.xht>
///
/// CSS2 float rule 7: a float that is wider than its containing block and has no other
/// float beside it may overflow the containing block's edge rather than being pushed down.
#[test]
fn oversized_float_overflows_containing_block() {
    let mut taffy = new_test_tree();

    let float_a = taffy.new_leaf(float_block(150.0, 50.0, Float::Left)).unwrap();
    let root = taffy.new_with_children(root_style(100.0), &[float_a]).unwrap();

    taffy.compute_layout(root, Size::MAX_CONTENT).unwrap();

    assert_eq!(taffy.layout(float_a).unwrap().location, Point { x: 0.0, y: 0.0 });
}

/// CSS2 float rules 3 & 7: a float with another float beside it must not overlap that float
/// (rule 3) nor extend past the containing block's opposite edge (rule 7) - it must move down.
#[test]
fn float_beside_existing_float_moves_down_instead_of_overflowing() {
    let mut taffy = new_test_tree();

    // Rule 3: an opposite-direction float that doesn't fit must move down, not overlap
    let left = taffy.new_leaf(float_block(60.0, 50.0, Float::Left)).unwrap();
    let right = taffy.new_leaf(float_block(60.0, 50.0, Float::Right)).unwrap();
    // Rule 7: a same-direction float that doesn't fit must move down, not overflow the
    // containing block's trailing edge
    let left2 = taffy.new_leaf(float_block(60.0, 50.0, Float::Left)).unwrap();

    let root = taffy.new_with_children(root_style(100.0), &[left, right, left2]).unwrap();

    taffy.compute_layout(root, Size::MAX_CONTENT).unwrap();

    assert_eq!(taffy.layout(left).unwrap().location, Point { x: 0.0, y: 0.0 });
    assert_eq!(taffy.layout(right).unwrap().location, Point { x: 40.0, y: 50.0 });
    assert_eq!(taffy.layout(left2).unwrap().location, Point { x: 0.0, y: 100.0 });
}

/// Regression test for auto-margins-used-values-with-floats.tentative.html.
#[test]
fn auto_margins_include_float_avoidance_offsets() {
    for direction in [Direction::Ltr, Direction::Rtl] {
        for (left_float_width, right_float_width) in [(0.0, 20.0), (20.0, 0.0), (20.0, 20.0), (0.0, 0.0)] {
            let mut taffy = new_test_tree();
            let left_float = taffy.new_leaf(float_block(left_float_width, 40.0, Float::Left)).unwrap();
            let right_float = taffy.new_leaf(float_block(right_float_width, 40.0, Float::Right)).unwrap();
            let auto_sides = [(true, true), (true, false), (false, true)];
            let boxes: Vec<_> = auto_sides
                .iter()
                .map(|&(left_auto, right_auto)| {
                    taffy
                        .new_leaf(Style {
                            display: Display::FlowRoot,
                            direction,
                            size: Size { width: length(40.0), height: length(10.0) },
                            margin: Rect {
                                left: if left_auto { auto() } else { zero() },
                                right: if right_auto { auto() } else { zero() },
                                top: auto(),
                                bottom: auto(),
                            },
                            ..Default::default()
                        })
                        .unwrap()
                })
                .collect();
            let children: Vec<_> = [left_float, right_float].into_iter().chain(boxes.iter().copied()).collect();
            let root = taffy
                .new_with_children(
                    Style {
                        display: Display::FlowRoot,
                        direction,
                        size: Size { width: length(100.0), height: auto() },
                        padding: Rect { left: length(5.0), right: length(5.0), top: length(5.0), bottom: length(5.0) },
                        ..Default::default()
                    },
                    &children,
                )
                .unwrap();

            taffy.compute_layout(root, Size::MAX_CONTENT).unwrap();

            for (index, (&node, &(left_auto, right_auto))) in boxes.iter().zip(&auto_sides).enumerate() {
                let layout = taffy.layout(node).unwrap();
                let free_space = 90.0 - left_float_width - right_float_width - 40.0;
                let alignment_margin = free_space / (u8::from(left_auto) + u8::from(right_auto)) as f32;
                let left_gap = if left_auto { alignment_margin } else { 0.0 };
                assert_eq!(
                    layout.location,
                    Point { x: 5.0 + left_float_width + left_gap, y: 5.0 + index as f32 * 10.0 }
                );
                assert_eq!(layout.size, Size { width: 40.0, height: 10.0 });
                assert_eq!(layout.margin.left, if left_auto { left_float_width + alignment_margin } else { 0.0 });
                assert_eq!(layout.margin.right, if right_auto { right_float_width + alignment_margin } else { 0.0 });
                assert_eq!(layout.margin.top, 0.0);
                assert_eq!(layout.margin.bottom, 0.0);
            }
        }
    }
}

#[test]
fn float_avoiding_auto_margins_in_nested_block_ignore_relative_insets_and_clearance() {
    for direction in [Direction::Ltr, Direction::Rtl] {
        let mut taffy = new_test_tree();
        let left_float = taffy.new_leaf(float_block(40.0, 40.0, Float::Left)).unwrap();
        let right_float = taffy.new_leaf(float_block(60.0, 40.0, Float::Right)).unwrap();
        let box_style = Style {
            display: Display::FlowRoot,
            direction,
            size: Size { width: length(40.0), height: length(10.0) },
            margin: Rect { left: auto(), right: auto(), top: zero(), bottom: zero() },
            ..Default::default()
        };
        let relative_box = taffy
            .new_leaf(Style {
                position: Position::Relative,
                inset: Rect { left: length(7.0), ..Rect::auto() },
                ..box_style.clone()
            })
            .unwrap();
        let cleared_box = taffy.new_leaf(Style { clear: Clear::Both, ..box_style }).unwrap();
        let nested = taffy
            .new_with_children(
                Style {
                    display: Display::Block,
                    direction,
                    margin: Rect { left: length(20.0), right: length(30.0), ..Rect::zero() },
                    padding: Rect { left: length(5.0), right: length(7.0), ..Rect::zero() },
                    border: Rect { left: length(3.0), right: length(5.0), ..Rect::zero() },
                    ..Default::default()
                },
                &[relative_box, cleared_box],
            )
            .unwrap();
        let root = taffy
            .new_with_children(
                Style {
                    display: Display::FlowRoot,
                    direction,
                    size: Size { width: length(200.0), height: auto() },
                    padding: Rect { left: length(10.0), right: length(10.0), top: length(10.0), bottom: length(10.0) },
                    ..Default::default()
                },
                &[left_float, right_float, nested],
            )
            .unwrap();

        taffy.compute_layout(root, Size::MAX_CONTENT).unwrap();

        let relative_layout = taffy.layout(relative_box).unwrap();
        assert_eq!(relative_layout.location, Point { x: 47.0, y: 0.0 });
        assert_eq!(relative_layout.margin.left, 32.0);
        assert_eq!(relative_layout.margin.right, 38.0);
        let cleared_layout = taffy.layout(cleared_box).unwrap();
        assert_eq!(cleared_layout.location, Point { x: 43.0, y: 40.0 });
        assert_eq!(cleared_layout.margin.left, 35.0);
        assert_eq!(cleared_layout.margin.right, 35.0);
        let nested_layout = taffy.layout(nested).unwrap();
        assert_eq!(nested_layout.location, Point { x: 30.0, y: 10.0 });
        assert_eq!(nested_layout.margin.left, 20.0);
        assert_eq!(nested_layout.margin.right, 30.0);
    }
}

/// Regression test for <https://wpt.live/css/CSS2/floats/zero-width-floats.html>
///
/// A BFC root with negative horizontal margins placed beside only zero-width floats
/// must not have those margins clamped: zero-width floats don't constrain the width
/// of adjacent boxes, so the box behaves as if no float were present.
#[test]
fn bfc_negative_margins_not_clamped_by_zero_width_floats() {
    let mut taffy = new_test_tree();

    let zero_width_left = taffy.new_leaf(float_block(0.0, 50.0, Float::Left)).unwrap();
    let zero_width_right =
        taffy.new_leaf(Style { clear: Clear::Left, ..float_block(0.0, 150.0, Float::Right) }).unwrap();
    let bfc = taffy
        .new_leaf(Style {
            display: Display::Block,
            overflow: Point { x: Overflow::Hidden, y: Overflow::Hidden },
            margin: Rect { left: length(-50.0), right: length(-50.0), top: zero(), bottom: zero() },
            size: Size { width: auto(), height: length(100.0) },
            ..Default::default()
        })
        .unwrap();

    let root = taffy.new_with_children(root_style(100.0), &[zero_width_left, zero_width_right, bfc]).unwrap();

    taffy.compute_layout(root, Size::MAX_CONTENT).unwrap();

    let layout = taffy.layout(bfc).unwrap();
    // Negative margins expand the BFC past the containing block on both sides
    assert_eq!(layout.location, Point { x: -50.0, y: 0.0 });
    assert_eq!(layout.size, Size { width: 200.0, height: 100.0 });
}

/// Regression test for <https://wpt.live/css/CSS2/floats/zero-width-floats-positioning.tentative.html>
///
/// When a BFC root is narrowed to fit beside a real (positive-width) float, a zero-width
/// float's edge acts as an obstacle: a negative margin may not move the border edge past it.
#[test]
fn zero_width_float_edge_blocks_negative_margin_beside_real_float() {
    let mut taffy = new_test_tree();

    let zero_width_left = taffy.new_leaf(float_block(0.0, 50.0, Float::Left)).unwrap();
    let real_right = taffy.new_leaf(Style { clear: Clear::Left, ..float_block(25.0, 50.0, Float::Right) }).unwrap();
    let bfc = taffy
        .new_leaf(Style {
            display: Display::Block,
            overflow: Point { x: Overflow::Hidden, y: Overflow::Hidden },
            margin: Rect { left: length(-50.0), right: zero(), top: zero(), bottom: zero() },
            size: Size { width: auto(), height: length(100.0) },
            ..Default::default()
        })
        .unwrap();

    let root = taffy.new_with_children(root_style(125.0), &[zero_width_left, real_right, bfc]).unwrap();

    taffy.compute_layout(root, Size::MAX_CONTENT).unwrap();

    let layout = taffy.layout(bfc).unwrap();
    // The zero-width left float's edge stops the negative left margin; the real right
    // float (below the zero-width float due to clear: left) narrows the box.
    assert_eq!(layout.location, Point { x: 0.0, y: 0.0 });
    assert_eq!(layout.size, Size { width: 100.0, height: 100.0 });
}

/// Regression test for <https://wpt.live/css/CSS2/floats/floats-wrap-bfc-with-margin-006.tentative.html>
///
/// A negative leading margin on a BFC root beside a real float on the trailing side only
/// is applied as usual (there is no float edge on the leading side to act as an obstacle).
#[test]
fn bfc_negative_leading_margin_beside_trailing_float() {
    let mut taffy = new_test_tree();

    let real_right = taffy.new_leaf(float_block(25.0, 50.0, Float::Right)).unwrap();
    let bfc = taffy
        .new_leaf(Style {
            display: Display::Block,
            overflow: Point { x: Overflow::Hidden, y: Overflow::Hidden },
            margin: Rect { left: length(-50.0), right: zero(), top: zero(), bottom: zero() },
            size: Size { width: auto(), height: length(50.0) },
            ..Default::default()
        })
        .unwrap();

    let root = taffy.new_with_children(root_style(50.0), &[real_right, bfc]).unwrap();

    taffy.compute_layout(root, Size::MAX_CONTENT).unwrap();

    let layout = taffy.layout(bfc).unwrap();
    // width = 50 (container) + 50 (negative margin) - 25 (float) = 75
    assert_eq!(layout.location, Point { x: -50.0, y: 0.0 });
    assert_eq!(layout.size, Size { width: 75.0, height: 50.0 });
}
