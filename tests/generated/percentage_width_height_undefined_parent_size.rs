#[test]
fn percentage_width_height_undefined_parent_size() {
    let layout = stretch::node::Node::new(
        stretch::style::Style { flex_direction: stretch::style::FlexDirection::Column, ..Default::default() },
        vec![&stretch::node::Node::new(
            stretch::style::Style {
                size: stretch::geometry::Size {
                    width: stretch::style::Dimension::Percent(0.5f32),
                    height: stretch::style::Dimension::Percent(0.5f32),
                    ..Default::default()
                },
                ..Default::default()
            },
            vec![],
        )],
    )
    .compute_layout(stretch::geometry::Size::undefined())
    .unwrap();
    assert_eq!(layout.size.width, 0f32);
    assert_eq!(layout.size.height, 0f32);
    assert_eq!(layout.location.x, 0f32);
    assert_eq!(layout.location.y, 0f32);
    assert_eq!(layout.children[0usize].size.width, 0f32);
    assert_eq!(layout.children[0usize].size.height, 0f32);
    assert_eq!(layout.children[0usize].location.x, 0f32);
    assert_eq!(layout.children[0usize].location.y, 0f32);
}
