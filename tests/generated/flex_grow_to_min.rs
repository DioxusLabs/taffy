#[test]
fn flex_grow_to_min() {
    let mut sprawl = sprawl::Sprawl::new();
    let node0 = sprawl
        .new_node(sprawl::style::FlexboxLayout { flex_grow: 1f32, flex_shrink: 1f32, ..Default::default() }, &[])
        .unwrap();
    let node1 = sprawl
        .new_node(
            sprawl::style::FlexboxLayout {
                size: sprawl::geometry::Size { height: sprawl::style::Dimension::Points(50f32), ..Default::default() },
                ..Default::default()
            },
            &[],
        )
        .unwrap();
    let node = sprawl
        .new_node(
            sprawl::style::FlexboxLayout {
                flex_direction: sprawl::style::FlexDirection::Column,
                size: sprawl::geometry::Size { width: sprawl::style::Dimension::Points(100f32), ..Default::default() },
                min_size: sprawl::geometry::Size {
                    height: sprawl::style::Dimension::Points(100f32),
                    ..Default::default()
                },
                max_size: sprawl::geometry::Size {
                    height: sprawl::style::Dimension::Points(500f32),
                    ..Default::default()
                },
                ..Default::default()
            },
            &[node0, node1],
        )
        .unwrap();
    sprawl.compute_layout(node, sprawl::geometry::Size::undefined()).unwrap();
    assert_eq!(sprawl.layout(node).unwrap().size.width, 100f32);
    assert_eq!(sprawl.layout(node).unwrap().size.height, 100f32);
    assert_eq!(sprawl.layout(node).unwrap().location.x, 0f32);
    assert_eq!(sprawl.layout(node).unwrap().location.y, 0f32);
    assert_eq!(sprawl.layout(node0).unwrap().size.width, 100f32);
    assert_eq!(sprawl.layout(node0).unwrap().size.height, 50f32);
    assert_eq!(sprawl.layout(node0).unwrap().location.x, 0f32);
    assert_eq!(sprawl.layout(node0).unwrap().location.y, 0f32);
    assert_eq!(sprawl.layout(node1).unwrap().size.width, 100f32);
    assert_eq!(sprawl.layout(node1).unwrap().size.height, 50f32);
    assert_eq!(sprawl.layout(node1).unwrap().location.x, 0f32);
    assert_eq!(sprawl.layout(node1).unwrap().location.y, 50f32);
}
