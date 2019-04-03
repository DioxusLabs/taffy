#[test]
fn wrap_nodes_with_content_sizing_overflowing_margin() {
    let layout = stretch::compute(
        &stretch::style::Node {
            flex_direction: stretch::style::FlexDirection::Column,
            size: stretch::geometry::Size {
                width: stretch::style::Dimension::Points(500f32),
                height: stretch::style::Dimension::Points(500f32),
                ..Default::default()
            },
            children: vec![stretch::style::Node {
                flex_wrap: stretch::style::FlexWrap::Wrap,
                size: stretch::geometry::Size { width: stretch::style::Dimension::Points(85f32), ..Default::default() },
                children: vec![
                    stretch::style::Node {
                        flex_direction: stretch::style::FlexDirection::Column,
                        children: vec![stretch::style::Node {
                            size: stretch::geometry::Size {
                                width: stretch::style::Dimension::Points(40f32),
                                height: stretch::style::Dimension::Points(40f32),
                                ..Default::default()
                            },
                            ..Default::default()
                        }],
                        ..Default::default()
                    },
                    stretch::style::Node {
                        flex_direction: stretch::style::FlexDirection::Column,
                        children: vec![stretch::style::Node {
                            size: stretch::geometry::Size {
                                width: stretch::style::Dimension::Points(40f32),
                                height: stretch::style::Dimension::Points(40f32),
                                ..Default::default()
                            },
                            ..Default::default()
                        }],
                        margin: stretch::geometry::Rect {
                            end: stretch::style::Dimension::Points(10f32),
                            ..Default::default()
                        },
                        ..Default::default()
                    },
                ],
                ..Default::default()
            }],
            ..Default::default()
        },
        stretch::geometry::Size::undefined(),
    )
    .unwrap();
    assert_eq!(layout.size.width, 500f32);
    assert_eq!(layout.size.height, 500f32);
    assert_eq!(layout.location.x, 0f32);
    assert_eq!(layout.location.y, 0f32);
    assert_eq!(layout.children[0usize].size.width, 85f32);
    assert_eq!(layout.children[0usize].size.height, 80f32);
    assert_eq!(layout.children[0usize].location.x, 0f32);
    assert_eq!(layout.children[0usize].location.y, 0f32);
    assert_eq!(layout.children[0usize].children[0usize].size.width, 40f32);
    assert_eq!(layout.children[0usize].children[0usize].size.height, 40f32);
    assert_eq!(layout.children[0usize].children[0usize].location.x, 0f32);
    assert_eq!(layout.children[0usize].children[0usize].location.y, 0f32);
    assert_eq!(layout.children[0usize].children[0usize].children[0usize].size.width, 40f32);
    assert_eq!(layout.children[0usize].children[0usize].children[0usize].size.height, 40f32);
    assert_eq!(layout.children[0usize].children[0usize].children[0usize].location.x, 0f32);
    assert_eq!(layout.children[0usize].children[0usize].children[0usize].location.y, 0f32);
    assert_eq!(layout.children[0usize].children[1usize].size.width, 40f32);
    assert_eq!(layout.children[0usize].children[1usize].size.height, 40f32);
    assert_eq!(layout.children[0usize].children[1usize].location.x, 0f32);
    assert_eq!(layout.children[0usize].children[1usize].location.y, 40f32);
    assert_eq!(layout.children[0usize].children[1usize].children[0usize].size.width, 40f32);
    assert_eq!(layout.children[0usize].children[1usize].children[0usize].size.height, 40f32);
    assert_eq!(layout.children[0usize].children[1usize].children[0usize].location.x, 0f32);
    assert_eq!(layout.children[0usize].children[1usize].children[0usize].location.y, 0f32);
}
