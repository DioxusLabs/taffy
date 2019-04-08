#[test]
fn width_smaller_then_content_with_flex_grow_unconstraint_size() {
    let layout = stretch::node::Node::new(
        stretch::style::Style { ..Default::default() },
        vec![
            &stretch::node::Node::new(
                stretch::style::Style {
                    flex_direction: stretch::style::FlexDirection::Column,
                    flex_grow: 1f32,
                    size: stretch::geometry::Size {
                        width: stretch::style::Dimension::Points(0f32),
                        ..Default::default()
                    },
                    ..Default::default()
                },
                vec![&stretch::node::Node::new(
                    stretch::style::Style {
                        size: stretch::geometry::Size {
                            width: stretch::style::Dimension::Points(70f32),
                            height: stretch::style::Dimension::Points(100f32),
                            ..Default::default()
                        },
                        ..Default::default()
                    },
                    vec![],
                )],
            ),
            &stretch::node::Node::new(
                stretch::style::Style {
                    flex_direction: stretch::style::FlexDirection::Column,
                    flex_grow: 1f32,
                    size: stretch::geometry::Size {
                        width: stretch::style::Dimension::Points(0f32),
                        ..Default::default()
                    },
                    ..Default::default()
                },
                vec![&stretch::node::Node::new(
                    stretch::style::Style {
                        size: stretch::geometry::Size {
                            width: stretch::style::Dimension::Points(20f32),
                            height: stretch::style::Dimension::Points(100f32),
                            ..Default::default()
                        },
                        ..Default::default()
                    },
                    vec![],
                )],
            ),
        ],
    )
    .compute_layout(stretch::geometry::Size::undefined())
    .unwrap();
    assert_eq!(layout.size.width, 0f32);
    assert_eq!(layout.size.height, 100f32);
    assert_eq!(layout.location.x, 0f32);
    assert_eq!(layout.location.y, 0f32);
    assert_eq!(layout.children[0usize].size.width, 0f32);
    assert_eq!(layout.children[0usize].size.height, 100f32);
    assert_eq!(layout.children[0usize].location.x, 0f32);
    assert_eq!(layout.children[0usize].location.y, 0f32);
    assert_eq!(layout.children[0usize].children[0usize].size.width, 70f32);
    assert_eq!(layout.children[0usize].children[0usize].size.height, 100f32);
    assert_eq!(layout.children[0usize].children[0usize].location.x, 0f32);
    assert_eq!(layout.children[0usize].children[0usize].location.y, 0f32);
    assert_eq!(layout.children[1usize].size.width, 0f32);
    assert_eq!(layout.children[1usize].size.height, 100f32);
    assert_eq!(layout.children[1usize].location.x, 0f32);
    assert_eq!(layout.children[1usize].location.y, 0f32);
    assert_eq!(layout.children[1usize].children[0usize].size.width, 20f32);
    assert_eq!(layout.children[1usize].children[0usize].size.height, 100f32);
    assert_eq!(layout.children[1usize].children[0usize].location.x, 0f32);
    assert_eq!(layout.children[1usize].children[0usize].location.y, 0f32);
}
