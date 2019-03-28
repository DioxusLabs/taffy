#[test]
fn child_min_max_width_flexing() {
    let layout = stretch::compute(
        &stretch::style::Node {
            size: stretch::geometry::Size {
                width: stretch::style::Dimension::Points(120.0000),
                height: stretch::style::Dimension::Points(50.0000),
                ..Default::default()
            },
            children: vec![
                stretch::style::Node {
                    flex_grow: 1.0000,
                    flex_shrink: 0.0000,
                    flex_basis: stretch::style::Dimension::Points(0.0000),
                    min_size: stretch::geometry::Size {
                        width: stretch::style::Dimension::Points(60.0000),
                        ..Default::default()
                    },
                    ..Default::default()
                },
                stretch::style::Node {
                    flex_grow: 1.0000,
                    flex_shrink: 0.0000,
                    flex_basis: stretch::style::Dimension::Percent(0.5000),
                    max_size: stretch::geometry::Size {
                        width: stretch::style::Dimension::Points(20.0000),
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

    assert_eq!(layout.size.width, 120.0000);
    assert_eq!(layout.size.height, 50.0000);
    assert_eq!(layout.location.x, 0.0000);
    assert_eq!(layout.location.y, 0.0000);

    assert_eq!(layout.children[0].size.width, 100.0000);
    assert_eq!(layout.children[0].size.height, 50.0000);
    assert_eq!(layout.children[0].location.x, 0.0000);
    assert_eq!(layout.children[0].location.y, 0.0000);

    assert_eq!(layout.children[1].size.width, 20.0000);
    assert_eq!(layout.children[1].size.height, 50.0000);
    assert_eq!(layout.children[1].location.x, 100.0000);
    assert_eq!(layout.children[1].location.y, 0.0000);
}
