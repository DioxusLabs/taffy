#[test]
fn percentage_position_bottom_right() {
    let mut stretch = stretch2::Stretch::new();
    let node0 = stretch
        .new_node(
            stretch2::style::Style {
                size: stretch2::geometry::Size {
                    width: stretch2::style::Dimension::Percent(0.55f32),
                    height: stretch2::style::Dimension::Percent(0.15f32),
                    ..Default::default()
                },
                position: stretch2::geometry::Rect {
                    end: stretch2::style::Dimension::Percent(0.2f32),
                    bottom: stretch2::style::Dimension::Percent(0.1f32),
                    ..Default::default()
                },
                ..Default::default()
            },
            &[],
        )
        .unwrap();
    let node = stretch
        .new_node(
            stretch2::style::Style {
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
    assert_eq!(stretch.layout(node0).unwrap().size.width, 275f32);
    assert_eq!(stretch.layout(node0).unwrap().size.height, 75f32);
    assert_eq!(stretch.layout(node0).unwrap().location.x, -100f32);
    assert_eq!(stretch.layout(node0).unwrap().location.y, -50f32);
}
