#[test]
fn flex_wrap_wrap_to_child_height() {
    let layout = stretch::compute(
        &stretch::style::Node {
            flex_direction: stretch::style::FlexDirection::Column,
            children: vec![
                stretch::style::Node {
                    flex_wrap: stretch::style::FlexWrap::Wrap,
                    align_items: stretch::style::AlignItems::FlexStart,
                    children: vec![stretch::style::Node {
                        flex_direction: stretch::style::FlexDirection::Column,
                        size: stretch::geometry::Size {
                            width: stretch::style::Dimension::Points(100.0000),
                            ..Default::default()
                        },
                        children: vec![stretch::style::Node {
                            size: stretch::geometry::Size {
                                width: stretch::style::Dimension::Points(100.0000),
                                height: stretch::style::Dimension::Points(100.0000),
                                ..Default::default()
                            },
                            ..Default::default()
                        }],
                        ..Default::default()
                    }],
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

    assert_eq!(layout.size.width, 100.0000);
    assert_eq!(layout.size.height, 200.0000);
    assert_eq!(layout.location.x, 0.0000);
    assert_eq!(layout.location.y, 0.0000);

    assert_eq!(layout.children[0].size.width, 100.0000);
    assert_eq!(layout.children[0].size.height, 100.0000);
    assert_eq!(layout.children[0].location.x, 0.0000);
    assert_eq!(layout.children[0].location.y, 0.0000);

    assert_eq!(layout.children[0].children[0].size.width, 100.0000);
    assert_eq!(layout.children[0].children[0].size.height, 100.0000);
    assert_eq!(layout.children[0].children[0].location.x, 0.0000);
    assert_eq!(layout.children[0].children[0].location.y, 0.0000);

    assert_eq!(layout.children[0].children[0].children[0].size.width, 100.0000);
    assert_eq!(layout.children[0].children[0].children[0].size.height, 100.0000);
    assert_eq!(layout.children[0].children[0].children[0].location.x, 0.0000);
    assert_eq!(layout.children[0].children[0].children[0].location.y, 0.0000);

    assert_eq!(layout.children[1].size.width, 100.0000);
    assert_eq!(layout.children[1].size.height, 100.0000);
    assert_eq!(layout.children[1].location.x, 0.0000);
    assert_eq!(layout.children[1].location.y, 100.0000);
}
