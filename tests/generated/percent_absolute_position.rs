#[test]
fn percent_absolute_position() {
    let mut stretch = stretch::Stretch::new();
    let node00 = stretch.new_node(
        stretch::style::Style {
            size: stretch::geometry::Size { width: stretch::style::Dimension::Percent(1f32), ..Default::default() },
            ..Default::default()
        },
        vec![],
    );
    let node01 = stretch.new_node(
        stretch::style::Style {
            size: stretch::geometry::Size { width: stretch::style::Dimension::Percent(1f32), ..Default::default() },
            ..Default::default()
        },
        vec![],
    );
    let node0 = stretch.new_node(
        stretch::style::Style {
            position_type: stretch::style::PositionType::Absolute,
            size: stretch::geometry::Size {
                width: stretch::style::Dimension::Percent(1f32),
                height: stretch::style::Dimension::Points(50f32),
                ..Default::default()
            },
            position: stretch::geometry::Rect {
                start: stretch::style::Dimension::Percent(0.5f32),
                ..Default::default()
            },
            ..Default::default()
        },
        vec![node00, node01],
    );
    let node = stretch.new_node(
        stretch::style::Style {
            flex_direction: stretch::style::FlexDirection::Column,
            size: stretch::geometry::Size {
                width: stretch::style::Dimension::Points(60f32),
                height: stretch::style::Dimension::Points(50f32),
                ..Default::default()
            },
            ..Default::default()
        },
        vec![node0],
    );
    stretch.compute_layout(node, stretch::geometry::Size::undefined()).unwrap();
    assert_eq!(stretch.layout(node).size.width, 60f32);
    assert_eq!(stretch.layout(node).size.height, 50f32);
    assert_eq!(stretch.layout(node).location.x, 0f32);
    assert_eq!(stretch.layout(node).location.y, 0f32);
    assert_eq!(stretch.layout(node0).size.width, 60f32);
    assert_eq!(stretch.layout(node0).size.height, 50f32);
    assert_eq!(stretch.layout(node0).location.x, 30f32);
    assert_eq!(stretch.layout(node0).location.y, 0f32);
    assert_eq!(stretch.layout(node00).size.width, 30f32);
    assert_eq!(stretch.layout(node00).size.height, 50f32);
    assert_eq!(stretch.layout(node00).location.x, 0f32);
    assert_eq!(stretch.layout(node00).location.y, 0f32);
    assert_eq!(stretch.layout(node01).size.width, 30f32);
    assert_eq!(stretch.layout(node01).size.height, 50f32);
    assert_eq!(stretch.layout(node01).location.x, 30f32);
    assert_eq!(stretch.layout(node01).location.y, 0f32);
}
