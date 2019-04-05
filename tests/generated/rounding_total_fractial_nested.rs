#[test]
fn rounding_total_fractial_nested() {
    let layout = stretch::node::Node::new(
        stretch::style::Style {
            flex_direction: stretch::style::FlexDirection::Column,
            size: stretch::geometry::Size {
                width: stretch::style::Dimension::Points(87.4f32),
                height: stretch::style::Dimension::Points(113.4f32),
                ..Default::default()
            },
            ..Default::default()
        },
        vec![
            &stretch::node::Node::new(
                stretch::style::Style {
                    flex_direction: stretch::style::FlexDirection::Column,
                    flex_grow: 0.7f32,
                    flex_basis: stretch::style::Dimension::Points(50.3f32),
                    size: stretch::geometry::Size {
                        height: stretch::style::Dimension::Points(20.3f32),
                        ..Default::default()
                    },
                    ..Default::default()
                },
                vec![
                    &stretch::node::Node::new(
                        stretch::style::Style {
                            flex_grow: 1f32,
                            flex_basis: stretch::style::Dimension::Points(0.3f32),
                            size: stretch::geometry::Size {
                                height: stretch::style::Dimension::Points(9.9f32),
                                ..Default::default()
                            },
                            position: stretch::geometry::Rect {
                                bottom: stretch::style::Dimension::Points(13.3f32),
                                ..Default::default()
                            },
                            ..Default::default()
                        },
                        vec![],
                    ),
                    &stretch::node::Node::new(
                        stretch::style::Style {
                            flex_grow: 4f32,
                            flex_basis: stretch::style::Dimension::Points(0.3f32),
                            size: stretch::geometry::Size {
                                height: stretch::style::Dimension::Points(1.1f32),
                                ..Default::default()
                            },
                            position: stretch::geometry::Rect {
                                top: stretch::style::Dimension::Points(13.3f32),
                                ..Default::default()
                            },
                            ..Default::default()
                        },
                        vec![],
                    ),
                ],
            ),
            &stretch::node::Node::new(
                stretch::style::Style {
                    flex_grow: 1.6f32,
                    size: stretch::geometry::Size {
                        height: stretch::style::Dimension::Points(10f32),
                        ..Default::default()
                    },
                    ..Default::default()
                },
                vec![],
            ),
            &stretch::node::Node::new(
                stretch::style::Style {
                    flex_grow: 1.1f32,
                    size: stretch::geometry::Size {
                        height: stretch::style::Dimension::Points(10.7f32),
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
    assert_eq!(layout.size.width, 87f32);
    assert_eq!(layout.size.height, 113f32);
    assert_eq!(layout.location.x, 0f32);
    assert_eq!(layout.location.y, 0f32);
    assert_eq!(layout.children[0usize].size.width, 87f32);
    assert_eq!(layout.children[0usize].size.height, 59f32);
    assert_eq!(layout.children[0usize].location.x, 0f32);
    assert_eq!(layout.children[0usize].location.y, 0f32);
    assert_eq!(layout.children[0usize].children[0usize].size.width, 87f32);
    assert_eq!(layout.children[0usize].children[0usize].size.height, 12f32);
    assert_eq!(layout.children[0usize].children[0usize].location.x, 0f32);
    assert_eq!(layout.children[0usize].children[0usize].location.y, -13f32);
    assert_eq!(layout.children[0usize].children[1usize].size.width, 87f32);
    assert_eq!(layout.children[0usize].children[1usize].size.height, 47f32);
    assert_eq!(layout.children[0usize].children[1usize].location.x, 0f32);
    assert_eq!(layout.children[0usize].children[1usize].location.y, 25f32);
    assert_eq!(layout.children[1usize].size.width, 87f32);
    assert_eq!(layout.children[1usize].size.height, 30f32);
    assert_eq!(layout.children[1usize].location.x, 0f32);
    assert_eq!(layout.children[1usize].location.y, 59f32);
    assert_eq!(layout.children[2usize].size.width, 87f32);
    assert_eq!(layout.children[2usize].size.height, 24f32);
    assert_eq!(layout.children[2usize].location.x, 0f32);
    assert_eq!(layout.children[2usize].location.y, 89f32);
}
