//! Tests for the `safe` and `unsafe` overflow-position keywords on align-* and justify-*.
//!
//! Spec: <https://www.w3.org/TR/css-align-3/#overflow-values>
//!
//! When the alignment subject overflows the alignment container, a `safe` value behaves
//! as logical `start`; an `unsafe` value (the default) keeps its requested position even
//! when that causes data loss at the start edge.

use taffy::prelude::*;
use taffy::style::Direction;
use taffy_test_helpers::new_test_tree;

#[cfg(feature = "grid")]
#[test]
fn grid_safe_align_self_falls_back_to_start_on_overflow() {
    let mut taffy = new_test_tree();
    let child = taffy
        .new_leaf(Style {
            size: Size { width: length(50.0), height: length(150.0) },
            align_self: Some(AlignSelf::SAFE_END),
            ..Default::default()
        })
        .unwrap();
    let root = taffy
        .new_with_children(
            Style {
                display: Display::Grid,
                size: Size { width: length(100.0), height: length(100.0) },
                grid_template_rows: vec![length(100.0)],
                grid_template_columns: vec![length(100.0)],
                ..Default::default()
            },
            &[child],
        )
        .unwrap();

    taffy.compute_layout(root, Size::MAX_CONTENT).unwrap();

    // Child overflows the 100-tall grid area, so safe end falls back to start (y == 0).
    assert_eq!(taffy.layout(child).unwrap().location.y, 0.0);
}

#[cfg(feature = "grid")]
#[test]
fn grid_safe_align_self_behaves_as_end_when_no_overflow() {
    let mut taffy = new_test_tree();
    let child = taffy
        .new_leaf(Style {
            size: Size { width: length(50.0), height: length(40.0) },
            align_self: Some(AlignSelf::SAFE_END),
            ..Default::default()
        })
        .unwrap();
    let root = taffy
        .new_with_children(
            Style {
                display: Display::Grid,
                size: Size { width: length(100.0), height: length(100.0) },
                grid_template_rows: vec![length(100.0)],
                grid_template_columns: vec![length(100.0)],
                ..Default::default()
            },
            &[child],
        )
        .unwrap();

    taffy.compute_layout(root, Size::MAX_CONTENT).unwrap();

    // No overflow, so safe end behaves as end: y == 100 - 40 == 60
    assert_eq!(taffy.layout(child).unwrap().location.y, 60.0);
}

#[cfg(feature = "grid")]
#[test]
fn grid_unsafe_align_self_keeps_overflowing_position() {
    let mut taffy = new_test_tree();
    let child = taffy
        .new_leaf(Style {
            size: Size { width: length(50.0), height: length(150.0) },
            align_self: Some(AlignSelf::END),
            ..Default::default()
        })
        .unwrap();
    let root = taffy
        .new_with_children(
            Style {
                display: Display::Grid,
                size: Size { width: length(100.0), height: length(100.0) },
                grid_template_rows: vec![length(100.0)],
                grid_template_columns: vec![length(100.0)],
                ..Default::default()
            },
            &[child],
        )
        .unwrap();

    taffy.compute_layout(root, Size::MAX_CONTENT).unwrap();

    // Plain End with overflow keeps end alignment: y == 100 - 150 == -50
    assert_eq!(taffy.layout(child).unwrap().location.y, -50.0);
}

#[cfg(feature = "grid")]
#[test]
fn grid_safe_justify_self_falls_back_to_start_on_overflow() {
    let mut taffy = new_test_tree();
    let child = taffy
        .new_leaf(Style {
            size: Size { width: length(150.0), height: length(50.0) },
            justify_self: Some(JustifySelf::SAFE_END),
            ..Default::default()
        })
        .unwrap();
    let root = taffy
        .new_with_children(
            Style {
                display: Display::Grid,
                size: Size { width: length(100.0), height: length(100.0) },
                grid_template_rows: vec![length(100.0)],
                grid_template_columns: vec![length(100.0)],
                ..Default::default()
            },
            &[child],
        )
        .unwrap();

    taffy.compute_layout(root, Size::MAX_CONTENT).unwrap();

    assert_eq!(taffy.layout(child).unwrap().location.x, 0.0);
}

