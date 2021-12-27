#[test]
fn wrap_nodes_with_content_sizing_margin_cross() {
    let mut stretch = stretch2::Stretch::new();
    let node000 = stretch
        .new_node(
            stretch2::style::Style {
                size: stretch2::geometry::Size {
                    width: stretch2::style::Dimension::Points(40f32),
                    height: stretch2::style::Dimension::Points(40f32),
                    ..Default::default()
                },
                ..Default::default()
            },
            &[],
        )
        .unwrap();
    let node00 = stretch
        .new_node(
            stretch2::style::Style { flex_direction: stretch2::style::FlexDirection::Column, ..Default::default() },
            &[node000],
        )
        .unwrap();
    let node010 = stretch
        .new_node(
            stretch2::style::Style {
                size: stretch2::geometry::Size {
                    width: stretch2::style::Dimension::Points(40f32),
                    height: stretch2::style::Dimension::Points(40f32),
                    ..Default::default()
                },
                ..Default::default()
            },
            &[],
        )
        .unwrap();
    let node01 = stretch
        .new_node(
            stretch2::style::Style {
                flex_direction: stretch2::style::FlexDirection::Column,
                margin: stretch2::geometry::Rect { top: stretch2::style::Dimension::Points(10f32), ..Default::default() },
                ..Default::default()
            },
            &[node010],
        )
        .unwrap();
    let node0 = stretch
        .new_node(
            stretch2::style::Style {
                flex_wrap: stretch2::style::FlexWrap::Wrap,
                size: stretch2::geometry::Size { width: stretch2::style::Dimension::Points(70f32), ..Default::default() },
                ..Default::default()
            },
            &[node00, node01],
        )
        .unwrap();
    let node = stretch
        .new_node(
            stretch2::style::Style {
                flex_direction: stretch2::style::FlexDirection::Column,
                size: stretch2::geometry::Size {
                    width: stretch2::style::Dimension::Points(500f32),
                    height: stretch2::style::Dimension::Points(500f32),
                    ..Default::default()
                },
                ..Default::default()
            },
            &[node0],
        )
        .unwrap();
    stretch.compute_layout(node, stretch2::geometry::Size::undefined()).unwrap();
    assert_eq!(stretch.layout(node).unwrap().size.width, 500f32);
    assert_eq!(stretch.layout(node).unwrap().size.height, 500f32);
    assert_eq!(stretch.layout(node).unwrap().location.x, 0f32);
    assert_eq!(stretch.layout(node).unwrap().location.y, 0f32);
    assert_eq!(stretch.layout(node0).unwrap().size.width, 70f32);
    assert_eq!(stretch.layout(node0).unwrap().size.height, 90f32);
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
    assert_eq!(stretch.layout(node01).unwrap().location.y, 50f32);
    assert_eq!(stretch.layout(node010).unwrap().size.width, 40f32);
    assert_eq!(stretch.layout(node010).unwrap().size.height, 40f32);
    assert_eq!(stretch.layout(node010).unwrap().location.x, 0f32);
    assert_eq!(stretch.layout(node010).unwrap().location.y, 0f32);
}
