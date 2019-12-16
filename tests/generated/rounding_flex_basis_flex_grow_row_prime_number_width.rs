#[test]
fn rounding_flex_basis_flex_grow_row_prime_number_width() {
    let mut stretch = stretch::Stretch::new();
    let node0 = stretch.new_node(stretch::style::Style { flex_grow: 1f32, ..Default::default() }, &[]).unwrap();
    let node1 = stretch.new_node(stretch::style::Style { flex_grow: 1f32, ..Default::default() }, &[]).unwrap();
    let node2 = stretch.new_node(stretch::style::Style { flex_grow: 1f32, ..Default::default() }, &[]).unwrap();
    let node3 = stretch.new_node(stretch::style::Style { flex_grow: 1f32, ..Default::default() }, &[]).unwrap();
    let node4 = stretch.new_node(stretch::style::Style { flex_grow: 1f32, ..Default::default() }, &[]).unwrap();
    let node = stretch
        .new_node(
            stretch::style::Style {
                size: stretch::geometry::Size {
                    width: stretch::style::Dimension::Points(113f32),
                    height: stretch::style::Dimension::Points(100f32),
                    ..Default::default()
                },
                ..Default::default()
            },
            &[node0, node1, node2, node3, node4],
        )
        .unwrap();
    stretch.compute_layout(node, stretch::geometry::Size::undefined()).unwrap();
    assert_eq!(stretch.layout(node).unwrap().size.width, 113f32);
    assert_eq!(stretch.layout(node).unwrap().size.height, 100f32);
    assert_eq!(stretch.layout(node).unwrap().location.x, 0f32);
    assert_eq!(stretch.layout(node).unwrap().location.y, 0f32);
    assert_eq!(stretch.layout(node0).unwrap().size.width, 23f32);
    assert_eq!(stretch.layout(node0).unwrap().size.height, 100f32);
    assert_eq!(stretch.layout(node0).unwrap().location.x, 0f32);
    assert_eq!(stretch.layout(node0).unwrap().location.y, 0f32);
    assert_eq!(stretch.layout(node1).unwrap().size.width, 22f32);
    assert_eq!(stretch.layout(node1).unwrap().size.height, 100f32);
    assert_eq!(stretch.layout(node1).unwrap().location.x, 23f32);
    assert_eq!(stretch.layout(node1).unwrap().location.y, 0f32);
    assert_eq!(stretch.layout(node2).unwrap().size.width, 23f32);
    assert_eq!(stretch.layout(node2).unwrap().size.height, 100f32);
    assert_eq!(stretch.layout(node2).unwrap().location.x, 45f32);
    assert_eq!(stretch.layout(node2).unwrap().location.y, 0f32);
    assert_eq!(stretch.layout(node3).unwrap().size.width, 22f32);
    assert_eq!(stretch.layout(node3).unwrap().size.height, 100f32);
    assert_eq!(stretch.layout(node3).unwrap().location.x, 68f32);
    assert_eq!(stretch.layout(node3).unwrap().location.y, 0f32);
    assert_eq!(stretch.layout(node4).unwrap().size.width, 23f32);
    assert_eq!(stretch.layout(node4).unwrap().size.height, 100f32);
    assert_eq!(stretch.layout(node4).unwrap().location.x, 90f32);
    assert_eq!(stretch.layout(node4).unwrap().location.y, 0f32);
}
