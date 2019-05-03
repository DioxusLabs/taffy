#[test]
fn max_height_overrides_height_on_root() {
    let mut stretch = stretch::Stretch::new();
    let node = stretch.new_node(
        stretch::style::Style {
            size: stretch::geometry::Size { height: stretch::style::Dimension::Points(200f32), ..Default::default() },
            max_size: stretch::geometry::Size {
                height: stretch::style::Dimension::Points(100f32),
                ..Default::default()
            },
            ..Default::default()
        },
        vec![],
    );
    stretch.compute_layout(node, stretch::geometry::Size::undefined()).unwrap();
    assert_eq!(stretch.layout(node).size.width, 0f32);
    assert_eq!(stretch.layout(node).size.height, 100f32);
    assert_eq!(stretch.layout(node).location.x, 0f32);
    assert_eq!(stretch.layout(node).location.y, 0f32);
}
