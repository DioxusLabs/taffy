#[test]
fn percentage_position_bottom_right() {
    let layout = stretch::compute(
        &stretch::style::Node {
            size: stretch::geometry::Size {
                width: stretch::style::Dimension::Points(500.0000),
                height: stretch::style::Dimension::Points(500.0000),
                ..Default::default()
            },
            children: vec![stretch::style::Node {
                size: stretch::geometry::Size {
                    width: stretch::style::Dimension::Percent(0.5500),
                    height: stretch::style::Dimension::Percent(0.1500),
                    ..Default::default()
                },
                position: stretch::geometry::Rect {
                    end: stretch::style::Dimension::Percent(0.2000),
                    bottom: stretch::style::Dimension::Percent(0.1000),
                    ..Default::default()
                },
                ..Default::default()
            }],
            ..Default::default()
        },
        stretch::geometry::Size::undefined(),
    )
    .unwrap();

    assert_eq!(layout.size.width, 500.0000);
    assert_eq!(layout.size.height, 500.0000);
    assert_eq!(layout.location.x, 0.0000);
    assert_eq!(layout.location.y, 0.0000);

    assert_eq!(layout.children[0].size.width, 275.0000);
    assert_eq!(layout.children[0].size.height, 75.0000);
    assert_eq!(layout.children[0].location.x, -100.0000);
    assert_eq!(layout.children[0].location.y, -50.0000);
}
