#[test]
fn align_items_min_max() {
    let layout = stretch::node::Node::new(
        stretch::style::Style {
            flex_direction: stretch::style::FlexDirection::Column,
            align_items: stretch::style::AlignItems::Center,
            size: stretch::geometry::Size { height: stretch::style::Dimension::Points(100f32), ..Default::default() },
            min_size: stretch::geometry::Size {
                width: stretch::style::Dimension::Points(100f32),
                ..Default::default()
            },
            max_size: stretch::geometry::Size {
                width: stretch::style::Dimension::Points(200f32),
                ..Default::default()
            },
            ..Default::default()
        },
        vec![&stretch::node::Node::new(
            stretch::style::Style {
                size: stretch::geometry::Size {
                    width: stretch::style::Dimension::Points(60f32),
                    height: stretch::style::Dimension::Points(60f32),
                    ..Default::default()
                },
                ..Default::default()
            },
            vec![],
        )],
    )
    .compute_layout(stretch::geometry::Size::undefined())
    .unwrap();
    assert_eq!(layout.size.width, 100f32);
    assert_eq!(layout.size.height, 100f32);
    assert_eq!(layout.location.x, 0f32);
    assert_eq!(layout.location.y, 0f32);
    assert_eq!(layout.children[0usize].size.width, 60f32);
    assert_eq!(layout.children[0usize].size.height, 60f32);
    assert_eq!(layout.children[0usize].location.x, 20f32);
    assert_eq!(layout.children[0usize].location.y, 0f32);
}
