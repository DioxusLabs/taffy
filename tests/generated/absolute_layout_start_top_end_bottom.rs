#[test]
fn absolute_layout_start_top_end_bottom() {
    let mut sprawl = sprawl::Sprawl::new();
    let node0 = sprawl
        .new_node(
            sprawl::style::Style {
                position_type: sprawl::style::PositionType::Absolute,
                position: sprawl::geometry::Rect {
                    main_start: sprawl::style::Dimension::Points(10f32),
                    main_end: sprawl::style::Dimension::Points(10f32),
                    cross_start: sprawl::style::Dimension::Points(10f32),
                    cross_end: sprawl::style::Dimension::Points(10f32),
                    ..Default::default()
                },
                ..Default::default()
            },
            &[],
        )
        .unwrap();
    let node = sprawl
        .new_node(
            sprawl::style::Style {
                size: sprawl::geometry::Size {
                    width: sprawl::style::Dimension::Points(100f32),
                    height: sprawl::style::Dimension::Points(100f32),
                    ..Default::default()
                },
                ..Default::default()
            },
            &[node0],
        )
        .unwrap();
    sprawl.compute_layout(node, sprawl::geometry::Size::undefined()).unwrap();
    assert_eq!(sprawl.layout(node).unwrap().size.width, 100f32);
    assert_eq!(sprawl.layout(node).unwrap().size.height, 100f32);
    assert_eq!(sprawl.layout(node).unwrap().location.x, 0f32);
    assert_eq!(sprawl.layout(node).unwrap().location.y, 0f32);
    assert_eq!(sprawl.layout(node0).unwrap().size.width, 80f32);
    assert_eq!(sprawl.layout(node0).unwrap().size.height, 80f32);
    assert_eq!(sprawl.layout(node0).unwrap().location.x, 10f32);
    assert_eq!(sprawl.layout(node0).unwrap().location.y, 10f32);
}
