#[test]
fn absolute_layout_percentage_bottom_based_on_parent_height() {
    let layout = stretch::compute(
        &stretch::style::Node {
            size: stretch::geometry::Size {
                width: stretch::style::Dimension::Points(100.0000),
                height: stretch::style::Dimension::Points(200.0000),
                ..Default::default()
            },
            children: vec![
                stretch::style::Node {
                    position_type: stretch::style::PositionType::Absolute,
                    size: stretch::geometry::Size {
                        width: stretch::style::Dimension::Points(10.0000),
                        height: stretch::style::Dimension::Points(10.0000),
                        ..Default::default()
                    },
                    position: stretch::geometry::Rect {
                        top: stretch::style::Dimension::Percent(0.5000),
                        ..Default::default()
                    },
                    ..Default::default()
                },
                stretch::style::Node {
                    position_type: stretch::style::PositionType::Absolute,
                    size: stretch::geometry::Size {
                        width: stretch::style::Dimension::Points(10.0000),
                        height: stretch::style::Dimension::Points(10.0000),
                        ..Default::default()
                    },
                    position: stretch::geometry::Rect {
                        bottom: stretch::style::Dimension::Percent(0.5000),
                        ..Default::default()
                    },
                    ..Default::default()
                },
                stretch::style::Node {
                    position_type: stretch::style::PositionType::Absolute,
                    size: stretch::geometry::Size {
                        width: stretch::style::Dimension::Points(10.0000),
                        ..Default::default()
                    },
                    position: stretch::geometry::Rect {
                        top: stretch::style::Dimension::Percent(0.1000),
                        bottom: stretch::style::Dimension::Percent(0.1000),
                        ..Default::default()
                    },
                    ..Default::default()
                },
            ],
            ..Default::default()
        },
        stretch::geometry::Size::undefined(),
    )
    .unwrap();

    assert_eq!(layout.size.width, 100.0000);
    assert_eq!(layout.size.height, 200.0000);
    assert_eq!(layout.location.x, 0.0000);
    assert_eq!(layout.location.y, 0.0000);

    assert_eq!(layout.children[0].size.width, 10.0000);
    assert_eq!(layout.children[0].size.height, 10.0000);
    assert_eq!(layout.children[0].location.x, 0.0000);
    assert_eq!(layout.children[0].location.y, 100.0000);

    assert_eq!(layout.children[1].size.width, 10.0000);
    assert_eq!(layout.children[1].size.height, 10.0000);
    assert_eq!(layout.children[1].location.x, 0.0000);
    assert_eq!(layout.children[1].location.y, 90.0000);

    assert_eq!(layout.children[2].size.width, 10.0000);
    assert_eq!(layout.children[2].size.height, 160.0000);
    assert_eq!(layout.children[2].location.x, 0.0000);
    assert_eq!(layout.children[2].location.y, 20.0000);
}
