#[test]
fn rounding_fractial_input_4() {
    let layout = stretch::compute(
        &stretch::style::Node {
            flex_direction: stretch::style::FlexDirection::Column,
            size: stretch::geometry::Size {
                width: stretch::style::Dimension::Points(100.0000),
                height: stretch::style::Dimension::Points(113.4000),
                ..Default::default()
            },
            children: vec![
                stretch::style::Node {
                    flex_grow: 1.0000,
                    flex_basis: stretch::style::Dimension::Points(50.0000),
                    size: stretch::geometry::Size {
                        height: stretch::style::Dimension::Points(20.0000),
                        ..Default::default()
                    },
                    ..Default::default()
                },
                stretch::style::Node {
                    flex_grow: 1.0000,
                    size: stretch::geometry::Size {
                        height: stretch::style::Dimension::Points(10.0000),
                        ..Default::default()
                    },
                    ..Default::default()
                },
                stretch::style::Node {
                    flex_grow: 1.0000,
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
    assert_eq!(layout.size.height, 113.0000);
    assert_eq!(layout.location.x, 0.0000);
    assert_eq!(layout.location.y, 0.0000);

    assert_eq!(layout.children[0].size.width, 100.0000);
    assert_eq!(layout.children[0].size.height, 64.0000);
    assert_eq!(layout.children[0].location.x, 0.0000);
    assert_eq!(layout.children[0].location.y, 0.0000);

    assert_eq!(layout.children[1].size.width, 100.0000);
    assert_eq!(layout.children[1].size.height, 25.0000);
    assert_eq!(layout.children[1].location.x, 0.0000);
    assert_eq!(layout.children[1].location.y, 64.0000);

    assert_eq!(layout.children[2].size.width, 100.0000);
    assert_eq!(layout.children[2].size.height, 24.0000);
    assert_eq!(layout.children[2].location.x, 0.0000);
    assert_eq!(layout.children[2].location.y, 89.0000);
}
