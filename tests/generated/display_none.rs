#[test]
fn display_none() {
    let layout = stretch::node::Node::new(
        stretch::style::Style {
            size: stretch::geometry::Size {
                width: stretch::style::Dimension::Points(100f32),
                height: stretch::style::Dimension::Points(100f32),
                ..Default::default()
            },
            ..Default::default()
        },
        vec![
            &stretch::node::Node::new(stretch::style::Style { flex_grow: 1f32, ..Default::default() }, vec![]),
            &stretch::node::Node::new(
                stretch::style::Style { display: stretch::style::Display::None, flex_grow: 1f32, ..Default::default() },
                vec![],
            ),
        ],
    )
    .compute_layout(stretch::geometry::Size::undefined())
    .unwrap();
    assert_eq!(layout.size.width, 100f32);
    assert_eq!(layout.size.height, 100f32);
    assert_eq!(layout.location.x, 0f32);
    assert_eq!(layout.location.y, 0f32);
    assert_eq!(layout.children[0usize].size.width, 100f32);
    assert_eq!(layout.children[0usize].size.height, 100f32);
    assert_eq!(layout.children[0usize].location.x, 0f32);
    assert_eq!(layout.children[0usize].location.y, 0f32);
    assert_eq!(layout.children[1usize].size.width, 0f32);
    assert_eq!(layout.children[1usize].size.height, 0f32);
    assert_eq!(layout.children[1usize].location.x, 0f32);
    assert_eq!(layout.children[1usize].location.y, 0f32);
}
