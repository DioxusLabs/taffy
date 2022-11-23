#[test]
fn margin_auto_left_child_bigger_than_parent() {
    let mut taffy = taffy::Taffy::new();
    let node0 = taffy
        .new_with_children(
            taffy::style::Style {
                size: taffy::geometry::Size {
                    width: taffy::style::Dimension::Points(72f32),
                    height: taffy::style::Dimension::Points(72f32),
                    ..Default::default()
                },
                margin: taffy::geometry::Rect { start: taffy::style::Dimension::Auto, ..Default::default() },
                ..Default::default()
            },
            &[],
        )
        .unwrap();
    let node = taffy
        .new_with_children(
            taffy::style::Style {
                justify_content: taffy::style::JustifyContent::Center,
                size: taffy::geometry::Size {
                    width: taffy::style::Dimension::Points(52f32),
                    height: taffy::style::Dimension::Points(52f32),
                    ..Default::default()
                },
                ..Default::default()
            },
            &[node0],
        )
        .unwrap();
    taffy.compute_layout(node, taffy::geometry::Size::MAX_CONTENT).unwrap();
    assert_eq!(taffy.layout(node).unwrap().size.width, 52f32);
    assert_eq!(taffy.layout(node).unwrap().size.height, 52f32);
    assert_eq!(taffy.layout(node).unwrap().location.x, 0f32);
    assert_eq!(taffy.layout(node).unwrap().location.y, 0f32);
    assert_eq!(taffy.layout(node0).unwrap().size.width, 52f32);
    assert_eq!(taffy.layout(node0).unwrap().size.height, 72f32);
    assert_eq!(taffy.layout(node0).unwrap().location.x, 0f32);
    assert_eq!(taffy.layout(node0).unwrap().location.y, 0f32);
}
