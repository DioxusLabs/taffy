#[test]
fn justify_content_overflow_min_max() {
    let layout = stretch::compute(
        &stretch::style::Node {
            flex_direction: stretch::style::FlexDirection::Column,
            justify_content: stretch::style::JustifyContent::Center,
            min_size: stretch::geometry::Size {
                height: stretch::style::Dimension::Points(100.0000),
                ..Default::default()
            },
            max_size: stretch::geometry::Size {
                height: stretch::style::Dimension::Points(110.0000),
                ..Default::default()
            },
            children: vec![
                stretch::style::Node {
                    flex_shrink: 0.0000,
                    size: stretch::geometry::Size {
                        width: stretch::style::Dimension::Points(50.0000),
                        height: stretch::style::Dimension::Points(50.0000),
                        ..Default::default()
                    },
                    ..Default::default()
                },
                stretch::style::Node {
                    flex_shrink: 0.0000,
                    size: stretch::geometry::Size {
                        width: stretch::style::Dimension::Points(50.0000),
                        height: stretch::style::Dimension::Points(50.0000),
                        ..Default::default()
                    },
                    ..Default::default()
                },
                stretch::style::Node {
                    flex_shrink: 0.0000,
                    size: stretch::geometry::Size {
                        width: stretch::style::Dimension::Points(50.0000),
                        height: stretch::style::Dimension::Points(50.0000),
                        ..Default::default()
                    },
                    ..Default::default()
                },
            ],
            ..Default::default()
        },
        stretch::geometry::Size::undefined(),
    )
    .unwrap();

    assert_eq!(layout.size.width, 50.0000);
    assert_eq!(layout.size.height, 110.0000);
    assert_eq!(layout.location.x, 0.0000);
    assert_eq!(layout.location.y, 0.0000);

    assert_eq!(layout.children[0].size.width, 50.0000);
    assert_eq!(layout.children[0].size.height, 50.0000);
    assert_eq!(layout.children[0].location.x, 0.0000);
    assert_eq!(layout.children[0].location.y, -20.0000);

    assert_eq!(layout.children[1].size.width, 50.0000);
    assert_eq!(layout.children[1].size.height, 50.0000);
    assert_eq!(layout.children[1].location.x, 0.0000);
    assert_eq!(layout.children[1].location.y, 30.0000);

    assert_eq!(layout.children[2].size.width, 50.0000);
    assert_eq!(layout.children[2].size.height, 50.0000);
    assert_eq!(layout.children[2].location.x, 0.0000);
    assert_eq!(layout.children[2].location.y, 80.0000);
}
