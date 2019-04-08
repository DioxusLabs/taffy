#[test]
fn percentage_container_in_wrapping_container() {
    let layout = stretch::node::Node::new(
        stretch::style::Style {
            flex_direction: stretch::style::FlexDirection::Column,
            align_items: stretch::style::AlignItems::Center,
            justify_content: stretch::style::JustifyContent::Center,
            size: stretch::geometry::Size {
                width: stretch::style::Dimension::Points(200f32),
                height: stretch::style::Dimension::Points(200f32),
                ..Default::default()
            },
            ..Default::default()
        },
        vec![&stretch::node::Node::new(
            stretch::style::Style { flex_direction: stretch::style::FlexDirection::Column, ..Default::default() },
            vec![&stretch::node::Node::new(
                stretch::style::Style {
                    justify_content: stretch::style::JustifyContent::Center,
                    size: stretch::geometry::Size {
                        width: stretch::style::Dimension::Percent(1f32),
                        ..Default::default()
                    },
                    ..Default::default()
                },
                vec![
                    &stretch::node::Node::new(
                        stretch::style::Style {
                            size: stretch::geometry::Size {
                                width: stretch::style::Dimension::Points(50f32),
                                height: stretch::style::Dimension::Points(50f32),
                                ..Default::default()
                            },
                            ..Default::default()
                        },
                        vec![],
                    ),
                    &stretch::node::Node::new(
                        stretch::style::Style {
                            size: stretch::geometry::Size {
                                width: stretch::style::Dimension::Points(50f32),
                                height: stretch::style::Dimension::Points(50f32),
                                ..Default::default()
                            },
                            ..Default::default()
                        },
                        vec![],
                    ),
                ],
            )],
        )],
    )
    .compute_layout(stretch::geometry::Size::undefined())
    .unwrap();
    assert_eq!(layout.size.width, 200f32);
    assert_eq!(layout.size.height, 200f32);
    assert_eq!(layout.location.x, 0f32);
    assert_eq!(layout.location.y, 0f32);
    assert_eq!(layout.children[0usize].size.width, 100f32);
    assert_eq!(layout.children[0usize].size.height, 50f32);
    assert_eq!(layout.children[0usize].location.x, 50f32);
    assert_eq!(layout.children[0usize].location.y, 75f32);
    assert_eq!(layout.children[0usize].children[0usize].size.width, 100f32);
    assert_eq!(layout.children[0usize].children[0usize].size.height, 50f32);
    assert_eq!(layout.children[0usize].children[0usize].location.x, 0f32);
    assert_eq!(layout.children[0usize].children[0usize].location.y, 0f32);
    assert_eq!(layout.children[0usize].children[0usize].children[0usize].size.width, 50f32);
    assert_eq!(layout.children[0usize].children[0usize].children[0usize].size.height, 50f32);
    assert_eq!(layout.children[0usize].children[0usize].children[0usize].location.x, 0f32);
    assert_eq!(layout.children[0usize].children[0usize].children[0usize].location.y, 0f32);
    assert_eq!(layout.children[0usize].children[0usize].children[1usize].size.width, 50f32);
    assert_eq!(layout.children[0usize].children[0usize].children[1usize].size.height, 50f32);
    assert_eq!(layout.children[0usize].children[0usize].children[1usize].location.x, 50f32);
    assert_eq!(layout.children[0usize].children[0usize].children[1usize].location.y, 0f32);
}
