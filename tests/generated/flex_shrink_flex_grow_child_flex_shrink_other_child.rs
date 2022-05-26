#[test]
fn flex_shrink_flex_grow_child_flex_shrink_other_child() {
    let mut stretch = sprawl::Stretch::new();
    let node0 = stretch
        .new_node(
            sprawl::style::Style {
                flex_grow: 0f32,
                flex_shrink: 1f32,
                size: sprawl::geometry::Size {
                    width: sprawl::style::Dimension::Points(500f32),
                    height: sprawl::style::Dimension::Points(100f32),
                    ..Default::default()
                },
                ..Default::default()
            },
            &[],
        )
        .unwrap();
    let node1 = stretch
        .new_node(
            sprawl::style::Style {
                flex_grow: 1f32,
                flex_shrink: 1f32,
                size: sprawl::geometry::Size {
                    width: sprawl::style::Dimension::Points(500f32),
                    height: sprawl::style::Dimension::Points(100f32),
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
                    width: sprawl::style::Dimension::Points(500f32),
                    height: sprawl::style::Dimension::Points(500f32),
                    ..Default::default()
                },
                ..Default::default()
            },
            &[node0, node1],
        )
        .unwrap();
    stretch.compute_layout(node, sprawl::geometry::Size::undefined()).unwrap();
    assert_eq!(stretch.layout(node).unwrap().size.width, 500f32);
    assert_eq!(stretch.layout(node).unwrap().size.height, 500f32);
    assert_eq!(stretch.layout(node).unwrap().location.x, 0f32);
    assert_eq!(stretch.layout(node).unwrap().location.y, 0f32);
    assert_eq!(stretch.layout(node0).unwrap().size.width, 250f32);
    assert_eq!(stretch.layout(node0).unwrap().size.height, 100f32);
    assert_eq!(stretch.layout(node0).unwrap().location.x, 0f32);
    assert_eq!(stretch.layout(node0).unwrap().location.y, 0f32);
    assert_eq!(stretch.layout(node1).unwrap().size.width, 250f32);
    assert_eq!(stretch.layout(node1).unwrap().size.height, 100f32);
    assert_eq!(stretch.layout(node1).unwrap().location.x, 250f32);
    assert_eq!(stretch.layout(node1).unwrap().location.y, 0f32);
}
