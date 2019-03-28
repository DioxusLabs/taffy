#[test]
fn wrap_reverse_column() {
    let layout = stretch::compute(
        &stretch::style::Node {
            flex_direction: stretch::style::FlexDirection::Column,
            flex_wrap: stretch::style::FlexWrap::WrapReverse,
            size: stretch::geometry::Size {
                width: stretch::style::Dimension::Points(100.0000),
                height: stretch::style::Dimension::Points(100.0000),
                ..Default::default()
            },
            children: vec![
                stretch::style::Node {
                    size: stretch::geometry::Size {
                        width: stretch::style::Dimension::Points(30.0000),
                        height: stretch::style::Dimension::Points(31.0000),
                        ..Default::default()
                    },
                    ..Default::default()
                },
                stretch::style::Node {
                    size: stretch::geometry::Size {
                        width: stretch::style::Dimension::Points(30.0000),
                        height: stretch::style::Dimension::Points(32.0000),
                        ..Default::default()
                    },
                    ..Default::default()
                },
                stretch::style::Node {
                    size: stretch::geometry::Size {
                        width: stretch::style::Dimension::Points(30.0000),
                        height: stretch::style::Dimension::Points(33.0000),
                        ..Default::default()
                    },
                    ..Default::default()
                },
                stretch::style::Node {
                    size: stretch::geometry::Size {
                        width: stretch::style::Dimension::Points(30.0000),
                        height: stretch::style::Dimension::Points(34.0000),
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

    assert_eq!(layout.children[0].size.width, 30.0000);
    assert_eq!(layout.children[0].size.height, 31.0000);
    assert_eq!(layout.children[0].location.x, 70.0000);
    assert_eq!(layout.children[0].location.y, 0.0000);

    assert_eq!(layout.children[1].size.width, 30.0000);
    assert_eq!(layout.children[1].size.height, 32.0000);
    assert_eq!(layout.children[1].location.x, 70.0000);
    assert_eq!(layout.children[1].location.y, 31.0000);

    assert_eq!(layout.children[2].size.width, 30.0000);
    assert_eq!(layout.children[2].size.height, 33.0000);
    assert_eq!(layout.children[2].location.x, 70.0000);
    assert_eq!(layout.children[2].location.y, 63.0000);

    assert_eq!(layout.children[3].size.width, 30.0000);
    assert_eq!(layout.children[3].size.height, 34.0000);
    assert_eq!(layout.children[3].location.x, 20.0000);
    assert_eq!(layout.children[3].location.y, 0.0000);
}
