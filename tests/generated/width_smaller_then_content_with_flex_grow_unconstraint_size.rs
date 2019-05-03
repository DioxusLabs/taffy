#[test]
fn width_smaller_then_content_with_flex_grow_unconstraint_size() {
    let mut stretch = stretch::Stretch::new();
    let node00 = stretch.new_node(
        stretch::style::Style {
            size: stretch::geometry::Size {
                width: stretch::style::Dimension::Points(70f32),
                height: stretch::style::Dimension::Points(100f32),
                ..Default::default()
            },
            ..Default::default()
        },
        vec![],
    );
    let node0 = stretch.new_node(
        stretch::style::Style {
            flex_direction: stretch::style::FlexDirection::Column,
            flex_grow: 1f32,
            size: stretch::geometry::Size { width: stretch::style::Dimension::Points(0f32), ..Default::default() },
            ..Default::default()
        },
        vec![node00],
    );
    let node10 = stretch.new_node(
        stretch::style::Style {
            size: stretch::geometry::Size {
                width: stretch::style::Dimension::Points(20f32),
                height: stretch::style::Dimension::Points(100f32),
                ..Default::default()
            },
            ..Default::default()
        },
        vec![],
    );
    let node1 = stretch.new_node(
        stretch::style::Style {
            flex_direction: stretch::style::FlexDirection::Column,
            flex_grow: 1f32,
            size: stretch::geometry::Size { width: stretch::style::Dimension::Points(0f32), ..Default::default() },
            ..Default::default()
        },
        vec![node10],
    );
    let node = stretch.new_node(stretch::style::Style { ..Default::default() }, vec![node0, node1]);
    stretch.compute_layout(node, stretch::geometry::Size::undefined()).unwrap();
    assert_eq!(stretch.layout(node).size.width, 0f32);
    assert_eq!(stretch.layout(node).size.height, 100f32);
    assert_eq!(stretch.layout(node).location.x, 0f32);
    assert_eq!(stretch.layout(node).location.y, 0f32);
    assert_eq!(stretch.layout(node0).size.width, 0f32);
    assert_eq!(stretch.layout(node0).size.height, 100f32);
    assert_eq!(stretch.layout(node0).location.x, 0f32);
    assert_eq!(stretch.layout(node0).location.y, 0f32);
    assert_eq!(stretch.layout(node00).size.width, 70f32);
    assert_eq!(stretch.layout(node00).size.height, 100f32);
    assert_eq!(stretch.layout(node00).location.x, 0f32);
    assert_eq!(stretch.layout(node00).location.y, 0f32);
    assert_eq!(stretch.layout(node1).size.width, 0f32);
    assert_eq!(stretch.layout(node1).size.height, 100f32);
    assert_eq!(stretch.layout(node1).location.x, 0f32);
    assert_eq!(stretch.layout(node1).location.y, 0f32);
    assert_eq!(stretch.layout(node10).size.width, 20f32);
    assert_eq!(stretch.layout(node10).size.height, 100f32);
    assert_eq!(stretch.layout(node10).location.x, 0f32);
    assert_eq!(stretch.layout(node10).location.y, 0f32);
}