#[cfg(feature = "grid")]
#[test]
fn grid_safe_justify_self_rtl_falls_back_to_rtl_start_edge() {
    let mut taffy = new_test_tree();
    let child = taffy
        .new_leaf(Style {
            size: Size { width: length(150.0), height: length(50.0) },
            justify_self: Some(JustifySelf::SAFE_END),
            ..Default::default()
        })
        .unwrap();
    let root = taffy
        .new_with_children(
            Style {
                display: Display::Grid,
                direction: Direction::Rtl,
                size: Size { width: length(100.0), height: length(100.0) },
                grid_template_rows: vec![length(100.0)],
                grid_template_columns: vec![length(100.0)],
                ..Default::default()
            },
            &[child],
        )
        .unwrap();

    taffy.compute_layout(root, Size::MAX_CONTENT).unwrap();

    // In RTL the inline-start edge is the right side. The 150-wide child in the
    // 100-wide container, anchored at the RTL Start edge, sits at x == -50.
    assert_eq!(taffy.layout(child).unwrap().location.x, -50.0);
}

#[cfg(feature = "grid")]
#[test]
fn grid_safe_align_content_overflow_falls_back_to_start() {
    let mut taffy = new_test_tree();
    let child_a = taffy
        .new_leaf(Style { size: Size { width: length(40.0), height: length(80.0) }, ..Default::default() })
        .unwrap();
    let child_b = taffy
        .new_leaf(Style { size: Size { width: length(40.0), height: length(80.0) }, ..Default::default() })
        .unwrap();
    let root = taffy
        .new_with_children(
            Style {
                display: Display::Grid,
                size: Size { width: length(100.0), height: length(100.0) },
                grid_template_rows: vec![length(80.0), length(80.0)],
                grid_template_columns: vec![length(40.0)],
                align_content: AlignContent::SAFE_END,
                ..Default::default()
            },
            &[child_a, child_b],
        )
        .unwrap();

    taffy.compute_layout(root, Size::MAX_CONTENT).unwrap();

    // Tracks total 160 in a 100-tall container — overflow. Safe end falls back to
    // start: first track at y == 0.
    assert_eq!(taffy.layout(child_a).unwrap().location.y, 0.0);
}

#[cfg(feature = "flexbox")]
#[test]
fn flex_safe_align_self_falls_back_to_start_on_overflow() {
    let mut taffy = new_test_tree();
    let child = taffy
        .new_leaf(Style {
            size: Size { width: length(50.0), height: length(150.0) },
            align_self: Some(AlignSelf::SAFE_END),
            ..Default::default()
        })
        .unwrap();
    let root = taffy
        .new_with_children(
            Style {
                display: Display::Flex,
                size: Size { width: length(100.0), height: length(100.0) },
                ..Default::default()
            },
            &[child],
        )
        .unwrap();

    taffy.compute_layout(root, Size::MAX_CONTENT).unwrap();

    // Cross-axis overflow: safe end falls back to start (y == 0).
    assert_eq!(taffy.layout(child).unwrap().location.y, 0.0);
}

#[cfg(feature = "flexbox")]
#[test]
fn flex_unsafe_align_self_keeps_overflowing_position() {
    let mut taffy = new_test_tree();
    let child = taffy
        .new_leaf(Style {
            size: Size { width: length(50.0), height: length(150.0) },
            align_self: Some(AlignSelf::END),
            ..Default::default()
        })
        .unwrap();
    let root = taffy
        .new_with_children(
            Style {
                display: Display::Flex,
                size: Size { width: length(100.0), height: length(100.0) },
                ..Default::default()
            },
            &[child],
        )
        .unwrap();

    taffy.compute_layout(root, Size::MAX_CONTENT).unwrap();

    // Plain End with cross-axis overflow keeps end alignment: y == 100 - 150 == -50
    assert_eq!(taffy.layout(child).unwrap().location.y, -50.0);
}

#[cfg(feature = "flexbox")]
#[test]
fn flex_safe_justify_content_falls_back_to_start_on_overflow() {
    let mut taffy = new_test_tree();
    let child_a = taffy
        .new_leaf(Style { size: Size { width: length(80.0), height: length(50.0) }, ..Default::default() })
        .unwrap();
    let child_b = taffy
        .new_leaf(Style { size: Size { width: length(80.0), height: length(50.0) }, ..Default::default() })
        .unwrap();
    let root = taffy
        .new_with_children(
            Style {
                display: Display::Flex,
                size: Size { width: length(100.0), height: length(100.0) },
                justify_content: JustifyContent::SAFE_END,
                ..Default::default()
            },
            &[child_a, child_b],
        )
        .unwrap();

    taffy.compute_layout(root, Size::MAX_CONTENT).unwrap();

    // Items total 160 in a 100-wide container — overflow. Safe end falls back to
    // start: first item anchored at x == 0.
    assert_eq!(taffy.layout(child_a).unwrap().location.x, 0.0);
}

