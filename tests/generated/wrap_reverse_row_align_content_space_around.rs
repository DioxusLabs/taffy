#[test]
fn wrap_reverse_row_align_content_space_around() {
    let layout = stretch::node::Node::new(
        stretch::style::Style {
            flex_wrap: stretch::style::FlexWrap::WrapReverse,
            align_content: stretch::style::AlignContent::SpaceAround,
            size: stretch::geometry::Size { width: stretch::style::Dimension::Points(100f32), ..Default::default() },
            ..Default::default()
        },
        vec![
            &stretch::node::Node::new(
                stretch::style::Style {
                    size: stretch::geometry::Size {
                        width: stretch::style::Dimension::Points(30f32),
                        height: stretch::style::Dimension::Points(10f32),
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
                        height: stretch::style::Dimension::Points(20f32),
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
                        height: stretch::style::Dimension::Points(30f32),
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
                        height: stretch::style::Dimension::Points(40f32),
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
                        height: stretch::style::Dimension::Points(50f32),
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
    assert_eq!(layout.size.height, 80f32);
    assert_eq!(layout.location.x, 0f32);
    assert_eq!(layout.location.y, 0f32);
    assert_eq!(layout.children[0usize].size.width, 30f32);
    assert_eq!(layout.children[0usize].size.height, 10f32);
    assert_eq!(layout.children[0usize].location.x, 0f32);
    assert_eq!(layout.children[0usize].location.y, 70f32);
    assert_eq!(layout.children[1usize].size.width, 30f32);
    assert_eq!(layout.children[1usize].size.height, 20f32);
    assert_eq!(layout.children[1usize].location.x, 30f32);
    assert_eq!(layout.children[1usize].location.y, 60f32);
    assert_eq!(layout.children[2usize].size.width, 30f32);
    assert_eq!(layout.children[2usize].size.height, 30f32);
    assert_eq!(layout.children[2usize].location.x, 60f32);
    assert_eq!(layout.children[2usize].location.y, 50f32);
    assert_eq!(layout.children[3usize].size.width, 30f32);
    assert_eq!(layout.children[3usize].size.height, 40f32);
    assert_eq!(layout.children[3usize].location.x, 0f32);
    assert_eq!(layout.children[3usize].location.y, 10f32);
    assert_eq!(layout.children[4usize].size.width, 30f32);
    assert_eq!(layout.children[4usize].size.height, 50f32);
    assert_eq!(layout.children[4usize].location.x, 30f32);
    assert_eq!(layout.children[4usize].location.y, 0f32);
}
