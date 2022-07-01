#[test]
fn absolute_layout_child_order() {
    let mut taffy = taffy::Taffy::new();
    let node0 = taffy.new_with_children(
        taffy::style::FlexboxLayout {
            size: taffy::geometry::Size {
                width: taffy::style::Dimension::Points(60f32),
                height: taffy::style::Dimension::Points(40f32),
                ..Default::default()
            },
            ..Default::default()
        },
        &[],
    );
    let node1 = taffy.new_with_children(
        taffy::style::FlexboxLayout {
            position_type: taffy::style::PositionType::Absolute,
            size: taffy::geometry::Size {
                width: taffy::style::Dimension::Points(60f32),
                height: taffy::style::Dimension::Points(40f32),
                ..Default::default()
            },
            ..Default::default()
        },
        &[],
    );
    let node2 = taffy.new_with_children(
        taffy::style::FlexboxLayout {
            size: taffy::geometry::Size {
                width: taffy::style::Dimension::Points(60f32),
                height: taffy::style::Dimension::Points(40f32),
                ..Default::default()
            },
            ..Default::default()
        },
        &[],
    );
    let node = taffy.new_with_children(
        taffy::style::FlexboxLayout {
            align_items: taffy::style::AlignItems::Center,
            justify_content: taffy::style::JustifyContent::Center,
            size: taffy::geometry::Size {
                width: taffy::style::Dimension::Points(110f32),
                height: taffy::style::Dimension::Points(100f32),
                ..Default::default()
            },
            ..Default::default()
        },
        &[node0, node1, node2],
    );
    taffy.compute_layout(node, taffy::geometry::Size::undefined()).unwrap();
    assert_eq!(taffy.layout(node).unwrap().size.width, 110f32);
    assert_eq!(taffy.layout(node).unwrap().size.height, 100f32);
    assert_eq!(taffy.layout(node).unwrap().location.x, 0f32);
    assert_eq!(taffy.layout(node).unwrap().location.y, 0f32);
    assert_eq!(taffy.layout(node0).unwrap().size.width, 55f32);
    assert_eq!(taffy.layout(node0).unwrap().size.height, 40f32);
    assert_eq!(taffy.layout(node0).unwrap().location.x, 0f32);
    assert_eq!(taffy.layout(node0).unwrap().location.y, 30f32);
    assert_eq!(taffy.layout(node1).unwrap().size.width, 60f32);
    assert_eq!(taffy.layout(node1).unwrap().size.height, 40f32);
    assert_eq!(taffy.layout(node1).unwrap().location.x, 25f32);
    assert_eq!(taffy.layout(node1).unwrap().location.y, 30f32);
    assert_eq!(taffy.layout(node2).unwrap().size.width, 55f32);
    assert_eq!(taffy.layout(node2).unwrap().size.height, 40f32);
    assert_eq!(taffy.layout(node2).unwrap().location.x, 55f32);
    assert_eq!(taffy.layout(node2).unwrap().location.y, 30f32);
}
