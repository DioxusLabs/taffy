//! Temporary smoke test for subgrid support (will be replaced by gentest fixtures)
use taffy::prelude::*;
use taffy::{GridTemplate, TaffyTree};

#[test]
fn subgrid_both_axes_fixed_tracks() {
    let mut tree: TaffyTree<()> = TaffyTree::new();

    // 4 grandchildren, auto-placed into the subgrid
    let a = tree.new_leaf(Style::default()).unwrap();
    let b = tree.new_leaf(Style::default()).unwrap();
    let c = tree.new_leaf(Style::default()).unwrap();
    let d = tree.new_leaf(Style::default()).unwrap();

    // Subgrid child spanning columns 1/3 and rows 1/3 of the parent
    let subgrid = tree
        .new_with_children(
            Style {
                display: Display::Grid,
                grid_template_rows: GridTemplate::Subgrid(vec![]),
                grid_template_columns: GridTemplate::Subgrid(vec![]),
                grid_row: Line { start: line(1), end: line(3) },
                grid_column: Line { start: line(1), end: line(3) },
                ..Default::default()
            },
            &[a, b, c, d],
        )
        .unwrap();

    // Parent: 3 columns of 100px, 2 rows of 100px
    let root = tree
        .new_with_children(
            Style {
                display: Display::Grid,
                size: Size { width: length(320.0), height: length(200.0) },
                grid_template_columns: vec![length(100.0), length(100.0), length(120.0)].into(),
                grid_template_rows: vec![length(100.0), length(100.0)].into(),
                ..Default::default()
            },
            &[subgrid],
        )
        .unwrap();

    tree.compute_layout(root, Size::MAX_CONTENT).unwrap();
    tree.print_tree(root);

    // Subgrid should be stretched over its 2x2 grid area
    let sg = tree.layout(subgrid).unwrap();
    assert_eq!(sg.location.x, 0.0);
    assert_eq!(sg.location.y, 0.0);
    assert_eq!(sg.size.width, 200.0);
    assert_eq!(sg.size.height, 200.0);

    // Grandchildren should be laid out against the parent's tracks
    let la = tree.layout(a).unwrap();
    let lb = tree.layout(b).unwrap();
    let lc = tree.layout(c).unwrap();
    let ld = tree.layout(d).unwrap();
    assert_eq!((la.location.x, la.location.y, la.size.width, la.size.height), (0.0, 0.0, 100.0, 100.0));
    assert_eq!((lb.location.x, lb.location.y, lb.size.width, lb.size.height), (100.0, 0.0, 100.0, 100.0));
    assert_eq!((lc.location.x, lc.location.y, lc.size.width, lc.size.height), (0.0, 100.0, 100.0, 100.0));
    assert_eq!((ld.location.x, ld.location.y, ld.size.width, ld.size.height), (100.0, 100.0, 100.0, 100.0));
}

#[test]
fn subgrid_with_padding_and_clamped_placement() {
    let mut tree: TaffyTree<()> = TaffyTree::new();

    let a = tree.new_leaf(Style::default()).unwrap();
    // This item's placement (column 5/7) is out of range of the subgrid's explicit grid
    // and should be clamped into it (a subgrid has no implicit tracks)
    let b = tree.new_leaf(Style { grid_column: Line { start: line(5), end: line(7) }, ..Default::default() }).unwrap();

    // Subgrid with 10px padding on all sides, subgridded in both axes
    let subgrid = tree
        .new_with_children(
            Style {
                display: Display::Grid,
                grid_template_rows: GridTemplate::Subgrid(vec![]),
                grid_template_columns: GridTemplate::Subgrid(vec![]),
                grid_row: Line { start: line(1), end: line(2) },
                grid_column: Line { start: line(1), end: line(3) },
                padding: Rect { left: length(10.0), right: length(10.0), top: length(10.0), bottom: length(10.0) },
                ..Default::default()
            },
            &[a, b],
        )
        .unwrap();

    let root = tree
        .new_with_children(
            Style {
                display: Display::Grid,
                size: Size { width: length(200.0), height: length(100.0) },
                grid_template_columns: vec![length(100.0), length(100.0)].into(),
                grid_template_rows: vec![length(100.0)].into(),
                ..Default::default()
            },
            &[subgrid],
        )
        .unwrap();

    tree.compute_layout(root, Size::MAX_CONTENT).unwrap();
    tree.print_tree(root);

    let sg = tree.layout(subgrid).unwrap();
    assert_eq!(sg.size.width, 200.0);
    assert_eq!(sg.size.height, 100.0);

    // Item `a` is placed in the first track: the subgrid's padding eats into that track,
    // so it is positioned at the content box origin and is 10px smaller on the start side
    let la = tree.layout(a).unwrap();
    assert_eq!((la.location.x, la.location.y), (10.0, 10.0));
    assert_eq!((la.size.width, la.size.height), (90.0, 80.0));

    // Item `b`'s out-of-range placement is clamped to the last track
    let lb = tree.layout(b).unwrap();
    assert_eq!((lb.location.x, lb.size.width), (100.0, 90.0));
}

