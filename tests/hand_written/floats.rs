#![cfg(feature = "float_layout")]
use taffy::geometry::Point;
use taffy::prelude::*;
use taffy::style::{Clear, Float};
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
