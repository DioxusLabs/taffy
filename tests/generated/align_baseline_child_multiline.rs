#[test]
fn align_baseline_child_multiline() {
    let layout = stretch::compute(
        &stretch::style::Node {
            align_items: stretch::style::AlignItems::Baseline,
            size: stretch::geometry::Size { width: stretch::style::Dimension::Points(100.0000), ..Default::default() },
            children: vec![
                stretch::style::Node {
                    size: stretch::geometry::Size {
                        width: stretch::style::Dimension::Points(50.0000),
                        height: stretch::style::Dimension::Points(60.0000),
                        ..Default::default()
                    },
                    ..Default::default()
                },
                stretch::style::Node {
                    flex_wrap: stretch::style::FlexWrap::Wrap,
                    size: stretch::geometry::Size {
                        width: stretch::style::Dimension::Points(50.0000),
                        ..Default::default()
                    },
                    children: vec![
                        stretch::style::Node {
                            size: stretch::geometry::Size {
                                width: stretch::style::Dimension::Points(25.0000),
                                height: stretch::style::Dimension::Points(20.0000),
                                ..Default::default()
                            },
                            ..Default::default()
                        },
                        stretch::style::Node {
                            size: stretch::geometry::Size {
                                width: stretch::style::Dimension::Points(25.0000),
                                height: stretch::style::Dimension::Points(10.0000),
                                ..Default::default()
                            },
                            ..Default::default()
                        },
                        stretch::style::Node {
                            size: stretch::geometry::Size {
                                width: stretch::style::Dimension::Points(25.0000),
                                height: stretch::style::Dimension::Points(20.0000),
                                ..Default::default()
                            },
                            ..Default::default()
                        },
                        stretch::style::Node {
                            size: stretch::geometry::Size {
                                width: stretch::style::Dimension::Points(25.0000),
                                height: stretch::style::Dimension::Points(10.0000),
                                ..Default::default()
                            },
                            ..Default::default()
                        },
                    ],
                    ..Default::default()
                },
            ],
            ..Default::default()
        },
        stretch::geometry::Size::undefined(),
    )
    .unwrap();

    assert_eq!(layout.size.width, 100.0000);
    assert_eq!(layout.size.height, 80.0000);
    assert_eq!(layout.location.x, 0.0000);
    assert_eq!(layout.location.y, 0.0000);

    assert_eq!(layout.children[0].size.width, 50.0000);
    assert_eq!(layout.children[0].size.height, 60.0000);
    assert_eq!(layout.children[0].location.x, 0.0000);
    assert_eq!(layout.children[0].location.y, 0.0000);

    assert_eq!(layout.children[1].size.width, 50.0000);
    assert_eq!(layout.children[1].size.height, 40.0000);
    assert_eq!(layout.children[1].location.x, 50.0000);
    assert_eq!(layout.children[1].location.y, 40.0000);

    assert_eq!(layout.children[1].children[0].size.width, 25.0000);
    assert_eq!(layout.children[1].children[0].size.height, 20.0000);
    assert_eq!(layout.children[1].children[0].location.x, 0.0000);
    assert_eq!(layout.children[1].children[0].location.y, 0.0000);

    assert_eq!(layout.children[1].children[1].size.width, 25.0000);
    assert_eq!(layout.children[1].children[1].size.height, 10.0000);
    assert_eq!(layout.children[1].children[1].location.x, 25.0000);
    assert_eq!(layout.children[1].children[1].location.y, 0.0000);

    assert_eq!(layout.children[1].children[2].size.width, 25.0000);
    assert_eq!(layout.children[1].children[2].size.height, 20.0000);
    assert_eq!(layout.children[1].children[2].location.x, 0.0000);
    assert_eq!(layout.children[1].children[2].location.y, 20.0000);

    assert_eq!(layout.children[1].children[3].size.width, 25.0000);
    assert_eq!(layout.children[1].children[3].size.height, 10.0000);
    assert_eq!(layout.children[1].children[3].location.x, 25.0000);
    assert_eq!(layout.children[1].children[3].location.y, 20.0000);
}
