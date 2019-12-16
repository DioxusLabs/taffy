#[test]
fn absolute_layout_child_order() {
    let mut stretch = stretch::Stretch::new();
    let node0 = stretch
        .new_node(
            stretch::style::Style {
                size: stretch::geometry::Size {
                    width: stretch::style::Dimension::Points(60f32),
                    height: stretch::style::Dimension::Points(40f32),
                    ..Default::default()
                },
                ..Default::default()
            },
            &[],
        )
        .unwrap();
    let node1 = stretch
        .new_node(
            stretch::style::Style {
                position_type: stretch::style::PositionType::Absolute,
                size: stretch::geometry::Size {
                    width: stretch::style::Dimension::Points(60f32),
                    height: stretch::style::Dimension::Points(40f32),
                    ..Default::default()
                },
                ..Default::default()
            },
            &[],
        )
        .unwrap();
    let node2 = stretch
        .new_node(
            stretch::style::Style {
                size: stretch::geometry::Size {
                    width: stretch::style::Dimension::Points(60f32),
                    height: stretch::style::Dimension::Points(40f32),
                    ..Default::default()
                },
                ..Default::default()
            },
            &[],
        )
        .unwrap();
    let node = stretch
        .new_node(
            stretch::style::Style {
                align_items: stretch::style::AlignItems::Center,
                justify_content: stretch::style::JustifyContent::Center,
                size: stretch::geometry::Size {
                    width: stretch::style::Dimension::Points(110f32),
                    height: stretch::style::Dimension::Points(100f32),
                    ..Default::default()
                },
                ..Default::default()
            },
            &[node0, node1, node2],
        )
        .unwrap();
    stretch.compute_layout(node, stretch::geometry::Size::undefined()).unwrap();
    assert_eq!(stretch.layout(node).unwrap().size.width, 110f32);
    assert_eq!(stretch.layout(node).unwrap().size.height, 100f32);
    assert_eq!(stretch.layout(node).unwrap().location.x, 0f32);
    assert_eq!(stretch.layout(node).unwrap().location.y, 0f32);
    assert_eq!(stretch.layout(node0).unwrap().size.width, 55f32);
    assert_eq!(stretch.layout(node0).unwrap().size.height, 40f32);
    assert_eq!(stretch.layout(node0).unwrap().location.x, 0f32);
    assert_eq!(stretch.layout(node0).unwrap().location.y, 30f32);
    assert_eq!(stretch.layout(node1).unwrap().size.width, 60f32);
    assert_eq!(stretch.layout(node1).unwrap().size.height, 40f32);
    assert_eq!(stretch.layout(node1).unwrap().location.x, 25f32);
    assert_eq!(stretch.layout(node1).unwrap().location.y, 30f32);
    assert_eq!(stretch.layout(node2).unwrap().size.width, 55f32);
    assert_eq!(stretch.layout(node2).unwrap().size.height, 40f32);
    assert_eq!(stretch.layout(node2).unwrap().location.x, 55f32);
    assert_eq!(stretch.layout(node2).unwrap().location.y, 30f32);
}
