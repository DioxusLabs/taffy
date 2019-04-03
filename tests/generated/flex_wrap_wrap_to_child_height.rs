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
                            width: stretch::style::Dimension::Points(100f32),
                            ..Default::default()
                        },
                        children: vec![stretch::style::Node {
                            size: stretch::geometry::Size {
                                width: stretch::style::Dimension::Points(100f32),
                                height: stretch::style::Dimension::Points(100f32),
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
                        width: stretch::style::Dimension::Points(100f32),
                        height: stretch::style::Dimension::Points(100f32),
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
    assert_eq!(layout.size.width, 100f32);
    assert_eq!(layout.size.height, 200f32);
    assert_eq!(layout.location.x, 0f32);
    assert_eq!(layout.location.y, 0f32);
    assert_eq!(layout.children[0usize].size.width, 100f32);
    assert_eq!(layout.children[0usize].size.height, 100f32);
    assert_eq!(layout.children[0usize].location.x, 0f32);
    assert_eq!(layout.children[0usize].location.y, 0f32);
    assert_eq!(layout.children[0usize].children[0usize].size.width, 100f32);
    assert_eq!(layout.children[0usize].children[0usize].size.height, 100f32);
    assert_eq!(layout.children[0usize].children[0usize].location.x, 0f32);
    assert_eq!(layout.children[0usize].children[0usize].location.y, 0f32);
    assert_eq!(layout.children[0usize].children[0usize].children[0usize].size.width, 100f32);
    assert_eq!(layout.children[0usize].children[0usize].children[0usize].size.height, 100f32);
    assert_eq!(layout.children[0usize].children[0usize].children[0usize].location.x, 0f32);
    assert_eq!(layout.children[0usize].children[0usize].children[0usize].location.y, 0f32);
    assert_eq!(layout.children[1usize].size.width, 100f32);
    assert_eq!(layout.children[1usize].size.height, 100f32);
    assert_eq!(layout.children[1usize].location.x, 0f32);
    assert_eq!(layout.children[1usize].location.y, 100f32);
}
