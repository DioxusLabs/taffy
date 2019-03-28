#[test]
fn percentage_flex_basis_cross_max_width() {
    let layout = stretch::compute(
        &stretch::style::Node {
            flex_direction: stretch::style::FlexDirection::Column,
            size: stretch::geometry::Size {
                width: stretch::style::Dimension::Points(200.0000),
                height: stretch::style::Dimension::Points(400.0000),
                ..Default::default()
            },
            children: vec![
                stretch::style::Node {
                    flex_grow: 1.0000,
                    flex_basis: stretch::style::Dimension::Percent(0.1000),
                    max_size: stretch::geometry::Size {
                        width: stretch::style::Dimension::Percent(0.6000),
                        ..Default::default()
                    },
                    ..Default::default()
                },
                stretch::style::Node {
                    flex_grow: 4.0000,
                    flex_basis: stretch::style::Dimension::Percent(0.1500),
                    max_size: stretch::geometry::Size {
                        width: stretch::style::Dimension::Percent(0.2000),
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

    assert_eq!(layout.size.width, 200.0000);
    assert_eq!(layout.size.height, 400.0000);
    assert_eq!(layout.location.x, 0.0000);
    assert_eq!(layout.location.y, 0.0000);

    assert_eq!(layout.children[0].size.width, 120.0000);
    assert_eq!(layout.children[0].size.height, 100.0000);
    assert_eq!(layout.children[0].location.x, 0.0000);
    assert_eq!(layout.children[0].location.y, 0.0000);

    assert_eq!(layout.children[1].size.width, 40.0000);
    assert_eq!(layout.children[1].size.height, 300.0000);
    assert_eq!(layout.children[1].location.x, 0.0000);
    assert_eq!(layout.children[1].location.y, 100.0000);
}
