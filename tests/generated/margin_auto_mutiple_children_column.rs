#[test]
fn margin_auto_mutiple_children_column() {
    let layout = stretch::compute(
        &stretch::style::Node {
            flex_direction: stretch::style::FlexDirection::Column,
            align_items: stretch::style::AlignItems::Center,
            size: stretch::geometry::Size {
                width: stretch::style::Dimension::Points(200.0000),
                height: stretch::style::Dimension::Points(200.0000),
                ..Default::default()
            },
            children: vec![
                stretch::style::Node {
                    size: stretch::geometry::Size {
                        width: stretch::style::Dimension::Points(50.0000),
                        height: stretch::style::Dimension::Points(50.0000),
                        ..Default::default()
                    },
                    margin: stretch::geometry::Rect { top: stretch::style::Dimension::Auto, ..Default::default() },
                    ..Default::default()
                },
                stretch::style::Node {
                    size: stretch::geometry::Size {
                        width: stretch::style::Dimension::Points(50.0000),
                        height: stretch::style::Dimension::Points(50.0000),
                        ..Default::default()
                    },
                    margin: stretch::geometry::Rect { top: stretch::style::Dimension::Auto, ..Default::default() },
                    ..Default::default()
                },
                stretch::style::Node {
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

    assert_eq!(layout.size.width, 200.0000);
    assert_eq!(layout.size.height, 200.0000);
    assert_eq!(layout.location.x, 0.0000);
    assert_eq!(layout.location.y, 0.0000);

    assert_eq!(layout.children[0].size.width, 50.0000);
    assert_eq!(layout.children[0].size.height, 50.0000);
    assert_eq!(layout.children[0].location.x, 75.0000);
    assert_eq!(layout.children[0].location.y, 25.0000);

    assert_eq!(layout.children[1].size.width, 50.0000);
    assert_eq!(layout.children[1].size.height, 50.0000);
    assert_eq!(layout.children[1].location.x, 75.0000);
    assert_eq!(layout.children[1].location.y, 100.0000);

    assert_eq!(layout.children[2].size.width, 50.0000);
    assert_eq!(layout.children[2].size.height, 50.0000);
    assert_eq!(layout.children[2].location.x, 75.0000);
    assert_eq!(layout.children[2].location.y, 150.0000);
}
