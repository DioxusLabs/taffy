#![cfg(feature = "block_layout")]
use taffy::prelude::*;
use taffy_test_helpers::new_test_tree;

fn block(margin_top: f32, margin_bottom: f32, height: Dimension) -> Style {
    Style {
        display: Display::Block,
        size: Size { width: auto(), height },
        margin: Rect { left: zero(), right: zero(), top: length(margin_top), bottom: length(margin_bottom) },
        ..Default::default()
    }
}

/// Margin collapsing changes where boxes are placed, but `Layout::margin` is each box's own margin
#[test]
fn layout_margin_excludes_collapsed_margins() {
    let mut taffy = new_test_tree();
    let inner = taffy.new_leaf(block(13.0, 20.0, length(10.0))).unwrap();
    let outer = taffy.new_with_children(block(8.0, 5.0, auto()), &[inner]).unwrap();
    let sibling = taffy.new_leaf(block(30.0, 0.0, length(10.0))).unwrap();
    let root = taffy
        .new_with_children(
            Style {
                display: Display::FlowRoot,
                size: Size { width: length(100.0), height: auto() },
                ..Default::default()
            },
            &[outer, sibling],
        )
        .unwrap();

    taffy.compute_layout(root, Size::MAX_CONTENT).unwrap();

    // The inner box's margins collapse through the outer box's edges
    let outer_layout = taffy.layout(outer).unwrap();
    assert_eq!(outer_layout.location.y, 13.0);
    assert_eq!(outer_layout.size.height, 10.0);
    assert_eq!(outer_layout.margin.top, 8.0);
    assert_eq!(outer_layout.margin.bottom, 5.0);

    let inner_layout = taffy.layout(inner).unwrap();
    assert_eq!(inner_layout.location.y, 0.0);
    assert_eq!(inner_layout.margin.top, 13.0);
    assert_eq!(inner_layout.margin.bottom, 20.0);

    // The sibling is placed after the largest of the adjoining margins (5, 20, 30)
    let sibling_layout = taffy.layout(sibling).unwrap();
    assert_eq!(sibling_layout.location.y, 53.0);
    assert_eq!(sibling_layout.margin.top, 30.0);
}
