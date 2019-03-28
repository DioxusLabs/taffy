#[test]
fn justify_content_row_space_evenly() {
    let layout = stretch::compute(
        &stretch::style::Node {
            justify_content: stretch::style::JustifyContent::SpaceEvenly,
            size: stretch::geometry::Size {
                width: stretch::style::Dimension::Points(100.0000),
                height: stretch::style::Dimension::Points(100.0000),
                ..Default::default()
            },
            children: vec![
                stretch::style::Node {
                    size: stretch::geometry::Size {
                        height: stretch::style::Dimension::Points(10.0000),
                        ..Default::default()
                    },
                    ..Default::default()
                },
                stretch::style::Node {
                    size: stretch::geometry::Size {
                        height: stretch::style::Dimension::Points(10.0000),
                        ..Default::default()
                    },
                    ..Default::default()
                },
                stretch::style::Node {
                    size: stretch::geometry::Size {
                        height: stretch::style::Dimension::Points(10.0000),
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

    assert_eq!(layout.children[0].size.width, 0.0000);
    assert_eq!(layout.children[0].size.height, 10.0000);
    assert_eq!(layout.children[0].location.x, 25.0000);
    assert_eq!(layout.children[0].location.y, 0.0000);

    assert_eq!(layout.children[1].size.width, 0.0000);
    assert_eq!(layout.children[1].size.height, 10.0000);
    assert_eq!(layout.children[1].location.x, 50.0000);
    assert_eq!(layout.children[1].location.y, 0.0000);

    assert_eq!(layout.children[2].size.width, 0.0000);
    assert_eq!(layout.children[2].size.height, 10.0000);
    assert_eq!(layout.children[2].location.x, 75.0000);
    assert_eq!(layout.children[2].location.y, 0.0000);
}
