#[test]
fn percentage_position_left_top() {
    let layout = stretch::compute(
        &stretch::style::Node {
            size: stretch::geometry::Size {
                width: stretch::style::Dimension::Points(400.0000),
                height: stretch::style::Dimension::Points(400.0000),
                ..Default::default()
            },
            children: vec![stretch::style::Node {
                size: stretch::geometry::Size {
                    width: stretch::style::Dimension::Percent(0.4500),
                    height: stretch::style::Dimension::Percent(0.5500),
                    ..Default::default()
                },
                position: stretch::geometry::Rect {
                    start: stretch::style::Dimension::Percent(0.1000),
                    top: stretch::style::Dimension::Percent(0.2000),
                    ..Default::default()
                },
                ..Default::default()
            }],
            ..Default::default()
        },
        stretch::geometry::Size::undefined(),
    )
    .unwrap();

    assert_eq!(layout.size.width, 400.0000);
    assert_eq!(layout.size.height, 400.0000);
    assert_eq!(layout.location.x, 0.0000);
    assert_eq!(layout.location.y, 0.0000);

    assert_eq!(layout.children[0].size.width, 180.0000);
    assert_eq!(layout.children[0].size.height, 220.0000);
    assert_eq!(layout.children[0].location.x, 40.0000);
    assert_eq!(layout.children[0].location.y, 80.0000);
}
