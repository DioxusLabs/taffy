#![cfg(feature = "float_layout")]
use taffy::geometry::Point;
use taffy::prelude::*;
use taffy::style::{Float, Overflow};
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

/// Regression test for <https://wpt.live/css/CSS2/floats/zero-width-floats.html>
///
/// A zero-width float still occupies a vertical range: a later float with `clear`
/// must be placed below it, even though it provides no horizontal inset.
#[test]
fn zero_width_float_participates_in_clearance() {
    let mut taffy = new_test_tree();

    let zero_width_left = taffy
        .new_leaf(Style {
            display: Display::Block,
            float: Float::Left,
            size: Size { width: length(0.0), height: length(50.0) },
            ..Default::default()
        })
        .unwrap();
    let cleared_right = taffy
        .new_leaf(Style {
            display: Display::Block,
            float: Float::Right,
            clear: taffy::style::Clear::Left,
            size: Size { width: length(25.0), height: length(150.0) },
            ..Default::default()
        })
        .unwrap();

    let root = taffy
        .new_with_children(
            Style {
                display: Display::Block,
                size: Size { width: length(100.0), height: auto() },
                ..Default::default()
            },
            &[zero_width_left, cleared_right],
        )
        .unwrap();

    taffy.compute_layout(root, Size::MAX_CONTENT).unwrap();

    assert_eq!(taffy.layout(zero_width_left).unwrap().location, Point { x: 0.0, y: 0.0 });
    // The cleared right float must be pushed below the zero-width left float
    assert_eq!(taffy.layout(cleared_right).unwrap().location, Point { x: 75.0, y: 50.0 });
}

/// Regression test for <https://wpt.live/css/CSS2/floats/zero-width-floats.html>
///
/// A BFC root with negative horizontal margins placed beside zero-width floats must
/// not have those margins clamped: zero-width floats provide no actual float edge,
/// so the box behaves as if no float were present on that side.
#[test]
fn bfc_negative_margins_not_clamped_by_zero_width_floats() {
    let mut taffy = new_test_tree();

    let zero_width_left = taffy
        .new_leaf(Style {
            display: Display::Block,
            float: Float::Left,
            size: Size { width: length(0.0), height: length(50.0) },
            ..Default::default()
        })
        .unwrap();
    let bfc = taffy
        .new_leaf(Style {
            display: Display::Block,
            overflow: Point { x: Overflow::Hidden, y: Overflow::Hidden },
            margin: Rect { left: length(-50.0), right: length(-50.0), top: zero(), bottom: zero() },
            size: Size { width: auto(), height: length(100.0) },
            ..Default::default()
        })
        .unwrap();

    let root = taffy
        .new_with_children(
            Style {
                display: Display::Block,
                size: Size { width: length(100.0), height: auto() },
                ..Default::default()
            },
            &[zero_width_left, bfc],
        )
        .unwrap();

    taffy.compute_layout(root, Size::MAX_CONTENT).unwrap();

    let layout = taffy.layout(bfc).unwrap();
    // Negative margins expand the BFC past the containing block on both sides
    assert_eq!(layout.location, Point { x: -50.0, y: 0.0 });
    assert_eq!(layout.size, Size { width: 200.0, height: 100.0 });
}
