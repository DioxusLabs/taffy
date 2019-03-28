#[test]
fn percent_absolute_position() {
    let layout = stretch::compute(
        &stretch::style::Node {
            flex_direction: stretch::style::FlexDirection::Column,
            size: stretch::geometry::Size {
                width: stretch::style::Dimension::Points(60.0000),
                height: stretch::style::Dimension::Points(50.0000),
                ..Default::default()
            },
            children: vec![stretch::style::Node {
                position_type: stretch::style::PositionType::Absolute,
                size: stretch::geometry::Size {
                    width: stretch::style::Dimension::Percent(1.0000),
                    height: stretch::style::Dimension::Points(50.0000),
                    ..Default::default()
                },
                position: stretch::geometry::Rect {
                    start: stretch::style::Dimension::Percent(0.5000),
                    ..Default::default()
                },
                children: vec![
                    stretch::style::Node {
                        size: stretch::geometry::Size {
                            width: stretch::style::Dimension::Percent(1.0000),
                            ..Default::default()
                        },
                        ..Default::default()
                    },
                    stretch::style::Node {
                        size: stretch::geometry::Size {
                            width: stretch::style::Dimension::Percent(1.0000),
                            ..Default::default()
                        },
                        ..Default::default()
                    },
                ],
                ..Default::default()
            }],
            ..Default::default()
        },
        stretch::geometry::Size::undefined(),
    )
    .unwrap();

    assert_eq!(layout.size.width, 60.0000);
    assert_eq!(layout.size.height, 50.0000);
    assert_eq!(layout.location.x, 0.0000);
    assert_eq!(layout.location.y, 0.0000);

    assert_eq!(layout.children[0].size.width, 60.0000);
    assert_eq!(layout.children[0].size.height, 50.0000);
    assert_eq!(layout.children[0].location.x, 30.0000);
    assert_eq!(layout.children[0].location.y, 0.0000);

    assert_eq!(layout.children[0].children[0].size.width, 30.0000);
    assert_eq!(layout.children[0].children[0].size.height, 50.0000);
    assert_eq!(layout.children[0].children[0].location.x, 0.0000);
    assert_eq!(layout.children[0].children[0].location.y, 0.0000);

    assert_eq!(layout.children[0].children[1].size.width, 30.0000);
    assert_eq!(layout.children[0].children[1].size.height, 50.0000);
    assert_eq!(layout.children[0].children[1].location.x, 30.0000);
    assert_eq!(layout.children[0].children[1].location.y, 0.0000);
}
