#[test]
fn flex_grow_flex_basis_percent_min_max() {
    let mut taffy = taffy::Taffy::new();
    let node0 = taffy
        .new_with_children(
            taffy::style::FlexboxLayout {
                flex_grow: 1f32,
                flex_shrink: 0f32,
                flex_basis: taffy::style::Dimension::Points(0f32),
                size: taffy::geometry::Size {
                    height: Some(taffy::style::Dimension::Points(20f32)),
                    ..Default::default()
                },
                min_size: taffy::geometry::Size { width: taffy::style::Dimension::Points(60f32), ..Default::default() },
                ..Default::default()
            },
            &[],
        )
        .unwrap();
    let node1 = taffy
        .new_with_children(
            taffy::style::FlexboxLayout {
                flex_grow: 1f32,
                flex_shrink: 0f32,
                flex_basis: taffy::style::Dimension::Percent(0.5f32),
                size: taffy::geometry::Size {
                    width: Some(taffy::style::Dimension::Points(20f32)),
                    height: Some(taffy::style::Dimension::Points(20f32)),
                    ..Default::default()
                },
                max_size: taffy::geometry::Size { width: taffy::style::Dimension::Points(20f32), ..Default::default() },
                ..Default::default()
            },
            &[],
        )
        .unwrap();
    let node = taffy
        .new_with_children(
            taffy::style::FlexboxLayout {
                size: taffy::geometry::Size {
                    width: Some(taffy::style::Dimension::Points(120f32)),
                    ..Default::default()
                },
                ..Default::default()
            },
            &[node0, node1],
        )
        .unwrap();
    taffy.compute_layout(node, taffy::geometry::Size::undefined()).unwrap();
    assert_eq!(taffy.layout(node).unwrap().size.width, 120f32);
    assert_eq!(taffy.layout(node).unwrap().size.height, 20f32);
    assert_eq!(taffy.layout(node).unwrap().location.x, 0f32);
    assert_eq!(taffy.layout(node).unwrap().location.y, 0f32);
    assert_eq!(taffy.layout(node0).unwrap().size.width, 100f32);
    assert_eq!(taffy.layout(node0).unwrap().size.height, 20f32);
    assert_eq!(taffy.layout(node0).unwrap().location.x, 0f32);
    assert_eq!(taffy.layout(node0).unwrap().location.y, 0f32);
    assert_eq!(taffy.layout(node1).unwrap().size.width, 20f32);
    assert_eq!(taffy.layout(node1).unwrap().size.height, 20f32);
    assert_eq!(taffy.layout(node1).unwrap().location.x, 100f32);
    assert_eq!(taffy.layout(node1).unwrap().location.y, 0f32);
}
