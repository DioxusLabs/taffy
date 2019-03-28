#[test]
fn absolute_layout_align_items_center() {
    let layout = stretch::compute(
        &stretch::style::Node {
            align_items: stretch::style::AlignItems::Center,
            size: stretch::geometry::Size {
                width: stretch::style::Dimension::Points(110.0000),
                height: stretch::style::Dimension::Points(100.0000),
                ..Default::default()
            },
            children: vec![stretch::style::Node {
                position_type: stretch::style::PositionType::Absolute,
                size: stretch::geometry::Size {
                    width: stretch::style::Dimension::Points(60.0000),
                    height: stretch::style::Dimension::Points(40.0000),
                    ..Default::default()
                },
                ..Default::default()
            }],
            ..Default::default()
        },
        stretch::geometry::Size::undefined(),
    )
    .unwrap();

    assert_eq!(layout.size.width, 110.0000);
    assert_eq!(layout.size.height, 100.0000);
    assert_eq!(layout.location.x, 0.0000);
    assert_eq!(layout.location.y, 0.0000);

    assert_eq!(layout.children[0].size.width, 60.0000);
    assert_eq!(layout.children[0].size.height, 40.0000);
    assert_eq!(layout.children[0].location.x, 0.0000);
    assert_eq!(layout.children[0].location.y, 30.0000);
}
