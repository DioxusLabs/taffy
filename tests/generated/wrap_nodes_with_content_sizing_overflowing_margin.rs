#[test]
fn wrap_nodes_with_content_sizing_overflowing_margin() {
    let layout = stretch::compute(
        &stretch::style::Node {
            flex_direction: stretch::style::FlexDirection::Column,
            size: stretch::geometry::Size {
                width: stretch::style::Dimension::Points(500.0000),
                height: stretch::style::Dimension::Points(500.0000),
                ..Default::default()
            },
            children: vec![stretch::style::Node {
                flex_wrap: stretch::style::FlexWrap::Wrap,
                size: stretch::geometry::Size {
                    width: stretch::style::Dimension::Points(85.0000),
                    ..Default::default()
                },
                children: vec![
                    stretch::style::Node {
                        flex_direction: stretch::style::FlexDirection::Column,
                        children: vec![stretch::style::Node {
                            size: stretch::geometry::Size {
                                width: stretch::style::Dimension::Points(40.0000),
                                height: stretch::style::Dimension::Points(40.0000),
                                ..Default::default()
                            },
                            ..Default::default()
                        }],
                        ..Default::default()
                    },
                    stretch::style::Node {
                        flex_direction: stretch::style::FlexDirection::Column,
                        margin: stretch::geometry::Rect {
                            end: stretch::style::Dimension::Points(10.0000),
                            ..Default::default()
                        },
                        children: vec![stretch::style::Node {
                            size: stretch::geometry::Size {
                                width: stretch::style::Dimension::Points(40.0000),
                                height: stretch::style::Dimension::Points(40.0000),
                                ..Default::default()
                            },
                            ..Default::default()
                        }],
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

    assert_eq!(layout.size.width, 500.0000);
    assert_eq!(layout.size.height, 500.0000);
    assert_eq!(layout.location.x, 0.0000);
    assert_eq!(layout.location.y, 0.0000);

    assert_eq!(layout.children[0].size.width, 85.0000);
    assert_eq!(layout.children[0].size.height, 80.0000);
    assert_eq!(layout.children[0].location.x, 0.0000);
    assert_eq!(layout.children[0].location.y, 0.0000);

    assert_eq!(layout.children[0].children[0].size.width, 40.0000);
    assert_eq!(layout.children[0].children[0].size.height, 40.0000);
    assert_eq!(layout.children[0].children[0].location.x, 0.0000);
    assert_eq!(layout.children[0].children[0].location.y, 0.0000);

    assert_eq!(layout.children[0].children[0].children[0].size.width, 40.0000);
    assert_eq!(layout.children[0].children[0].children[0].size.height, 40.0000);
    assert_eq!(layout.children[0].children[0].children[0].location.x, 0.0000);
    assert_eq!(layout.children[0].children[0].children[0].location.y, 0.0000);

    assert_eq!(layout.children[0].children[1].size.width, 40.0000);
    assert_eq!(layout.children[0].children[1].size.height, 40.0000);
    assert_eq!(layout.children[0].children[1].location.x, 0.0000);
    assert_eq!(layout.children[0].children[1].location.y, 40.0000);

    assert_eq!(layout.children[0].children[1].children[0].size.width, 40.0000);
    assert_eq!(layout.children[0].children[1].children[0].size.height, 40.0000);
    assert_eq!(layout.children[0].children[1].children[0].location.x, 0.0000);
    assert_eq!(layout.children[0].children[1].children[0].location.y, 0.0000);
}