#[test]
fn subgrid_content_sizes_parent_auto_tracks() {
    let mut tree: TaffyTree<()> = TaffyTree::new();

    // Two fixed-size grandchildren: their sizes should size the PARENT's auto tracks
    let a = tree
        .new_leaf(Style { size: Size { width: length(30.0), height: length(20.0) }, ..Default::default() })
        .unwrap();
    let b = tree
        .new_leaf(Style { size: Size { width: length(80.0), height: length(40.0) }, ..Default::default() })
        .unwrap();

    let subgrid = tree
        .new_with_children(
            Style {
                display: Display::Grid,
                grid_template_rows: GridTemplate::Subgrid(vec![]),
                grid_template_columns: GridTemplate::Subgrid(vec![]),
                grid_row: Line { start: line(1), end: line(2) },
                grid_column: Line { start: line(1), end: line(3) },
                ..Default::default()
            },
            &[a, b],
        )
        .unwrap();

    // Parent has auto-sized tracks and no explicit size: it should be sized by the
    // subgrid's items (hoisted into the parent's track sizing)
    let root = tree
        .new_with_children(
            Style {
                display: Display::Grid,
                grid_template_columns: vec![auto(), auto()].into(),
                grid_template_rows: vec![auto()].into(),
                ..Default::default()
            },
            &[subgrid],
        )
        .unwrap();

    tree.compute_layout(root, Size::MAX_CONTENT).unwrap();
    tree.print_tree(root);

    // Parent tracks should be sized by the subgrid's items: 30px and 80px columns, 40px row
    let root_layout = tree.layout(root).unwrap();
    assert_eq!(root_layout.size.width, 110.0);
    assert_eq!(root_layout.size.height, 40.0);

    let la = tree.layout(a).unwrap();
    let lb = tree.layout(b).unwrap();
    assert_eq!((la.location.x, la.size.width), (0.0, 30.0));
    assert_eq!((lb.location.x, lb.size.width), (30.0, 80.0));
}

#[test]
fn nested_subgrid_content_sizes_outer_auto_tracks() {
    let mut tree: TaffyTree<()> = TaffyTree::new();

    let a = tree
        .new_leaf(Style { size: Size { width: length(40.0), height: length(10.0) }, ..Default::default() })
        .unwrap();
    let b = tree
        .new_leaf(Style { size: Size { width: length(60.0), height: length(10.0) }, ..Default::default() })
        .unwrap();

    // Nested subgrid (columns) inside a subgrid (columns), inside the outer grid
    let inner_subgrid = tree
        .new_with_children(
            Style {
                display: Display::Grid,
                grid_template_columns: GridTemplate::Subgrid(vec![]),
                grid_column: Line { start: line(1), end: line(3) },
                ..Default::default()
            },
            &[a, b],
        )
        .unwrap();
    let outer_subgrid = tree
        .new_with_children(
            Style {
                display: Display::Grid,
                grid_template_columns: GridTemplate::Subgrid(vec![]),
                grid_column: Line { start: line(1), end: line(3) },
                ..Default::default()
            },
            &[inner_subgrid],
        )
        .unwrap();

    let root = tree
        .new_with_children(
            Style {
                display: Display::Grid,
                grid_template_columns: vec![auto(), auto()].into(),
                grid_template_rows: vec![auto()].into(),
                ..Default::default()
            },
            &[outer_subgrid],
        )
        .unwrap();

    tree.compute_layout(root, Size::MAX_CONTENT).unwrap();
    tree.print_tree(root);

    // The innermost items should size the outer grid's auto tracks through both subgrid layers
    let root_layout = tree.layout(root).unwrap();
    assert_eq!(root_layout.size.width, 100.0);

    let la = tree.layout(a).unwrap();
    let lb = tree.layout(b).unwrap();
    assert_eq!((la.location.x, la.size.width), (0.0, 40.0));
    assert_eq!((lb.location.x, lb.size.width), (40.0, 60.0));
}

