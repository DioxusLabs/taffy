//! The scrollable overflow rectangle captures start-side (negative) overflow as well as
//! end-side overflow. Boxes wholly in the unreachable scrollable overflow region are
//! excluded for scroll containers, while boxes only partially in the unreachable region
//! contribute their border box, clipped to the reachable side of each scroll origin.

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

        assert_eq!(layout.scrollable_overflow_rect.rect(), Rect::ZERO, "{display:?}");
        assert_eq!(layout.scroll_width(), 0.0, "{display:?}");
    }
}

#[test]
fn wholly_unreachable_boxes_contribute_to_non_scroll_containers() {
    for display in [Display::Block, Display::Flex, Display::Grid] {
        // A box that is not a scroll container has no unreachable region of its own, so
        // start-side overflow is captured in its scrollable overflow rect.
        let layout = container_with_absolute_child(display, Overflow::Visible, Direction::Ltr, Some(-100.0), None);

        assert_eq!(layout.scrollable_overflow_rect.rect().left, -100.0, "{display:?}");
    }
}

#[test]
fn partially_unreachable_boxes_are_clipped_at_the_normal_origin() {
    for display in [Display::Block, Display::Flex, Display::Grid] {
        // Child straddling the scroll origin (x from -25 to 25): the unreachable part is clipped.
        let layout = container_with_absolute_child(display, Overflow::Scroll, Direction::Ltr, Some(-25.0), None);

        assert_eq!(layout.scrollable_overflow_rect.rect().left, 0.0, "{display:?}");
        assert_eq!(layout.scrollable_overflow_rect.rect().right, 25.0, "{display:?}");
        // Only the reachable extent counts towards the scroll width
        assert_eq!(layout.scroll_width(), 0.0, "{display:?}");
    }
}

#[test]
fn end_side_overflow_is_captured_and_scrollable() {
    for display in [Display::Block, Display::Flex, Display::Grid] {
        // Child extending past the end edge (x from 200 to 250)
        let layout = container_with_absolute_child(display, Overflow::Scroll, Direction::Ltr, Some(CONTAINER), None);

        assert_eq!(layout.scrollable_overflow_rect.rect().right, CONTAINER + CHILD, "{display:?}");
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

        assert_eq!(layout.scrollable_overflow_rect.rect().right, 20.0, "{display:?}");
        assert_eq!(layout.scrollable_overflow_rect.rect().bottom, 30.0, "{display:?}");
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

        assert_eq!(layout.scrollable_overflow_rect.rect().bottom, CONTAINER, "{display:?}");
    }
}

#[test]
fn clip_in_one_axis_does_not_clip_the_other() {
    for display in [Display::Block, Display::Flex, Display::Grid] {
        // `Clip` does not establish a scroll container: overflow in the other (visible) axis
        // still propagates.
        let layout = nested_overflow_layout(display, Point { x: Overflow::Clip, y: Overflow::Visible });

        assert_eq!(layout.scrollable_overflow_rect.rect().bottom, CONTAINER + CHILD, "{display:?}");
    }
}

#[test]
fn wholly_unreachable_boxes_are_excluded_for_rtl_scroll_containers() {
    for display in [Display::Block, Display::Flex, Display::Grid] {
        // In RTL the scroll origin is the top-right corner: a child entirely past the
        // *right* edge is wholly unreachable.
        let layout = container_with_absolute_child(display, Overflow::Scroll, Direction::Rtl, None, Some(-100.0));

        assert_eq!(layout.scrollable_overflow_rect.rect().left, 0.0, "{display:?}");
        assert_eq!(layout.scrollable_overflow_rect.is_reversed(), Point { x: true, y: false }, "{display:?}");
        assert_eq!(layout.scroll_width(), 0.0, "{display:?}");
    }
}

#[test]
fn end_side_overflow_is_captured_for_rtl_scroll_containers() {
    for display in [Display::Block, Display::Flex, Display::Grid] {
        // In RTL, overflow past the *left* edge is reachable end-side overflow.
        let layout = container_with_absolute_child(display, Overflow::Scroll, Direction::Rtl, Some(-CHILD), None);

        assert_eq!(layout.scrollable_overflow_rect.rect().left, -CHILD, "{display:?}");
        assert_eq!(layout.scroll_width(), CHILD, "{display:?}");
    }
}

