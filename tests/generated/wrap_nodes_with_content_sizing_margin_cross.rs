#[test]
fn wrap_nodes_with_content_sizing_margin_cross() {
    let layout = stretch::node::Node::new(
        stretch::style::Style {
            flex_direction: stretch::style::FlexDirection::Column,
            size: stretch::geometry::Size {
                width: stretch::style::Dimension::Points(500f32),
                height: stretch::style::Dimension::Points(500f32),
                ..Default::default()
            },
            ..Default::default()
        },
        vec![&stretch::node::Node::new(
            stretch::style::Style {
                flex_wrap: stretch::style::FlexWrap::Wrap,
                size: stretch::geometry::Size { width: stretch::style::Dimension::Points(70f32), ..Default::default() },
                ..Default::default()
            },
            vec![
                &stretch::node::Node::new(
                    stretch::style::Style {
                        flex_direction: stretch::style::FlexDirection::Column,
                        ..Default::default()
                    },
                    vec![&stretch::node::Node::new(
                        stretch::style::Style {
                            size: stretch::geometry::Size {
                                width: stretch::style::Dimension::Points(40f32),
                                height: stretch::style::Dimension::Points(40f32),
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
                        margin: stretch::geometry::Rect {
                            top: stretch::style::Dimension::Points(10f32),
                            ..Default::default()
                        },
                        ..Default::default()
                    },
                    vec![&stretch::node::Node::new(
                        stretch::style::Style {
                            size: stretch::geometry::Size {
                                width: stretch::style::Dimension::Points(40f32),
                                height: stretch::style::Dimension::Points(40f32),
                                ..Default::default()
                            },
                            ..Default::default()
                        },
                        vec![],
                    )],
                ),
            ],
        )],
    )
    .compute_layout(stretch::geometry::Size::undefined())
    .unwrap();
    assert_eq!(layout.size.width, 500f32);
    assert_eq!(layout.size.height, 500f32);
    assert_eq!(layout.location.x, 0f32);
    assert_eq!(layout.location.y, 0f32);
    assert_eq!(layout.children[0usize].size.width, 70f32);
    assert_eq!(layout.children[0usize].size.height, 90f32);
    assert_eq!(layout.children[0usize].location.x, 0f32);
    assert_eq!(layout.children[0usize].location.y, 0f32);
    assert_eq!(layout.children[0usize].children[0usize].size.width, 40f32);
    assert_eq!(layout.children[0usize].children[0usize].size.height, 40f32);
    assert_eq!(layout.children[0usize].children[0usize].location.x, 0f32);
    assert_eq!(layout.children[0usize].children[0usize].location.y, 0f32);
    assert_eq!(layout.children[0usize].children[0usize].children[0usize].size.width, 40f32);
    assert_eq!(layout.children[0usize].children[0usize].children[0usize].size.height, 40f32);
    assert_eq!(layout.children[0usize].children[0usize].children[0usize].location.x, 0f32);
    assert_eq!(layout.children[0usize].children[0usize].children[0usize].location.y, 0f32);
    assert_eq!(layout.children[0usize].children[1usize].size.width, 40f32);
    assert_eq!(layout.children[0usize].children[1usize].size.height, 40f32);
    assert_eq!(layout.children[0usize].children[1usize].location.x, 0f32);
    assert_eq!(layout.children[0usize].children[1usize].location.y, 50f32);
    assert_eq!(layout.children[0usize].children[1usize].children[0usize].size.width, 40f32);
    assert_eq!(layout.children[0usize].children[1usize].children[0usize].size.height, 40f32);
    assert_eq!(layout.children[0usize].children[1usize].children[0usize].location.x, 0f32);
    assert_eq!(layout.children[0usize].children[1usize].children[0usize].location.y, 0f32);
}
