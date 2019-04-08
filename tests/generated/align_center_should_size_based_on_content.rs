#[test]
fn align_center_should_size_based_on_content() {
    let layout = stretch::node::Node::new(
        stretch::style::Style {
            align_items: stretch::style::AlignItems::Center,
            size: stretch::geometry::Size {
                width: stretch::style::Dimension::Points(100f32),
                height: stretch::style::Dimension::Points(100f32),
                ..Default::default()
            },
            ..Default::default()
        },
        vec![&stretch::node::Node::new(
            stretch::style::Style {
                justify_content: stretch::style::JustifyContent::Center,
                flex_grow: 0f32,
                flex_shrink: 1f32,
                ..Default::default()
            },
            vec![&stretch::node::Node::new(
                stretch::style::Style { flex_grow: 1f32, flex_shrink: 1f32, ..Default::default() },
                vec![&stretch::node::Node::new(
                    stretch::style::Style {
                        size: stretch::geometry::Size {
                            width: stretch::style::Dimension::Points(20f32),
                            height: stretch::style::Dimension::Points(20f32),
                            ..Default::default()
                        },
                        ..Default::default()
                    },
                    vec![],
                )],
            )],
        )],
    )
    .compute_layout(stretch::geometry::Size::undefined())
    .unwrap();
    assert_eq!(layout.size.width, 100f32);
    assert_eq!(layout.size.height, 100f32);
    assert_eq!(layout.location.x, 0f32);
    assert_eq!(layout.location.y, 0f32);
    assert_eq!(layout.children[0usize].size.width, 20f32);
    assert_eq!(layout.children[0usize].size.height, 20f32);
    assert_eq!(layout.children[0usize].location.x, 0f32);
    assert_eq!(layout.children[0usize].location.y, 40f32);
    assert_eq!(layout.children[0usize].children[0usize].size.width, 20f32);
    assert_eq!(layout.children[0usize].children[0usize].size.height, 20f32);
    assert_eq!(layout.children[0usize].children[0usize].location.x, 0f32);
    assert_eq!(layout.children[0usize].children[0usize].location.y, 0f32);
    assert_eq!(layout.children[0usize].children[0usize].children[0usize].size.width, 20f32);
    assert_eq!(layout.children[0usize].children[0usize].children[0usize].size.height, 20f32);
    assert_eq!(layout.children[0usize].children[0usize].children[0usize].location.x, 0f32);
    assert_eq!(layout.children[0usize].children[0usize].children[0usize].location.y, 0f32);
}
