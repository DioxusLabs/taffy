#[test]
fn percentage_multiple_nested_with_padding_margin_and_percentage_values() {
    let layout = stretch::compute(
        &stretch::style::Node {
            flex_direction: stretch::style::FlexDirection::Column,
            size: stretch::geometry::Size {
                width: stretch::style::Dimension::Points(200.0000),
                height: stretch::style::Dimension::Points(200.0000),
                ..Default::default()
            },
            children: vec![
                stretch::style::Node {
                    flex_direction: stretch::style::FlexDirection::Column,
                    flex_grow: 1.0000,
                    flex_basis: stretch::style::Dimension::Percent(0.1000),
                    min_size: stretch::geometry::Size {
                        width: stretch::style::Dimension::Percent(0.6000),
                        ..Default::default()
                    },
                    margin: stretch::geometry::Rect {
                        start: stretch::style::Dimension::Points(5.0000),
                        end: stretch::style::Dimension::Points(5.0000),
                        top: stretch::style::Dimension::Points(5.0000),
                        bottom: stretch::style::Dimension::Points(5.0000),
                        ..Default::default()
                    },
                    padding: stretch::geometry::Rect {
                        start: stretch::style::Dimension::Points(3.0000),
                        end: stretch::style::Dimension::Points(3.0000),
                        top: stretch::style::Dimension::Points(3.0000),
                        bottom: stretch::style::Dimension::Points(3.0000),
                        ..Default::default()
                    },
                    children: vec![stretch::style::Node {
                        flex_direction: stretch::style::FlexDirection::Column,
                        size: stretch::geometry::Size {
                            width: stretch::style::Dimension::Percent(0.5000),
                            ..Default::default()
                        },
                        margin: stretch::geometry::Rect {
                            start: stretch::style::Dimension::Points(5.0000),
                            end: stretch::style::Dimension::Points(5.0000),
                            top: stretch::style::Dimension::Points(5.0000),
                            bottom: stretch::style::Dimension::Points(5.0000),
                            ..Default::default()
                        },
                        padding: stretch::geometry::Rect {
                            start: stretch::style::Dimension::Percent(0.0300),
                            end: stretch::style::Dimension::Percent(0.0300),
                            top: stretch::style::Dimension::Percent(0.0300),
                            bottom: stretch::style::Dimension::Percent(0.0300),
                            ..Default::default()
                        },
                        children: vec![stretch::style::Node {
                            size: stretch::geometry::Size {
                                width: stretch::style::Dimension::Percent(0.4500),
                                ..Default::default()
                            },
                            margin: stretch::geometry::Rect {
                                start: stretch::style::Dimension::Percent(0.0500),
                                end: stretch::style::Dimension::Percent(0.0500),
                                top: stretch::style::Dimension::Percent(0.0500),
                                bottom: stretch::style::Dimension::Percent(0.0500),
                                ..Default::default()
                            },
                            padding: stretch::geometry::Rect {
                                start: stretch::style::Dimension::Points(3.0000),
                                end: stretch::style::Dimension::Points(3.0000),
                                top: stretch::style::Dimension::Points(3.0000),
                                bottom: stretch::style::Dimension::Points(3.0000),
                                ..Default::default()
                            },
                            ..Default::default()
                        }],
                        ..Default::default()
                    }],
                    ..Default::default()
                },
                stretch::style::Node {
                    flex_grow: 4.0000,
                    flex_basis: stretch::style::Dimension::Percent(0.1500),
                    min_size: stretch::geometry::Size {
                        width: stretch::style::Dimension::Percent(0.2000),
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

    assert_eq!(layout.children[0].size.width, 190.0000);
    assert_eq!(layout.children[0].size.height, 48.0000);
    assert_eq!(layout.children[0].location.x, 5.0000);
    assert_eq!(layout.children[0].location.y, 5.0000);

    assert_eq!(layout.children[0].children[0].size.width, 92.0000);
    assert_eq!(layout.children[0].children[0].size.height, 25.0000);
    assert_eq!(layout.children[0].children[0].location.x, 8.0000);
    assert_eq!(layout.children[0].children[0].location.y, 8.0000);

    assert_eq!(layout.children[0].children[0].children[0].size.width, 36.0000);
    assert_eq!(layout.children[0].children[0].children[0].size.height, 6.0000);
    assert_eq!(layout.children[0].children[0].children[0].location.x, 10.0000);
    assert_eq!(layout.children[0].children[0].children[0].location.y, 10.0000);

    assert_eq!(layout.children[1].size.width, 200.0000);
    assert_eq!(layout.children[1].size.height, 142.0000);
    assert_eq!(layout.children[1].location.x, 0.0000);
    assert_eq!(layout.children[1].location.y, 58.0000);
}
