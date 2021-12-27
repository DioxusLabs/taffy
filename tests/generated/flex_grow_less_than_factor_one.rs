#[test]
fn flex_grow_less_than_factor_one() {
    let mut stretch = stretch2::Stretch::new();
    let node0 = stretch
        .new_node(
            stretch2::style::Style {
                flex_grow: 0.2f32,
                flex_shrink: 0f32,
                flex_basis: stretch2::style::Dimension::Points(40f32),
                ..Default::default()
            },
            &[],
        )
        .unwrap();
    let node1 = stretch
        .new_node(stretch2::style::Style { flex_grow: 0.2f32, flex_shrink: 0f32, ..Default::default() }, &[])
        .unwrap();
    let node2 = stretch
        .new_node(stretch2::style::Style { flex_grow: 0.4f32, flex_shrink: 0f32, ..Default::default() }, &[])
        .unwrap();
    let node = stretch
        .new_node(
            stretch2::style::Style {
                size: stretch2::geometry::Size {
                    width: stretch2::style::Dimension::Points(500f32),
                    height: stretch2::style::Dimension::Points(200f32),
                    ..Default::default()
                },
                ..Default::default()
            },
            &[node0, node1, node2],
        )
        .unwrap();
    stretch.compute_layout(node, stretch2::geometry::Size::undefined()).unwrap();
    assert_eq!(stretch.layout(node).unwrap().size.width, 500f32);
    assert_eq!(stretch.layout(node).unwrap().size.height, 200f32);
    assert_eq!(stretch.layout(node).unwrap().location.x, 0f32);
    assert_eq!(stretch.layout(node).unwrap().location.y, 0f32);
    assert_eq!(stretch.layout(node0).unwrap().size.width, 132f32);
    assert_eq!(stretch.layout(node0).unwrap().size.height, 200f32);
    assert_eq!(stretch.layout(node0).unwrap().location.x, 0f32);
    assert_eq!(stretch.layout(node0).unwrap().location.y, 0f32);
    assert_eq!(stretch.layout(node1).unwrap().size.width, 92f32);
    assert_eq!(stretch.layout(node1).unwrap().size.height, 200f32);
    assert_eq!(stretch.layout(node1).unwrap().location.x, 132f32);
    assert_eq!(stretch.layout(node1).unwrap().location.y, 0f32);
    assert_eq!(stretch.layout(node2).unwrap().size.width, 184f32);
    assert_eq!(stretch.layout(node2).unwrap().size.height, 200f32);
    assert_eq!(stretch.layout(node2).unwrap().location.x, 224f32);
    assert_eq!(stretch.layout(node2).unwrap().location.y, 0f32);
}