#[test]
fn subgrid_mbp_contributes_to_parent_auto_tracks() {
    let mut tree: TaffyTree<()> = TaffyTree::new();

    let a = tree
        .new_leaf(Style { size: Size { width: length(50.0), height: length(20.0) }, ..Default::default() })
        .unwrap();

    // Subgrid with 10px padding: the padding should be added to its item's contribution
    // to the parent's auto track
    let subgrid = tree
        .new_with_children(
            Style {
                display: Display::Grid,
                grid_template_columns: GridTemplate::Subgrid(vec![]),
                grid_column: Line { start: line(1), end: line(2) },
                padding: Rect { left: length(10.0), right: length(10.0), top: length(0.0), bottom: length(0.0) },
                ..Default::default()
            },
            &[a],
        )
        .unwrap();

    let root = tree
        .new_with_children(
            Style {
                display: Display::Grid,
                grid_template_columns: vec![auto()].into(),
                grid_template_rows: vec![auto()].into(),
                ..Default::default()
            },
            &[subgrid],
        )
        .unwrap();

    tree.compute_layout(root, Size::MAX_CONTENT).unwrap();
    tree.print_tree(root);

    // Parent's auto column should be 50 (item) + 10 + 10 (subgrid padding) = 70
    let root_layout = tree.layout(root).unwrap();
    assert_eq!(root_layout.size.width, 70.0);

    let la = tree.layout(a).unwrap();
    assert_eq!((la.location.x, la.size.width), (10.0, 50.0));
}

#[test]
fn subgrid_columns_only_with_gap() {
    let mut tree: TaffyTree<()> = TaffyTree::new();

    let a = tree.new_leaf(Style { size: Size { width: auto(), height: length(30.0) }, ..Default::default() }).unwrap();
    let b = tree.new_leaf(Style { size: Size { width: auto(), height: length(50.0) }, ..Default::default() }).unwrap();

    // Subgrid child subgridded in the column axis only, with its own rows
    let subgrid = tree
        .new_with_children(
            Style {
                display: Display::Grid,
                grid_template_columns: GridTemplate::Subgrid(vec![]),
                grid_column: Line { start: line(1), end: line(3) },
                ..Default::default()
            },
            &[a, b],
        )
        .unwrap();

    // Parent: 2 columns (100px, 200px) with a 10px column gap
    let root = tree
        .new_with_children(
            Style {
                display: Display::Grid,
                size: Size { width: length(310.0), height: length(100.0) },
                grid_template_columns: vec![length(100.0), length(200.0)].into(),
                gap: Size { width: length(10.0), height: length(0.0) },
                ..Default::default()
            },
            &[subgrid],
        )
        .unwrap();

    tree.compute_layout(root, Size::MAX_CONTENT).unwrap();
    tree.print_tree(root);

    let sg = tree.layout(subgrid).unwrap();
    assert_eq!(sg.size.width, 310.0);

    // Items adopt the parent's column tracks (including the parent's gap)
    let la = tree.layout(a).unwrap();
    let lb = tree.layout(b).unwrap();
    assert_eq!((la.location.x, la.size.width), (0.0, 100.0));
    assert_eq!((lb.location.x, lb.size.width), (110.0, 200.0));
    assert_eq!(la.size.height, 30.0);
    assert_eq!(lb.size.height, 50.0);
}
