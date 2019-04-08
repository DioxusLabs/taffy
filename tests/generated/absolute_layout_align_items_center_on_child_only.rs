#[test]
fn absolute_layout_align_items_center_on_child_only() {
    let layout = stretch::node::Node::new(
        stretch::style::Style {
            size: stretch::geometry::Size {
                width: stretch::style::Dimension::Points(110f32),
                height: stretch::style::Dimension::Points(100f32),
                ..Default::default()
            },
            ..Default::default()
        },
        vec![&stretch::node::Node::new(
            stretch::style::Style {
                position_type: stretch::style::PositionType::Absolute,
                align_self: stretch::style::AlignSelf::Center,
                size: stretch::geometry::Size {
                    width: stretch::style::Dimension::Points(60f32),
                    height: stretch::style::Dimension::Points(40f32),
                    ..Default::default()
                },
                ..Default::default()
            },
            vec![],
        )],
    )
    .compute_layout(stretch::geometry::Size::undefined())
    .unwrap();
    assert_eq!(layout.size.width, 110f32);
    assert_eq!(layout.size.height, 100f32);
    assert_eq!(layout.location.x, 0f32);
    assert_eq!(layout.location.y, 0f32);
    assert_eq!(layout.children[0usize].size.width, 60f32);
    assert_eq!(layout.children[0usize].size.height, 40f32);
    assert_eq!(layout.children[0usize].location.x, 0f32);
    assert_eq!(layout.children[0usize].location.y, 30f32);
}
