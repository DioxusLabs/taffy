#[test]
fn wrap_reverse_row_single_line_different_size() {
    let mut stretch = stretch::Stretch::new();
    let node0 = stretch
        .new_node(
            stretch::style::Style {
                size: stretch::geometry::Size {
                    width: stretch::style::Dimension::Points(30f32),
                    height: stretch::style::Dimension::Points(10f32),
                    ..Default::default()
                },
                ..Default::default()
            },
            &[],
        )
        .unwrap();
    let node1 = stretch
        .new_node(
            stretch::style::Style {
                size: stretch::geometry::Size {
                    width: stretch::style::Dimension::Points(30f32),
                    height: stretch::style::Dimension::Points(20f32),
                    ..Default::default()
                },
                ..Default::default()
            },
            &[],
        )
        .unwrap();
    let node2 = stretch
        .new_node(
            stretch::style::Style {
                size: stretch::geometry::Size {
                    width: stretch::style::Dimension::Points(30f32),
                    height: stretch::style::Dimension::Points(30f32),
                    ..Default::default()
                },
                ..Default::default()
            },
            &[],
        )
        .unwrap();
    let node3 = stretch
        .new_node(
            stretch::style::Style {
                size: stretch::geometry::Size {
                    width: stretch::style::Dimension::Points(30f32),
                    height: stretch::style::Dimension::Points(40f32),
                    ..Default::default()
                },
                ..Default::default()
            },
            &[],
        )
        .unwrap();
    let node4 = stretch
        .new_node(
            stretch::style::Style {
                size: stretch::geometry::Size {
                    width: stretch::style::Dimension::Points(30f32),
                    height: stretch::style::Dimension::Points(50f32),
                    ..Default::default()
                },
                ..Default::default()
            },
            &[],
        )
        .unwrap();
    let node = stretch
        .new_node(
            stretch::style::Style {
                flex_wrap: stretch::style::FlexWrap::WrapReverse,
                align_content: stretch::style::AlignContent::FlexStart,
                size: stretch::geometry::Size {
                    width: stretch::style::Dimension::Points(300f32),
                    ..Default::default()
                },
                ..Default::default()
            },
            &[node0, node1, node2, node3, node4],
        )
        .unwrap();
    stretch.compute_layout(node, stretch::geometry::Size::undefined()).unwrap();
    assert_eq!(stretch.layout(node).unwrap().size.width, 300f32);
    assert_eq!(stretch.layout(node).unwrap().size.height, 50f32);
    assert_eq!(stretch.layout(node).unwrap().location.x, 0f32);
    assert_eq!(stretch.layout(node).unwrap().location.y, 0f32);
    assert_eq!(stretch.layout(node0).unwrap().size.width, 30f32);
    assert_eq!(stretch.layout(node0).unwrap().size.height, 10f32);
    assert_eq!(stretch.layout(node0).unwrap().location.x, 0f32);
    assert_eq!(stretch.layout(node0).unwrap().location.y, 40f32);
    assert_eq!(stretch.layout(node1).unwrap().size.width, 30f32);
    assert_eq!(stretch.layout(node1).unwrap().size.height, 20f32);
    assert_eq!(stretch.layout(node1).unwrap().location.x, 30f32);
    assert_eq!(stretch.layout(node1).unwrap().location.y, 30f32);
    assert_eq!(stretch.layout(node2).unwrap().size.width, 30f32);
    assert_eq!(stretch.layout(node2).unwrap().size.height, 30f32);
    assert_eq!(stretch.layout(node2).unwrap().location.x, 60f32);
    assert_eq!(stretch.layout(node2).unwrap().location.y, 20f32);
    assert_eq!(stretch.layout(node3).unwrap().size.width, 30f32);
    assert_eq!(stretch.layout(node3).unwrap().size.height, 40f32);
    assert_eq!(stretch.layout(node3).unwrap().location.x, 90f32);
    assert_eq!(stretch.layout(node3).unwrap().location.y, 10f32);
    assert_eq!(stretch.layout(node4).unwrap().size.width, 30f32);
    assert_eq!(stretch.layout(node4).unwrap().size.height, 50f32);
    assert_eq!(stretch.layout(node4).unwrap().location.x, 120f32);
    assert_eq!(stretch.layout(node4).unwrap().location.y, 0f32);
}
