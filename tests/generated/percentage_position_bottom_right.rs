#[test]
fn percentage_position_bottom_right() {
    let mut stretch = stretch::Stretch::new();
    let node0 = stretch.new_node(
        stretch::style::Style {
            size: stretch::geometry::Size {
                width: stretch::style::Dimension::Percent(0.55f32),
                height: stretch::style::Dimension::Percent(0.15f32),
                ..Default::default()
            },
            position: stretch::geometry::Rect {
                end: stretch::style::Dimension::Percent(0.2f32),
                bottom: stretch::style::Dimension::Percent(0.1f32),
                ..Default::default()
            },
            ..Default::default()
        },
        vec![],
    );
    let node = stretch.new_node(
        stretch::style::Style {
            size: stretch::geometry::Size {
                width: stretch::style::Dimension::Points(500f32),
                height: stretch::style::Dimension::Points(500f32),
                ..Default::default()
            },
            ..Default::default()
        },
        vec![node0],
    );
    stretch.compute_layout(node, stretch::geometry::Size::undefined()).unwrap();
    assert_eq!(stretch.layout(node).size.width, 500f32);
    assert_eq!(stretch.layout(node).size.height, 500f32);
    assert_eq!(stretch.layout(node).location.x, 0f32);
    assert_eq!(stretch.layout(node).location.y, 0f32);
    assert_eq!(stretch.layout(node0).size.width, 275f32);
    assert_eq!(stretch.layout(node0).size.height, 75f32);
    assert_eq!(stretch.layout(node0).location.x, -100f32);
    assert_eq!(stretch.layout(node0).location.y, -50f32);
}
