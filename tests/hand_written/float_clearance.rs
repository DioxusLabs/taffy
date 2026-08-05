//! Regression tests for clearance calculation and its interaction with margin collapsing.
//!
//! Scenarios are reduced from WPT tests in css/CSS2/floats-clear and css/CSS2/floats.
use taffy::prelude::*;
use taffy::{Clear, Float};

fn block(tree: &mut TaffyTree<()>, style: Style, children: &[NodeId]) -> NodeId {
    tree.new_with_children(Style { display: Display::Block, ..style }, children).unwrap()
}

fn assert_layout(tree: &TaffyTree<()>, node: NodeId, x: f32, y: f32, width: f32, height: f32) {
    let layout = tree.layout(node).unwrap();
    assert_eq!(layout.location.x, x, "x of {node:?}");
    assert_eq!(layout.location.y, y, "y of {node:?}");
    assert_eq!(layout.size.width, width, "width of {node:?}");
    assert_eq!(layout.size.height, height, "height of {node:?}");
}

/// Reduced from WPT css/CSS2/floats-clear/clear-002.xht
///
/// A cleared float should clear past prior floats, but `clear` on the first float
/// (with no prior floats on the relevant side) should not move it down.
#[test]
fn clear_on_float_with_and_without_preceding_float() {
    let t = &mut TaffyTree::new();
    let f1 = block(
        t,
        Style { float: Float::Right, size: Size { width: length(96.0), height: length(96.0) }, ..Default::default() },
        &[],
    );
    let f2 = block(
        t,
        Style {
            float: Float::Right,
            clear: Clear::Right,
            size: Size { width: length(96.0), height: length(96.0) },
            ..Default::default()
        },
        &[],
    );
    let container =
        block(t, Style { size: Size { width: length(500.0), height: auto() }, ..Default::default() }, &[f1, f2]);
    t.compute_layout(container, Size::MAX_CONTENT).unwrap();

    assert_layout(t, f1, 404.0, 0.0, 96.0, 96.0);
    assert_layout(t, f2, 404.0, 96.0, 96.0, 96.0);
}

/// Reduced from WPT css/CSS2/floats-clear/clear-clearance-calculation-004.xht
///
/// When the hypothetical position of a cleared block (with its top margin) is already
/// past the bottom of the float, negative clearance keeps it at the float's bottom edge.
#[test]
fn negative_clearance_from_large_top_margin() {
    let t = &mut TaffyTree::new();
    let first = block(
        t,
        Style {
            size: Size { width: auto(), height: length(25.0) },
            margin: Rect { bottom: length(25.0), ..Rect::zero() },
            ..Default::default()
        },
        &[],
    );
    let float = block(
        t,
        Style { float: Float::Left, size: Size { width: length(50.0), height: length(50.0) }, ..Default::default() },
        &[],
    );
    let last = block(
        t,
        Style {
            clear: Clear::Left,
            margin: Rect { top: length(75.0), ..Rect::zero() },
            size: Size { width: auto(), height: length(50.0) },
            ..Default::default()
        },
        &[],
    );
    let container = block(
        t,
        Style { size: Size { width: length(300.0), height: auto() }, ..Default::default() },
        &[first, float, last],
    );
    t.compute_layout(container, Size::MAX_CONTENT).unwrap();

    assert_layout(t, float, 0.0, 50.0, 50.0, 50.0);
    assert_layout(t, last, 0.0, 100.0, 300.0, 50.0);
    assert_layout(t, container, 0.0, 0.0, 300.0, 150.0);
}
