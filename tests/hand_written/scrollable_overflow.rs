//! The scrollable overflow rectangle is in physical scrollport coordinates and captures
//! start-side (negative) overflow as well as end-side overflow. For scroll containers the
//! unreachable scrollable overflow region (on the far side of the scroll origin) is excluded.

use taffy::geometry::Point;
use taffy::prelude::*;
use taffy::style::{Direction, Display, Overflow, Position};
use taffy_test_helpers::new_test_tree;

const CONTAINER: f32 = 200.0;
const CHILD: f32 = 50.0;

fn container_with_absolute_child(
    display: Display,
    overflow: Overflow,
    direction: Direction,
    inset_left: Option<f32>,
    inset_right: Option<f32>,
) -> Layout {
    let mut tree = new_test_tree();
    let child = tree
        .new_leaf(Style {
            position: Position::Absolute,
            inset: Rect {
                left: inset_left.map(length).unwrap_or(auto()),
                right: inset_right.map(length).unwrap_or(auto()),
                top: length(0.0),
                bottom: auto(),
            },
            size: Size { width: length(CHILD), height: length(CHILD) },
            ..Default::default()
        })
        .unwrap();
    let node = tree
        .new_with_children(
            Style {
                display,
                direction,
                position: Position::Relative,
                size: Size { width: length(CONTAINER), height: length(CONTAINER) },
                overflow: Point { x: overflow, y: overflow },
                ..Default::default()
            },
            &[child],
        )
        .unwrap();

    tree.compute_layout(node, Size::MAX_CONTENT).unwrap();
    *tree.layout(node).unwrap()
}

#[test]
fn wholly_unreachable_boxes_are_excluded_for_scroll_containers() {
    for display in [Display::Block, Display::Flex, Display::Grid] {
        // Child entirely before the scroll origin (x from -100 to -50)
        let layout = container_with_absolute_child(display, Overflow::Scroll, Direction::Ltr, Some(-100.0), None);

        assert_eq!(layout.scrollable_overflow_rect.rect, Rect::ZERO, "{display:?}");
        assert_eq!(layout.scroll_width(), 0.0, "{display:?}");
    }
}

#[test]
fn wholly_unreachable_boxes_contribute_to_non_scroll_containers() {
    for display in [Display::Block, Display::Flex, Display::Grid] {
        // A box that is not a scroll container has no unreachable region of its own, so
        // start-side overflow is captured in its scrollable overflow rect.
        let layout = container_with_absolute_child(display, Overflow::Visible, Direction::Ltr, Some(-100.0), None);

        assert_eq!(layout.scrollable_overflow_rect.left, -100.0, "{display:?}");
    }
}

#[test]
fn partially_unreachable_boxes_contribute_their_reachable_part() {
    for display in [Display::Block, Display::Flex, Display::Grid] {
        // Child straddling the scroll origin (x from -25 to 25): the part before the scroll
        // origin is unreachable and is excluded.
        let layout = container_with_absolute_child(display, Overflow::Scroll, Direction::Ltr, Some(-25.0), None);

        assert_eq!(layout.scrollable_overflow_rect.left, 0.0, "{display:?}");
        assert_eq!(layout.scrollable_overflow_rect.right, 25.0, "{display:?}");
        assert_eq!(layout.scroll_width(), 0.0, "{display:?}");
    }
}

#[test]
fn end_side_overflow_is_captured_and_scrollable() {
    for display in [Display::Block, Display::Flex, Display::Grid] {
        // Child extending past the end edge (x from 200 to 250)
        let layout = container_with_absolute_child(display, Overflow::Scroll, Direction::Ltr, Some(CONTAINER), None);

        assert_eq!(layout.scrollable_overflow_rect.right, CONTAINER + CHILD, "{display:?}");
        assert_eq!(layout.scroll_width(), CHILD, "{display:?}");
    }
}

#[test]
fn absolute_only_containers_include_scroll_container_end_padding() {
    for display in [Display::Block, Display::Flex, Display::Grid] {
        let mut tree = new_test_tree();
        let abs = tree
            .new_leaf(Style {
                position: Position::Absolute,
                inset: Rect { left: length(0.0), right: auto(), top: length(0.0), bottom: auto() },
                size: Size { width: length(5.0), height: length(5.0) },
                ..Default::default()
            })
            .unwrap();
        let node = tree
            .new_with_children(
                Style {
                    display,
                    position: Position::Relative,
                    overflow: Point { x: Overflow::Scroll, y: Overflow::Scroll },
                    padding: Rect { left: length(0.0), right: length(20.0), top: length(0.0), bottom: length(30.0) },
                    size: Size { width: length(CONTAINER), height: length(CONTAINER) },
                    ..Default::default()
                },
                &[abs],
            )
            .unwrap();

        tree.compute_layout(node, Size::MAX_CONTENT).unwrap();
        let layout = tree.layout(node).unwrap();

        assert_eq!(layout.scrollable_overflow_rect.right, 20.0, "{display:?}");
        assert_eq!(layout.scrollable_overflow_rect.bottom, 30.0, "{display:?}");
    }
}

