#[test]
fn wrapped_column_max_height() {
    let layout = stretch::compute(
        &stretch::style::Node {
            flex_direction: stretch::style::FlexDirection::Column,
            flex_wrap: stretch::style::FlexWrap::Wrap,
            align_items: stretch::style::AlignItems::Center,
            align_content: stretch::style::AlignContent::Center,
            justify_content: stretch::style::JustifyContent::Center,
            size: stretch::geometry::Size {
                width: stretch::style::Dimension::Points(700.0000),
                height: stretch::style::Dimension::Points(500.0000),
                ..Default::default()
            },
            children: vec![
                stretch::style::Node {
                    size: stretch::geometry::Size {
                        width: stretch::style::Dimension::Points(100.0000),
                        height: stretch::style::Dimension::Points(500.0000),
                        ..Default::default()
                    },
                    max_size: stretch::geometry::Size {
                        height: stretch::style::Dimension::Points(200.0000),
                        ..Default::default()
                    },
                    ..Default::default()
                },
                stretch::style::Node {
                    size: stretch::geometry::Size {
                        width: stretch::style::Dimension::Points(200.0000),
                        height: stretch::style::Dimension::Points(200.0000),
                        ..Default::default()
                    },
                    margin: stretch::geometry::Rect {
                        start: stretch::style::Dimension::Points(20.0000),
                        end: stretch::style::Dimension::Points(20.0000),
                        top: stretch::style::Dimension::Points(20.0000),
                        bottom: stretch::style::Dimension::Points(20.0000),
                        ..Default::default()
                    },
                    ..Default::default()
                },
                stretch::style::Node {
                    size: stretch::geometry::Size {
                        width: stretch::style::Dimension::Points(100.0000),
                        height: stretch::style::Dimension::Points(100.0000),
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

    assert_eq!(layout.size.width, 700.0000);
    assert_eq!(layout.size.height, 500.0000);
    assert_eq!(layout.location.x, 0.0000);
    assert_eq!(layout.location.y, 0.0000);

    assert_eq!(layout.children[0].size.width, 100.0000);
    assert_eq!(layout.children[0].size.height, 200.0000);
    assert_eq!(layout.children[0].location.x, 250.0000);
    assert_eq!(layout.children[0].location.y, 30.0000);

    assert_eq!(layout.children[1].size.width, 200.0000);
    assert_eq!(layout.children[1].size.height, 200.0000);
    assert_eq!(layout.children[1].location.x, 200.0000);
    assert_eq!(layout.children[1].location.y, 250.0000);

    assert_eq!(layout.children[2].size.width, 100.0000);
    assert_eq!(layout.children[2].size.height, 100.0000);
    assert_eq!(layout.children[2].location.x, 420.0000);
    assert_eq!(layout.children[2].location.y, 200.0000);
}
