//! Tests for boxes that establish an independent formatting context (e.g. `overflow: hidden`)
//! laid out beside floats. Per CSS2 §9.5 the border box of such a box must not overlap any
//! floats in the same block formatting context: it either narrows to fit beside the float or
//! moves down below it.
#![cfg(feature = "float_layout")]

use taffy::prelude::*;
use taffy::{Direction, Float, Overflow, Point, Rect};

fn block() -> Style {
    Style { display: taffy::Display::Block, ..Default::default() }
}

fn bfc_root() -> Style {
    Style { overflow: Point { x: Overflow::Hidden, y: Overflow::Hidden }, ..block() }
}

fn container(taffy: &mut TaffyTree<()>, width: f32, direction: Direction, children: &[NodeId]) -> NodeId {
    taffy
        .new_with_children(
            Style { size: Size { width: length(width), height: auto() }, direction, ..block() },
            children,
        )
        .unwrap()
}

fn float_left(taffy: &mut TaffyTree<()>, width: f32, height: f32) -> NodeId {
    taffy
        .new_leaf(Style { float: Float::Left, size: Size { width: length(width), height: length(height) }, ..block() })
        .unwrap()
}

fn compute(taffy: &mut TaffyTree<()>, root: NodeId) {
    taffy.compute_layout(root, Size::MAX_CONTENT).unwrap();
}

fn assert_rect(taffy: &TaffyTree<()>, node: NodeId, x: f32, y: f32, width: f32) {
    let layout = taffy.layout(node).unwrap();
    assert_eq!(layout.location.x, x, "x of {node:?}");
    assert_eq!(layout.location.y, y, "y of {node:?}");
    assert_eq!(layout.size.width, width, "width of {node:?}");
}

/// An auto-width BFC root fits beside the float and stretches to fill the remaining space
#[test]
fn bfc_narrows_beside_float() {
    let mut taffy: TaffyTree<()> = TaffyTree::new();
    let float = float_left(&mut taffy, 50.0, 100.0);
    let bfc = taffy.new_leaf(Style { size: Size { width: auto(), height: length(50.0) }, ..bfc_root() }).unwrap();
    let root = container(&mut taffy, 100.0, Direction::Ltr, &[float, bfc]);
    compute(&mut taffy, root);
    assert_rect(&taffy, bfc, 50.0, 0.0, 50.0);
}

/// A fixed-width BFC root that is too wide to fit beside the float moves below it
#[test]
fn bfc_moves_below_float() {
    let mut taffy: TaffyTree<()> = TaffyTree::new();
    let float = float_left(&mut taffy, 150.0, 150.0);
    let bfc =
        taffy.new_leaf(Style { size: Size { width: length(200.0), height: length(150.0) }, ..bfc_root() }).unwrap();
    let root = container(&mut taffy, 300.0, Direction::Ltr, &[float, bfc]);
    compute(&mut taffy, root);
    assert_rect(&taffy, bfc, 0.0, 150.0, 200.0);
    assert_eq!(taffy.layout(root).unwrap().size.height, 300.0);
}

/// A BFC root with a large top margin separates from an adjoining float (similar to clearance):
/// the margin positions the top of the float, and the BFC is placed below the float
#[test]
fn bfc_separates_from_adjoining_float() {
    let mut taffy: TaffyTree<()> = TaffyTree::new();
    let float = float_left(&mut taffy, 200.0, 200.0);
    let inner = taffy.new_with_children(block(), &[float]).unwrap();
    let bfc = taffy
        .new_leaf(Style {
            size: Size { width: length(200.0), height: length(1.0) },
            margin: Rect { top: length(200.0), left: zero(), right: zero(), bottom: zero() },
            ..bfc_root()
        })
        .unwrap();
    let wrapper = taffy.new_with_children(block(), &[inner, bfc]).unwrap();
    let root = taffy
        .new_with_children(Style { size: Size { width: length(200.0), height: auto() }, ..bfc_root() }, &[wrapper])
        .unwrap();
    compute(&mut taffy, root);
    assert_rect(&taffy, float, 0.0, 0.0, 200.0);
    assert_rect(&taffy, bfc, 0.0, 200.0, 200.0);
    assert_eq!(taffy.layout(root).unwrap().size.height, 201.0);
}

