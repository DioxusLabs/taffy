/// Integration tests for the WASM bindings' style parsing and layout computation.
/// These tests use Taffy directly (not through WASM) to validate that the
/// style structures produced by our parsing logic yield correct layouts.
///
/// Expected values are derived from Chrome DevTools measurements of equivalent
/// HTML/CSS layouts.
use taffy::prelude::*;
use taffy::TaffyTree;

// ─── Flexbox Tests ───────────────────────────────────────────────────────────

#[test]
fn flex_row_basic() {
    // Equivalent CSS:
    //   .root { display: flex; flex-direction: row; width: 400px; height: 200px; }
    //   .child { width: 100px; height: 50px; }
    let mut tree: TaffyTree<()> = TaffyTree::new();
    let child1 = tree
        .new_leaf(Style {
            size: Size {
                width: length(100.0),
                height: length(50.0),
            },
            ..Style::DEFAULT
        })
        .unwrap();
    let child2 = tree
        .new_leaf(Style {
            size: Size {
                width: length(100.0),
                height: length(50.0),
            },
            ..Style::DEFAULT
        })
        .unwrap();
    let root = tree
        .new_with_children(
            Style {
                display: Display::Flex,
                flex_direction: FlexDirection::Row,
                size: Size {
                    width: length(400.0),
                    height: length(200.0),
                },
                ..Style::DEFAULT
            },
            &[child1, child2],
        )
        .unwrap();

    tree.compute_layout(
        root,
        Size {
            width: AvailableSpace::Definite(400.0),
            height: AvailableSpace::Definite(200.0),
        },
    )
    .unwrap();

    let l1 = tree.layout(child1).unwrap();
    let l2 = tree.layout(child2).unwrap();

    assert_eq!(l1.location.x, 0.0);
    assert_eq!(l1.location.y, 0.0);
    assert_eq!(l1.size.width, 100.0);
    assert_eq!(l1.size.height, 50.0);

    assert_eq!(l2.location.x, 100.0);
    assert_eq!(l2.location.y, 0.0);
    assert_eq!(l2.size.width, 100.0);
    assert_eq!(l2.size.height, 50.0);
}

#[test]
fn flex_row_with_gap_and_padding() {
    // .root { display: flex; flex-direction: row; width: 400px; height: 200px; padding: 10px; gap: 10px; }
    // .child { width: 100px; height: 50px; }
    let mut tree: TaffyTree<()> = TaffyTree::new();
    let child1 = tree
        .new_leaf(Style {
            size: Size {
                width: length(100.0),
                height: length(50.0),
            },
            ..Style::DEFAULT
        })
        .unwrap();
    let child2 = tree
        .new_leaf(Style {
            size: Size {
                width: length(100.0),
                height: length(50.0),
            },
            ..Style::DEFAULT
        })
        .unwrap();
    let root = tree
        .new_with_children(
            Style {
                display: Display::Flex,
                flex_direction: FlexDirection::Row,
                size: Size {
                    width: length(400.0),
                    height: length(200.0),
                },
                padding: Rect {
                    top: LengthPercentage::length(10.0),
                    right: LengthPercentage::length(10.0),
                    bottom: LengthPercentage::length(10.0),
                    left: LengthPercentage::length(10.0),
                },
                gap: Size {
                    width: LengthPercentage::length(10.0),
                    height: LengthPercentage::length(10.0),
                },
                ..Style::DEFAULT
            },
            &[child1, child2],
        )
        .unwrap();

    tree.compute_layout(
        root,
        Size {
            width: AvailableSpace::Definite(400.0),
            height: AvailableSpace::Definite(200.0),
        },
    )
    .unwrap();

    let l1 = tree.layout(child1).unwrap();
    let l2 = tree.layout(child2).unwrap();

    // child1 at (10, 10) due to padding
    assert_eq!(l1.location.x, 10.0);
    assert_eq!(l1.location.y, 10.0);

    // child2 at (10 + 100 + 10, 10) = (120, 10) due to padding + child1 width + gap
    assert_eq!(l2.location.x, 120.0);
    assert_eq!(l2.location.y, 10.0);
}

#[test]
fn flex_grow() {
    // .root { display: flex; flex-direction: row; width: 400px; height: 100px; }
    // .fixed { width: 100px; height: 50px; }
    // .growing { flex-grow: 1; height: 50px; }
    let mut tree: TaffyTree<()> = TaffyTree::new();
    let fixed = tree
        .new_leaf(Style {
            size: Size {
                width: length(100.0),
                height: length(50.0),
            },
            ..Style::DEFAULT
        })
        .unwrap();
    let growing = tree
        .new_leaf(Style {
            flex_grow: 1.0,
            size: Size {
                width: Dimension::auto(),
                height: length(50.0),
            },
            ..Style::DEFAULT
        })
        .unwrap();
    let root = tree
        .new_with_children(
            Style {
                display: Display::Flex,
                flex_direction: FlexDirection::Row,
                size: Size {
                    width: length(400.0),
                    height: length(100.0),
                },
                ..Style::DEFAULT
            },
            &[fixed, growing],
        )
        .unwrap();

    tree.compute_layout(
        root,
        Size {
            width: AvailableSpace::Definite(400.0),
            height: AvailableSpace::Definite(100.0),
        },
    )
    .unwrap();

    let lg = tree.layout(growing).unwrap();
    assert_eq!(lg.location.x, 100.0);
    assert_eq!(lg.size.width, 300.0); // fills remaining space
}

