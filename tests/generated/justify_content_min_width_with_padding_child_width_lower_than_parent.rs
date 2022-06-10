#[test]
fn justify_content_min_width_with_padding_child_width_lower_than_parent() {
    let mut sprawl = sprawl::Sprawl::new();
    let node000 = sprawl
        .new_node(
            sprawl::style::FlexboxLayout {
                size: sprawl::geometry::Size {
                    width: sprawl::style::Dimension::Points(199f32),
                    height: sprawl::style::Dimension::Points(100f32),
                    ..Default::default()
                },
                ..Default::default()
            },
            &[],
        )
        .unwrap();
    let node00 = sprawl
        .new_node(
            sprawl::style::FlexboxLayout {
                justify_content: sprawl::style::JustifyContent::Center,
                min_size: sprawl::geometry::Size {
                    width: sprawl::style::Dimension::Points(400f32),
                    ..Default::default()
                },
                padding: sprawl::geometry::Rect {
                    start: sprawl::style::Dimension::Points(100f32),
                    end: sprawl::style::Dimension::Points(100f32),
                    ..Default::default()
                },
                ..Default::default()
            },
            &[node000],
        )
        .unwrap();
    let node0 = sprawl.new_node(sprawl::style::FlexboxLayout { ..Default::default() }, &[node00]).unwrap();
    let node = sprawl
        .new_node(
            sprawl::style::FlexboxLayout {
                flex_direction: sprawl::style::FlexDirection::Column,
                size: sprawl::geometry::Size {
                    width: sprawl::style::Dimension::Points(1080f32),
                    height: sprawl::style::Dimension::Points(1584f32),
                    ..Default::default()
                },
                ..Default::default()
            },
            &[node0],
        )
        .unwrap();
    sprawl.compute_layout(node, sprawl::geometry::Size::undefined()).unwrap();
    assert_eq!(sprawl.layout(node).unwrap().size.width, 1080f32);
    assert_eq!(sprawl.layout(node).unwrap().size.height, 1584f32);
    assert_eq!(sprawl.layout(node).unwrap().location.x, 0f32);
    assert_eq!(sprawl.layout(node).unwrap().location.y, 0f32);
    assert_eq!(sprawl.layout(node0).unwrap().size.width, 1080f32);
    assert_eq!(sprawl.layout(node0).unwrap().size.height, 100f32);
    assert_eq!(sprawl.layout(node0).unwrap().location.x, 0f32);
    assert_eq!(sprawl.layout(node0).unwrap().location.y, 0f32);
    assert_eq!(sprawl.layout(node00).unwrap().size.width, 400f32);
    assert_eq!(sprawl.layout(node00).unwrap().size.height, 100f32);
    assert_eq!(sprawl.layout(node00).unwrap().location.x, 0f32);
    assert_eq!(sprawl.layout(node00).unwrap().location.y, 0f32);
    assert_eq!(sprawl.layout(node000).unwrap().size.width, 199f32);
    assert_eq!(sprawl.layout(node000).unwrap().size.height, 100f32);
    assert_eq!(sprawl.layout(node000).unwrap().location.x, 101f32);
    assert_eq!(sprawl.layout(node000).unwrap().location.y, 0f32);
}
