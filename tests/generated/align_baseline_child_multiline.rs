#[test]
fn align_baseline_child_multiline() {
    let layout = stretch::node::Node::new(
        stretch::style::Style {
            align_items: stretch::style::AlignItems::Baseline,
            size: stretch::geometry::Size { width: stretch::style::Dimension::Points(100f32), ..Default::default() },
            ..Default::default()
        },
        vec![
            &stretch::node::Node::new(
                stretch::style::Style {
                    size: stretch::geometry::Size {
                        width: stretch::style::Dimension::Points(50f32),
                        height: stretch::style::Dimension::Points(60f32),
                        ..Default::default()
                    },
                    ..Default::default()
                },
                vec![],
            ),
            &stretch::node::Node::new(
                stretch::style::Style {
                    flex_wrap: stretch::style::FlexWrap::Wrap,
                    size: stretch::geometry::Size {
                        width: stretch::style::Dimension::Points(50f32),
                        ..Default::default()
                    },
                    ..Default::default()
                },
                vec![
                    &stretch::node::Node::new(
                        stretch::style::Style {
                            size: stretch::geometry::Size {
                                width: stretch::style::Dimension::Points(25f32),
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
                                width: stretch::style::Dimension::Points(25f32),
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
                                width: stretch::style::Dimension::Points(25f32),
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
                                width: stretch::style::Dimension::Points(25f32),
                                height: stretch::style::Dimension::Points(10f32),
                                ..Default::default()
                            },
                            ..Default::default()
                        },
                        vec![],
                    ),
                ],
            ),
        ],
    )
    .compute_layout(stretch::geometry::Size::undefined())
    .unwrap();
    assert_eq!(layout.size.width, 100f32);
    assert_eq!(layout.size.height, 80f32);
    assert_eq!(layout.location.x, 0f32);
    assert_eq!(layout.location.y, 0f32);
    assert_eq!(layout.children[0usize].size.width, 50f32);
    assert_eq!(layout.children[0usize].size.height, 60f32);
    assert_eq!(layout.children[0usize].location.x, 0f32);
    assert_eq!(layout.children[0usize].location.y, 0f32);
    assert_eq!(layout.children[1usize].size.width, 50f32);
    assert_eq!(layout.children[1usize].size.height, 40f32);
    assert_eq!(layout.children[1usize].location.x, 50f32);
    assert_eq!(layout.children[1usize].location.y, 40f32);
    assert_eq!(layout.children[1usize].children[0usize].size.width, 25f32);
    assert_eq!(layout.children[1usize].children[0usize].size.height, 20f32);
    assert_eq!(layout.children[1usize].children[0usize].location.x, 0f32);
    assert_eq!(layout.children[1usize].children[0usize].location.y, 0f32);
    assert_eq!(layout.children[1usize].children[1usize].size.width, 25f32);
    assert_eq!(layout.children[1usize].children[1usize].size.height, 10f32);
    assert_eq!(layout.children[1usize].children[1usize].location.x, 25f32);
    assert_eq!(layout.children[1usize].children[1usize].location.y, 0f32);
    assert_eq!(layout.children[1usize].children[2usize].size.width, 25f32);
    assert_eq!(layout.children[1usize].children[2usize].size.height, 20f32);
    assert_eq!(layout.children[1usize].children[2usize].location.x, 0f32);
    assert_eq!(layout.children[1usize].children[2usize].location.y, 20f32);
    assert_eq!(layout.children[1usize].children[3usize].size.width, 25f32);
    assert_eq!(layout.children[1usize].children[3usize].size.height, 10f32);
    assert_eq!(layout.children[1usize].children[3usize].location.x, 25f32);
    assert_eq!(layout.children[1usize].children[3usize].location.y, 20f32);
}
