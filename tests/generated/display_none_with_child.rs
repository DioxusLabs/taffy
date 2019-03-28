#[test]
fn display_none_with_child() {
    let layout = stretch::compute(
        &stretch::style::Node {
            size: stretch::geometry::Size {
                width: stretch::style::Dimension::Points(100.0000),
                height: stretch::style::Dimension::Points(100.0000),
                ..Default::default()
            },
            children: vec![
                stretch::style::Node {
                    flex_grow: 1.0000,
                    flex_shrink: 1.0000,
                    flex_basis: stretch::style::Dimension::Percent(0.0000),
                    ..Default::default()
                },
                stretch::style::Node {
                    display: stretch::style::Display::None,
                    flex_direction: stretch::style::FlexDirection::Column,
                    flex_grow: 1.0000,
                    flex_shrink: 1.0000,
                    flex_basis: stretch::style::Dimension::Percent(0.0000),
                    children: vec![stretch::style::Node {
                        flex_grow: 1.0000,
                        flex_shrink: 1.0000,
                        flex_basis: stretch::style::Dimension::Percent(0.0000),
                        size: stretch::geometry::Size {
                            width: stretch::style::Dimension::Points(20.0000),
                            ..Default::default()
                        },
                        ..Default::default()
                    }],
                    ..Default::default()
                },
                stretch::style::Node {
                    flex_grow: 1.0000,
                    flex_shrink: 1.0000,
                    flex_basis: stretch::style::Dimension::Percent(0.0000),
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

    assert_eq!(layout.children[0].size.width, 50.0000);
    assert_eq!(layout.children[0].size.height, 100.0000);
    assert_eq!(layout.children[0].location.x, 0.0000);
    assert_eq!(layout.children[0].location.y, 0.0000);

    assert_eq!(layout.children[1].size.width, 0.0000);
    assert_eq!(layout.children[1].size.height, 0.0000);
    assert_eq!(layout.children[1].location.x, 0.0000);
    assert_eq!(layout.children[1].location.y, 0.0000);

    assert_eq!(layout.children[1].children[0].size.width, 0.0000);
    assert_eq!(layout.children[1].children[0].size.height, 0.0000);
    assert_eq!(layout.children[1].children[0].location.x, 0.0000);
    assert_eq!(layout.children[1].children[0].location.y, 0.0000);

    assert_eq!(layout.children[2].size.width, 50.0000);
    assert_eq!(layout.children[2].size.height, 100.0000);
    assert_eq!(layout.children[2].location.x, 50.0000);
    assert_eq!(layout.children[2].location.y, 0.0000);
}
