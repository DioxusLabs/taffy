#[test]
fn flex_direction_row_no_width() {
    let layout = stretch::compute(
        &stretch::style::Node {
            size: stretch::geometry::Size { height: stretch::style::Dimension::Points(100.0000), ..Default::default() },
            children: vec![
                stretch::style::Node {
                    size: stretch::geometry::Size {
                        width: stretch::style::Dimension::Points(10.0000),
                        ..Default::default()
                    },
                    ..Default::default()
                },
                stretch::style::Node {
                    size: stretch::geometry::Size {
                        width: stretch::style::Dimension::Points(10.0000),
                        ..Default::default()
                    },
                    ..Default::default()
                },
                stretch::style::Node {
                    size: stretch::geometry::Size {
                        width: stretch::style::Dimension::Points(10.0000),
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

    assert_eq!(layout.size.width, 30.0000);
    assert_eq!(layout.size.height, 100.0000);
    assert_eq!(layout.location.x, 0.0000);
    assert_eq!(layout.location.y, 0.0000);

    assert_eq!(layout.children[0].size.width, 10.0000);
    assert_eq!(layout.children[0].size.height, 100.0000);
    assert_eq!(layout.children[0].location.x, 0.0000);
    assert_eq!(layout.children[0].location.y, 0.0000);

    assert_eq!(layout.children[1].size.width, 10.0000);
    assert_eq!(layout.children[1].size.height, 100.0000);
    assert_eq!(layout.children[1].location.x, 10.0000);
    assert_eq!(layout.children[1].location.y, 0.0000);

    assert_eq!(layout.children[2].size.width, 10.0000);
    assert_eq!(layout.children[2].size.height, 100.0000);
    assert_eq!(layout.children[2].location.x, 20.0000);
    assert_eq!(layout.children[2].location.y, 0.0000);
}
