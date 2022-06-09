#[test]
fn flex_grow_less_than_factor_one() {
    let mut sprawl = sprawl::Sprawl::new();
    let node0 = sprawl
        .new_with_children(
            sprawl::style::Style {
                flex_grow: 0.2f32,
                flex_shrink: 0f32,
                flex_basis: sprawl::style::Dimension::Points(40f32),
                ..Default::default()
            },
            &[],
        )
        .unwrap();
    let node1 = sprawl
        .new_with_children(sprawl::style::Style { flex_grow: 0.2f32, flex_shrink: 0f32, ..Default::default() }, &[])
        .unwrap();
    let node2 = sprawl
        .new_with_children(sprawl::style::Style { flex_grow: 0.4f32, flex_shrink: 0f32, ..Default::default() }, &[])
        .unwrap();
    let node = sprawl
        .new_with_children(
            sprawl::style::Style {
                size: sprawl::geometry::Size {
                    width: sprawl::style::Dimension::Points(500f32),
                    height: sprawl::style::Dimension::Points(200f32),
                    ..Default::default()
                },
                ..Default::default()
            },
            &[node0, node1, node2],
        )
        .unwrap();
    sprawl.compute_layout(node, sprawl::geometry::Size::undefined()).unwrap();
    assert_eq!(sprawl.layout(node).unwrap().size.width, 500f32);
    assert_eq!(sprawl.layout(node).unwrap().size.height, 200f32);
    assert_eq!(sprawl.layout(node).unwrap().location.x, 0f32);
    assert_eq!(sprawl.layout(node).unwrap().location.y, 0f32);
    assert_eq!(sprawl.layout(node0).unwrap().size.width, 132f32);
    assert_eq!(sprawl.layout(node0).unwrap().size.height, 200f32);
    assert_eq!(sprawl.layout(node0).unwrap().location.x, 0f32);
    assert_eq!(sprawl.layout(node0).unwrap().location.y, 0f32);
    assert_eq!(sprawl.layout(node1).unwrap().size.width, 92f32);
    assert_eq!(sprawl.layout(node1).unwrap().size.height, 200f32);
    assert_eq!(sprawl.layout(node1).unwrap().location.x, 132f32);
    assert_eq!(sprawl.layout(node1).unwrap().location.y, 0f32);
    assert_eq!(sprawl.layout(node2).unwrap().size.width, 184f32);
    assert_eq!(sprawl.layout(node2).unwrap().size.height, 200f32);
    assert_eq!(sprawl.layout(node2).unwrap().location.x, 224f32);
    assert_eq!(sprawl.layout(node2).unwrap().location.y, 0f32);
}
