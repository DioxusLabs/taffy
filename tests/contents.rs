use taffy::prelude::*;

/// Basic: contents node's children are promoted and laid out by the parent flex container.
/// Equivalent to PR #534's contents_flex_basic test.
///
/// Layout: flex row, 400x300, justify-content: space-between
///   [30px] [30px] [30px] [contents: [30px] [30px]]
///
/// Should lay out as 5 equal-width items spaced across 400px.
#[test]
fn contents_flex_basic() {
    let mut taffy = TaffyTree::<()>::new();

    let node0 = taffy.new_leaf(Style {
        size: Size { width: Dimension::from_length(30.0), height: auto() },
        ..Default::default()
    }).unwrap();
    let node1 = taffy.new_leaf(Style {
        size: Size { width: Dimension::from_length(30.0), height: auto() },
        ..Default::default()
    }).unwrap();
    let node2 = taffy.new_leaf(Style {
        size: Size { width: Dimension::from_length(30.0), height: auto() },
        ..Default::default()
    }).unwrap();

    let node30 = taffy.new_leaf(Style {
        size: Size { width: Dimension::from_length(30.0), height: auto() },
        ..Default::default()
    }).unwrap();
    let node31 = taffy.new_leaf(Style {
        size: Size { width: Dimension::from_length(30.0), height: auto() },
        ..Default::default()
    }).unwrap();

    let contents = taffy.new_with_children(
        Style { display: Display::Contents, ..Default::default() },
        &[node30, node31],
    ).unwrap();

    let root = taffy.new_with_children(
        Style {
            display: Display::Flex,
            justify_content: Some(JustifyContent::SpaceBetween),
            size: Size {
                width: Dimension::from_length(400.0),
                height: Dimension::from_length(300.0),
            },
            ..Default::default()
        },
        &[node0, node1, node2, contents],
    ).unwrap();

    taffy.compute_layout(root, Size::MAX_CONTENT).unwrap();

    // Contents node itself has zero layout
    let cl = taffy.layout(contents).unwrap();
    assert_eq!(cl.size.width, 0.0);
    assert_eq!(cl.size.height, 0.0);

    // 5 items, 5 * 30 = 150px used, 250px remaining, 4 gaps = 62.5px each
    // Positions: 0, 92.5, 185, 277.5, 370
    let l0 = taffy.layout(node0).unwrap();
    assert_eq!(l0.size.width, 30.0);
    assert_eq!(l0.location.x, 0.0);

    let l1 = taffy.layout(node1).unwrap();
    assert_eq!(l1.size.width, 30.0);

    let l30 = taffy.layout(node30).unwrap();
    assert_eq!(l30.size.width, 30.0);
    assert_eq!(l30.size.height, 300.0, "promoted child should stretch to parent height");

    let l31 = taffy.layout(node31).unwrap();
    assert_eq!(l31.size.width, 30.0);
    assert_eq!(l31.location.x, 370.0, "last promoted child at far right");
}

/// Nested: contents within contents, children promoted recursively.
#[test]
fn contents_flex_nested() {
    let mut taffy = TaffyTree::<()>::new();

    let leaf1 = taffy.new_leaf(Style {
        size: Size { width: Dimension::from_length(50.0), height: auto() },
        ..Default::default()
    }).unwrap();
    let leaf2 = taffy.new_leaf(Style {
        size: Size { width: Dimension::from_length(50.0), height: auto() },
        ..Default::default()
    }).unwrap();

    let inner_contents = taffy.new_with_children(
        Style { display: Display::Contents, ..Default::default() },
        &[leaf2],
    ).unwrap();

    let outer_contents = taffy.new_with_children(
        Style { display: Display::Contents, ..Default::default() },
        &[leaf1, inner_contents],
    ).unwrap();

    let root = taffy.new_with_children(
        Style {
            display: Display::Flex,
            size: Size {
                width: Dimension::from_length(200.0),
                height: Dimension::from_length(100.0),
            },
            ..Default::default()
        },
        &[outer_contents],
    ).unwrap();

    taffy.compute_layout(root, Size::MAX_CONTENT).unwrap();

    // Both leaves should be laid out as direct children of root
    let l1 = taffy.layout(leaf1).unwrap();
    assert_eq!(l1.size.width, 50.0);
    assert_eq!(l1.size.height, 100.0); // stretch
    assert_eq!(l1.location.x, 0.0);

    let l2 = taffy.layout(leaf2).unwrap();
    assert_eq!(l2.size.width, 50.0);
    assert_eq!(l2.location.x, 50.0);

    // Both contents nodes have zero layout
    assert_eq!(taffy.layout(outer_contents).unwrap().size.width, 0.0);
    assert_eq!(taffy.layout(inner_contents).unwrap().size.width, 0.0);
}

