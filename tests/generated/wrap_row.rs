#[test]
fn wrap_row() {
    let layout = stretch::compute(
        &stretch::style::Node {
            flex_wrap: stretch::style::FlexWrap::Wrap,
            size: stretch::geometry::Size { width: stretch::style::Dimension::Points(100.0000), ..Default::default() },
            children: vec![
                stretch::style::Node {
                    size: stretch::geometry::Size {
                        width: stretch::style::Dimension::Points(31.0000),
                        height: stretch::style::Dimension::Points(30.0000),
                        ..Default::default()
                    },
                    ..Default::default()
                },
                stretch::style::Node {
                    size: stretch::geometry::Size {
                        width: stretch::style::Dimension::Points(32.0000),
                        height: stretch::style::Dimension::Points(30.0000),
                        ..Default::default()
                    },
                    ..Default::default()
                },
                stretch::style::Node {
                    size: stretch::geometry::Size {
                        width: stretch::style::Dimension::Points(33.0000),
                        height: stretch::style::Dimension::Points(30.0000),
                        ..Default::default()
                    },
                    ..Default::default()
                },
                stretch::style::Node {
                    size: stretch::geometry::Size {
                        width: stretch::style::Dimension::Points(34.0000),
                        height: stretch::style::Dimension::Points(30.0000),
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
    assert_eq!(layout.size.height, 60.0000);
    assert_eq!(layout.location.x, 0.0000);
    assert_eq!(layout.location.y, 0.0000);

    assert_eq!(layout.children[0].size.width, 31.0000);
    assert_eq!(layout.children[0].size.height, 30.0000);
    assert_eq!(layout.children[0].location.x, 0.0000);
    assert_eq!(layout.children[0].location.y, 0.0000);

    assert_eq!(layout.children[1].size.width, 32.0000);
    assert_eq!(layout.children[1].size.height, 30.0000);
    assert_eq!(layout.children[1].location.x, 31.0000);
    assert_eq!(layout.children[1].location.y, 0.0000);

    assert_eq!(layout.children[2].size.width, 33.0000);
    assert_eq!(layout.children[2].size.height, 30.0000);
    assert_eq!(layout.children[2].location.x, 63.0000);
    assert_eq!(layout.children[2].location.y, 0.0000);

    assert_eq!(layout.children[3].size.width, 34.0000);
    assert_eq!(layout.children[3].size.height, 30.0000);
    assert_eq!(layout.children[3].location.x, 0.0000);
    assert_eq!(layout.children[3].location.y, 30.0000);
}
