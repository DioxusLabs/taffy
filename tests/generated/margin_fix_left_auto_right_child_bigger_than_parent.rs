#[test]
fn margin_fix_left_auto_right_child_bigger_than_parent() {
    let mut stretch = stretch::Stretch::new();
    let node0 = stretch.new_node(
        stretch::style::Style {
            size: stretch::geometry::Size {
                width: stretch::style::Dimension::Points(72f32),
                height: stretch::style::Dimension::Points(72f32),
                ..Default::default()
            },
            margin: stretch::geometry::Rect {
                start: stretch::style::Dimension::Points(10f32),
                end: stretch::style::Dimension::Auto,
                ..Default::default()
            },
            ..Default::default()
        },
        vec![],
    );
    let node = stretch.new_node(
        stretch::style::Style {
            justify_content: stretch::style::JustifyContent::Center,
            size: stretch::geometry::Size {
                width: stretch::style::Dimension::Points(52f32),
                height: stretch::style::Dimension::Points(52f32),
                ..Default::default()
            },
            ..Default::default()
        },
        vec![node0],
    );
    stretch.compute_layout(node, stretch::geometry::Size::undefined()).unwrap();
    assert_eq!(stretch.layout(node).size.width, 52f32);
    assert_eq!(stretch.layout(node).size.height, 52f32);
    assert_eq!(stretch.layout(node).location.x, 0f32);
    assert_eq!(stretch.layout(node).location.y, 0f32);
    assert_eq!(stretch.layout(node0).size.width, 42f32);
    assert_eq!(stretch.layout(node0).size.height, 72f32);
    assert_eq!(stretch.layout(node0).location.x, 10f32);
    assert_eq!(stretch.layout(node0).location.y, 0f32);
}
