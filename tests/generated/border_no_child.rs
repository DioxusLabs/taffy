#[test]
fn border_no_child() {
    let mut stretch = stretch2::Stretch::new();
    let node = stretch
        .new_node(
            stretch2::style::Style {
                border: stretch2::geometry::Rect {
                    start: stretch2::style::Dimension::Points(10f32),
                    end: stretch2::style::Dimension::Points(10f32),
                    top: stretch2::style::Dimension::Points(10f32),
                    bottom: stretch2::style::Dimension::Points(10f32),
                    ..Default::default()
                },
                ..Default::default()
            },
            &[],
        )
        .unwrap();
    stretch.compute_layout(node, stretch2::geometry::Size::undefined()).unwrap();
    assert_eq!(stretch.layout(node).unwrap().size.width, 20f32);
    assert_eq!(stretch.layout(node).unwrap().size.height, 20f32);
    assert_eq!(stretch.layout(node).unwrap().location.x, 0f32);
    assert_eq!(stretch.layout(node).unwrap().location.y, 0f32);
}
