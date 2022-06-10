#[test]
fn justify_content_row_space_evenly() {
    let mut taffy = taffy::Taffy::new();
    let node0 = taffy
        .new_node(
            taffy::style::FlexboxLayout {
                size: taffy::geometry::Size { height: taffy::style::Dimension::Points(10f32), ..Default::default() },
                ..Default::default()
            },
            &[],
        )
        .unwrap();
    let node1 = taffy
        .new_node(
            taffy::style::FlexboxLayout {
                size: taffy::geometry::Size { height: taffy::style::Dimension::Points(10f32), ..Default::default() },
                ..Default::default()
            },
            &[],
        )
        .unwrap();
    let node2 = taffy
        .new_node(
            taffy::style::FlexboxLayout {
                size: taffy::geometry::Size { height: taffy::style::Dimension::Points(10f32), ..Default::default() },
                ..Default::default()
            },
            &[],
        )
        .unwrap();
    let node = taffy
        .new_node(
            taffy::style::FlexboxLayout {
                justify_content: taffy::style::JustifyContent::SpaceEvenly,
                size: taffy::geometry::Size {
                    width: taffy::style::Dimension::Points(100f32),
                    height: taffy::style::Dimension::Points(100f32),
                    ..Default::default()
                },
                ..Default::default()
            },
            &[node0, node1, node2],
        )
        .unwrap();
    taffy.compute_layout(node, taffy::geometry::Size::undefined()).unwrap();
    assert_eq!(taffy.layout(node).unwrap().size.width, 100f32);
    assert_eq!(taffy.layout(node).unwrap().size.height, 100f32);
    assert_eq!(taffy.layout(node).unwrap().location.x, 0f32);
    assert_eq!(taffy.layout(node).unwrap().location.y, 0f32);
    assert_eq!(taffy.layout(node0).unwrap().size.width, 0f32);
    assert_eq!(taffy.layout(node0).unwrap().size.height, 10f32);
    assert_eq!(taffy.layout(node0).unwrap().location.x, 25f32);
    assert_eq!(taffy.layout(node0).unwrap().location.y, 0f32);
    assert_eq!(taffy.layout(node1).unwrap().size.width, 0f32);
    assert_eq!(taffy.layout(node1).unwrap().size.height, 10f32);
    assert_eq!(taffy.layout(node1).unwrap().location.x, 50f32);
    assert_eq!(taffy.layout(node1).unwrap().location.y, 0f32);
    assert_eq!(taffy.layout(node2).unwrap().size.width, 0f32);
    assert_eq!(taffy.layout(node2).unwrap().size.height, 10f32);
    assert_eq!(taffy.layout(node2).unwrap().location.x, 75f32);
    assert_eq!(taffy.layout(node2).unwrap().location.y, 0f32);
}
