#[test]
fn percent_absolute_position() {
    let mut stretch = stretch2::Stretch::new();
    let node00 = stretch
        .new_node(
            stretch2::style::Style {
                size: stretch2::geometry::Size { width: stretch2::style::Dimension::Percent(1f32), ..Default::default() },
                ..Default::default()
            },
            &[],
        )
        .unwrap();
    let node01 = stretch
        .new_node(
            stretch2::style::Style {
                size: stretch2::geometry::Size { width: stretch2::style::Dimension::Percent(1f32), ..Default::default() },
                ..Default::default()
            },
            &[],
        )
        .unwrap();
    let node0 = stretch
        .new_node(
            stretch2::style::Style {
                position_type: stretch2::style::PositionType::Absolute,
                size: stretch2::geometry::Size {
                    width: stretch2::style::Dimension::Percent(1f32),
                    height: stretch2::style::Dimension::Points(50f32),
                    ..Default::default()
                },
                position: stretch2::geometry::Rect {
                    start: stretch2::style::Dimension::Percent(0.5f32),
                    ..Default::default()
                },
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
                    width: stretch2::style::Dimension::Points(60f32),
                    height: stretch2::style::Dimension::Points(50f32),
                    ..Default::default()
                },
                ..Default::default()
            },
            &[node0],
        )
        .unwrap();
    stretch.compute_layout(node, stretch2::geometry::Size::undefined()).unwrap();
    assert_eq!(stretch.layout(node).unwrap().size.width, 60f32);
    assert_eq!(stretch.layout(node).unwrap().size.height, 50f32);
    assert_eq!(stretch.layout(node).unwrap().location.x, 0f32);
    assert_eq!(stretch.layout(node).unwrap().location.y, 0f32);
    assert_eq!(stretch.layout(node0).unwrap().size.width, 60f32);
    assert_eq!(stretch.layout(node0).unwrap().size.height, 50f32);
    assert_eq!(stretch.layout(node0).unwrap().location.x, 30f32);
    assert_eq!(stretch.layout(node0).unwrap().location.y, 0f32);
    assert_eq!(stretch.layout(node00).unwrap().size.width, 30f32);
    assert_eq!(stretch.layout(node00).unwrap().size.height, 50f32);
    assert_eq!(stretch.layout(node00).unwrap().location.x, 0f32);
    assert_eq!(stretch.layout(node00).unwrap().location.y, 0f32);
    assert_eq!(stretch.layout(node01).unwrap().size.width, 30f32);
    assert_eq!(stretch.layout(node01).unwrap().size.height, 50f32);
    assert_eq!(stretch.layout(node01).unwrap().location.x, 30f32);
    assert_eq!(stretch.layout(node01).unwrap().location.y, 0f32);
}
