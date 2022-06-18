#[test]
fn absolute_layout_percentage_bottom_based_on_parent_height() {
    let mut taffy = taffy::Taffy::new();
    let node0 = taffy
        .new_with_children(
            taffy::style::FlexboxLayout {
                position_type: taffy::style::PositionType::Absolute,
                size: taffy::geometry::Size {
                    width: Some(taffy::style::Dimension::Points(10f32)),
                    height: Some(taffy::style::Dimension::Points(10f32)),
                    ..Default::default()
                },
                position: taffy::geometry::Rect { top: taffy::style::Dimension::Percent(0.5f32), ..Default::default() },
                ..Default::default()
            },
            &[],
        )
        .unwrap();
    let node1 = taffy
        .new_with_children(
            taffy::style::FlexboxLayout {
                position_type: taffy::style::PositionType::Absolute,
                size: taffy::geometry::Size {
                    width: Some(taffy::style::Dimension::Points(10f32)),
                    height: Some(taffy::style::Dimension::Points(10f32)),
                    ..Default::default()
                },
                position: taffy::geometry::Rect {
                    bottom: taffy::style::Dimension::Percent(0.5f32),
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
                position_type: taffy::style::PositionType::Absolute,
                size: taffy::geometry::Size {
                    width: Some(taffy::style::Dimension::Points(10f32)),
                    ..Default::default()
                },
                position: taffy::geometry::Rect {
                    top: taffy::style::Dimension::Percent(0.1f32),
                    bottom: taffy::style::Dimension::Percent(0.1f32),
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
                size: taffy::geometry::Size {
                    width: Some(taffy::style::Dimension::Points(100f32)),
                    height: Some(taffy::style::Dimension::Points(200f32)),
                    ..Default::default()
                },
                ..Default::default()
            },
            &[node0, node1, node2],
        )
        .unwrap();
    taffy.compute_layout(node, taffy::geometry::Size::undefined()).unwrap();
    assert_eq!(taffy.layout(node).unwrap().size.width, 100f32);
    assert_eq!(taffy.layout(node).unwrap().size.height, 200f32);
    assert_eq!(taffy.layout(node).unwrap().location.x, 0f32);
    assert_eq!(taffy.layout(node).unwrap().location.y, 0f32);
    assert_eq!(taffy.layout(node0).unwrap().size.width, 10f32);
    assert_eq!(taffy.layout(node0).unwrap().size.height, 10f32);
    assert_eq!(taffy.layout(node0).unwrap().location.x, 0f32);
    assert_eq!(taffy.layout(node0).unwrap().location.y, 100f32);
    assert_eq!(taffy.layout(node1).unwrap().size.width, 10f32);
    assert_eq!(taffy.layout(node1).unwrap().size.height, 10f32);
    assert_eq!(taffy.layout(node1).unwrap().location.x, 0f32);
    assert_eq!(taffy.layout(node1).unwrap().location.y, 90f32);
    assert_eq!(taffy.layout(node2).unwrap().size.width, 10f32);
    assert_eq!(taffy.layout(node2).unwrap().size.height, 160f32);
    assert_eq!(taffy.layout(node2).unwrap().location.x, 0f32);
    assert_eq!(taffy.layout(node2).unwrap().location.y, 20f32);
}