fn nested_overflow_layout(display: Display, child_overflow: Point<Overflow>) -> Layout {
    let mut tree = new_test_tree();
    // Grandchild overflowing the child in the y axis (extends to y=300 in a 200-high child)
    let grandchild = tree
        .new_leaf(Style {
            position: Position::Absolute,
            inset: Rect { left: length(0.0), right: auto(), top: length(CONTAINER), bottom: auto() },
            size: Size { width: length(CHILD), height: length(CHILD) },
            ..Default::default()
        })
        .unwrap();
    let child = tree
        .new_with_children(
            Style {
                display,
                position: Position::Relative,
                size: Size { width: length(CONTAINER), height: length(CONTAINER) },
                overflow: child_overflow,
                ..Default::default()
            },
            &[grandchild],
        )
        .unwrap();
    let node = tree
        .new_with_children(
            Style { display, size: Size { width: length(CONTAINER), height: length(CONTAINER) }, ..Default::default() },
            &[child],
        )
        .unwrap();

    tree.compute_layout(node, Size::MAX_CONTENT).unwrap();
    *tree.layout(node).unwrap()
}

#[test]
fn scroll_container_in_one_axis_clips_both_axes() {
    for display in [Display::Block, Display::Flex, Display::Grid] {
        // `Hidden`/`Scroll` in either axis makes the child a scroll container, which clips
        // overflow in both axes: the grandchild's y overflow must not propagate.
        let layout = nested_overflow_layout(display, Point { x: Overflow::Hidden, y: Overflow::Visible });

        assert_eq!(layout.scrollable_overflow_rect.bottom, CONTAINER, "{display:?}");
    }
}

#[test]
fn clip_in_one_axis_does_not_clip_the_other() {
    for display in [Display::Block, Display::Flex, Display::Grid] {
        // `Clip` does not establish a scroll container: overflow in the other (visible) axis
        // still propagates.
        let layout = nested_overflow_layout(display, Point { x: Overflow::Clip, y: Overflow::Visible });

        assert_eq!(layout.scrollable_overflow_rect.bottom, CONTAINER + CHILD, "{display:?}");
    }
}

#[test]
fn wholly_unreachable_boxes_are_excluded_for_rtl_scroll_containers() {
    for display in [Display::Block, Display::Flex, Display::Grid] {
        // In RTL the scroll origin is the top-right corner: a child entirely past the
        // *right* edge is wholly unreachable, leaving an empty rect at the scroll origin.
        let layout = container_with_absolute_child(display, Overflow::Scroll, Direction::Rtl, None, Some(-100.0));

        let expected = Rect { left: CONTAINER, right: CONTAINER, top: 0.0, bottom: 0.0 };
        assert_eq!(layout.scrollable_overflow_rect.rect, expected, "{display:?}");
        assert_eq!(layout.scroll_width(), 0.0, "{display:?}");
        assert_eq!(layout.scroll_range().left, 0.0, "{display:?}");
    }
}

#[test]
fn end_side_overflow_is_captured_for_rtl_scroll_containers() {
    for display in [Display::Block, Display::Flex, Display::Grid] {
        // In RTL, overflow past the *left* edge is reachable, and is represented by a
        // negative `left` (it is reached by scrolling to negative scroll offsets).
        let layout = container_with_absolute_child(display, Overflow::Scroll, Direction::Rtl, Some(-CHILD), None);

        assert_eq!(layout.scrollable_overflow_rect.left, -CHILD, "{display:?}");
        assert_eq!(layout.scrollable_overflow_rect.right, CONTAINER, "{display:?}");
        assert_eq!(layout.scroll_width(), CHILD, "{display:?}");
        assert_eq!(layout.scroll_size().width, CONTAINER + CHILD, "{display:?}");
        assert_eq!(layout.scroll_range().left, -CHILD, "{display:?}");
        assert_eq!(layout.scroll_range().right, 0.0, "{display:?}");
        assert!(layout.scrollable_overflow_rect.scroll_origin_at_end().x, "{display:?}");
    }
}

