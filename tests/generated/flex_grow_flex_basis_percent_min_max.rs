#[test]
fn flex_grow_flex_basis_percent_min_max() {
    let mut stretch = stretch::Stretch::new();
    let node0 = stretch.new_node(
        stretch::style::Style {
            flex_grow: 1f32,
            flex_shrink: 0f32,
            flex_basis: stretch::style::Dimension::Points(0f32),
            size: stretch::geometry::Size { height: stretch::style::Dimension::Points(20f32), ..Default::default() },
            min_size: stretch::geometry::Size { width: stretch::style::Dimension::Points(60f32), ..Default::default() },
            ..Default::default()
        },
        vec![],
    );
    let node1 = stretch.new_node(
        stretch::style::Style {
            flex_grow: 1f32,
            flex_shrink: 0f32,
            flex_basis: stretch::style::Dimension::Percent(0.5f32),
            size: stretch::geometry::Size {
                width: stretch::style::Dimension::Points(20f32),
                height: stretch::style::Dimension::Points(20f32),
                ..Default::default()
            },
            max_size: stretch::geometry::Size { width: stretch::style::Dimension::Points(20f32), ..Default::default() },
            ..Default::default()
        },
        vec![],
    );
    let node = stretch.new_node(
        stretch::style::Style {
            size: stretch::geometry::Size { width: stretch::style::Dimension::Points(120f32), ..Default::default() },
            ..Default::default()
        },
        vec![node0, node1],
    );
    stretch.compute_layout(node, stretch::geometry::Size::undefined()).unwrap();
    assert_eq!(stretch.layout(node).size.width, 120f32);
    assert_eq!(stretch.layout(node).size.height, 20f32);
    assert_eq!(stretch.layout(node).location.x, 0f32);
    assert_eq!(stretch.layout(node).location.y, 0f32);
    assert_eq!(stretch.layout(node0).size.width, 100f32);
    assert_eq!(stretch.layout(node0).size.height, 20f32);
    assert_eq!(stretch.layout(node0).location.x, 0f32);
    assert_eq!(stretch.layout(node0).location.y, 0f32);
    assert_eq!(stretch.layout(node1).size.width, 20f32);
    assert_eq!(stretch.layout(node1).size.height, 20f32);
    assert_eq!(stretch.layout(node1).location.x, 100f32);
    assert_eq!(stretch.layout(node1).location.y, 0f32);
}
