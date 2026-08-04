use taffy::prelude::*;
use taffy::{Overflow, Point};

#[test]
fn grid_content_size_empty_track() {
    let mut taffy: TaffyTree<()> = TaffyTree::new();

    let child_style = Style { size: Size { width: length(50.0), height: length(50.0) }, ..Default::default() };

    let child = taffy.new_leaf(child_style).unwrap();

    let grid_style = Style {
        display: Display::Grid,
        size: Size { width: length(100.0), height: length(100.0) },
        grid_template_rows: vec![length(500.0)],
        overflow: Point { x: Overflow::Scroll, y: Overflow::Scroll },
        ..Default::default()
    };

    let grid = taffy.new_with_children(grid_style, &[child]).unwrap();

    taffy.compute_layout(grid, Size::MAX_CONTENT).unwrap();

    let layout = taffy.layout(grid).unwrap();
    println!("Grid layout size: {:?}", layout.size);
    println!("Grid layout content_size: {:?}", layout.content_size);

    // The content size should be at least 500.0 because of the explicit track!
    assert_eq!(layout.content_size.height, 500.0, "Content height should include empty tracks");
}
