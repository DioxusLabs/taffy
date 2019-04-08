#[test]
fn percentage_position_bottom_right() {
    let layout = stretch::node::Node::new(
        stretch::style::Style {
            size: stretch::geometry::Size {
                width: stretch::style::Dimension::Points(500f32),
                height: stretch::style::Dimension::Points(500f32),
                ..Default::default()
            },
            ..Default::default()
        },
        vec![&stretch::node::Node::new(
            stretch::style::Style {
                size: stretch::geometry::Size {
                    width: stretch::style::Dimension::Percent(0.55f32),
                    height: stretch::style::Dimension::Percent(0.15f32),
                    ..Default::default()
                },
                position: stretch::geometry::Rect {
                    end: stretch::style::Dimension::Percent(0.2f32),
                    bottom: stretch::style::Dimension::Percent(0.1f32),
                    ..Default::default()
                },
                ..Default::default()
            },
            vec![],
        )],
    )
    .compute_layout(stretch::geometry::Size::undefined())
    .unwrap();
    assert_eq!(layout.size.width, 500f32);
    assert_eq!(layout.size.height, 500f32);
    assert_eq!(layout.location.x, 0f32);
    assert_eq!(layout.location.y, 0f32);
    assert_eq!(layout.children[0usize].size.width, 275f32);
    assert_eq!(layout.children[0usize].size.height, 75f32);
    assert_eq!(layout.children[0usize].location.x, -100f32);
    assert_eq!(layout.children[0usize].location.y, -50f32);
}
