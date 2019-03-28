#[test]
fn percentage_container_in_wrapping_container() {
    let layout = stretch::compute(
        &stretch::style::Node {
            flex_direction: stretch::style::FlexDirection::Column,
            align_items: stretch::style::AlignItems::Center,
            justify_content: stretch::style::JustifyContent::Center,
            size: stretch::geometry::Size {
                width: stretch::style::Dimension::Points(200.0000),
                height: stretch::style::Dimension::Points(200.0000),
                ..Default::default()
            },
            children: vec![stretch::style::Node {
                flex_direction: stretch::style::FlexDirection::Column,
                children: vec![stretch::style::Node {
                    justify_content: stretch::style::JustifyContent::Center,
                    size: stretch::geometry::Size {
                        width: stretch::style::Dimension::Percent(1.0000),
                        ..Default::default()
                    },
                    children: vec![
                        stretch::style::Node {
                            size: stretch::geometry::Size {
                                width: stretch::style::Dimension::Points(50.0000),
                                height: stretch::style::Dimension::Points(50.0000),
                                ..Default::default()
                            },
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
                }],
                ..Default::default()
            }],
            ..Default::default()
        },
        stretch::geometry::Size::undefined(),
    )
    .unwrap();

    assert_eq!(layout.size.width, 200.0000);
    assert_eq!(layout.size.height, 200.0000);
    assert_eq!(layout.location.x, 0.0000);
    assert_eq!(layout.location.y, 0.0000);

    assert_eq!(layout.children[0].size.width, 100.0000);
    assert_eq!(layout.children[0].size.height, 50.0000);
    assert_eq!(layout.children[0].location.x, 50.0000);
    assert_eq!(layout.children[0].location.y, 75.0000);

    assert_eq!(layout.children[0].children[0].size.width, 100.0000);
    assert_eq!(layout.children[0].children[0].size.height, 50.0000);
    assert_eq!(layout.children[0].children[0].location.x, 0.0000);
    assert_eq!(layout.children[0].children[0].location.y, 0.0000);

    assert_eq!(layout.children[0].children[0].children[0].size.width, 50.0000);
    assert_eq!(layout.children[0].children[0].children[0].size.height, 50.0000);
    assert_eq!(layout.children[0].children[0].children[0].location.x, 0.0000);
    assert_eq!(layout.children[0].children[0].children[0].location.y, 0.0000);

    assert_eq!(layout.children[0].children[0].children[1].size.width, 50.0000);
    assert_eq!(layout.children[0].children[0].children[1].size.height, 50.0000);
    assert_eq!(layout.children[0].children[0].children[1].location.x, 50.0000);
    assert_eq!(layout.children[0].children[0].children[1].location.y, 0.0000);
}
