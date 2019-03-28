#[test]
fn justify_content_min_width_with_padding_child_width_greater_than_parent() {
    let layout = stretch::compute(
        &stretch::style::Node {
            flex_direction: stretch::style::FlexDirection::Column,
            size: stretch::geometry::Size {
                width: stretch::style::Dimension::Points(1000.0000),
                height: stretch::style::Dimension::Points(1584.0000),
                ..Default::default()
            },
            children: vec![stretch::style::Node {
                children: vec![stretch::style::Node {
                    justify_content: stretch::style::JustifyContent::Center,
                    min_size: stretch::geometry::Size {
                        width: stretch::style::Dimension::Points(400.0000),
                        ..Default::default()
                    },
                    padding: stretch::geometry::Rect {
                        start: stretch::style::Dimension::Points(100.0000),
                        end: stretch::style::Dimension::Points(100.0000),
                        ..Default::default()
                    },
                    children: vec![stretch::style::Node {
                        size: stretch::geometry::Size {
                            width: stretch::style::Dimension::Points(300.0000),
                            height: stretch::style::Dimension::Points(100.0000),
                            ..Default::default()
                        },
                        ..Default::default()
                    }],
                    ..Default::default()
                }],
                ..Default::default()
            }],
            ..Default::default()
        },
        stretch::geometry::Size::undefined(),
    )
    .unwrap();

    assert_eq!(layout.size.width, 1000.0000);
    assert_eq!(layout.size.height, 1584.0000);
    assert_eq!(layout.location.x, 0.0000);
    assert_eq!(layout.location.y, 0.0000);

    assert_eq!(layout.children[0].size.width, 1000.0000);
    assert_eq!(layout.children[0].size.height, 100.0000);
    assert_eq!(layout.children[0].location.x, 0.0000);
    assert_eq!(layout.children[0].location.y, 0.0000);

    assert_eq!(layout.children[0].children[0].size.width, 500.0000);
    assert_eq!(layout.children[0].children[0].size.height, 100.0000);
    assert_eq!(layout.children[0].children[0].location.x, 0.0000);
    assert_eq!(layout.children[0].children[0].location.y, 0.0000);

    assert_eq!(layout.children[0].children[0].children[0].size.width, 300.0000);
    assert_eq!(layout.children[0].children[0].children[0].size.height, 100.0000);
    assert_eq!(layout.children[0].children[0].children[0].location.x, 100.0000);
    assert_eq!(layout.children[0].children[0].children[0].location.y, 0.0000);
}
