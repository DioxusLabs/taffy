#[test]
fn align_items_center_child_without_margin_bigger_than_parent() {
    let mut stretch = stretch::Stretch::new();
    let node00 = stretch.new_node(
        stretch::style::Style {
            size: stretch::geometry::Size {
                width: stretch::style::Dimension::Points(70f32),
                height: stretch::style::Dimension::Points(70f32),
                ..Default::default()
            },
            ..Default::default()
        },
        vec![],
    );
    let node0 = stretch.new_node(
        stretch::style::Style { align_items: stretch::style::AlignItems::Center, ..Default::default() },
        vec![node00],
    );
    let node = stretch.new_node(
        stretch::style::Style {
            align_items: stretch::style::AlignItems::Center,
            justify_content: stretch::style::JustifyContent::Center,
            size: stretch::geometry::Size {
                width: stretch::style::Dimension::Points(50f32),
                height: stretch::style::Dimension::Points(50f32),
                ..Default::default()
            },
            ..Default::default()
        },
        vec![node0],
    );
    stretch.compute_layout(node, stretch::geometry::Size::undefined()).unwrap();
    assert_eq!(stretch.layout(node).size.width, 50f32);
    assert_eq!(stretch.layout(node).size.height, 50f32);
    assert_eq!(stretch.layout(node).location.x, 0f32);
    assert_eq!(stretch.layout(node).location.y, 0f32);
    assert_eq!(stretch.layout(node0).size.width, 70f32);
    assert_eq!(stretch.layout(node0).size.height, 70f32);
    assert_eq!(stretch.layout(node0).location.x, -10f32);
    assert_eq!(stretch.layout(node0).location.y, -10f32);
    assert_eq!(stretch.layout(node00).size.width, 70f32);
    assert_eq!(stretch.layout(node00).size.height, 70f32);
    assert_eq!(stretch.layout(node00).location.x, 0f32);
    assert_eq!(stretch.layout(node00).location.y, 0f32);
}
