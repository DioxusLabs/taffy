#[test]
fn flex_wrap_align_stretch_fits_one_row() {
    let mut stretch = stretch2::Stretch::new();
    let node0 = stretch
        .new_node(
            stretch2::style::Style {
                size: stretch2::geometry::Size { width: stretch2::style::Dimension::Points(50f32), ..Default::default() },
                ..Default::default()
            },
            &[],
        )
        .unwrap();
    let node1 = stretch
        .new_node(
            stretch2::style::Style {
                size: stretch2::geometry::Size { width: stretch2::style::Dimension::Points(50f32), ..Default::default() },
                ..Default::default()
            },
            &[],
        )
        .unwrap();
    let node = stretch
        .new_node(
            stretch2::style::Style {
                flex_wrap: stretch2::style::FlexWrap::Wrap,
                size: stretch2::geometry::Size {
                    width: stretch2::style::Dimension::Points(150f32),
                    height: stretch2::style::Dimension::Points(100f32),
                    ..Default::default()
                },
                ..Default::default()
            },
            &[node0, node1],
        )
        .unwrap();
    stretch.compute_layout(node, stretch2::geometry::Size::undefined()).unwrap();
    assert_eq!(stretch.layout(node).unwrap().size.width, 150f32);
    assert_eq!(stretch.layout(node).unwrap().size.height, 100f32);
    assert_eq!(stretch.layout(node).unwrap().location.x, 0f32);
    assert_eq!(stretch.layout(node).unwrap().location.y, 0f32);
    assert_eq!(stretch.layout(node0).unwrap().size.width, 50f32);
    assert_eq!(stretch.layout(node0).unwrap().size.height, 100f32);
    assert_eq!(stretch.layout(node0).unwrap().location.x, 0f32);
    assert_eq!(stretch.layout(node0).unwrap().location.y, 0f32);
    assert_eq!(stretch.layout(node1).unwrap().size.width, 50f32);
    assert_eq!(stretch.layout(node1).unwrap().size.height, 100f32);
    assert_eq!(stretch.layout(node1).unwrap().location.x, 50f32);
    assert_eq!(stretch.layout(node1).unwrap().location.y, 0f32);
}
