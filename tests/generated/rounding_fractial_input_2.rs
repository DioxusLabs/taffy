#[test]
fn rounding_fractial_input_2() {
    let mut stretch = stretch::Stretch::new();
    let node0 = stretch.new_node(
        stretch::style::Style {
            flex_grow: 1f32,
            flex_basis: stretch::style::Dimension::Points(50f32),
            size: stretch::geometry::Size { height: stretch::style::Dimension::Points(20f32), ..Default::default() },
            ..Default::default()
        },
        vec![],
    );
    let node1 = stretch.new_node(
        stretch::style::Style {
            flex_grow: 1f32,
            size: stretch::geometry::Size { height: stretch::style::Dimension::Points(10f32), ..Default::default() },
            ..Default::default()
        },
        vec![],
    );
    let node2 = stretch.new_node(
        stretch::style::Style {
            flex_grow: 1f32,
            size: stretch::geometry::Size { height: stretch::style::Dimension::Points(10f32), ..Default::default() },
            ..Default::default()
        },
        vec![],
    );
    let node = stretch.new_node(
        stretch::style::Style {
            flex_direction: stretch::style::FlexDirection::Column,
            size: stretch::geometry::Size {
                width: stretch::style::Dimension::Points(100f32),
                height: stretch::style::Dimension::Points(113.6f32),
                ..Default::default()
            },
            ..Default::default()
        },
        vec![node0, node1, node2],
    );
    stretch.compute_layout(node, stretch::geometry::Size::undefined()).unwrap();
    assert_eq!(stretch.layout(node).size.width, 100f32);
    assert_eq!(stretch.layout(node).size.height, 114f32);
    assert_eq!(stretch.layout(node).location.x, 0f32);
    assert_eq!(stretch.layout(node).location.y, 0f32);
    assert_eq!(stretch.layout(node0).size.width, 100f32);
    assert_eq!(stretch.layout(node0).size.height, 65f32);
    assert_eq!(stretch.layout(node0).location.x, 0f32);
    assert_eq!(stretch.layout(node0).location.y, 0f32);
    assert_eq!(stretch.layout(node1).size.width, 100f32);
    assert_eq!(stretch.layout(node1).size.height, 24f32);
    assert_eq!(stretch.layout(node1).location.x, 0f32);
    assert_eq!(stretch.layout(node1).location.y, 65f32);
    assert_eq!(stretch.layout(node2).size.width, 100f32);
    assert_eq!(stretch.layout(node2).size.height, 25f32);
    assert_eq!(stretch.layout(node2).location.x, 0f32);
    assert_eq!(stretch.layout(node2).location.y, 89f32);
}