/// Contents children are NOT hidden (unlike Display::None)
#[test]
fn contents_children_not_hidden() {
    let mut taffy = TaffyTree::<()>::new();

    let contents_child = taffy.new_leaf(Style {
        size: Size { width: Dimension::from_length(50.0), height: Dimension::from_length(50.0) },
        ..Default::default()
    }).unwrap();
    let none_child = taffy.new_leaf(Style {
        size: Size { width: Dimension::from_length(50.0), height: Dimension::from_length(50.0) },
        ..Default::default()
    }).unwrap();

    let contents = taffy.new_with_children(
        Style { display: Display::Contents, ..Default::default() },
        &[contents_child],
    ).unwrap();
    let none = taffy.new_with_children(
        Style { display: Display::None, ..Default::default() },
        &[none_child],
    ).unwrap();

    let root = taffy.new_with_children(
        Style {
            display: Display::Flex,
            size: Size { width: Dimension::from_length(200.0), height: Dimension::from_length(200.0) },
            ..Default::default()
        },
        &[contents, none],
    ).unwrap();

    taffy.compute_layout(root, Size::MAX_CONTENT).unwrap();

    // Contents child is promoted and laid out (has explicit 50x50 size)
    let cl = taffy.layout(contents_child).unwrap();
    assert_eq!(cl.size.width, 50.0);
    assert_eq!(cl.size.height, 50.0);

    // None child is hidden
    let nl = taffy.layout(none_child).unwrap();
    assert_eq!(nl.size.width, 0.0);
    assert_eq!(nl.size.height, 0.0);
}

/// Style roundtrip
#[test]
fn contents_style_roundtrip() {
    let mut taffy = TaffyTree::<()>::new();
    let node = taffy.new_leaf(Style { display: Display::Contents, ..Default::default() }).unwrap();
    assert_eq!(taffy.style(node).unwrap().display, Display::Contents);

    taffy.set_style(node, Style { display: Display::None, ..Default::default() }).unwrap();
    assert_eq!(taffy.style(node).unwrap().display, Display::None);

    taffy.set_style(node, Style { display: Display::Contents, ..Default::default() }).unwrap();
    assert_eq!(taffy.style(node).unwrap().display, Display::Contents);
}

/// No overhead when no contents nodes exist
#[test]
fn no_contents_no_overhead() {
    let mut taffy = TaffyTree::<()>::new();

    let child = taffy.new_leaf(Style {
        size: Size { width: Dimension::from_length(50.0), height: Dimension::from_length(50.0) },
        ..Default::default()
    }).unwrap();
    let root = taffy.new_with_children(
        Style {
            display: Display::Flex,
            size: Size { width: Dimension::from_length(200.0), height: Dimension::from_length(200.0) },
            ..Default::default()
        },
        &[child],
    ).unwrap();

    // This should not walk the tree (contents_count == 0)
    taffy.compute_layout(root, Size::MAX_CONTENT).unwrap();

    let cl = taffy.layout(child).unwrap();
    assert_eq!(cl.size.width, 50.0);
}
