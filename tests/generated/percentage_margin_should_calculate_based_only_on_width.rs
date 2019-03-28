#[test]
fn percentage_margin_should_calculate_based_only_on_width() {
    let layout = stretch::compute(
        &stretch::style::Node {
            flex_direction: stretch::style::FlexDirection::Column,
            size: stretch::geometry::Size {
                width: stretch::style::Dimension::Points(200.0000),
                height: stretch::style::Dimension::Points(100.0000),
                ..Default::default()
            },
            children: vec![stretch::style::Node {
                flex_direction: stretch::style::FlexDirection::Column,
                flex_grow: 1.0000,
                margin: stretch::geometry::Rect {
                    start: stretch::style::Dimension::Percent(0.1000),
                    end: stretch::style::Dimension::Percent(0.1000),
                    top: stretch::style::Dimension::Percent(0.1000),
                    bottom: stretch::style::Dimension::Percent(0.1000),
                    ..Default::default()
                },
                children: vec![stretch::style::Node {
                    size: stretch::geometry::Size {
                        width: stretch::style::Dimension::Points(10.0000),
                        height: stretch::style::Dimension::Points(10.0000),
                        ..Default::default()
                    },
                    ..Default::default()
                }],
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

    assert_eq!(layout.children[0].size.width, 160.0000);
    assert_eq!(layout.children[0].size.height, 60.0000);
    assert_eq!(layout.children[0].location.x, 20.0000);
    assert_eq!(layout.children[0].location.y, 20.0000);

    assert_eq!(layout.children[0].children[0].size.width, 10.0000);
    assert_eq!(layout.children[0].children[0].size.height, 10.0000);
    assert_eq!(layout.children[0].children[0].location.x, 0.0000);
    assert_eq!(layout.children[0].children[0].location.y, 0.0000);
}
