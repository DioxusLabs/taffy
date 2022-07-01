#[test]
fn percentage_position_left_top() {
    let mut taffy = taffy::Taffy::new();
    let node0 = taffy.new_with_children(
        taffy::style::FlexboxLayout {
            size: taffy::geometry::Size {
                width: taffy::style::Dimension::Percent(0.45f32),
                height: taffy::style::Dimension::Percent(0.55f32),
                ..Default::default()
            },
            position: taffy::geometry::Rect {
                start: taffy::style::Dimension::Percent(0.1f32),
                top: taffy::style::Dimension::Percent(0.2f32),
                ..Default::default()
            },
            ..Default::default()
        },
        &[],
    );
    let node = taffy.new_with_children(
        taffy::style::FlexboxLayout {
            size: taffy::geometry::Size {
                width: taffy::style::Dimension::Points(400f32),
                height: taffy::style::Dimension::Points(400f32),
                ..Default::default()
            },
            ..Default::default()
        },
        &[node0],
    );
    taffy.compute_layout(node, taffy::geometry::Size::undefined()).unwrap();
    assert_eq!(taffy.layout(node).unwrap().size.width, 400f32);
    assert_eq!(taffy.layout(node).unwrap().size.height, 400f32);
    assert_eq!(taffy.layout(node).unwrap().location.x, 0f32);
    assert_eq!(taffy.layout(node).unwrap().location.y, 0f32);
    assert_eq!(taffy.layout(node0).unwrap().size.width, 180f32);
    assert_eq!(taffy.layout(node0).unwrap().size.height, 220f32);
    assert_eq!(taffy.layout(node0).unwrap().location.x, 40f32);
    assert_eq!(taffy.layout(node0).unwrap().location.y, 80f32);
}
