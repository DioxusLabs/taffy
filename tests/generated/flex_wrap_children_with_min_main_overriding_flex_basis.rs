#[test]
fn flex_wrap_children_with_min_main_overriding_flex_basis() {
    let layout = stretch::compute(
        &stretch::style::Node {
            flex_wrap: stretch::style::FlexWrap::Wrap,
            size: stretch::geometry::Size { width: stretch::style::Dimension::Points(100.0000), ..Default::default() },
            children: vec![
                stretch::style::Node {
                    flex_basis: stretch::style::Dimension::Points(50.0000),
                    size: stretch::geometry::Size {
                        height: stretch::style::Dimension::Points(50.0000),
                        ..Default::default()
                    },
                    min_size: stretch::geometry::Size {
                        width: stretch::style::Dimension::Points(55.0000),
                        ..Default::default()
                    },
                    ..Default::default()
                },
                stretch::style::Node {
                    flex_basis: stretch::style::Dimension::Points(50.0000),
                    size: stretch::geometry::Size {
                        height: stretch::style::Dimension::Points(50.0000),
                        ..Default::default()
                    },
                    min_size: stretch::geometry::Size {
                        width: stretch::style::Dimension::Points(55.0000),
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

    assert_eq!(layout.children[0].size.width, 55.0000);
    assert_eq!(layout.children[0].size.height, 50.0000);
    assert_eq!(layout.children[0].location.x, 0.0000);
    assert_eq!(layout.children[0].location.y, 0.0000);

    assert_eq!(layout.children[1].size.width, 55.0000);
    assert_eq!(layout.children[1].size.height, 50.0000);
    assert_eq!(layout.children[1].location.x, 0.0000);
    assert_eq!(layout.children[1].location.y, 50.0000);
}
