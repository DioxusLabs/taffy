#[test]
fn justify_content_overflow_min_max() {
    let mut taffy = taffy::Taffy::new();
    let node0 = taffy
        .new_with_children(
            taffy::style::FlexboxLayout {
                flex_shrink: 0f32,
                size: taffy::geometry::Size {
                    width: Some(taffy::style::Dimension::Points(50f32)),
                    height: Some(taffy::style::Dimension::Points(50f32)),
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
                flex_shrink: 0f32,
                size: taffy::geometry::Size {
                    width: Some(taffy::style::Dimension::Points(50f32)),
                    height: Some(taffy::style::Dimension::Points(50f32)),
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
                flex_shrink: 0f32,
                size: taffy::geometry::Size {
                    width: Some(taffy::style::Dimension::Points(50f32)),
                    height: Some(taffy::style::Dimension::Points(50f32)),
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
                flex_direction: taffy::style::FlexDirection::Column,
                justify_content: taffy::style::JustifyContent::Center,
                min_size: taffy::geometry::Size {
                    height: taffy::style::Dimension::Points(100f32),
                    ..Default::default()
                },
                max_size: taffy::geometry::Size {
                    height: taffy::style::Dimension::Points(110f32),
                    ..Default::default()
                },
                ..Default::default()
            },
            &[node0, node1, node2],
        )
        .unwrap();
    taffy.compute_layout(node, taffy::geometry::Size::undefined()).unwrap();
    assert_eq!(taffy.layout(node).unwrap().size.width, 50f32);
    assert_eq!(taffy.layout(node).unwrap().size.height, 110f32);
    assert_eq!(taffy.layout(node).unwrap().location.x, 0f32);
    assert_eq!(taffy.layout(node).unwrap().location.y, 0f32);
    assert_eq!(taffy.layout(node0).unwrap().size.width, 50f32);
    assert_eq!(taffy.layout(node0).unwrap().size.height, 50f32);
    assert_eq!(taffy.layout(node0).unwrap().location.x, 0f32);
    assert_eq!(taffy.layout(node0).unwrap().location.y, -20f32);
    assert_eq!(taffy.layout(node1).unwrap().size.width, 50f32);
    assert_eq!(taffy.layout(node1).unwrap().size.height, 50f32);
    assert_eq!(taffy.layout(node1).unwrap().location.x, 0f32);
    assert_eq!(taffy.layout(node1).unwrap().location.y, 30f32);
    assert_eq!(taffy.layout(node2).unwrap().size.width, 50f32);
    assert_eq!(taffy.layout(node2).unwrap().size.height, 50f32);
    assert_eq!(taffy.layout(node2).unwrap().location.x, 0f32);
    assert_eq!(taffy.layout(node2).unwrap().location.y, 80f32);
}