fn flex_overflow_layout(direction: Direction, flex_direction: FlexDirection, wrap: FlexWrap, absolute: bool) -> Layout {
    let mut tree = new_test_tree();
    let child = tree
        .new_leaf(Style {
            position: if absolute { Position::Absolute } else { Position::Relative },
            flex_shrink: 0.0,
            size: Size { width: length(300.0), height: length(300.0) },
            ..Default::default()
        })
        .unwrap();
    let node = tree
        .new_with_children(
            Style {
                display: Display::Flex,
                direction,
                flex_direction,
                flex_wrap: wrap,
                position: Position::Relative,
                overflow: Point { x: Overflow::Scroll, y: Overflow::Scroll },
                size: Size { width: length(CONTAINER), height: length(CONTAINER) },
                ..Default::default()
            },
            &[child],
        )
        .unwrap();
    tree.compute_layout(node, Size::MAX_CONTENT).unwrap();
    *tree.layout(node).unwrap()
}

#[test]
fn flex_reversed_origins_include_negative_overflow() {
    for direction in [Direction::Ltr, Direction::Rtl] {
        for flex_direction in
            [FlexDirection::Row, FlexDirection::RowReverse, FlexDirection::Column, FlexDirection::ColumnReverse]
        {
            for wrap in [FlexWrap::NoWrap, FlexWrap::WrapReverse] {
                // Exercise the same origins for in-flow and absolutely positioned flex children.
                for absolute in [false, true] {
                    let layout = flex_overflow_layout(direction, flex_direction, wrap, absolute);
                    let main_reversed =
                        matches!(flex_direction, FlexDirection::RowReverse | FlexDirection::ColumnReverse);
                    let (x, y) = if matches!(flex_direction, FlexDirection::Row | FlexDirection::RowReverse) {
                        (main_reversed, wrap == FlexWrap::WrapReverse)
                    } else {
                        (wrap == FlexWrap::WrapReverse, main_reversed)
                    };
                    let reversed = Point { x: x ^ direction.is_rtl(), y };
                    let context = (direction, flex_direction, wrap, absolute);
                    assert_eq!(layout.scrollable_overflow_rect.is_reversed(), reversed, "{context:?}");
                    let rect = layout.scrollable_overflow_rect.rect();
                    assert_eq!(rect.left, if reversed.x { -100.0 } else { 0.0 }, "{context:?}");
                    assert_eq!(rect.top, if reversed.y { -100.0 } else { 0.0 }, "{context:?}");
                    assert_eq!(layout.scroll_width(), 100.0, "{context:?}");
                    assert_eq!(layout.scroll_height(), 100.0, "{context:?}");
                }
            }
        }
    }
}

#[test]
fn wrap_reverse_lines_overflow_above_the_scrollport() {
    for direction in [Direction::Ltr, Direction::Rtl] {
        let mut tree = new_test_tree();
        let mut children = Vec::new();
        for _ in 0..3 {
            children.push(
                tree.new_leaf(Style {
                    flex_shrink: 0.0,
                    size: Size { width: length(100.0), height: length(100.0) },
                    ..Default::default()
                })
                .unwrap(),
            );
        }
        let node = tree
            .new_with_children(
                Style {
                    display: Display::Flex,
                    direction,
                    flex_wrap: FlexWrap::WrapReverse,
                    size: Size { width: length(200.0), height: length(100.0) },
                    overflow: Point { x: Overflow::Scroll, y: Overflow::Scroll },
                    ..Default::default()
                },
                &children,
            )
            .unwrap();
        tree.compute_layout(node, Size::MAX_CONTENT).unwrap();
        let layout = tree.layout(node).unwrap();
        assert_eq!(tree.layout(children[2]).unwrap().location.y, -100.0);
        assert_eq!(layout.scrollable_overflow_rect.rect().top, -100.0);
        assert_eq!(layout.scrollable_overflow_rect.is_reversed(), Point { x: direction.is_rtl(), y: true });
        assert_eq!(layout.scroll_height(), 100.0);
    }
}

#[test]
fn empty_scroll_ranges_and_visible_container_origins() {
    for display in [Display::Block, Display::Flex, Display::Grid] {
        for overflow in [Overflow::Visible, Overflow::Scroll] {
            let mut tree = new_test_tree();
            let node = tree
                .new_leaf(Style {
                    display,
                    direction: Direction::Rtl,
                    flex_direction: FlexDirection::ColumnReverse,
                    flex_wrap: FlexWrap::WrapReverse,
                    size: Size { width: length(CONTAINER), height: length(CONTAINER) },
                    overflow: Point { x: overflow, y: overflow },
                    ..Default::default()
                })
                .unwrap();
            tree.compute_layout(node, Size::MAX_CONTENT).unwrap();
            let layout = tree.layout(node).unwrap();
            if overflow == Overflow::Visible {
                assert_eq!(layout.scrollable_overflow_rect.is_reversed(), Point { x: false, y: false });
            }
            assert_eq!(layout.scroll_width(), 0.0);
            assert_eq!(layout.scroll_height(), 0.0);
        }
    }
}

