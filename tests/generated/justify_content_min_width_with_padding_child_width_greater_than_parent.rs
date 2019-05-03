#[test]
fn justify_content_min_width_with_padding_child_width_greater_than_parent() {
    let mut stretch = stretch::Stretch::new();
    let node000 = stretch.new_node(
        stretch::style::Style {
            size: stretch::geometry::Size {
                width: stretch::style::Dimension::Points(300f32),
                height: stretch::style::Dimension::Points(100f32),
                ..Default::default()
            },
            ..Default::default()
        },
        vec![],
    );
    let node00 = stretch.new_node(
        stretch::style::Style {
            justify_content: stretch::style::JustifyContent::Center,
            min_size: stretch::geometry::Size {
                width: stretch::style::Dimension::Points(400f32),
                ..Default::default()
            },
            padding: stretch::geometry::Rect {
                start: stretch::style::Dimension::Points(100f32),
                end: stretch::style::Dimension::Points(100f32),
                ..Default::default()
            },
            ..Default::default()
        },
        vec![node000],
    );
    let node0 = stretch.new_node(stretch::style::Style { ..Default::default() }, vec![node00]);
    let node = stretch.new_node(
        stretch::style::Style {
            flex_direction: stretch::style::FlexDirection::Column,
            size: stretch::geometry::Size {
                width: stretch::style::Dimension::Points(1000f32),
                height: stretch::style::Dimension::Points(1584f32),
                ..Default::default()
            },
            ..Default::default()
        },
        vec![node0],
    );
    stretch.compute_layout(node, stretch::geometry::Size::undefined()).unwrap();
    assert_eq!(stretch.layout(node).size.width, 1000f32);
    assert_eq!(stretch.layout(node).size.height, 1584f32);
    assert_eq!(stretch.layout(node).location.x, 0f32);
    assert_eq!(stretch.layout(node).location.y, 0f32);
    assert_eq!(stretch.layout(node0).size.width, 1000f32);
    assert_eq!(stretch.layout(node0).size.height, 100f32);
    assert_eq!(stretch.layout(node0).location.x, 0f32);
    assert_eq!(stretch.layout(node0).location.y, 0f32);
    assert_eq!(stretch.layout(node00).size.width, 500f32);
    assert_eq!(stretch.layout(node00).size.height, 100f32);
    assert_eq!(stretch.layout(node00).location.x, 0f32);
    assert_eq!(stretch.layout(node00).location.y, 0f32);
    assert_eq!(stretch.layout(node000).size.width, 300f32);
    assert_eq!(stretch.layout(node000).size.height, 100f32);
    assert_eq!(stretch.layout(node000).location.x, 100f32);
    assert_eq!(stretch.layout(node000).location.y, 0f32);
}
