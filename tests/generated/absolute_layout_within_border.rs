#[test]
fn absolute_layout_within_border() {
    let layout = stretch::compute(
        &stretch::style::Node {
            size: stretch::geometry::Size {
                width: stretch::style::Dimension::Points(100.0000),
                height: stretch::style::Dimension::Points(100.0000),
                ..Default::default()
            },
            padding: stretch::geometry::Rect {
                start: stretch::style::Dimension::Points(10.0000),
                end: stretch::style::Dimension::Points(10.0000),
                top: stretch::style::Dimension::Points(10.0000),
                bottom: stretch::style::Dimension::Points(10.0000),
                ..Default::default()
            },
            border: stretch::geometry::Rect {
                start: stretch::style::Dimension::Points(10.0000),
                end: stretch::style::Dimension::Points(10.0000),
                top: stretch::style::Dimension::Points(10.0000),
                bottom: stretch::style::Dimension::Points(10.0000),
                ..Default::default()
            },
            children: vec![
                stretch::style::Node {
                    position_type: stretch::style::PositionType::Absolute,
                    size: stretch::geometry::Size {
                        width: stretch::style::Dimension::Points(50.0000),
                        height: stretch::style::Dimension::Points(50.0000),
                        ..Default::default()
                    },
                    position: stretch::geometry::Rect {
                        start: stretch::style::Dimension::Points(0.0000),
                        top: stretch::style::Dimension::Points(0.0000),
                        ..Default::default()
                    },
                    ..Default::default()
                },
                stretch::style::Node {
                    position_type: stretch::style::PositionType::Absolute,
                    size: stretch::geometry::Size {
                        width: stretch::style::Dimension::Points(50.0000),
                        height: stretch::style::Dimension::Points(50.0000),
                        ..Default::default()
                    },
                    position: stretch::geometry::Rect {
                        end: stretch::style::Dimension::Points(0.0000),
                        bottom: stretch::style::Dimension::Points(0.0000),
                        ..Default::default()
                    },
                    ..Default::default()
                },
                stretch::style::Node {
                    position_type: stretch::style::PositionType::Absolute,
                    size: stretch::geometry::Size {
                        width: stretch::style::Dimension::Points(50.0000),
                        height: stretch::style::Dimension::Points(50.0000),
                        ..Default::default()
                    },
                    margin: stretch::geometry::Rect {
                        start: stretch::style::Dimension::Points(10.0000),
                        end: stretch::style::Dimension::Points(10.0000),
                        top: stretch::style::Dimension::Points(10.0000),
                        bottom: stretch::style::Dimension::Points(10.0000),
                        ..Default::default()
                    },
                    position: stretch::geometry::Rect {
                        start: stretch::style::Dimension::Points(0.0000),
                        top: stretch::style::Dimension::Points(0.0000),
                        ..Default::default()
                    },
                    ..Default::default()
                },
                stretch::style::Node {
                    position_type: stretch::style::PositionType::Absolute,
                    size: stretch::geometry::Size {
                        width: stretch::style::Dimension::Points(50.0000),
                        height: stretch::style::Dimension::Points(50.0000),
                        ..Default::default()
                    },
                    margin: stretch::geometry::Rect {
                        start: stretch::style::Dimension::Points(10.0000),
                        end: stretch::style::Dimension::Points(10.0000),
                        top: stretch::style::Dimension::Points(10.0000),
                        bottom: stretch::style::Dimension::Points(10.0000),
                        ..Default::default()
                    },
                    position: stretch::geometry::Rect {
                        end: stretch::style::Dimension::Points(0.0000),
                        bottom: stretch::style::Dimension::Points(0.0000),
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
    assert_eq!(layout.size.height, 100.0000);
    assert_eq!(layout.location.x, 0.0000);
    assert_eq!(layout.location.y, 0.0000);

    assert_eq!(layout.children[0].size.width, 50.0000);
    assert_eq!(layout.children[0].size.height, 50.0000);
    assert_eq!(layout.children[0].location.x, 10.0000);
    assert_eq!(layout.children[0].location.y, 10.0000);

    assert_eq!(layout.children[1].size.width, 50.0000);
    assert_eq!(layout.children[1].size.height, 50.0000);
    assert_eq!(layout.children[1].location.x, 40.0000);
    assert_eq!(layout.children[1].location.y, 40.0000);

    assert_eq!(layout.children[2].size.width, 50.0000);
    assert_eq!(layout.children[2].size.height, 50.0000);
    assert_eq!(layout.children[2].location.x, 20.0000);
    assert_eq!(layout.children[2].location.y, 20.0000);

    assert_eq!(layout.children[3].size.width, 50.0000);
    assert_eq!(layout.children[3].size.height, 50.0000);
    assert_eq!(layout.children[3].location.x, 30.0000);
    assert_eq!(layout.children[3].location.y, 30.0000);
}
