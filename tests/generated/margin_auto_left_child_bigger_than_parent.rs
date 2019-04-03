#[test]
fn margin_auto_left_child_bigger_than_parent() {
    let layout = stretch::compute(
        &stretch::style::Node {
            justify_content: stretch::style::JustifyContent::Center,
            size: stretch::geometry::Size {
                width: stretch::style::Dimension::Points(52f32),
                height: stretch::style::Dimension::Points(52f32),
                ..Default::default()
            },
            children: vec![stretch::style::Node {
                size: stretch::geometry::Size {
                    width: stretch::style::Dimension::Points(72f32),
                    height: stretch::style::Dimension::Points(72f32),
                    ..Default::default()
                },
                margin: stretch::geometry::Rect { start: stretch::style::Dimension::Auto, ..Default::default() },
                ..Default::default()
            }],
            ..Default::default()
        },
        stretch::geometry::Size::undefined(),
    )
    .unwrap();
    assert_eq!(layout.size.width, 52f32);
    assert_eq!(layout.size.height, 52f32);
    assert_eq!(layout.location.x, 0f32);
    assert_eq!(layout.location.y, 0f32);
    assert_eq!(layout.children[0usize].size.width, 52f32);
    assert_eq!(layout.children[0usize].size.height, 72f32);
    assert_eq!(layout.children[0usize].location.x, 0f32);
    assert_eq!(layout.children[0usize].location.y, 0f32);
}
