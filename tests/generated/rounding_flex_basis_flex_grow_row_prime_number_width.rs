#[test]
fn rounding_flex_basis_flex_grow_row_prime_number_width() {
    let mut sprawl = sprawl::Sprawl::new();
    let node0 = sprawl.new_node(sprawl::style::FlexboxLayout { flex_grow: 1f32, ..Default::default() }, &[]).unwrap();
    let node1 = sprawl.new_node(sprawl::style::FlexboxLayout { flex_grow: 1f32, ..Default::default() }, &[]).unwrap();
    let node2 = sprawl.new_node(sprawl::style::FlexboxLayout { flex_grow: 1f32, ..Default::default() }, &[]).unwrap();
    let node3 = sprawl.new_node(sprawl::style::FlexboxLayout { flex_grow: 1f32, ..Default::default() }, &[]).unwrap();
    let node4 = sprawl.new_node(sprawl::style::FlexboxLayout { flex_grow: 1f32, ..Default::default() }, &[]).unwrap();
    let node = sprawl
        .new_node(
            sprawl::style::FlexboxLayout {
                size: sprawl::geometry::Size {
                    width: sprawl::style::Dimension::Points(113f32),
                    height: sprawl::style::Dimension::Points(100f32),
                    ..Default::default()
                },
                ..Default::default()
            },
            &[node0, node1, node2, node3, node4],
        )
        .unwrap();
    sprawl.compute_layout(node, sprawl::geometry::Size::undefined()).unwrap();
    assert_eq!(sprawl.layout(node).unwrap().size.width, 113f32);
    assert_eq!(sprawl.layout(node).unwrap().size.height, 100f32);
    assert_eq!(sprawl.layout(node).unwrap().location.x, 0f32);
    assert_eq!(sprawl.layout(node).unwrap().location.y, 0f32);
    assert_eq!(sprawl.layout(node0).unwrap().size.width, 23f32);
    assert_eq!(sprawl.layout(node0).unwrap().size.height, 100f32);
    assert_eq!(sprawl.layout(node0).unwrap().location.x, 0f32);
    assert_eq!(sprawl.layout(node0).unwrap().location.y, 0f32);
    assert_eq!(sprawl.layout(node1).unwrap().size.width, 22f32);
    assert_eq!(sprawl.layout(node1).unwrap().size.height, 100f32);
    assert_eq!(sprawl.layout(node1).unwrap().location.x, 23f32);
    assert_eq!(sprawl.layout(node1).unwrap().location.y, 0f32);
    assert_eq!(sprawl.layout(node2).unwrap().size.width, 23f32);
    assert_eq!(sprawl.layout(node2).unwrap().size.height, 100f32);
    assert_eq!(sprawl.layout(node2).unwrap().location.x, 45f32);
    assert_eq!(sprawl.layout(node2).unwrap().location.y, 0f32);
    assert_eq!(sprawl.layout(node3).unwrap().size.width, 22f32);
    assert_eq!(sprawl.layout(node3).unwrap().size.height, 100f32);
    assert_eq!(sprawl.layout(node3).unwrap().location.x, 68f32);
    assert_eq!(sprawl.layout(node3).unwrap().location.y, 0f32);
    assert_eq!(sprawl.layout(node4).unwrap().size.width, 23f32);
    assert_eq!(sprawl.layout(node4).unwrap().size.height, 100f32);
    assert_eq!(sprawl.layout(node4).unwrap().location.x, 90f32);
    assert_eq!(sprawl.layout(node4).unwrap().location.y, 0f32);
}
