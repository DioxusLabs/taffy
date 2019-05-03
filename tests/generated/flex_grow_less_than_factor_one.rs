#[test]
fn flex_grow_less_than_factor_one() {
    let mut stretch = stretch::Stretch::new();
    let node0 = stretch.new_node(
        stretch::style::Style {
            flex_grow: 0.2f32,
            flex_shrink: 0f32,
            flex_basis: stretch::style::Dimension::Points(40f32),
            ..Default::default()
        },
        vec![],
    );
    let node1 =
        stretch.new_node(stretch::style::Style { flex_grow: 0.2f32, flex_shrink: 0f32, ..Default::default() }, vec![]);
    let node2 =
        stretch.new_node(stretch::style::Style { flex_grow: 0.4f32, flex_shrink: 0f32, ..Default::default() }, vec![]);
    let node = stretch.new_node(
        stretch::style::Style {
            size: stretch::geometry::Size {
                width: stretch::style::Dimension::Points(500f32),
                height: stretch::style::Dimension::Points(200f32),
                ..Default::default()
            },
            ..Default::default()
        },
        vec![node0, node1, node2],
    );
    stretch.compute_layout(node, stretch::geometry::Size::undefined()).unwrap();
    assert_eq!(stretch.layout(node).size.width, 500f32);
    assert_eq!(stretch.layout(node).size.height, 200f32);
    assert_eq!(stretch.layout(node).location.x, 0f32);
    assert_eq!(stretch.layout(node).location.y, 0f32);
    assert_eq!(stretch.layout(node0).size.width, 132f32);
    assert_eq!(stretch.layout(node0).size.height, 200f32);
    assert_eq!(stretch.layout(node0).location.x, 0f32);
    assert_eq!(stretch.layout(node0).location.y, 0f32);
    assert_eq!(stretch.layout(node1).size.width, 92f32);
    assert_eq!(stretch.layout(node1).size.height, 200f32);
    assert_eq!(stretch.layout(node1).location.x, 132f32);
    assert_eq!(stretch.layout(node1).location.y, 0f32);
    assert_eq!(stretch.layout(node2).size.width, 184f32);
    assert_eq!(stretch.layout(node2).size.height, 200f32);
    assert_eq!(stretch.layout(node2).location.x, 224f32);
    assert_eq!(stretch.layout(node2).location.y, 0f32);
}
