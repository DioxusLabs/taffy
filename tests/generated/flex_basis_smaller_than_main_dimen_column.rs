#[test]
fn flex_basis_smaller_than_main_dimen_column() {
    let layout = stretch::compute(
        &stretch::style::Node {
            flex_direction: stretch::style::FlexDirection::Column,
            size: stretch::geometry::Size { height: stretch::style::Dimension::Points(100.0000), ..Default::default() },
            children: vec![stretch::style::Node {
                flex_basis: stretch::style::Dimension::Points(10.0000),
                size: stretch::geometry::Size {
                    width: stretch::style::Dimension::Points(50.0000),
                    height: stretch::style::Dimension::Points(50.0000),
                    ..Default::default()
                },
                ..Default::default()
            }],
            ..Default::default()
        },
        stretch::geometry::Size::undefined(),
    )
    .unwrap();

    assert_eq!(layout.size.width, 50.0000);
    assert_eq!(layout.size.height, 100.0000);
    assert_eq!(layout.location.x, 0.0000);
    assert_eq!(layout.location.y, 0.0000);

    assert_eq!(layout.children[0].size.width, 50.0000);
    assert_eq!(layout.children[0].size.height, 10.0000);
    assert_eq!(layout.children[0].location.x, 0.0000);
    assert_eq!(layout.children[0].location.y, 0.0000);
}
