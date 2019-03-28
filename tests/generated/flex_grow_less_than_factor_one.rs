#[test]
fn flex_grow_less_than_factor_one() {
    let layout = stretch::compute(
        &stretch::style::Node {
            size: stretch::geometry::Size {
                width: stretch::style::Dimension::Points(500.0000),
                height: stretch::style::Dimension::Points(200.0000),
                ..Default::default()
            },
            children: vec![
                stretch::style::Node {
                    flex_grow: 0.2000,
                    flex_shrink: 0.0000,
                    flex_basis: stretch::style::Dimension::Points(40.0000),
                    ..Default::default()
                },
                stretch::style::Node { flex_grow: 0.2000, flex_shrink: 0.0000, ..Default::default() },
                stretch::style::Node { flex_grow: 0.4000, flex_shrink: 0.0000, ..Default::default() },
            ],
            ..Default::default()
        },
        stretch::geometry::Size::undefined(),
    )
    .unwrap();

    assert_eq!(layout.size.width, 500.0000);
    assert_eq!(layout.size.height, 200.0000);
    assert_eq!(layout.location.x, 0.0000);
    assert_eq!(layout.location.y, 0.0000);

    assert_eq!(layout.children[0].size.width, 132.0000);
    assert_eq!(layout.children[0].size.height, 200.0000);
    assert_eq!(layout.children[0].location.x, 0.0000);
    assert_eq!(layout.children[0].location.y, 0.0000);

    assert_eq!(layout.children[1].size.width, 92.0000);
    assert_eq!(layout.children[1].size.height, 200.0000);
    assert_eq!(layout.children[1].location.x, 132.0000);
    assert_eq!(layout.children[1].location.y, 0.0000);

    assert_eq!(layout.children[2].size.width, 184.0000);
    assert_eq!(layout.children[2].size.height, 200.0000);
    assert_eq!(layout.children[2].location.x, 224.0000);
    assert_eq!(layout.children[2].location.y, 0.0000);
}
