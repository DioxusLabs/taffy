#[test]
fn rounding_flex_basis_flex_grow_row_prime_number_width() {
    let layout = stretch::node::Node::new(
        stretch::style::Style {
            size: stretch::geometry::Size {
                width: stretch::style::Dimension::Points(113f32),
                height: stretch::style::Dimension::Points(100f32),
                ..Default::default()
            },
            ..Default::default()
        },
        vec![
            &stretch::node::Node::new(stretch::style::Style { flex_grow: 1f32, ..Default::default() }, vec![]),
            &stretch::node::Node::new(stretch::style::Style { flex_grow: 1f32, ..Default::default() }, vec![]),
            &stretch::node::Node::new(stretch::style::Style { flex_grow: 1f32, ..Default::default() }, vec![]),
            &stretch::node::Node::new(stretch::style::Style { flex_grow: 1f32, ..Default::default() }, vec![]),
            &stretch::node::Node::new(stretch::style::Style { flex_grow: 1f32, ..Default::default() }, vec![]),
        ],
    )
    .compute_layout(stretch::geometry::Size::undefined())
    .unwrap();
    assert_eq!(layout.size.width, 113f32);
    assert_eq!(layout.size.height, 100f32);
    assert_eq!(layout.location.x, 0f32);
    assert_eq!(layout.location.y, 0f32);
    assert_eq!(layout.children[0usize].size.width, 23f32);
    assert_eq!(layout.children[0usize].size.height, 100f32);
    assert_eq!(layout.children[0usize].location.x, 0f32);
    assert_eq!(layout.children[0usize].location.y, 0f32);
    assert_eq!(layout.children[1usize].size.width, 22f32);
    assert_eq!(layout.children[1usize].size.height, 100f32);
    assert_eq!(layout.children[1usize].location.x, 23f32);
    assert_eq!(layout.children[1usize].location.y, 0f32);
    assert_eq!(layout.children[2usize].size.width, 23f32);
    assert_eq!(layout.children[2usize].size.height, 100f32);
    assert_eq!(layout.children[2usize].location.x, 45f32);
    assert_eq!(layout.children[2usize].location.y, 0f32);
    assert_eq!(layout.children[3usize].size.width, 22f32);
    assert_eq!(layout.children[3usize].size.height, 100f32);
    assert_eq!(layout.children[3usize].location.x, 68f32);
    assert_eq!(layout.children[3usize].location.y, 0f32);
    assert_eq!(layout.children[4usize].size.width, 23f32);
    assert_eq!(layout.children[4usize].size.height, 100f32);
    assert_eq!(layout.children[4usize].location.x, 90f32);
    assert_eq!(layout.children[4usize].location.y, 0f32);
}
