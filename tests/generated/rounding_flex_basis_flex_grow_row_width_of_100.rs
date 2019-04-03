#[test]
fn rounding_flex_basis_flex_grow_row_width_of_100() {
    let layout = stretch::compute(
        &stretch::style::Node {
            size: stretch::geometry::Size {
                width: stretch::style::Dimension::Points(100f32),
                height: stretch::style::Dimension::Points(100f32),
                ..Default::default()
            },
            children: vec![
                stretch::style::Node { flex_grow: 1f32, ..Default::default() },
                stretch::style::Node { flex_grow: 1f32, ..Default::default() },
                stretch::style::Node { flex_grow: 1f32, ..Default::default() },
            ],
            ..Default::default()
        },
        stretch::geometry::Size::undefined(),
    )
    .unwrap();
    assert_eq!(layout.size.width, 100f32);
    assert_eq!(layout.size.height, 100f32);
    assert_eq!(layout.location.x, 0f32);
    assert_eq!(layout.location.y, 0f32);
    assert_eq!(layout.children[0usize].size.width, 33f32);
    assert_eq!(layout.children[0usize].size.height, 100f32);
    assert_eq!(layout.children[0usize].location.x, 0f32);
    assert_eq!(layout.children[0usize].location.y, 0f32);
    assert_eq!(layout.children[1usize].size.width, 34f32);
    assert_eq!(layout.children[1usize].size.height, 100f32);
    assert_eq!(layout.children[1usize].location.x, 33f32);
    assert_eq!(layout.children[1usize].location.y, 0f32);
    assert_eq!(layout.children[2usize].size.width, 33f32);
    assert_eq!(layout.children[2usize].size.height, 100f32);
    assert_eq!(layout.children[2usize].location.x, 67f32);
    assert_eq!(layout.children[2usize].location.y, 0f32);
}