/// A small negative margin on the float side does not pull an auto-width BFC root under the
/// float: the border edge does not move past the float edge
#[test]
fn bfc_negative_margin_beside_float_is_clamped() {
    let mut taffy: TaffyTree<()> = TaffyTree::new();
    let float = float_left(&mut taffy, 50.0, 100.0);
    let bfc = taffy
        .new_leaf(Style {
            size: Size { width: auto(), height: length(100.0) },
            margin: Rect { left: length(-20.0), right: zero(), top: zero(), bottom: zero() },
            ..bfc_root()
        })
        .unwrap();
    let root = container(&mut taffy, 100.0, Direction::Ltr, &[float, bfc]);
    compute(&mut taffy, root);
    assert_rect(&taffy, bfc, 50.0, 0.0, 50.0);
}

/// A large negative margin makes an auto-width BFC root too wide to fit beside the float
/// (its width must be at least the negation of its margins), so it moves below the float
/// and resolves its width against the full containing block
#[test]
fn bfc_large_negative_margin_moves_below_float() {
    let mut taffy: TaffyTree<()> = TaffyTree::new();
    let float = float_left(&mut taffy, 50.0, 100.0);
    let bfc = taffy
        .new_leaf(Style {
            size: Size { width: auto(), height: length(50.0) },
            margin: Rect { left: length(-60.0), right: zero(), top: zero(), bottom: zero() },
            ..bfc_root()
        })
        .unwrap();
    let root = container(&mut taffy, 100.0, Direction::Ltr, &[float, bfc]);
    compute(&mut taffy, root);
    assert_rect(&taffy, bfc, -60.0, 100.0, 160.0);
}

/// A positive margin on the float side is "absorbed" by the float: the border edge is the
/// further-in of the float edge and the margin-inset containing block edge
#[test]
fn bfc_positive_margin_absorbed_by_float() {
    let mut taffy: TaffyTree<()> = TaffyTree::new();
    let float = float_left(&mut taffy, 50.0, 100.0);
    let bfc = taffy
        .new_leaf(Style {
            size: Size { width: auto(), height: length(50.0) },
            margin: Rect { left: length(20.0), right: zero(), top: zero(), bottom: zero() },
            ..bfc_root()
        })
        .unwrap();
    let root = container(&mut taffy, 100.0, Direction::Ltr, &[float, bfc]);
    compute(&mut taffy, root);
    assert_rect(&taffy, bfc, 50.0, 0.0, 50.0);
}

/// A trailing margin reduces the width an auto width resolves to, but does not prevent a
/// fixed-width box from fitting beside the float (it may overflow the containing block)
#[test]
fn bfc_trailing_margin() {
    let mut taffy: TaffyTree<()> = TaffyTree::new();

    // Auto width: trailing margin is subtracted from the stretch width
    let float = float_left(&mut taffy, 50.0, 100.0);
    let bfc = taffy
        .new_leaf(Style {
            size: Size { width: auto(), height: length(50.0) },
            margin: Rect { right: length(10.0), left: zero(), top: zero(), bottom: zero() },
            ..bfc_root()
        })
        .unwrap();
    let root = container(&mut taffy, 100.0, Direction::Ltr, &[float, bfc]);
    compute(&mut taffy, root);
    assert_rect(&taffy, bfc, 50.0, 0.0, 40.0);

    // Fixed width filling the whole space beside the float: the trailing margin
    // does not force the box below the float
    let float = float_left(&mut taffy, 50.0, 100.0);
    let bfc = taffy
        .new_leaf(Style {
            size: Size { width: length(50.0), height: length(50.0) },
            margin: Rect { right: length(10.0), left: zero(), top: zero(), bottom: zero() },
            ..bfc_root()
        })
        .unwrap();
    let root = container(&mut taffy, 100.0, Direction::Ltr, &[float, bfc]);
    compute(&mut taffy, root);
    assert_rect(&taffy, bfc, 50.0, 0.0, 50.0);
}

/// In an RTL container, a negative margin on the trailing (left) side with a left float does
/// not prevent the BFC root from fitting beside the float
#[test]
fn bfc_beside_float_rtl() {
    let mut taffy: TaffyTree<()> = TaffyTree::new();
    let float = float_left(&mut taffy, 50.0, 100.0);
    let bfc = taffy
        .new_leaf(Style {
            size: Size { width: auto(), height: length(100.0) },
            margin: Rect { left: length(-20.0), right: zero(), top: zero(), bottom: zero() },
            ..bfc_root()
        })
        .unwrap();
    let root = container(&mut taffy, 100.0, Direction::Rtl, &[float, bfc]);
    compute(&mut taffy, root);
    assert_rect(&taffy, bfc, 50.0, 0.0, 50.0);
}
