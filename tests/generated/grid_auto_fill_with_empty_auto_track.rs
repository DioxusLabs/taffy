#[test]
fn grid_auto_fill_with_empty_auto_track() {
    #[allow(unused_imports)]
    use taffy::prelude::*;
    let mut taffy = taffy::Taffy::new();
    let node0 = taffy.new_leaf(taffy::style::Style { ..Default::default() }).unwrap();
    let node1 = taffy.new_leaf(taffy::style::Style { ..Default::default() }).unwrap();
    let node = taffy
        .new_with_children(
            taffy::style::Style {
                display: taffy::style::Display::Grid,
                justify_content: Some(taffy::style::JustifyContent::SpaceEvenly),
                grid_template_rows: vec![points(40f32), points(40f32), points(40f32)],
                grid_template_columns: vec![repeat(GridTrackRepetition::AutoFill, vec![points(40f32)])],
                size: taffy::geometry::Size {
                    width: taffy::style::Dimension::Points(120f32),
                    height: taffy::style::Dimension::Points(120f32),
                },
                ..Default::default()
            },
            &[node0, node1],
        )
        .unwrap();
    taffy.compute_layout(node, taffy::geometry::Size::MAX_CONTENT).unwrap();
    println!("\nComputed tree:");
    taffy::debug::print_tree(&taffy, node);
    println!();
    assert_eq!(taffy.layout(node).unwrap().size.width, 120f32);
    assert_eq!(taffy.layout(node).unwrap().size.height, 120f32);
    assert_eq!(taffy.layout(node).unwrap().location.x, 0f32);
    assert_eq!(taffy.layout(node).unwrap().location.y, 0f32);
    assert_eq!(taffy.layout(node0).unwrap().size.width, 40f32);
    assert_eq!(taffy.layout(node0).unwrap().size.height, 40f32);
    assert_eq!(taffy.layout(node0).unwrap().location.x, 0f32);
    assert_eq!(taffy.layout(node0).unwrap().location.y, 0f32);
    assert_eq!(taffy.layout(node1).unwrap().size.width, 40f32);
    assert_eq!(taffy.layout(node1).unwrap().size.height, 40f32);
    assert_eq!(taffy.layout(node1).unwrap().location.x, 40f32);
    assert_eq!(taffy.layout(node1).unwrap().location.y, 0f32);
}
