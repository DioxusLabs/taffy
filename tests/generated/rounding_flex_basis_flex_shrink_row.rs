#[test]
fn rounding_flex_basis_flex_shrink_row() {
    let mut sprawl = sprawl::Sprawl::new();
    let node0 = sprawl
        .new_node(
            sprawl::style::Style {
                flex_shrink: 1f32,
                flex_basis: sprawl::style::Dimension::Points(100f32),
                ..Default::default()
            },
            &[],
        )
        .unwrap();
    let node1 = sprawl
        .new_node(
            sprawl::style::Style { flex_basis: sprawl::style::Dimension::Points(25f32), ..Default::default() },
            &[],
        )
        .unwrap();
    let node2 = sprawl
        .new_node(
            sprawl::style::Style { flex_basis: sprawl::style::Dimension::Points(25f32), ..Default::default() },
            &[],
        )
        .unwrap();
    let node = sprawl
        .new_node(
            sprawl::style::Style {
                size: sprawl::geometry::Size {
                    width: sprawl::style::Dimension::Points(101f32),
                    height: sprawl::style::Dimension::Points(100f32),
                    ..Default::default()
                },
                ..Default::default()
            },
            &[node0, node1, node2],
        )
        .unwrap();
    sprawl.compute_layout(node, sprawl::geometry::Size::undefined()).unwrap();
    assert_eq!(sprawl.layout(node).unwrap().size.width, 101f32);
    assert_eq!(sprawl.layout(node).unwrap().size.height, 100f32);
    assert_eq!(sprawl.layout(node).unwrap().location.x, 0f32);
    assert_eq!(sprawl.layout(node).unwrap().location.y, 0f32);
    assert_eq!(sprawl.layout(node0).unwrap().size.width, 67f32);
    assert_eq!(sprawl.layout(node0).unwrap().size.height, 100f32);
    assert_eq!(sprawl.layout(node0).unwrap().location.x, 0f32);
    assert_eq!(sprawl.layout(node0).unwrap().location.y, 0f32);
    assert_eq!(sprawl.layout(node1).unwrap().size.width, 17f32);
    assert_eq!(sprawl.layout(node1).unwrap().size.height, 100f32);
    assert_eq!(sprawl.layout(node1).unwrap().location.x, 67f32);
    assert_eq!(sprawl.layout(node1).unwrap().location.y, 0f32);
    assert_eq!(sprawl.layout(node2).unwrap().size.width, 17f32);
    assert_eq!(sprawl.layout(node2).unwrap().size.height, 100f32);
    assert_eq!(sprawl.layout(node2).unwrap().location.x, 84f32);
    assert_eq!(sprawl.layout(node2).unwrap().location.y, 0f32);
}
