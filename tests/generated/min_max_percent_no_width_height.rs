#[test]
fn min_max_percent_no_width_height() {
    let layout = stretch::compute(
        &stretch::style::Node {
            flex_direction: stretch::style::FlexDirection::Column,
            align_items: stretch::style::AlignItems::FlexStart,
            size: stretch::geometry::Size {
                width: stretch::style::Dimension::Points(100.0000),
                height: stretch::style::Dimension::Points(100.0000),
                ..Default::default()
            },
            children: vec![stretch::style::Node {
                min_size: stretch::geometry::Size {
                    width: stretch::style::Dimension::Percent(0.1000),
                    height: stretch::style::Dimension::Percent(0.1000),
                    ..Default::default()
                },
                max_size: stretch::geometry::Size {
                    width: stretch::style::Dimension::Percent(0.1000),
                    height: stretch::style::Dimension::Percent(0.1000),
                    ..Default::default()
                },
                ..Default::default()
            }],
            ..Default::default()
        },
        stretch::geometry::Size::undefined(),
    )
    .unwrap();

    assert_eq!(layout.size.width, 100.0000);
    assert_eq!(layout.size.height, 100.0000);
    assert_eq!(layout.location.x, 0.0000);
    assert_eq!(layout.location.y, 0.0000);

    assert_eq!(layout.children[0].size.width, 10.0000);
    assert_eq!(layout.children[0].size.height, 10.0000);
    assert_eq!(layout.children[0].location.x, 0.0000);
    assert_eq!(layout.children[0].location.y, 0.0000);
}
