#[test]
fn percentage_size_based_on_parent_inner_size() {
    let layout = stretch::node::Node::new(
        stretch::style::Style {
            flex_direction: stretch::style::FlexDirection::Column,
            size: stretch::geometry::Size {
                width: stretch::style::Dimension::Points(200f32),
                height: stretch::style::Dimension::Points(400f32),
                ..Default::default()
            },
            padding: stretch::geometry::Rect {
                start: stretch::style::Dimension::Points(20f32),
                end: stretch::style::Dimension::Points(20f32),
                top: stretch::style::Dimension::Points(20f32),
                bottom: stretch::style::Dimension::Points(20f32),
                ..Default::default()
            },
            ..Default::default()
        },
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
    assert_eq!(layout.size.width, 200f32);
    assert_eq!(layout.size.height, 400f32);
    assert_eq!(layout.location.x, 0f32);
    assert_eq!(layout.location.y, 0f32);
    assert_eq!(layout.children[0usize].size.width, 80f32);
    assert_eq!(layout.children[0usize].size.height, 180f32);
    assert_eq!(layout.children[0usize].location.x, 20f32);
    assert_eq!(layout.children[0usize].location.y, 20f32);
}
