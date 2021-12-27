#[test]
fn flex_basis_smaller_then_content_with_flex_grow_large_size() {
    let mut stretch = stretch2::Stretch::new();
    let node00 = stretch
        .new_node(
            stretch2::style::Style {
                size: stretch2::geometry::Size {
                    width: stretch2::style::Dimension::Points(70f32),
                    height: stretch2::style::Dimension::Points(100f32),
                    ..Default::default()
                },
                ..Default::default()
            },
            &[],
        )
        .unwrap();
    let node0 = stretch
        .new_node(
            stretch2::style::Style {
                flex_direction: stretch2::style::FlexDirection::Column,
                flex_grow: 1f32,
                flex_basis: stretch2::style::Dimension::Points(0f32),
                ..Default::default()
            },
            &[node00],
        )
        .unwrap();
    let node10 = stretch
        .new_node(
            stretch2::style::Style {
                size: stretch2::geometry::Size {
                    width: stretch2::style::Dimension::Points(20f32),
                    height: stretch2::style::Dimension::Points(100f32),
                    ..Default::default()
                },
                ..Default::default()
            },
            &[],
        )
        .unwrap();
    let node1 = stretch
        .new_node(
            stretch2::style::Style {
                flex_direction: stretch2::style::FlexDirection::Column,
                flex_grow: 1f32,
                flex_basis: stretch2::style::Dimension::Points(0f32),
                ..Default::default()
            },
            &[node10],
        )
        .unwrap();
    let node = stretch
        .new_node(
            stretch2::style::Style {
                size: stretch2::geometry::Size {
                    width: stretch2::style::Dimension::Points(100f32),
                    ..Default::default()
                },
                ..Default::default()
            },
            &[node0, node1],
        )
        .unwrap();
    stretch.compute_layout(node, stretch2::geometry::Size::undefined()).unwrap();
    assert_eq!(stretch.layout(node).unwrap().size.width, 100f32);
    assert_eq!(stretch.layout(node).unwrap().size.height, 100f32);
    assert_eq!(stretch.layout(node).unwrap().location.x, 0f32);
    assert_eq!(stretch.layout(node).unwrap().location.y, 0f32);
    assert_eq!(stretch.layout(node0).unwrap().size.width, 70f32);
    assert_eq!(stretch.layout(node0).unwrap().size.height, 100f32);
    assert_eq!(stretch.layout(node0).unwrap().location.x, 0f32);
    assert_eq!(stretch.layout(node0).unwrap().location.y, 0f32);
    assert_eq!(stretch.layout(node00).unwrap().size.width, 70f32);
    assert_eq!(stretch.layout(node00).unwrap().size.height, 100f32);
    assert_eq!(stretch.layout(node00).unwrap().location.x, 0f32);
    assert_eq!(stretch.layout(node00).unwrap().location.y, 0f32);
    assert_eq!(stretch.layout(node1).unwrap().size.width, 30f32);
    assert_eq!(stretch.layout(node1).unwrap().size.height, 100f32);
    assert_eq!(stretch.layout(node1).unwrap().location.x, 70f32);
    assert_eq!(stretch.layout(node1).unwrap().location.y, 0f32);
    assert_eq!(stretch.layout(node10).unwrap().size.width, 20f32);
    assert_eq!(stretch.layout(node10).unwrap().size.height, 100f32);
    assert_eq!(stretch.layout(node10).unwrap().location.x, 0f32);
    assert_eq!(stretch.layout(node10).unwrap().location.y, 0f32);
}
