#[test]
fn rounding_total_fractial() {
    let layout = stretch::compute(
        &stretch::style::Node {
            flex_direction: stretch::style::FlexDirection::Column,
            size: stretch::geometry::Size {
                width: stretch::style::Dimension::Points(87.4000),
                height: stretch::style::Dimension::Points(113.4000),
                ..Default::default()
            },
            children: vec![
                stretch::style::Node {
                    flex_grow: 0.7000,
                    flex_basis: stretch::style::Dimension::Points(50.3000),
                    size: stretch::geometry::Size {
                        height: stretch::style::Dimension::Points(20.3000),
                        ..Default::default()
                    },
                    ..Default::default()
                },
                stretch::style::Node {
                    flex_grow: 1.6000,
                    size: stretch::geometry::Size {
                        height: stretch::style::Dimension::Points(10.0000),
                        ..Default::default()
                    },
                    ..Default::default()
                },
                stretch::style::Node {
                    flex_grow: 1.1000,
                    size: stretch::geometry::Size {
                        height: stretch::style::Dimension::Points(10.7000),
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

    assert_eq!(layout.size.width, 87.0000);
    assert_eq!(layout.size.height, 113.0000);
    assert_eq!(layout.location.x, 0.0000);
    assert_eq!(layout.location.y, 0.0000);

    assert_eq!(layout.children[0].size.width, 87.0000);
    assert_eq!(layout.children[0].size.height, 59.0000);
    assert_eq!(layout.children[0].location.x, 0.0000);
    assert_eq!(layout.children[0].location.y, 0.0000);

    assert_eq!(layout.children[1].size.width, 87.0000);
    assert_eq!(layout.children[1].size.height, 30.0000);
    assert_eq!(layout.children[1].location.x, 0.0000);
    assert_eq!(layout.children[1].location.y, 59.0000);

    assert_eq!(layout.children[2].size.width, 87.0000);
    assert_eq!(layout.children[2].size.height, 24.0000);
    assert_eq!(layout.children[2].location.x, 0.0000);
    assert_eq!(layout.children[2].location.y, 89.0000);
}
