#[test]
fn align_center_should_size_based_on_content() {
    let mut stretch = stretch::Stretch::new();
    let node000 = stretch.new_node(
        stretch::style::Style {
            size: stretch::geometry::Size {
                width: stretch::style::Dimension::Points(20f32),
                height: stretch::style::Dimension::Points(20f32),
                ..Default::default()
            },
            ..Default::default()
        },
        vec![],
    );
    let node00 = stretch
        .new_node(stretch::style::Style { flex_grow: 1f32, flex_shrink: 1f32, ..Default::default() }, vec![node000]);
    let node0 = stretch.new_node(
        stretch::style::Style {
            justify_content: stretch::style::JustifyContent::Center,
            flex_grow: 0f32,
            flex_shrink: 1f32,
            ..Default::default()
        },
        vec![node00],
    );
    let node = stretch.new_node(
        stretch::style::Style {
            align_items: stretch::style::AlignItems::Center,
            size: stretch::geometry::Size {
                width: stretch::style::Dimension::Points(100f32),
                height: stretch::style::Dimension::Points(100f32),
                ..Default::default()
            },
            ..Default::default()
        },
        vec![node0],
    );
    stretch.compute_layout(node, stretch::geometry::Size::undefined()).unwrap();
    assert_eq!(stretch.layout(node).size.width, 100f32);
    assert_eq!(stretch.layout(node).size.height, 100f32);
    assert_eq!(stretch.layout(node).location.x, 0f32);
    assert_eq!(stretch.layout(node).location.y, 0f32);
    assert_eq!(stretch.layout(node0).size.width, 20f32);
    assert_eq!(stretch.layout(node0).size.height, 20f32);
    assert_eq!(stretch.layout(node0).location.x, 0f32);
    assert_eq!(stretch.layout(node0).location.y, 40f32);
    assert_eq!(stretch.layout(node00).size.width, 20f32);
    assert_eq!(stretch.layout(node00).size.height, 20f32);
    assert_eq!(stretch.layout(node00).location.x, 0f32);
    assert_eq!(stretch.layout(node00).location.y, 0f32);
    assert_eq!(stretch.layout(node000).size.width, 20f32);
    assert_eq!(stretch.layout(node000).size.height, 20f32);
    assert_eq!(stretch.layout(node000).location.x, 0f32);
    assert_eq!(stretch.layout(node000).location.y, 0f32);
}
