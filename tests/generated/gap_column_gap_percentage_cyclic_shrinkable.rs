#[test]
fn gap_column_gap_percentage_cyclic_shrinkable() {
    let mut taffy = taffy::Taffy::new();
    let node0 = taffy
        .new_with_children(
            taffy::style::FlexboxLayout {
                size: taffy::geometry::Size {
                    width: taffy::style::Dimension::Points(20f32),
                    height: taffy::style::Dimension::Points(40f32),
                    ..Default::default()
                },
                ..Default::default()
            },
            &[],
        )
        .unwrap();
    let node1 = taffy
        .new_with_children(
            taffy::style::FlexboxLayout {
                size: taffy::geometry::Size {
                    width: taffy::style::Dimension::Points(20f32),
                    height: taffy::style::Dimension::Points(40f32),
                    ..Default::default()
                },
                ..Default::default()
            },
            &[],
        )
        .unwrap();
    let node2 = taffy
        .new_with_children(
            taffy::style::FlexboxLayout {
                size: taffy::geometry::Size {
                    width: taffy::style::Dimension::Points(20f32),
                    height: taffy::style::Dimension::Points(40f32),
                    ..Default::default()
                },
                ..Default::default()
            },
            &[],
        )
        .unwrap();
    let node = taffy
        .new_with_children(
            taffy::style::FlexboxLayout {
                gap: taffy::geometry::Size { width: taffy::style::Dimension::Percent(0.2f32), ..Default::default() },
                ..Default::default()
            },
            &[node0, node1, node2],
        )
        .unwrap();
    taffy.compute_layout(node, taffy::geometry::Size::MAX_CONTENT).unwrap();
    assert_eq!(taffy.layout(node).unwrap().size.width, 60f32);
    assert_eq!(taffy.layout(node).unwrap().size.height, 40f32);
    assert_eq!(taffy.layout(node).unwrap().location.x, 0f32);
    assert_eq!(taffy.layout(node).unwrap().location.y, 0f32);
    assert_eq!(taffy.layout(node0).unwrap().size.width, 12f32);
    assert_eq!(taffy.layout(node0).unwrap().size.height, 40f32);
    assert_eq!(taffy.layout(node0).unwrap().location.x, 0f32);
    assert_eq!(taffy.layout(node0).unwrap().location.y, 0f32);
    assert_eq!(taffy.layout(node1).unwrap().size.width, 12f32);
    assert_eq!(taffy.layout(node1).unwrap().size.height, 40f32);
    assert_eq!(taffy.layout(node1).unwrap().location.x, 24f32);
    assert_eq!(taffy.layout(node1).unwrap().location.y, 0f32);
    assert_eq!(taffy.layout(node2).unwrap().size.width, 12f32);
    assert_eq!(taffy.layout(node2).unwrap().size.height, 40f32);
    assert_eq!(taffy.layout(node2).unwrap().location.x, 48f32);
    assert_eq!(taffy.layout(node2).unwrap().location.y, 0f32);
}
