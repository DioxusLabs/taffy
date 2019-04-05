#[test]
fn flex_grow_flex_basis_percent_min_max() {
    let layout = stretch::node::Node::new(
        stretch::style::Style {
            size: stretch::geometry::Size { width: stretch::style::Dimension::Points(120f32), ..Default::default() },
            ..Default::default()
        },
        vec![
            &stretch::node::Node::new(
                stretch::style::Style {
                    flex_grow: 1f32,
                    flex_shrink: 0f32,
                    flex_basis: stretch::style::Dimension::Points(0f32),
                    size: stretch::geometry::Size {
                        height: stretch::style::Dimension::Points(20f32),
                        ..Default::default()
                    },
                    min_size: stretch::geometry::Size {
                        width: stretch::style::Dimension::Points(60f32),
                        ..Default::default()
                    },
                    ..Default::default()
                },
                vec![],
            ),
            &stretch::node::Node::new(
                stretch::style::Style {
                    flex_grow: 1f32,
                    flex_shrink: 0f32,
                    flex_basis: stretch::style::Dimension::Percent(0.5f32),
                    size: stretch::geometry::Size {
                        width: stretch::style::Dimension::Points(20f32),
                        height: stretch::style::Dimension::Points(20f32),
                        ..Default::default()
                    },
                    max_size: stretch::geometry::Size {
                        width: stretch::style::Dimension::Points(20f32),
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
    assert_eq!(layout.size.width, 120f32);
    assert_eq!(layout.size.height, 20f32);
    assert_eq!(layout.location.x, 0f32);
    assert_eq!(layout.location.y, 0f32);
    assert_eq!(layout.children[0usize].size.width, 100f32);
    assert_eq!(layout.children[0usize].size.height, 20f32);
    assert_eq!(layout.children[0usize].location.x, 0f32);
    assert_eq!(layout.children[0usize].location.y, 0f32);
    assert_eq!(layout.children[1usize].size.width, 20f32);
    assert_eq!(layout.children[1usize].size.height, 20f32);
    assert_eq!(layout.children[1usize].location.x, 100f32);
    assert_eq!(layout.children[1usize].location.y, 0f32);
}
