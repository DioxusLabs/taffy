#[test]
fn absolute_layout_align_items_center_on_child_only() {
    let mut stretch = sprawl::Stretch::new();
    let node0 = stretch
        .new_node(
            sprawl::style::Style {
                position_type: sprawl::style::PositionType::Absolute,
                align_self: sprawl::style::AlignSelf::Center,
                size: sprawl::geometry::Size {
                    width: sprawl::style::Dimension::Points(60f32),
                    height: sprawl::style::Dimension::Points(40f32),
                    ..Default::default()
                },
                ..Default::default()
            },
            &[],
        )
        .unwrap();
    let node = stretch
        .new_node(
            sprawl::style::Style {
                size: sprawl::geometry::Size {
                    width: sprawl::style::Dimension::Points(110f32),
                    height: sprawl::style::Dimension::Points(100f32),
                    ..Default::default()
                },
                ..Default::default()
            },
            &[node0],
        )
        .unwrap();
    stretch.compute_layout(node, sprawl::geometry::Size::undefined()).unwrap();
    assert_eq!(stretch.layout(node).unwrap().size.width, 110f32);
    assert_eq!(stretch.layout(node).unwrap().size.height, 100f32);
    assert_eq!(stretch.layout(node).unwrap().location.x, 0f32);
    assert_eq!(stretch.layout(node).unwrap().location.y, 0f32);
    assert_eq!(stretch.layout(node0).unwrap().size.width, 60f32);
    assert_eq!(stretch.layout(node0).unwrap().size.height, 40f32);
    assert_eq!(stretch.layout(node0).unwrap().location.x, 0f32);
    assert_eq!(stretch.layout(node0).unwrap().location.y, 30f32);
}
