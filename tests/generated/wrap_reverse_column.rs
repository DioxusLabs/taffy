#[test]
fn wrap_reverse_column() {
    let mut stretch = stretch::Stretch::new();
    let node0 = stretch
        .new_node(
            stretch::style::Style {
                size: stretch::geometry::Size {
                    width: stretch::style::Dimension::Points(30f32),
                    height: stretch::style::Dimension::Points(31f32),
                    ..Default::default()
                },
                ..Default::default()
            },
            &[],
        )
        .unwrap();
    let node1 = stretch
        .new_node(
            stretch::style::Style {
                size: stretch::geometry::Size {
                    width: stretch::style::Dimension::Points(30f32),
                    height: stretch::style::Dimension::Points(32f32),
                    ..Default::default()
                },
                ..Default::default()
            },
            &[],
        )
        .unwrap();
    let node2 = stretch
        .new_node(
            stretch::style::Style {
                size: stretch::geometry::Size {
                    width: stretch::style::Dimension::Points(30f32),
                    height: stretch::style::Dimension::Points(33f32),
                    ..Default::default()
                },
                ..Default::default()
            },
            &[],
        )
        .unwrap();
    let node3 = stretch
        .new_node(
            stretch::style::Style {
                size: stretch::geometry::Size {
                    width: stretch::style::Dimension::Points(30f32),
                    height: stretch::style::Dimension::Points(34f32),
                    ..Default::default()
                },
                ..Default::default()
            },
            &[],
        )
        .unwrap();
    let node = stretch
        .new_node(
            stretch::style::Style {
                flex_direction: stretch::style::FlexDirection::Column,
                flex_wrap: stretch::style::FlexWrap::WrapReverse,
                size: stretch::geometry::Size {
                    width: stretch::style::Dimension::Points(100f32),
                    height: stretch::style::Dimension::Points(100f32),
                    ..Default::default()
                },
                ..Default::default()
            },
            &[node0, node1, node2, node3],
        )
        .unwrap();
    stretch.compute_layout(node, stretch::geometry::Size::undefined()).unwrap();
    assert_eq!(stretch.layout(node).unwrap().size.width, 100f32);
    assert_eq!(stretch.layout(node).unwrap().size.height, 100f32);
    assert_eq!(stretch.layout(node).unwrap().location.x, 0f32);
    assert_eq!(stretch.layout(node).unwrap().location.y, 0f32);
    assert_eq!(stretch.layout(node0).unwrap().size.width, 30f32);
    assert_eq!(stretch.layout(node0).unwrap().size.height, 31f32);
    assert_eq!(stretch.layout(node0).unwrap().location.x, 70f32);
    assert_eq!(stretch.layout(node0).unwrap().location.y, 0f32);
    assert_eq!(stretch.layout(node1).unwrap().size.width, 30f32);
    assert_eq!(stretch.layout(node1).unwrap().size.height, 32f32);
    assert_eq!(stretch.layout(node1).unwrap().location.x, 70f32);
    assert_eq!(stretch.layout(node1).unwrap().location.y, 31f32);
    assert_eq!(stretch.layout(node2).unwrap().size.width, 30f32);
    assert_eq!(stretch.layout(node2).unwrap().size.height, 33f32);
    assert_eq!(stretch.layout(node2).unwrap().location.x, 70f32);
    assert_eq!(stretch.layout(node2).unwrap().location.y, 63f32);
    assert_eq!(stretch.layout(node3).unwrap().size.width, 30f32);
    assert_eq!(stretch.layout(node3).unwrap().size.height, 34f32);
    assert_eq!(stretch.layout(node3).unwrap().location.x, 20f32);
    assert_eq!(stretch.layout(node3).unwrap().location.y, 0f32);
}
