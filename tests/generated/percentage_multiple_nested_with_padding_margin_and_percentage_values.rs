#[test]
fn percentage_multiple_nested_with_padding_margin_and_percentage_values() {
    let layout = stretch::node::Node::new(
        stretch::style::Style {
            flex_direction: stretch::style::FlexDirection::Column,
            size: stretch::geometry::Size {
                width: stretch::style::Dimension::Points(200f32),
                height: stretch::style::Dimension::Points(200f32),
                ..Default::default()
            },
            ..Default::default()
        },
        vec![
            &stretch::node::Node::new(
                stretch::style::Style {
                    flex_direction: stretch::style::FlexDirection::Column,
                    flex_grow: 1f32,
                    flex_basis: stretch::style::Dimension::Percent(0.1f32),
                    min_size: stretch::geometry::Size {
                        width: stretch::style::Dimension::Percent(0.6f32),
                        ..Default::default()
                    },
                    margin: stretch::geometry::Rect {
                        start: stretch::style::Dimension::Points(5f32),
                        end: stretch::style::Dimension::Points(5f32),
                        top: stretch::style::Dimension::Points(5f32),
                        bottom: stretch::style::Dimension::Points(5f32),
                        ..Default::default()
                    },
                    padding: stretch::geometry::Rect {
                        start: stretch::style::Dimension::Points(3f32),
                        end: stretch::style::Dimension::Points(3f32),
                        top: stretch::style::Dimension::Points(3f32),
                        bottom: stretch::style::Dimension::Points(3f32),
                        ..Default::default()
                    },
                    ..Default::default()
                },
                vec![&stretch::node::Node::new(
                    stretch::style::Style {
                        flex_direction: stretch::style::FlexDirection::Column,
                        size: stretch::geometry::Size {
                            width: stretch::style::Dimension::Percent(0.5f32),
                            ..Default::default()
                        },
                        margin: stretch::geometry::Rect {
                            start: stretch::style::Dimension::Points(5f32),
                            end: stretch::style::Dimension::Points(5f32),
                            top: stretch::style::Dimension::Points(5f32),
                            bottom: stretch::style::Dimension::Points(5f32),
                            ..Default::default()
                        },
                        padding: stretch::geometry::Rect {
                            start: stretch::style::Dimension::Percent(0.03f32),
                            end: stretch::style::Dimension::Percent(0.03f32),
                            top: stretch::style::Dimension::Percent(0.03f32),
                            bottom: stretch::style::Dimension::Percent(0.03f32),
                            ..Default::default()
                        },
                        ..Default::default()
                    },
                    vec![&stretch::node::Node::new(
                        stretch::style::Style {
                            size: stretch::geometry::Size {
                                width: stretch::style::Dimension::Percent(0.45f32),
                                ..Default::default()
                            },
                            margin: stretch::geometry::Rect {
                                start: stretch::style::Dimension::Percent(0.05f32),
                                end: stretch::style::Dimension::Percent(0.05f32),
                                top: stretch::style::Dimension::Percent(0.05f32),
                                bottom: stretch::style::Dimension::Percent(0.05f32),
                                ..Default::default()
                            },
                            padding: stretch::geometry::Rect {
                                start: stretch::style::Dimension::Points(3f32),
                                end: stretch::style::Dimension::Points(3f32),
                                top: stretch::style::Dimension::Points(3f32),
                                bottom: stretch::style::Dimension::Points(3f32),
                                ..Default::default()
                            },
                            ..Default::default()
                        },
                        vec![],
                    )],
                )],
            ),
            &stretch::node::Node::new(
                stretch::style::Style {
                    flex_grow: 4f32,
                    flex_basis: stretch::style::Dimension::Percent(0.15f32),
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
    assert_eq!(layout.size.height, 200f32);
    assert_eq!(layout.location.x, 0f32);
    assert_eq!(layout.location.y, 0f32);
    assert_eq!(layout.children[0usize].size.width, 190f32);
    assert_eq!(layout.children[0usize].size.height, 48f32);
    assert_eq!(layout.children[0usize].location.x, 5f32);
    assert_eq!(layout.children[0usize].location.y, 5f32);
    assert_eq!(layout.children[0usize].children[0usize].size.width, 92f32);
    assert_eq!(layout.children[0usize].children[0usize].size.height, 25f32);
    assert_eq!(layout.children[0usize].children[0usize].location.x, 8f32);
    assert_eq!(layout.children[0usize].children[0usize].location.y, 8f32);
    assert_eq!(layout.children[0usize].children[0usize].children[0usize].size.width, 36f32);
    assert_eq!(layout.children[0usize].children[0usize].children[0usize].size.height, 6f32);
    assert_eq!(layout.children[0usize].children[0usize].children[0usize].location.x, 10f32);
    assert_eq!(layout.children[0usize].children[0usize].children[0usize].location.y, 10f32);
    assert_eq!(layout.children[1usize].size.width, 200f32);
    assert_eq!(layout.children[1usize].size.height, 142f32);
    assert_eq!(layout.children[1usize].location.x, 0f32);
    assert_eq!(layout.children[1usize].location.y, 58f32);
}
