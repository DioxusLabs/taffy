#[test]
fn wrap_column() {
    let layout = stretch::node::Node::new(
        stretch::style::Style {
            flex_direction: stretch::style::FlexDirection::Column,
            flex_wrap: stretch::style::FlexWrap::Wrap,
            size: stretch::geometry::Size {
                width: stretch::style::Dimension::Points(100f32),
                height: stretch::style::Dimension::Points(100f32),
                ..Default::default()
            },
            ..Default::default()
        },
        vec![
            &stretch::node::Node::new(
                stretch::style::Style {
                    size: stretch::geometry::Size {
                        width: stretch::style::Dimension::Points(30f32),
                        height: stretch::style::Dimension::Points(31f32),
                        ..Default::default()
                    },
                    ..Default::default()
                },
                vec![],
            ),
            &stretch::node::Node::new(
                stretch::style::Style {
                    size: stretch::geometry::Size {
                        width: stretch::style::Dimension::Points(30f32),
                        height: stretch::style::Dimension::Points(32f32),
                        ..Default::default()
                    },
                    ..Default::default()
                },
                vec![],
            ),
            &stretch::node::Node::new(
                stretch::style::Style {
                    size: stretch::geometry::Size {
                        width: stretch::style::Dimension::Points(30f32),
                        height: stretch::style::Dimension::Points(33f32),
                        ..Default::default()
                    },
                    ..Default::default()
                },
                vec![],
            ),
            &stretch::node::Node::new(
                stretch::style::Style {
                    size: stretch::geometry::Size {
                        width: stretch::style::Dimension::Points(30f32),
                        height: stretch::style::Dimension::Points(34f32),
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
    assert_eq!(layout.size.width, 100f32);
    assert_eq!(layout.size.height, 100f32);
    assert_eq!(layout.location.x, 0f32);
    assert_eq!(layout.location.y, 0f32);
    assert_eq!(layout.children[0usize].size.width, 30f32);
    assert_eq!(layout.children[0usize].size.height, 31f32);
    assert_eq!(layout.children[0usize].location.x, 0f32);
    assert_eq!(layout.children[0usize].location.y, 0f32);
    assert_eq!(layout.children[1usize].size.width, 30f32);
    assert_eq!(layout.children[1usize].size.height, 32f32);
    assert_eq!(layout.children[1usize].location.x, 0f32);
    assert_eq!(layout.children[1usize].location.y, 31f32);
    assert_eq!(layout.children[2usize].size.width, 30f32);
    assert_eq!(layout.children[2usize].size.height, 33f32);
    assert_eq!(layout.children[2usize].location.x, 0f32);
    assert_eq!(layout.children[2usize].location.y, 63f32);
    assert_eq!(layout.children[3usize].size.width, 30f32);
    assert_eq!(layout.children[3usize].size.height, 34f32);
    assert_eq!(layout.children[3usize].location.x, 50f32);
    assert_eq!(layout.children[3usize].location.y, 0f32);
}
