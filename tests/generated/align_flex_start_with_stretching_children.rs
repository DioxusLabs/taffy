#[test]
fn align_flex_start_with_stretching_children() {
    let mut stretch = stretch::Stretch::new();
    let node000 =
        stretch.new_node(stretch::style::Style { flex_grow: 1f32, flex_shrink: 1f32, ..Default::default() }, vec![]);
    let node00 = stretch
        .new_node(stretch::style::Style { flex_grow: 1f32, flex_shrink: 1f32, ..Default::default() }, vec![node000]);
    let node0 = stretch.new_node(stretch::style::Style { ..Default::default() }, vec![node00]);
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
    assert_eq!(stretch.layout(node0).size.width, 0f32);
    assert_eq!(stretch.layout(node0).size.height, 500f32);
    assert_eq!(stretch.layout(node0).location.x, 0f32);
    assert_eq!(stretch.layout(node0).location.y, 0f32);
    assert_eq!(stretch.layout(node00).size.width, 0f32);
    assert_eq!(stretch.layout(node00).size.height, 500f32);
    assert_eq!(stretch.layout(node00).location.x, 0f32);
    assert_eq!(stretch.layout(node00).location.y, 0f32);
    assert_eq!(stretch.layout(node000).size.width, 0f32);
    assert_eq!(stretch.layout(node000).size.height, 500f32);
    assert_eq!(stretch.layout(node000).location.x, 0f32);
    assert_eq!(stretch.layout(node000).location.y, 0f32);
}
