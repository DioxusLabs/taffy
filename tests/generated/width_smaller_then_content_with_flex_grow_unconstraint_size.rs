#[test]
fn width_smaller_then_content_with_flex_grow_unconstraint_size() {
    let layout = stretch::compute(
        &stretch::style::Node {
            children: vec![
                stretch::style::Node {
                    flex_direction: stretch::style::FlexDirection::Column,
                    flex_grow: 1.0000,
                    size: stretch::geometry::Size {
                        width: stretch::style::Dimension::Points(0.0000),
                        ..Default::default()
                    },
                    children: vec![stretch::style::Node {
                        size: stretch::geometry::Size {
                            width: stretch::style::Dimension::Points(70.0000),
                            height: stretch::style::Dimension::Points(100.0000),
                            ..Default::default()
                        },
                        ..Default::default()
                    }],
                    ..Default::default()
                },
                stretch::style::Node {
                    flex_direction: stretch::style::FlexDirection::Column,
                    flex_grow: 1.0000,
                    size: stretch::geometry::Size {
                        width: stretch::style::Dimension::Points(0.0000),
                        ..Default::default()
                    },
                    children: vec![stretch::style::Node {
                        size: stretch::geometry::Size {
                            width: stretch::style::Dimension::Points(20.0000),
                            height: stretch::style::Dimension::Points(100.0000),
                            ..Default::default()
                        },
                        ..Default::default()
                    }],
                    ..Default::default()
                },
            ],
            ..Default::default()
        },
        stretch::geometry::Size::undefined(),
    )
    .unwrap();

    assert_eq!(layout.size.width, 0.0000);
    assert_eq!(layout.size.height, 100.0000);
    assert_eq!(layout.location.x, 0.0000);
    assert_eq!(layout.location.y, 0.0000);

    assert_eq!(layout.children[0].size.width, 0.0000);
    assert_eq!(layout.children[0].size.height, 100.0000);
    assert_eq!(layout.children[0].location.x, 0.0000);
    assert_eq!(layout.children[0].location.y, 0.0000);

    assert_eq!(layout.children[0].children[0].size.width, 70.0000);
    assert_eq!(layout.children[0].children[0].size.height, 100.0000);
    assert_eq!(layout.children[0].children[0].location.x, 0.0000);
    assert_eq!(layout.children[0].children[0].location.y, 0.0000);

    assert_eq!(layout.children[1].size.width, 0.0000);
    assert_eq!(layout.children[1].size.height, 100.0000);
    assert_eq!(layout.children[1].location.x, 0.0000);
    assert_eq!(layout.children[1].location.y, 0.0000);

    assert_eq!(layout.children[1].children[0].size.width, 20.0000);
    assert_eq!(layout.children[1].children[0].size.height, 100.0000);
    assert_eq!(layout.children[1].children[0].location.x, 0.0000);
    assert_eq!(layout.children[1].children[0].location.y, 0.0000);
}
