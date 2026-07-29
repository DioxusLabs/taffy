//! The maximum scroll offset is the content size less the size of the padding
//! box, so it must not depend on which side a border sits on, and must agree
//! across the layout algorithms.

use taffy::geometry::Point;
use taffy::prelude::*;
use taffy::style::{BoxSizing, Display, Overflow};
use taffy_test_helpers::new_test_tree;

const CONTENT: f32 = 1000.0;
const CONTAINER: f32 = 200.0;
const BORDER: f32 = 20.0;

fn scroller(display: Display, border: Rect<LengthPercentage>) -> Layout {
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
                overflow: Point { x: Overflow::Scroll, y: Overflow::Scroll },
                ..Default::default()
            },
            &[child],
        )
        .unwrap();

    tree.compute_layout(node, Size::MAX_CONTENT).unwrap();
    *tree.layout(node).unwrap()
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
fn content_size_excludes_the_containers_own_border() {
    for display in [Display::Block, Display::Flex, Display::Grid] {
        for (top, bottom) in [(BORDER, 0.0), (0.0, BORDER), (BORDER, BORDER), (0.0, 0.0)] {
            let layout = scroller(display, edge(top, bottom));

            assert_eq!(layout.content_size.height, CONTENT, "{display:?} with border {top}/{bottom}");
        }
    }
}
