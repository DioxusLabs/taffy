#[test]
fn percentage_size_based_on_parent_inner_size() {
    let layout = stretch::compute(
        &stretch::style::Node {
            flex_direction: stretch::style::FlexDirection::Column,
            size: stretch::geometry::Size {
                width: stretch::style::Dimension::Points(200.0000),
                height: stretch::style::Dimension::Points(400.0000),
                ..Default::default()
            },
            padding: stretch::geometry::Rect {
                start: stretch::style::Dimension::Points(20.0000),
                end: stretch::style::Dimension::Points(20.0000),
                top: stretch::style::Dimension::Points(20.0000),
                bottom: stretch::style::Dimension::Points(20.0000),
                ..Default::default()
            },
            children: vec![stretch::style::Node {
                size: stretch::geometry::Size {
                    width: stretch::style::Dimension::Percent(0.5000),
                    height: stretch::style::Dimension::Percent(0.5000),
                    ..Default::default()
                },
                ..Default::default()
            }],
            ..Default::default()
        },
        stretch::geometry::Size::undefined(),
    )
    .unwrap();

    assert_eq!(layout.size.width, 200.0000);
    assert_eq!(layout.size.height, 400.0000);
    assert_eq!(layout.location.x, 0.0000);
    assert_eq!(layout.location.y, 0.0000);

    assert_eq!(layout.children[0].size.width, 80.0000);
    assert_eq!(layout.children[0].size.height, 180.0000);
    assert_eq!(layout.children[0].location.x, 20.0000);
    assert_eq!(layout.children[0].location.y, 20.0000);
}
