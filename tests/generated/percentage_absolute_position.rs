#[test]
fn percentage_absolute_position() {
    let layout = stretch::compute(
        &stretch::style::Node {
            flex_direction: stretch::style::FlexDirection::Column,
            size: stretch::geometry::Size {
                width: stretch::style::Dimension::Points(200.0000),
                height: stretch::style::Dimension::Points(100.0000),
                ..Default::default()
            },
            children: vec![stretch::style::Node {
                position_type: stretch::style::PositionType::Absolute,
                size: stretch::geometry::Size {
                    width: stretch::style::Dimension::Points(10.0000),
                    height: stretch::style::Dimension::Points(10.0000),
                    ..Default::default()
                },
                position: stretch::geometry::Rect {
                    start: stretch::style::Dimension::Percent(0.3000),
                    top: stretch::style::Dimension::Percent(0.1000),
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
    assert_eq!(layout.size.height, 100.0000);
    assert_eq!(layout.location.x, 0.0000);
    assert_eq!(layout.location.y, 0.0000);

    assert_eq!(layout.children[0].size.width, 10.0000);
    assert_eq!(layout.children[0].size.height, 10.0000);
    assert_eq!(layout.children[0].location.x, 60.0000);
    assert_eq!(layout.children[0].location.y, 10.0000);
}
