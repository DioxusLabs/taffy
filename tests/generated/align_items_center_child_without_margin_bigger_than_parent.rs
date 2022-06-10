#[test]
fn align_items_center_child_without_margin_bigger_than_parent() {
    let mut taffy = taffy::Taffy::new();
    let node00 = taffy
        .new_node(
            taffy::style::FlexboxLayout {
                size: taffy::geometry::Size {
                    width: taffy::style::Dimension::Points(70f32),
                    height: taffy::style::Dimension::Points(70f32),
                    ..Default::default()
                },
                ..Default::default()
            },
            &[],
        )
        .unwrap();
    let node0 = taffy
        .new_node(
            taffy::style::FlexboxLayout { align_items: taffy::style::AlignItems::Center, ..Default::default() },
            &[node00],
        )
        .unwrap();
    let node = taffy
        .new_node(
            taffy::style::FlexboxLayout {
                align_items: taffy::style::AlignItems::Center,
                justify_content: taffy::style::JustifyContent::Center,
                size: taffy::geometry::Size {
                    width: taffy::style::Dimension::Points(50f32),
                    height: taffy::style::Dimension::Points(50f32),
                    ..Default::default()
                },
                ..Default::default()
            },
            &[node0],
        )
        .unwrap();
    taffy.compute_layout(node, taffy::geometry::Size::undefined()).unwrap();
    assert_eq!(taffy.layout(node).unwrap().size.width, 50f32);
    assert_eq!(taffy.layout(node).unwrap().size.height, 50f32);
    assert_eq!(taffy.layout(node).unwrap().location.x, 0f32);
    assert_eq!(taffy.layout(node).unwrap().location.y, 0f32);
    assert_eq!(taffy.layout(node0).unwrap().size.width, 70f32);
    assert_eq!(taffy.layout(node0).unwrap().size.height, 70f32);
    assert_eq!(taffy.layout(node0).unwrap().location.x, -10f32);
    assert_eq!(taffy.layout(node0).unwrap().location.y, -10f32);
    assert_eq!(taffy.layout(node00).unwrap().size.width, 70f32);
    assert_eq!(taffy.layout(node00).unwrap().size.height, 70f32);
    assert_eq!(taffy.layout(node00).unwrap().location.x, 0f32);
    assert_eq!(taffy.layout(node00).unwrap().location.y, 0f32);
}
