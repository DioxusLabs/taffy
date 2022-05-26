#[test]
fn wrap_nodes_with_content_sizing_overflowing_margin() {
    let mut stretch = sprawl::Stretch::new();
    let node000 = stretch
        .new_node(
            sprawl::style::Style {
                size: sprawl::geometry::Size {
                    width: sprawl::style::Dimension::Points(40f32),
                    height: sprawl::style::Dimension::Points(40f32),
                    ..Default::default()
                },
                ..Default::default()
            },
            &[],
        )
        .unwrap();
    let node00 = stretch
        .new_node(
            sprawl::style::Style { flex_direction: sprawl::style::FlexDirection::Column, ..Default::default() },
            &[node000],
        )
        .unwrap();
    let node010 = stretch
        .new_node(
            sprawl::style::Style {
                size: sprawl::geometry::Size {
                    width: sprawl::style::Dimension::Points(40f32),
                    height: sprawl::style::Dimension::Points(40f32),
                    ..Default::default()
                },
                ..Default::default()
            },
            &[],
        )
        .unwrap();
    let node01 = stretch
        .new_node(
            sprawl::style::Style {
                flex_direction: sprawl::style::FlexDirection::Column,
                margin: sprawl::geometry::Rect { end: sprawl::style::Dimension::Points(10f32), ..Default::default() },
                ..Default::default()
            },
            &[node010],
        )
        .unwrap();
    let node0 = stretch
        .new_node(
            sprawl::style::Style {
                flex_wrap: sprawl::style::FlexWrap::Wrap,
                size: sprawl::geometry::Size { width: sprawl::style::Dimension::Points(85f32), ..Default::default() },
                ..Default::default()
            },
            &[node00, node01],
        )
        .unwrap();
    let node = stretch
        .new_node(
            sprawl::style::Style {
                flex_direction: sprawl::style::FlexDirection::Column,
                size: sprawl::geometry::Size {
                    width: sprawl::style::Dimension::Points(500f32),
                    height: sprawl::style::Dimension::Points(500f32),
                    ..Default::default()
                },
                ..Default::default()
            },
            &[node0],
        )
        .unwrap();
    stretch.compute_layout(node, sprawl::geometry::Size::undefined()).unwrap();
    assert_eq!(stretch.layout(node).unwrap().size.width, 500f32);
    assert_eq!(stretch.layout(node).unwrap().size.height, 500f32);
    assert_eq!(stretch.layout(node).unwrap().location.x, 0f32);
    assert_eq!(stretch.layout(node).unwrap().location.y, 0f32);
    assert_eq!(stretch.layout(node0).unwrap().size.width, 85f32);
    assert_eq!(stretch.layout(node0).unwrap().size.height, 80f32);
    assert_eq!(stretch.layout(node0).unwrap().location.x, 0f32);
    assert_eq!(stretch.layout(node0).unwrap().location.y, 0f32);
    assert_eq!(stretch.layout(node00).unwrap().size.width, 40f32);
    assert_eq!(stretch.layout(node00).unwrap().size.height, 40f32);
    assert_eq!(stretch.layout(node00).unwrap().location.x, 0f32);
    assert_eq!(stretch.layout(node00).unwrap().location.y, 0f32);
    assert_eq!(stretch.layout(node000).unwrap().size.width, 40f32);
    assert_eq!(stretch.layout(node000).unwrap().size.height, 40f32);
    assert_eq!(stretch.layout(node000).unwrap().location.x, 0f32);
    assert_eq!(stretch.layout(node000).unwrap().location.y, 0f32);
    assert_eq!(stretch.layout(node01).unwrap().size.width, 40f32);
    assert_eq!(stretch.layout(node01).unwrap().size.height, 40f32);
    assert_eq!(stretch.layout(node01).unwrap().location.x, 0f32);
    assert_eq!(stretch.layout(node01).unwrap().location.y, 40f32);
    assert_eq!(stretch.layout(node010).unwrap().size.width, 40f32);
    assert_eq!(stretch.layout(node010).unwrap().size.height, 40f32);
    assert_eq!(stretch.layout(node010).unwrap().location.x, 0f32);
    assert_eq!(stretch.layout(node010).unwrap().location.y, 0f32);
}
