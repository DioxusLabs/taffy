#[test]
fn flex_wrap_align_stretch_fits_one_row() {
    let mut stretch = stretch::Stretch::new();
    let node0 = stretch.new_node(
        stretch::style::Style {
            size: stretch::geometry::Size { width: stretch::style::Dimension::Points(50f32), ..Default::default() },
            ..Default::default()
        },
        vec![],
    );
    let node1 = stretch.new_node(
        stretch::style::Style {
            size: stretch::geometry::Size { width: stretch::style::Dimension::Points(50f32), ..Default::default() },
            ..Default::default()
        },
        vec![],
    );
    let node = stretch.new_node(
        stretch::style::Style {
            flex_wrap: stretch::style::FlexWrap::Wrap,
            size: stretch::geometry::Size {
                width: stretch::style::Dimension::Points(150f32),
                height: stretch::style::Dimension::Points(100f32),
                ..Default::default()
            },
            ..Default::default()
        },
        vec![node0, node1],
    );
    stretch.compute_layout(node, stretch::geometry::Size::undefined()).unwrap();
    assert_eq!(stretch.layout(node).size.width, 150f32);
    assert_eq!(stretch.layout(node).size.height, 100f32);
    assert_eq!(stretch.layout(node).location.x, 0f32);
    assert_eq!(stretch.layout(node).location.y, 0f32);
    assert_eq!(stretch.layout(node0).size.width, 50f32);
    assert_eq!(stretch.layout(node0).size.height, 100f32);
    assert_eq!(stretch.layout(node0).location.x, 0f32);
    assert_eq!(stretch.layout(node0).location.y, 0f32);
    assert_eq!(stretch.layout(node1).size.width, 50f32);
    assert_eq!(stretch.layout(node1).size.height, 100f32);
    assert_eq!(stretch.layout(node1).location.x, 50f32);
    assert_eq!(stretch.layout(node1).location.y, 0f32);
}