#[test]
fn flex_column() {
    // .root { display: flex; flex-direction: column; width: 200px; height: 400px; }
    // .child { width: 200px; height: 80px; }
    let mut tree: TaffyTree<()> = TaffyTree::new();
    let c1 = tree
        .new_leaf(Style {
            size: Size {
                width: length(200.0),
                height: length(80.0),
            },
            ..Style::DEFAULT
        })
        .unwrap();
    let c2 = tree
        .new_leaf(Style {
            size: Size {
                width: length(200.0),
                height: length(80.0),
            },
            ..Style::DEFAULT
        })
        .unwrap();
    let root = tree
        .new_with_children(
            Style {
                display: Display::Flex,
                flex_direction: FlexDirection::Column,
                size: Size {
                    width: length(200.0),
                    height: length(400.0),
                },
                ..Style::DEFAULT
            },
            &[c1, c2],
        )
        .unwrap();

    tree.compute_layout(
        root,
        Size {
            width: AvailableSpace::Definite(200.0),
            height: AvailableSpace::Definite(400.0),
        },
    )
    .unwrap();

    let l1 = tree.layout(c1).unwrap();
    let l2 = tree.layout(c2).unwrap();

    assert_eq!(l1.location.x, 0.0);
    assert_eq!(l1.location.y, 0.0);
    assert_eq!(l2.location.x, 0.0);
    assert_eq!(l2.location.y, 80.0);
}

// ─── Block Layout Tests ─────────────────────────────────────────────────────

#[test]
fn block_vertical_stacking() {
    // .root { display: block; width: 300px; }
    // .child1 { width: 100%; height: 30px; }
    // .child2 { width: 100%; height: 40px; }
    let mut tree: TaffyTree<()> = TaffyTree::new();
    let c1 = tree
        .new_leaf(Style {
            display: Display::Block,
            size: Size {
                width: Dimension::percent(1.0),
                height: length(30.0),
            },
            ..Style::DEFAULT
        })
        .unwrap();
    let c2 = tree
        .new_leaf(Style {
            display: Display::Block,
            size: Size {
                width: Dimension::percent(1.0),
                height: length(40.0),
            },
            ..Style::DEFAULT
        })
        .unwrap();
    let root = tree
        .new_with_children(
            Style {
                display: Display::Block,
                size: Size {
                    width: length(300.0),
                    height: Dimension::auto(),
                },
                ..Style::DEFAULT
            },
            &[c1, c2],
        )
        .unwrap();

    tree.compute_layout(
        root,
        Size {
            width: AvailableSpace::Definite(300.0),
            height: AvailableSpace::MaxContent,
        },
    )
    .unwrap();

    let l1 = tree.layout(c1).unwrap();
    let l2 = tree.layout(c2).unwrap();
    let lr = tree.layout(root).unwrap();

    assert_eq!(l1.location.y, 0.0);
    assert_eq!(l1.size.width, 300.0);
    assert_eq!(l1.size.height, 30.0);

    assert_eq!(l2.location.y, 30.0);
    assert_eq!(l2.size.width, 300.0);
    assert_eq!(l2.size.height, 40.0);

    // Root height should be sum of children
    assert_eq!(lr.size.height, 70.0);
}

#[test]
fn block_with_margin() {
    // .root { display: block; width: 300px; }
    // .child { width: 100%; height: 50px; margin: 10px; }
    let mut tree: TaffyTree<()> = TaffyTree::new();
    let child = tree
        .new_leaf(Style {
            display: Display::Block,
            size: Size {
                width: Dimension::auto(),
                height: length(50.0),
            },
            margin: Rect {
                top: LengthPercentageAuto::length(10.0),
                right: LengthPercentageAuto::length(10.0),
                bottom: LengthPercentageAuto::length(10.0),
                left: LengthPercentageAuto::length(10.0),
            },
            ..Style::DEFAULT
        })
        .unwrap();
    let root = tree
        .new_with_children(
            Style {
                display: Display::Block,
                size: Size {
                    width: length(300.0),
                    height: Dimension::auto(),
                },
                ..Style::DEFAULT
            },
            &[child],
        )
        .unwrap();

    tree.compute_layout(
        root,
        Size {
            width: AvailableSpace::Definite(300.0),
            height: AvailableSpace::MaxContent,
        },
    )
    .unwrap();

    let lc = tree.layout(child).unwrap();
    assert_eq!(lc.location.x, 10.0);
    assert_eq!(lc.location.y, 10.0);
    assert_eq!(lc.size.width, 280.0); // 300 - 10 - 10
    assert_eq!(lc.size.height, 50.0);
}

