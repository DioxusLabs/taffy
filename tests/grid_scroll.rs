use taffy::prelude::*;

#[test]
fn grid_content_size_overflow() {
    let mut taffy: TaffyTree<()> = TaffyTree::new();

    let child_style = Style { size: Size { width: length(100.0), height: length(100.0) }, ..Default::default() };

    let children = (0..6).map(|_| taffy.new_leaf(child_style.clone()).unwrap()).collect::<Vec<_>>();

    let grid_style = Style {
        display: Display::Grid,
        size: Size { width: length(200.0), height: length(150.0) },
        grid_template_columns: vec![minmax(length(100.0), fr(1.0))],
        ..Default::default()
    };

    let grid = taffy.new_with_children(grid_style, &children).unwrap();

    taffy.compute_layout(grid, Size::MAX_CONTENT).unwrap();

    let layout = taffy.layout(grid).unwrap();
    println!("Grid layout size: {:?}", layout.size);
    println!("Grid layout content_size: {:?}", layout.content_size);
    for child in children {
        println!("Child layout: {:?}", taffy.layout(child).unwrap());
    }
    assert_eq!(layout.content_size.height, 600.0, "Content height should include all tracks");
}
