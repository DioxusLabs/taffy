//! The maximum scroll offset is the content size less the size of the padding
//! box, so it must not depend on which side a border sits on, and must agree
//! across the layout algorithms.

use taffy::geometry::Point;
use taffy::prelude::*;
use taffy::style::{BoxSizing, Display, Overflow};
use taffy_test_helpers::{new_test_tree, test_measure_function, TestNodeContext};

const CONTENT: f32 = 1000.0;
const CONTAINER: f32 = 200.0;
const BORDER: f32 = 20.0;
const PADDING: f32 = 20.0;

fn scroller_with_padding(display: Display, border: Rect<LengthPercentage>, padding: Rect<LengthPercentage>) -> Layout {
    let mut tree = new_test_tree();
    let child = tree
        .new_leaf(Style { size: Size { width: length(100.0), height: length(CONTENT) }, ..Default::default() })
        .unwrap();
    let node = tree
        .new_with_children(
            Style {
                display,
                box_sizing: BoxSizing::BorderBox,
                size: Size { width: length(300.0), height: length(CONTAINER) },
                border,
                padding,
                overflow: Point { x: Overflow::Scroll, y: Overflow::Scroll },
                ..Default::default()
            },
            &[child],
        )
        .unwrap();

    tree.compute_layout(node, Size::MAX_CONTENT).unwrap();
    *tree.layout(node).unwrap()
}

fn scroller(display: Display, border: Rect<LengthPercentage>) -> Layout {
    scroller_with_padding(display, border, edge(0.0, 0.0))
}

fn edge(top: f32, bottom: f32) -> Rect<LengthPercentage> {
    Rect {
        left: LengthPercentage::from_length(0.0),
        right: LengthPercentage::from_length(0.0),
        top: LengthPercentage::from_length(top),
        bottom: LengthPercentage::from_length(bottom),
    }
}

#[test]
fn scroll_height_does_not_depend_on_which_edge_the_border_is_on() {
    for display in [Display::Block, Display::Flex, Display::Grid] {
        let top = scroller(display, edge(BORDER, 0.0));
        let bottom = scroller(display, edge(0.0, BORDER));

        assert_eq!(
            top.scroll_height(),
            bottom.scroll_height(),
            "{display:?}: border-top gave {} and border-bottom gave {}",
            top.scroll_height(),
            bottom.scroll_height(),
        );
    }
}

#[test]
fn scroll_height_is_the_content_beyond_the_padding_box() {
    for display in [Display::Block, Display::Flex, Display::Grid] {
        for (top, bottom) in [(BORDER, 0.0), (0.0, BORDER), (BORDER, BORDER), (0.0, 0.0)] {
            let layout = scroller(display, edge(top, bottom));
            let padding_box = CONTAINER - top - bottom;

            assert_eq!(layout.scroll_height(), CONTENT - padding_box, "{display:?} with border {top}/{bottom}",);
        }
    }
}

#[test]
fn scrollable_overflow_excludes_the_containers_own_border() {
    for display in [Display::Block, Display::Flex, Display::Grid] {
        for (top, bottom) in [(BORDER, 0.0), (0.0, BORDER), (BORDER, BORDER), (0.0, 0.0)] {
            let layout = scroller(display, edge(top, bottom));

            assert_eq!(layout.scrollable_overflow_rect.bottom, CONTENT, "{display:?} with border {top}/{bottom}");
        }
    }
}

#[test]
fn scrollable_overflow_includes_the_containers_own_padding() {
    for display in [Display::Block, Display::Flex, Display::Grid] {
        for (top, bottom) in [(PADDING, 0.0), (0.0, PADDING), (PADDING, PADDING), (0.0, 0.0)] {
            let layout = scroller_with_padding(display, edge(0.0, 0.0), edge(top, bottom));

            assert_eq!(
                layout.scrollable_overflow_rect.bottom,
                CONTENT + top + bottom,
                "{display:?} with padding {top}/{bottom}"
            );
        }
    }
}

