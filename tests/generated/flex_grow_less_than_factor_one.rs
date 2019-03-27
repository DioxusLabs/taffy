#[test]
fn flex_grow_less_than_factor_one() {
    let layout = stretch::compute(
        &stretch::style::Node {
            size: stretch::geometry::Size {
                width: stretch::style::Dimension::Points(500f32),
                height: stretch::style::Dimension::Points(200f32),
                ..Default::default()
            },
            children: vec![
                stretch::style::Node {
                    flex_grow: 0.2f32,
                    flex_shrink: 0f32,
                    flex_basis: stretch::style::Dimension::Points(40f32),
                    ..Default::default()
                },
                stretch::style::Node { flex_grow: 0.2f32, flex_shrink: 0f32, ..Default::default() },
                stretch::style::Node { flex_grow: 0.4f32, flex_shrink: 0f32, ..Default::default() },
            ],
            ..Default::default()
        },
        stretch::geometry::Size::undefined(),
    )
    .unwrap();
    assert_eq!(layout.size.width, 500f32);
    assert_eq!(layout.size.height, 200f32);
    assert_eq!(layout.location.x, 0f32);
    assert_eq!(layout.location.y, 0f32);
    assert_eq!(layout.children[0usize].size.width, 132f32);
    assert_eq!(layout.children[0usize].size.height, 200f32);
    assert_eq!(layout.children[0usize].location.x, 0f32);
    assert_eq!(layout.children[0usize].location.y, 0f32);
    assert_eq!(layout.children[1usize].size.width, 92f32);
    assert_eq!(layout.children[1usize].size.height, 200f32);
    assert_eq!(layout.children[1usize].location.x, 132f32);
    assert_eq!(layout.children[1usize].location.y, 0f32);
    assert_eq!(layout.children[2usize].size.width, 184f32);
    assert_eq!(layout.children[2usize].size.height, 200f32);
    assert_eq!(layout.children[2usize].location.x, 224f32);
    assert_eq!(layout.children[2usize].location.y, 0f32);
}
