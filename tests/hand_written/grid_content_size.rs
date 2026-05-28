use taffy::prelude::*;
use taffy::{Overflow, Point};

#[test]
#[cfg(feature = "grid")]
fn grid_content_size_includes_explicit_tracks() {
    let mut taffy: TaffyTree<()> = TaffyTree::new();

    let child = taffy
        .new_leaf(Style { size: Size { width: length(50.0), height: length(50.0) }, ..Default::default() })
        .unwrap();

    let grid = taffy
        .new_with_children(
            Style {
                display: Display::Grid,
                size: Size { width: length(100.0), height: length(100.0) },
                grid_template_rows: vec![length(500.0)],
                overflow: Point { x: Overflow::Scroll, y: Overflow::Scroll },
                ..Default::default()
            },
            &[child],
        )
        .unwrap();

    taffy.compute_layout(grid, Size::MAX_CONTENT).unwrap();

    assert_eq!(taffy.layout(grid).unwrap().content_size.height, 500.0);
}

#[test]
#[cfg(feature = "grid")]
fn grid_content_size_includes_auto_placed_rows() {
    let mut taffy: TaffyTree<()> = TaffyTree::new();

    let child_style = Style { size: Size { width: length(100.0), height: length(100.0) }, ..Default::default() };
    let children = (0..6).map(|_| taffy.new_leaf(child_style.clone()).unwrap()).collect::<Vec<_>>();

    let grid = taffy
        .new_with_children(
            Style {
                display: Display::Grid,
                size: Size { width: length(200.0), height: length(150.0) },
                grid_template_columns: vec![minmax(length(100.0), fr(1.0))],
                ..Default::default()
            },
            &children,
        )
        .unwrap();

    taffy.compute_layout(grid, Size::MAX_CONTENT).unwrap();

    assert_eq!(taffy.layout(grid).unwrap().content_size.height, 600.0);
}
