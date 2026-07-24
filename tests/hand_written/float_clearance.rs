//! Regression tests for clearance calculation and its interaction with margin collapsing.
//!
//! Scenarios are reduced from WPT tests in css/CSS2/floats-clear and css/CSS2/floats.
use taffy::prelude::*;
use taffy::{Clear, Float, Overflow, Point};

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

/// Reduced from WPT css/CSS2/floats-clear/clear-with-top-margin-after-cleared-empty-block.html
///
/// Clearance on a self-collapsing block suppresses margin collapsing with following
/// siblings, and a following cleared sibling with a large top margin gets no clearance.
#[test]
fn clear_with_top_margin_after_cleared_empty_block() {
    let t = &mut TaffyTree::new();
    let float = block(
        t,
        Style { float: Float::Left, size: Size { width: length(100.0), height: length(20.0) }, ..Default::default() },
        &[],
    );
    let c1 = block(t, Style { clear: Clear::Both, ..Default::default() }, &[]);
    let c2 = block(
        t,
        Style {
            clear: Clear::Both,
            margin: Rect { top: length(100.0), ..Rect::zero() },
            size: Size { width: auto(), height: length(20.0) },
            ..Default::default()
        },
        &[],
    );
    let container = block(
        t,
        Style {
            size: Size { width: length(100.0), height: auto() },
            border: Rect { top: length(1.0), ..Rect::zero() },
            ..Default::default()
        },
        &[float, c1, c2],
    );
    t.compute_layout(container, Size::MAX_CONTENT).unwrap();

    assert_layout(t, c1, 0.0, 21.0, 100.0, 0.0);
    assert_layout(t, c2, 0.0, 121.0, 100.0, 20.0);
    assert_layout(t, container, 0.0, 0.0, 100.0, 141.0);
}

/// Reduced from WPT css/CSS2/floats/new-fc-separates-from-float.html
///
/// A block establishing a new formatting context that does not fit beside a float is
/// pushed below it, and its top margin no longer collapses with preceding margins
/// (so the float, which adjoins the unresolved margin strut, does not move down).
#[test]
fn new_fc_separates_from_float() {
    let t = &mut TaffyTree::new();
    let float = block(
        t,
        Style { float: Float::Left, size: Size { width: length(200.0), height: length(200.0) }, ..Default::default() },
        &[],
    );
    let inner = block(t, Style::default(), &[float]);
    let bfc = block(
        t,
        Style {
            overflow: Point { x: Overflow::Hidden, y: Overflow::Hidden },
            margin: Rect { top: length(200.0), ..Rect::zero() },
            size: Size { width: length(200.0), height: length(1.0) },
            ..Default::default()
        },
        &[],
    );
    let mid = block(t, Style::default(), &[inner, bfc]);
    let container = block(
        t,
        Style {
            overflow: Point { x: Overflow::Hidden, y: Overflow::Hidden },
            size: Size { width: length(200.0), height: auto() },
            ..Default::default()
        },
        &[mid],
    );
    t.compute_layout(container, Size::MAX_CONTENT).unwrap();

    assert_layout(t, mid, 0.0, 0.0, 200.0, 201.0);
    assert_layout(t, float, 0.0, 0.0, 200.0, 200.0);
    assert_layout(t, bfc, 0.0, 200.0, 200.0, 1.0);
    assert_layout(t, container, 0.0, 0.0, 200.0, 201.0);
}

/// Reduced from WPT css/CSS2/floats-clear/margin-collapse-clear-013.xht
///
/// The top and bottom margins of a self-collapsing block with clearance collapse with
/// each other and are applied inside the parent (they do not collapse with the parent's
/// bottom margin).
#[test]
fn self_collapsing_block_with_clearance_margins() {
    let t = &mut TaffyTree::new();
    let float = block(
        t,
        Style { float: Float::Left, size: Size { width: length(100.0), height: length(100.0) }, ..Default::default() },
        &[],
    );
    let cleared = block(
        t,
        Style {
            clear: Clear::Left,
            margin: Rect { top: length(40.0), bottom: length(140.0), ..Rect::zero() },
            ..Default::default()
        },
        &[],
    );
    let parent = block(t, Style::default(), &[float, cleared]);
    let next = block(t, Style { size: Size { width: auto(), height: length(100.0) }, ..Default::default() }, &[]);
    let root =
        block(t, Style { size: Size { width: length(300.0), height: auto() }, ..Default::default() }, &[parent, next]);
    t.compute_layout(root, Size::MAX_CONTENT).unwrap();

    assert_layout(t, cleared, 0.0, 100.0, 300.0, 0.0);
    assert_layout(t, parent, 0.0, 0.0, 300.0, 200.0);
    assert_layout(t, next, 0.0, 200.0, 300.0, 100.0);
}
