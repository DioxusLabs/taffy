#[test]
fn margin_fix_left_auto_right_child_bigger_than_parent() {
    let layout = stretch::node::Node::new(
        stretch::style::Style {
            justify_content: stretch::style::JustifyContent::Center,
            size: stretch::geometry::Size {
                width: stretch::style::Dimension::Points(52f32),
                height: stretch::style::Dimension::Points(52f32),
                ..Default::default()
            },
            ..Default::default()
        },
        vec![&stretch::node::Node::new(
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
        )],
    )
    .compute_layout(stretch::geometry::Size::undefined())
    .unwrap();
    assert_eq!(layout.size.width, 52f32);
    assert_eq!(layout.size.height, 52f32);
    assert_eq!(layout.location.x, 0f32);
    assert_eq!(layout.location.y, 0f32);
    assert_eq!(layout.children[0usize].size.width, 42f32);
    assert_eq!(layout.children[0usize].size.height, 72f32);
    assert_eq!(layout.children[0usize].location.x, 10f32);
    assert_eq!(layout.children[0usize].location.y, 0f32);
}