#[cfg(feature = "flexbox")]
#[test]
fn flex_safe_align_content_falls_back_to_start_on_multiline_overflow() {
    let mut taffy = new_test_tree();
    let child_a = taffy
        .new_leaf(Style { size: Size { width: length(60.0), height: length(80.0) }, ..Default::default() })
        .unwrap();
    let child_b = taffy
        .new_leaf(Style { size: Size { width: length(60.0), height: length(80.0) }, ..Default::default() })
        .unwrap();
    let root = taffy
        .new_with_children(
            Style {
                display: Display::Flex,
                flex_wrap: FlexWrap::Wrap,
                size: Size { width: length(100.0), height: length(100.0) },
                align_content: AlignContent::SAFE_END,
                ..Default::default()
            },
            &[child_a, child_b],
        )
        .unwrap();

    taffy.compute_layout(root, Size::MAX_CONTENT).unwrap();

    // Two 80-tall lines wrap and overflow the 100-tall container. Safe end falls back
    // to start: first line at y == 0.
    assert_eq!(taffy.layout(child_a).unwrap().location.y, 0.0);
}

/// With both insets set, `safe` on an absolutely positioned box takes the same overflow steps as
/// the default overflow alignment: a box that overflows the inset-modified containing block but
/// fits the overflow limit rect covers the former and is aligned as specified within the latter.
/// <https://drafts.csswg.org/css-align-3/#auto-safety-position>
///
/// The containing block is 400 wide. Chrome 158 still start-aligns within the inset-modified
/// containing block, so these expectations follow the spec (and Safari) rather than Chrome.
#[cfg(feature = "block_layout")]
#[test]
fn block_absolute_safe_alignment_between_insets_uses_overflow_limit_rect() {
    for direction in [Direction::Ltr, Direction::Rtl] {
        for (left, right, width, justify_self, x_ltr, x_rtl) in [
            // Fits the IMCB (200..400): aligned as specified
            (200.0, 0.0, 100.0, JustifySelf::SAFE_END, 300.0, 200.0),
            // Overflows the IMCB, fits the overflow limit rect (0..400): kept inside the limit
            (200.0, 0.0, 260.0, JustifySelf::SAFE_END, 140.0, 140.0),
            (200.0, 0.0, 260.0, JustifySelf::SAFE_CENTER, 140.0, 140.0),
            (0.0, 200.0, 260.0, JustifySelf::SAFE_START, 0.0, 0.0),
            // Overflows the overflow limit rect: start-aligned within it
            (200.0, 0.0, 500.0, JustifySelf::SAFE_END, 0.0, -100.0),
        ] {
            let mut taffy = new_test_tree();
            let child = taffy
                .new_leaf(Style {
                    position: Position::Absolute,
                    direction,
                    size: Size { width: length(width), height: length(10.0) },
                    inset: Rect { left: length(left), right: length(right), top: length(0.0), bottom: auto() },
                    justify_self: Some(justify_self),
                    ..Default::default()
                })
                .unwrap();
            let root = taffy
                .new_with_children(
                    Style {
                        display: Display::Block,
                        position: Position::Relative,
                        direction,
                        size: Size { width: length(400.0), height: length(100.0) },
                        ..Default::default()
                    },
                    &[child],
                )
                .unwrap();
            taffy.compute_layout(root, Size::MAX_CONTENT).unwrap();
            let x = if direction == Direction::Rtl { x_rtl } else { x_ltr };
            assert_eq!(
                taffy.layout(child).unwrap().location.x,
                x,
                "{direction:?}: {width} wide, left {left}, right {right}, {justify_self:?}"
            );
        }
    }
}

#[cfg(feature = "block_layout")]
#[test]
fn block_absolute_safe_align_self_between_insets_uses_overflow_limit_rect() {
    let mut taffy = new_test_tree();
    let child = taffy
        .new_leaf(Style {
            position: Position::Absolute,
            size: Size { width: length(10.0), height: length(260.0) },
            inset: Rect { left: length(0.0), right: auto(), top: length(200.0), bottom: length(0.0) },
            align_self: Some(AlignSelf::SAFE_END),
            ..Default::default()
        })
        .unwrap();
    let root = taffy
        .new_with_children(
            Style {
                display: Display::Block,
                position: Position::Relative,
                size: Size { width: length(100.0), height: length(400.0) },
                ..Default::default()
            },
            &[child],
        )
        .unwrap();
    taffy.compute_layout(root, Size::MAX_CONTENT).unwrap();

    // The IMCB spans 200..400 and the 260-tall box overflows it, but it fits the containing
    // block, so `safe end` keeps it ending at 400 instead of falling back to y == 200.
    assert_eq!(taffy.layout(child).unwrap().location.y, 140.0);
}