#[test]
fn negative_overflow_is_preserved_for_visible_flex_containers() {
    let mut tree = new_test_tree();
    let child = tree
        .new_leaf(Style {
            flex_shrink: 0.0,
            size: Size { width: length(300.0), height: length(300.0) },
            ..Default::default()
        })
        .unwrap();
    let node = tree
        .new_with_children(
            Style {
                display: Display::Flex,
                flex_direction: FlexDirection::ColumnReverse,
                flex_wrap: FlexWrap::WrapReverse,
                size: Size { width: length(CONTAINER), height: length(CONTAINER) },
                ..Default::default()
            },
            &[child],
        )
        .unwrap();
    tree.compute_layout(node, Size::MAX_CONTENT).unwrap();
    let overflow = tree.layout(node).unwrap().scrollable_overflow_rect;
    assert_eq!(overflow.is_reversed(), Point { x: false, y: false });
    assert_eq!(overflow.rect().left, -100.0);
    assert_eq!(overflow.rect().top, -100.0);
}

#[test]
fn reversed_flex_origins_clip_unreachable_absolute_overflow() {
    for flex_direction in [FlexDirection::RowReverse, FlexDirection::ColumnReverse] {
        for offset in [-100.0_f32, -25.0, 175.0, 200.0] {
            let mut tree = new_test_tree();
            let row = flex_direction == FlexDirection::RowReverse;
            let child = tree
                .new_leaf(Style {
                    position: Position::Absolute,
                    inset: Rect {
                        left: length(if row { offset } else { 0.0 }),
                        top: length(if row { 0.0 } else { offset }),
                        right: auto(),
                        bottom: auto(),
                    },
                    size: Size { width: length(CHILD), height: length(CHILD) },
                    ..Default::default()
                })
                .unwrap();
            let node = tree
                .new_with_children(
                    Style {
                        display: Display::Flex,
                        flex_direction,
                        position: Position::Relative,
                        size: Size { width: length(CONTAINER), height: length(CONTAINER) },
                        overflow: Point { x: Overflow::Scroll, y: Overflow::Scroll },
                        ..Default::default()
                    },
                    &[child],
                )
                .unwrap();
            tree.compute_layout(node, Size::MAX_CONTENT).unwrap();
            let layout = tree.layout(node).unwrap();
            let rect = layout.scrollable_overflow_rect.rect();
            assert!(rect.right <= CONTAINER && rect.bottom <= CONTAINER);
            assert_eq!(
                if row { layout.scroll_width() } else { layout.scroll_height() },
                (-offset).max(0.0),
                "{flex_direction:?} {offset}"
            );
        }
    }
}

#[test]
fn reversed_flex_scroll_ranges_include_padding_and_exclude_borders() {
    for direction in [Direction::Ltr, Direction::Rtl] {
        for flex_direction in
            [FlexDirection::Row, FlexDirection::RowReverse, FlexDirection::Column, FlexDirection::ColumnReverse]
        {
            for wrap in [FlexWrap::NoWrap, FlexWrap::WrapReverse] {
                let mut tree = new_test_tree();
                let child = tree
                    .new_leaf(Style {
                        flex_shrink: 0.0,
                        size: Size { width: length(300.0), height: length(300.0) },
                        ..Default::default()
                    })
                    .unwrap();
                let node = tree
                    .new_with_children(
                        Style {
                            display: Display::Flex,
                            direction,
                            flex_direction,
                            flex_wrap: wrap,
                            size: Size { width: length(CONTAINER), height: length(CONTAINER) },
                            border: Rect {
                                left: length(20.0),
                                right: length(20.0),
                                top: length(20.0),
                                bottom: length(20.0),
                            },
                            padding: Rect {
                                left: length(10.0),
                                right: length(20.0),
                                top: length(30.0),
                                bottom: length(40.0),
                            },
                            overflow: Point { x: Overflow::Scroll, y: Overflow::Scroll },
                            ..Default::default()
                        },
                        &[child],
                    )
                    .unwrap();
                tree.compute_layout(node, Size::MAX_CONTENT).unwrap();
                let layout = tree.layout(node).unwrap();
                let context = (direction, flex_direction, wrap);
                assert_eq!(layout.scroll_width(), 170.0, "{context:?}");
                assert_eq!(layout.scroll_height(), 210.0, "{context:?}");
            }
        }
    }
}
