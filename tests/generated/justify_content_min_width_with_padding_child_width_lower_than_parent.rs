#[test]
fn justify_content_min_width_with_padding_child_width_lower_than_parent() {
    let mut taffy = taffy::Taffy::new();
    let node000 = taffy
        .new_with_children(
            taffy::style::FlexboxLayout {
                size: taffy::geometry::Size {
                    width: Some(taffy::style::Dimension::Points(199f32)),
                    height: Some(taffy::style::Dimension::Points(100f32)),
                    ..Default::default()
                },
                ..Default::default()
            },
            &[],
        )
        .unwrap();
    let node00 = taffy
        .new_with_children(
            taffy::style::FlexboxLayout {
                justify_content: taffy::style::JustifyContent::Center,
                min_size: taffy::geometry::Size {
                    width: taffy::style::Dimension::Points(400f32),
                    ..Default::default()
                },
                padding: taffy::geometry::Rect {
                    start: Some(taffy::style::Dimension::Points(100f32)),
                    end: Some(taffy::style::Dimension::Points(100f32)),
                    ..Default::default()
                },
                ..Default::default()
            },
            &[node000],
        )
        .unwrap();
    let node0 = taffy.new_with_children(taffy::style::FlexboxLayout { ..Default::default() }, &[node00]).unwrap();
    let node = taffy
        .new_with_children(
            taffy::style::FlexboxLayout {
                flex_direction: taffy::style::FlexDirection::Column,
                size: taffy::geometry::Size {
                    width: Some(taffy::style::Dimension::Points(1080f32)),
                    height: Some(taffy::style::Dimension::Points(1584f32)),
                    ..Default::default()
                },
                ..Default::default()
            },
            &[node0],
        )
        .unwrap();
    taffy.compute_layout(node, taffy::geometry::Size::undefined()).unwrap();
    assert_eq!(taffy.layout(node).unwrap().size.width, 1080f32);
    assert_eq!(taffy.layout(node).unwrap().size.height, 1584f32);
    assert_eq!(taffy.layout(node).unwrap().location.x, 0f32);
    assert_eq!(taffy.layout(node).unwrap().location.y, 0f32);
    assert_eq!(taffy.layout(node0).unwrap().size.width, 1080f32);
    assert_eq!(taffy.layout(node0).unwrap().size.height, 100f32);
    assert_eq!(taffy.layout(node0).unwrap().location.x, 0f32);
    assert_eq!(taffy.layout(node0).unwrap().location.y, 0f32);
    assert_eq!(taffy.layout(node00).unwrap().size.width, 400f32);
    assert_eq!(taffy.layout(node00).unwrap().size.height, 100f32);
    assert_eq!(taffy.layout(node00).unwrap().location.x, 0f32);
    assert_eq!(taffy.layout(node00).unwrap().location.y, 0f32);
    assert_eq!(taffy.layout(node000).unwrap().size.width, 199f32);
    assert_eq!(taffy.layout(node000).unwrap().size.height, 100f32);
    assert_eq!(taffy.layout(node000).unwrap().location.x, 101f32);
    assert_eq!(taffy.layout(node000).unwrap().location.y, 0f32);
}
