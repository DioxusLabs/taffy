#[test]
fn percentage_flex_basis_main_max_height() {
    let mut stretch = stretch::Stretch::new();
    let node0 = stretch.new_node(
        stretch::style::Style {
            flex_grow: 1f32,
            flex_basis: stretch::style::Dimension::Percent(0.1f32),
            max_size: stretch::geometry::Size {
                height: stretch::style::Dimension::Percent(0.6f32),
                ..Default::default()
            },
            ..Default::default()
        },
        vec![],
    );
    let node1 = stretch.new_node(
        stretch::style::Style {
            flex_grow: 4f32,
            flex_basis: stretch::style::Dimension::Percent(0.1f32),
            max_size: stretch::geometry::Size {
                height: stretch::style::Dimension::Percent(0.2f32),
                ..Default::default()
            },
            ..Default::default()
        },
        vec![],
    );
    let node = stretch.new_node(
        stretch::style::Style {
            size: stretch::geometry::Size {
                width: stretch::style::Dimension::Points(200f32),
                height: stretch::style::Dimension::Points(400f32),
                ..Default::default()
            },
            ..Default::default()
        },
        vec![node0, node1],
    );
    stretch.compute_layout(node, stretch::geometry::Size::undefined()).unwrap();
    assert_eq!(stretch.layout(node).size.width, 200f32);
    assert_eq!(stretch.layout(node).size.height, 400f32);
    assert_eq!(stretch.layout(node).location.x, 0f32);
    assert_eq!(stretch.layout(node).location.y, 0f32);
    assert_eq!(stretch.layout(node0).size.width, 52f32);
    assert_eq!(stretch.layout(node0).size.height, 240f32);
    assert_eq!(stretch.layout(node0).location.x, 0f32);
    assert_eq!(stretch.layout(node0).location.y, 0f32);
    assert_eq!(stretch.layout(node1).size.width, 148f32);
    assert_eq!(stretch.layout(node1).size.height, 80f32);
    assert_eq!(stretch.layout(node1).location.x, 52f32);
    assert_eq!(stretch.layout(node1).location.y, 0f32);
}
