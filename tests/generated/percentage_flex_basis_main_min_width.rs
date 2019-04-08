#[test]
fn percentage_flex_basis_main_min_width() {
    let layout = stretch::node::Node::new(
        stretch::style::Style {
            size: stretch::geometry::Size {
                width: stretch::style::Dimension::Points(200f32),
                height: stretch::style::Dimension::Points(400f32),
                ..Default::default()
            },
            ..Default::default()
        },
        vec![
            &stretch::node::Node::new(
                stretch::style::Style {
                    flex_grow: 1f32,
                    flex_basis: stretch::style::Dimension::Percent(0.15f32),
                    min_size: stretch::geometry::Size {
                        width: stretch::style::Dimension::Percent(0.6f32),
                        ..Default::default()
                    },
                    ..Default::default()
                },
                vec![],
            ),
            &stretch::node::Node::new(
                stretch::style::Style {
                    flex_grow: 4f32,
                    flex_basis: stretch::style::Dimension::Percent(0.1f32),
                    min_size: stretch::geometry::Size {
                        width: stretch::style::Dimension::Percent(0.2f32),
                        ..Default::default()
                    },
                    ..Default::default()
                },
                vec![],
            ),
        ],
    )
    .compute_layout(stretch::geometry::Size::undefined())
    .unwrap();
    assert_eq!(layout.size.width, 200f32);
    assert_eq!(layout.size.height, 400f32);
    assert_eq!(layout.location.x, 0f32);
    assert_eq!(layout.location.y, 0f32);
    assert_eq!(layout.children[0usize].size.width, 120f32);
    assert_eq!(layout.children[0usize].size.height, 400f32);
    assert_eq!(layout.children[0usize].location.x, 0f32);
    assert_eq!(layout.children[0usize].location.y, 0f32);
    assert_eq!(layout.children[1usize].size.width, 80f32);
    assert_eq!(layout.children[1usize].size.height, 400f32);
    assert_eq!(layout.children[1usize].location.x, 120f32);
    assert_eq!(layout.children[1usize].location.y, 0f32);
}
