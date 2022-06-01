#[test]
fn child_min_max_width_flexing() {
    let mut sprawl = sprawl::Sprawl::new();
    let node0 = sprawl
        .new_node(
            sprawl::style::Style {
                flex_grow: 1f32,
                flex_shrink: 0f32,
                flex_basis: sprawl::style::Dimension::Points(0f32),
                min_size: sprawl::geometry::Size {
                    width: sprawl::style::Dimension::Points(60f32),
                    ..Default::default()
                },
                ..Default::default()
            },
            &[],
        )
        .unwrap();
    let node1 = sprawl
        .new_node(
            sprawl::style::Style {
                flex_grow: 1f32,
                flex_shrink: 0f32,
                flex_basis: sprawl::style::Dimension::Percent(0.5f32),
                max_size: sprawl::geometry::Size {
                    width: sprawl::style::Dimension::Points(20f32),
                    ..Default::default()
                },
                ..Default::default()
            },
            &[],
        )
        .unwrap();
    let node = sprawl
        .new_node(
            sprawl::style::Style {
                size: sprawl::geometry::Size {
                    width: sprawl::style::Dimension::Points(120f32),
                    height: sprawl::style::Dimension::Points(50f32),
                    ..Default::default()
                },
                ..Default::default()
            },
            &[node0, node1],
        )
        .unwrap();
    sprawl.compute_layout(node, sprawl::geometry::Size::undefined()).unwrap();
    assert_eq!(sprawl.layout(node).unwrap().size.width, 120f32);
    assert_eq!(sprawl.layout(node).unwrap().size.height, 50f32);
    assert_eq!(sprawl.layout(node).unwrap().location.x, 0f32);
    assert_eq!(sprawl.layout(node).unwrap().location.y, 0f32);
    assert_eq!(sprawl.layout(node0).unwrap().size.width, 100f32);
    assert_eq!(sprawl.layout(node0).unwrap().size.height, 50f32);
    assert_eq!(sprawl.layout(node0).unwrap().location.x, 0f32);
    assert_eq!(sprawl.layout(node0).unwrap().location.y, 0f32);
    assert_eq!(sprawl.layout(node1).unwrap().size.width, 20f32);
    assert_eq!(sprawl.layout(node1).unwrap().size.height, 50f32);
    assert_eq!(sprawl.layout(node1).unwrap().location.x, 100f32);
    assert_eq!(sprawl.layout(node1).unwrap().location.y, 0f32);
}