#[test]
fn scrollable_overflow_excludes_the_own_padding_of_a_non_scroll_container() {
    for display in [Display::Block, Display::Flex, Display::Grid] {
        for (top, bottom) in [(PADDING, 0.0), (0.0, PADDING), (PADDING, PADDING), (0.0, 0.0)] {
            let mut tree = new_test_tree();
            let child = tree
                .new_leaf(Style { size: Size { width: length(100.0), height: length(CONTENT) }, ..Default::default() })
                .unwrap();
            let node = tree
                .new_with_children(
                    Style {
                        display,
                        box_sizing: BoxSizing::BorderBox,
                        size: Size { width: length(300.0), height: length(CONTAINER) },
                        padding: edge(top, bottom),
                        ..Default::default()
                    },
                    &[child],
                )
                .unwrap();

            tree.compute_layout(node, Size::MAX_CONTENT).unwrap();
            let layout = tree.layout(node).unwrap();

            // A box that is not a scroll container does not extend its scrollable overflow
            // region by its own padding: only the content contributes.
            assert_eq!(
                layout.scrollable_overflow_rect.bottom,
                top + CONTENT,
                "{display:?} with padding {top}/{bottom}"
            );
        }
    }
}

fn measured_leaf(overflow: Overflow, padding: Rect<LengthPercentage>) -> Layout {
    let mut tree = new_test_tree();
    let node = tree
        .new_leaf_with_context(
            Style {
                box_sizing: BoxSizing::BorderBox,
                size: Size { width: length(300.0), height: length(CONTAINER) },
                padding,
                overflow: Point { x: overflow, y: overflow },
                ..Default::default()
            },
            TestNodeContext::fixed(100.0, CONTENT),
        )
        .unwrap();

    tree.compute_layout_with_measure(node, Size::MAX_CONTENT, test_measure_function).unwrap();
    *tree.layout(node).unwrap()
}

#[test]
fn leaf_content_size_includes_the_containers_own_padding() {
    for (top, bottom) in [(PADDING, 0.0), (0.0, PADDING), (PADDING, PADDING), (0.0, 0.0)] {
        let layout = measured_leaf(Overflow::Scroll, edge(top, bottom));

        assert_eq!(layout.content_size.height, top + CONTENT + bottom, "Leaf with padding {top}/{bottom}");
    }
}

#[test]
fn leaf_content_size_excludes_the_own_padding_of_a_non_scroll_container() {
    for (top, bottom) in [(PADDING, 0.0), (0.0, PADDING), (PADDING, PADDING), (0.0, 0.0)] {
        let layout = measured_leaf(Overflow::Visible, edge(top, bottom));

        // A box that is not a scroll container does not extend its scrollable overflow
        // region by its own padding: only the content contributes.
        assert_eq!(layout.content_size.height, top + CONTENT, "Leaf with padding {top}/{bottom}");
    }
}

#[test]
fn scroll_height_accounts_for_the_containers_own_padding() {
    for display in [Display::Block, Display::Flex, Display::Grid] {
        for (top, bottom) in [(PADDING, 0.0), (0.0, PADDING), (PADDING, PADDING), (0.0, 0.0)] {
            let layout = scroller_with_padding(display, edge(0.0, 0.0), edge(top, bottom));

            // The scrollable overflow region runs from the padding-box origin to the end of
            // the content plus the container's end-side padding.
            let expected = (top + CONTENT + bottom) - CONTAINER;
            assert_eq!(layout.scroll_height(), expected, "{display:?} with padding {top}/{bottom}");
        }
    }
}

/// Items in overflowing rows must contribute their position within the container (not just
/// their size) to the container's content size.
#[test]
fn grid_scrollable_overflow_includes_item_positions_in_overflowing_tracks() {
    let mut tree = new_test_tree();
    let child = || Style { size: Size { width: length(100.0), height: length(500.0) }, ..Default::default() };
    let c1 = tree.new_leaf(child()).unwrap();
    let c2 = tree.new_leaf(child()).unwrap();
    let node = tree
        .new_with_children(
            Style {
                display: Display::Grid,
                size: Size { width: length(300.0), height: length(CONTAINER) },
                overflow: Point { x: Overflow::Scroll, y: Overflow::Scroll },
                ..Default::default()
            },
            &[c1, c2],
        )
        .unwrap();

    tree.compute_layout(node, Size::MAX_CONTENT).unwrap();
    let layout = tree.layout(node).unwrap();

    assert_eq!(layout.scrollable_overflow_rect.bottom, 1000.0);
    assert_eq!(layout.scroll_height(), 1000.0 - CONTAINER);
}

#[test]
fn scroll_height_agrees_with_borders_and_padding_combined() {
    for display in [Display::Block, Display::Flex, Display::Grid] {
        let layout = scroller_with_padding(display, edge(BORDER, BORDER), edge(PADDING, PADDING));

        let padding_box = CONTAINER - BORDER - BORDER;
        let expected = (PADDING + CONTENT + PADDING) - padding_box;
        assert_eq!(layout.scroll_height(), expected, "{display:?}");
    }
}