/// A 100x100 flex scroll container with a single 300x300 in-flow item.
fn overflowing_flex_scroller(
    direction: Direction,
    flex_direction: FlexDirection,
    flex_wrap: FlexWrap,
    padding: f32,
) -> Layout {
    const SIZE: f32 = 100.0;
    const ITEM: f32 = 300.0;
    let mut tree = new_test_tree();
    let item = tree
        .new_leaf(Style {
            flex_shrink: 0.0,
            size: Size { width: length(ITEM), height: length(ITEM) },
            ..Default::default()
        })
        .unwrap();
    let node = tree
        .new_with_children(
            Style {
                display: Display::Flex,
                direction,
                flex_direction,
                flex_wrap,
                align_items: Some(AlignItems::FLEX_START),
                align_content: Some(AlignContent::FLEX_START),
                padding: Rect {
                    left: length(padding),
                    right: length(padding),
                    top: length(padding),
                    bottom: length(padding),
                },
                size: Size { width: length(SIZE), height: length(SIZE) },
                overflow: Point { x: Overflow::Scroll, y: Overflow::Scroll },
                ..Default::default()
            },
            &[item],
        )
        .unwrap();

    tree.compute_layout(node, Size::MAX_CONTENT).unwrap();
    *tree.layout(node).unwrap()
}

#[test]
fn reverse_flex_directions_have_reachable_start_side_overflow() {
    // (direction, flex-direction, flex-wrap, scroll origin at end in x, scroll origin at end in y)
    let cases = [
        (Direction::Ltr, FlexDirection::Row, FlexWrap::NoWrap, false, false),
        (Direction::Ltr, FlexDirection::RowReverse, FlexWrap::NoWrap, true, false),
        (Direction::Rtl, FlexDirection::Row, FlexWrap::NoWrap, true, false),
        (Direction::Rtl, FlexDirection::RowReverse, FlexWrap::NoWrap, false, false),
        (Direction::Ltr, FlexDirection::Row, FlexWrap::WrapReverse, false, true),
        (Direction::Ltr, FlexDirection::Column, FlexWrap::NoWrap, false, false),
        (Direction::Ltr, FlexDirection::ColumnReverse, FlexWrap::NoWrap, false, true),
        (Direction::Ltr, FlexDirection::Column, FlexWrap::WrapReverse, true, false),
        (Direction::Rtl, FlexDirection::Column, FlexWrap::WrapReverse, false, false),
        (Direction::Rtl, FlexDirection::ColumnReverse, FlexWrap::Wrap, true, true),
    ];
    for (direction, flex_direction, flex_wrap, x_at_end, y_at_end) in cases {
        let layout = overflowing_flex_scroller(direction, flex_direction, flex_wrap, 0.0);
        let msg = format!("{direction:?} {flex_direction:?} {flex_wrap:?}");
        let rect = layout.scrollable_overflow_rect;

        // The whole 300px item is reachable in both axes regardless of where the origin is
        assert_eq!(layout.scroll_size(), Size { width: 300.0, height: 300.0 }, "{msg}");
        assert_eq!(layout.scroll_width(), 200.0, "{msg}");
        assert_eq!(layout.scroll_height(), 200.0, "{msg}");
        assert_eq!(rect.scroll_origin_at_end(), Point { x: x_at_end, y: y_at_end }, "{msg}");

        let expected_rect = Rect {
            left: if x_at_end { -200.0 } else { 0.0 },
            right: if x_at_end { 100.0 } else { 300.0 },
            top: if y_at_end { -200.0 } else { 0.0 },
            bottom: if y_at_end { 100.0 } else { 300.0 },
        };
        assert_eq!(rect.rect, expected_rect, "{msg}");

        let expected_range = Rect {
            left: if x_at_end { -200.0 } else { 0.0 },
            right: if x_at_end { 0.0 } else { 200.0 },
            top: if y_at_end { -200.0 } else { 0.0 },
            bottom: if y_at_end { 0.0 } else { 200.0 },
        };
        assert_eq!(layout.scroll_range(), expected_range, "{msg}");
        assert_eq!(layout.clamp_scroll_offset(Point { x: 0.0, y: 0.0 }), Point::ZERO, "{msg}");
        assert_eq!(
            layout.clamp_scroll_offset(Point { x: -1000.0, y: 1000.0 }),
            Point { x: expected_range.left, y: expected_range.bottom },
            "{msg}"
        );
    }
}

#[test]
fn reverse_flex_scroll_container_padding_extends_reachable_side() {
    // Padding at the far (from the scroll origin) end of the content is part of the
    // scrollable overflow area.
    let layout = overflowing_flex_scroller(Direction::Ltr, FlexDirection::RowReverse, FlexWrap::NoWrap, 10.0);
    // The scrollport is the 100x100 padding box. The 300px item's right edge is at x=90 so it
    // extends to x=-210, and the left padding extends the overflow to x=-220. Vertically the
    // item spans y=10 to y=310 and the bottom padding extends the overflow to y=320.
    assert_eq!(layout.scrollable_overflow_rect.rect, Rect { left: -220.0, right: 100.0, top: 0.0, bottom: 320.0 });
    assert_eq!(layout.scroll_size(), Size { width: 320.0, height: 320.0 });
    assert_eq!(layout.scroll_range(), Rect { left: -220.0, right: 0.0, top: 0.0, bottom: 220.0 });
}
