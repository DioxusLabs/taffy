#[test]
fn percentage_margin_should_calculate_based_only_on_width() {
    let layout = stretch::node::Node::new(
        stretch::style::Style {
            flex_direction: stretch::style::FlexDirection::Column,
            size: stretch::geometry::Size {
                width: stretch::style::Dimension::Points(200f32),
                height: stretch::style::Dimension::Points(100f32),
                ..Default::default()
            },
            ..Default::default()
        },
        vec![&stretch::node::Node::new(
            stretch::style::Style {
                flex_direction: stretch::style::FlexDirection::Column,
                flex_grow: 1f32,
                margin: stretch::geometry::Rect {
                    start: stretch::style::Dimension::Percent(0.1f32),
                    end: stretch::style::Dimension::Percent(0.1f32),
                    top: stretch::style::Dimension::Percent(0.1f32),
                    bottom: stretch::style::Dimension::Percent(0.1f32),
                    ..Default::default()
                },
                ..Default::default()
            },
            vec![&stretch::node::Node::new(
                stretch::style::Style {
                    size: stretch::geometry::Size {
                        width: stretch::style::Dimension::Points(10f32),
                        height: stretch::style::Dimension::Points(10f32),
                        ..Default::default()
                    },
                    ..Default::default()
                },
                vec![],
            )],
        )],
    )
    .compute_layout(stretch::geometry::Size::undefined())
    .unwrap();
    assert_eq!(layout.size.width, 200f32);
    assert_eq!(layout.size.height, 100f32);
    assert_eq!(layout.location.x, 0f32);
    assert_eq!(layout.location.y, 0f32);
    assert_eq!(layout.children[0usize].size.width, 160f32);
    assert_eq!(layout.children[0usize].size.height, 60f32);
    assert_eq!(layout.children[0usize].location.x, 20f32);
    assert_eq!(layout.children[0usize].location.y, 20f32);
    assert_eq!(layout.children[0usize].children[0usize].size.width, 10f32);
    assert_eq!(layout.children[0usize].children[0usize].size.height, 10f32);
    assert_eq!(layout.children[0usize].children[0usize].location.x, 0f32);
    assert_eq!(layout.children[0usize].children[0usize].location.y, 0f32);
}