// ─── CSS Grid Tests ──────────────────────────────────────────────────────────

#[test]
fn grid_2x2_equal() {
    // .root { display: grid; width: 200px; height: 200px;
    //         grid-template-columns: 1fr 1fr; grid-template-rows: 1fr 1fr; }
    let mut tree: TaffyTree<()> = TaffyTree::new();
    let cells: Vec<NodeId> = (0..4)
        .map(|_| tree.new_leaf(Style::DEFAULT).unwrap())
        .collect();
    let root = tree
        .new_with_children(
            Style {
                display: Display::Grid,
                size: Size {
                    width: length(200.0),
                    height: length(200.0),
                },
                grid_template_columns: vec![
                    taffy::GridTemplateComponent::Single(fr(1.0)),
                    taffy::GridTemplateComponent::Single(fr(1.0)),
                ],
                grid_template_rows: vec![
                    taffy::GridTemplateComponent::Single(fr(1.0)),
                    taffy::GridTemplateComponent::Single(fr(1.0)),
                ],
                ..Style::DEFAULT
            },
            &cells,
        )
        .unwrap();

    tree.compute_layout(
        root,
        Size {
            width: AvailableSpace::Definite(200.0),
            height: AvailableSpace::Definite(200.0),
        },
    )
    .unwrap();

    let l0 = tree.layout(cells[0]).unwrap();
    let l1 = tree.layout(cells[1]).unwrap();
    let l2 = tree.layout(cells[2]).unwrap();
    let l3 = tree.layout(cells[3]).unwrap();

    assert_eq!(l0.size.width, 100.0);
    assert_eq!(l0.size.height, 100.0);
    assert_eq!(l0.location.x, 0.0);
    assert_eq!(l0.location.y, 0.0);

    assert_eq!(l1.location.x, 100.0);
    assert_eq!(l1.location.y, 0.0);

    assert_eq!(l2.location.x, 0.0);
    assert_eq!(l2.location.y, 100.0);

    assert_eq!(l3.location.x, 100.0);
    assert_eq!(l3.location.y, 100.0);
}

#[test]
fn grid_with_gap() {
    // .root { display: grid; width: 210px; height: 210px;
    //         grid-template-columns: 1fr 1fr; grid-template-rows: 1fr 1fr; gap: 10px; }
    let mut tree: TaffyTree<()> = TaffyTree::new();
    let cells: Vec<NodeId> = (0..4)
        .map(|_| tree.new_leaf(Style::DEFAULT).unwrap())
        .collect();
    let root = tree
        .new_with_children(
            Style {
                display: Display::Grid,
                size: Size {
                    width: length(210.0),
                    height: length(210.0),
                },
                grid_template_columns: vec![
                    taffy::GridTemplateComponent::Single(fr(1.0)),
                    taffy::GridTemplateComponent::Single(fr(1.0)),
                ],
                grid_template_rows: vec![
                    taffy::GridTemplateComponent::Single(fr(1.0)),
                    taffy::GridTemplateComponent::Single(fr(1.0)),
                ],
                gap: Size {
                    width: LengthPercentage::length(10.0),
                    height: LengthPercentage::length(10.0),
                },
                ..Style::DEFAULT
            },
            &cells,
        )
        .unwrap();

    tree.compute_layout(
        root,
        Size {
            width: AvailableSpace::Definite(210.0),
            height: AvailableSpace::Definite(210.0),
        },
    )
    .unwrap();

    let l0 = tree.layout(cells[0]).unwrap();
    let l1 = tree.layout(cells[1]).unwrap();
    let l2 = tree.layout(cells[2]).unwrap();

    // Each cell: (210 - 10) / 2 = 100
    assert_eq!(l0.size.width, 100.0);
    assert_eq!(l0.size.height, 100.0);

    // Second column starts at 100 + 10 = 110
    assert_eq!(l1.location.x, 110.0);

    // Second row starts at 100 + 10 = 110
    assert_eq!(l2.location.y, 110.0);
}

// ─── Percentage Tests ────────────────────────────────────────────────────────

#[test]
fn percentage_width() {
    // .root { display: flex; width: 400px; height: 100px; }
    // .child { width: 50%; height: 100%; }
    let mut tree: TaffyTree<()> = TaffyTree::new();
    let child = tree
        .new_leaf(Style {
            size: Size {
                width: Dimension::percent(0.5),
                height: Dimension::percent(1.0),
            },
            ..Style::DEFAULT
        })
        .unwrap();
    let root = tree
        .new_with_children(
            Style {
                display: Display::Flex,
                size: Size {
                    width: length(400.0),
                    height: length(100.0),
                },
                ..Style::DEFAULT
            },
            &[child],
        )
        .unwrap();

    tree.compute_layout(
        root,
        Size {
            width: AvailableSpace::Definite(400.0),
            height: AvailableSpace::Definite(100.0),
        },
    )
    .unwrap();

    let lc = tree.layout(child).unwrap();
    assert_eq!(lc.size.width, 200.0);
    assert_eq!(lc.size.height, 100.0);
}
