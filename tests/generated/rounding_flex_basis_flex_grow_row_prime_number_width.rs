#[test]
fn rounding_flex_basis_flex_grow_row_prime_number_width() {
    let layout = stretch::compute(
        &stretch::style::Node {
            size: stretch::geometry::Size {
                width: stretch::style::Dimension::Points(113.0000),
                height: stretch::style::Dimension::Points(100.0000),
                ..Default::default()
            },
            children: vec![
                stretch::style::Node { flex_grow: 1.0000, ..Default::default() },
                stretch::style::Node { flex_grow: 1.0000, ..Default::default() },
                stretch::style::Node { flex_grow: 1.0000, ..Default::default() },
                stretch::style::Node { flex_grow: 1.0000, ..Default::default() },
                stretch::style::Node { flex_grow: 1.0000, ..Default::default() },
            ],
            ..Default::default()
        },
        stretch::geometry::Size::undefined(),
    )
    .unwrap();

    assert_eq!(layout.size.width, 113.0000);
    assert_eq!(layout.size.height, 100.0000);
    assert_eq!(layout.location.x, 0.0000);
    assert_eq!(layout.location.y, 0.0000);

    assert_eq!(layout.children[0].size.width, 23.0000);
    assert_eq!(layout.children[0].size.height, 100.0000);
    assert_eq!(layout.children[0].location.x, 0.0000);
    assert_eq!(layout.children[0].location.y, 0.0000);

    assert_eq!(layout.children[1].size.width, 22.0000);
    assert_eq!(layout.children[1].size.height, 100.0000);
    assert_eq!(layout.children[1].location.x, 23.0000);
    assert_eq!(layout.children[1].location.y, 0.0000);

    assert_eq!(layout.children[2].size.width, 23.0000);
    assert_eq!(layout.children[2].size.height, 100.0000);
    assert_eq!(layout.children[2].location.x, 45.0000);
    assert_eq!(layout.children[2].location.y, 0.0000);

    assert_eq!(layout.children[3].size.width, 22.0000);
    assert_eq!(layout.children[3].size.height, 100.0000);
    assert_eq!(layout.children[3].location.x, 68.0000);
    assert_eq!(layout.children[3].location.y, 0.0000);

    assert_eq!(layout.children[4].size.width, 23.0000);
    assert_eq!(layout.children[4].size.height, 100.0000);
    assert_eq!(layout.children[4].location.x, 90.0000);
    assert_eq!(layout.children[4].location.y, 0.0000);
}
