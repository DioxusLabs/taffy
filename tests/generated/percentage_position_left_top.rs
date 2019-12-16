#[test]
fn percentage_position_left_top() {
    let mut stretch = stretch::Stretch::new();
    let node0 = stretch
        .new_node(
            stretch::style::Style {
                size: stretch::geometry::Size {
                    width: stretch::style::Dimension::Percent(0.45f32),
                    height: stretch::style::Dimension::Percent(0.55f32),
                    ..Default::default()
                },
                position: stretch::geometry::Rect {
                    start: stretch::style::Dimension::Percent(0.1f32),
                    top: stretch::style::Dimension::Percent(0.2f32),
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
                size: stretch::geometry::Size {
                    width: stretch::style::Dimension::Points(400f32),
                    height: stretch::style::Dimension::Points(400f32),
                    ..Default::default()
                },
                ..Default::default()
            },
            &[node0],
        )
        .unwrap();
    stretch.compute_layout(node, stretch::geometry::Size::undefined()).unwrap();
    assert_eq!(stretch.layout(node).unwrap().size.width, 400f32);
    assert_eq!(stretch.layout(node).unwrap().size.height, 400f32);
    assert_eq!(stretch.layout(node).unwrap().location.x, 0f32);
    assert_eq!(stretch.layout(node).unwrap().location.y, 0f32);
    assert_eq!(stretch.layout(node0).unwrap().size.width, 180f32);
    assert_eq!(stretch.layout(node0).unwrap().size.height, 220f32);
    assert_eq!(stretch.layout(node0).unwrap().location.x, 40f32);
    assert_eq!(stretch.layout(node0).unwrap().location.y, 80f32);
}
