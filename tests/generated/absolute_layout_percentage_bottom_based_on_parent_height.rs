#[test]
fn absolute_layout_percentage_bottom_based_on_parent_height() {
    let layout = stretch::node::Node::new(
        stretch::style::Style {
            size: stretch::geometry::Size {
                width: stretch::style::Dimension::Points(100f32),
                height: stretch::style::Dimension::Points(200f32),
                ..Default::default()
            },
            ..Default::default()
        },
        vec![
            &stretch::node::Node::new(
                stretch::style::Style {
                    position_type: stretch::style::PositionType::Absolute,
                    size: stretch::geometry::Size {
                        width: stretch::style::Dimension::Points(10f32),
                        height: stretch::style::Dimension::Points(10f32),
                        ..Default::default()
                    },
                    position: stretch::geometry::Rect {
                        top: stretch::style::Dimension::Percent(0.5f32),
                        ..Default::default()
                    },
                    ..Default::default()
                },
                vec![],
            ),
            &stretch::node::Node::new(
                stretch::style::Style {
                    position_type: stretch::style::PositionType::Absolute,
                    size: stretch::geometry::Size {
                        width: stretch::style::Dimension::Points(10f32),
                        height: stretch::style::Dimension::Points(10f32),
                        ..Default::default()
                    },
                    position: stretch::geometry::Rect {
                        bottom: stretch::style::Dimension::Percent(0.5f32),
                        ..Default::default()
                    },
                    ..Default::default()
                },
                vec![],
            ),
            &stretch::node::Node::new(
                stretch::style::Style {
                    position_type: stretch::style::PositionType::Absolute,
                    size: stretch::geometry::Size {
                        width: stretch::style::Dimension::Points(10f32),
                        ..Default::default()
                    },
                    position: stretch::geometry::Rect {
                        top: stretch::style::Dimension::Percent(0.1f32),
                        bottom: stretch::style::Dimension::Percent(0.1f32),
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
    assert_eq!(layout.size.height, 200f32);
    assert_eq!(layout.location.x, 0f32);
    assert_eq!(layout.location.y, 0f32);
    assert_eq!(layout.children[0usize].size.width, 10f32);
    assert_eq!(layout.children[0usize].size.height, 10f32);
    assert_eq!(layout.children[0usize].location.x, 0f32);
    assert_eq!(layout.children[0usize].location.y, 100f32);
    assert_eq!(layout.children[1usize].size.width, 10f32);
    assert_eq!(layout.children[1usize].size.height, 10f32);
    assert_eq!(layout.children[1usize].location.x, 0f32);
    assert_eq!(layout.children[1usize].location.y, 90f32);
    assert_eq!(layout.children[2usize].size.width, 10f32);
    assert_eq!(layout.children[2usize].size.height, 160f32);
    assert_eq!(layout.children[2usize].location.x, 0f32);
    assert_eq!(layout.children[2usize].location.y, 20f32);
}
