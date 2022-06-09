#[test]
fn absolute_layout_within_border() {
    let mut sprawl = sprawl::Sprawl::new();
    let node0 = sprawl
        .new_node(
            sprawl::style::Style {
                position_type: sprawl::style::PositionType::Absolute,
                size: sprawl::geometry::Size {
                    width: sprawl::style::Dimension::Points(50f32),
                    height: sprawl::style::Dimension::Points(50f32),
                    ..Default::default()
                },
                position: sprawl::geometry::Rect {
                    main_start: sprawl::style::Dimension::Points(0f32),
                    cross_start: sprawl::style::Dimension::Points(0f32),
                    ..Default::default()
                },
                ..Default::default()
            },
            &[],
        )
        .unwrap();
    let node1 = sprawl
        .new_node(
            sprawl::style::Style {
                position_type: sprawl::style::PositionType::Absolute,
                size: sprawl::geometry::Size {
                    width: sprawl::style::Dimension::Points(50f32),
                    height: sprawl::style::Dimension::Points(50f32),
                    ..Default::default()
                },
                position: sprawl::geometry::Rect {
                    main_end: sprawl::style::Dimension::Points(0f32),
                    cross_end: sprawl::style::Dimension::Points(0f32),
                    ..Default::default()
                },
                ..Default::default()
            },
            &[],
        )
        .unwrap();
    let node2 = sprawl
        .new_node(
            sprawl::style::Style {
                position_type: sprawl::style::PositionType::Absolute,
                size: sprawl::geometry::Size {
                    width: sprawl::style::Dimension::Points(50f32),
                    height: sprawl::style::Dimension::Points(50f32),
                    ..Default::default()
                },
                margin: sprawl::geometry::Rect {
                    main_start: sprawl::style::Dimension::Points(10f32),
                    main_end: sprawl::style::Dimension::Points(10f32),
                    cross_start: sprawl::style::Dimension::Points(10f32),
                    cross_end: sprawl::style::Dimension::Points(10f32),
                    ..Default::default()
                },
                position: sprawl::geometry::Rect {
                    main_start: sprawl::style::Dimension::Points(0f32),
                    cross_start: sprawl::style::Dimension::Points(0f32),
                    ..Default::default()
                },
                ..Default::default()
            },
            &[],
        )
        .unwrap();
    let node3 = sprawl
        .new_node(
            sprawl::style::Style {
                position_type: sprawl::style::PositionType::Absolute,
                size: sprawl::geometry::Size {
                    width: sprawl::style::Dimension::Points(50f32),
                    height: sprawl::style::Dimension::Points(50f32),
                    ..Default::default()
                },
                margin: sprawl::geometry::Rect {
                    main_start: sprawl::style::Dimension::Points(10f32),
                    main_end: sprawl::style::Dimension::Points(10f32),
                    cross_start: sprawl::style::Dimension::Points(10f32),
                    cross_end: sprawl::style::Dimension::Points(10f32),
                    ..Default::default()
                },
                position: sprawl::geometry::Rect {
                    main_end: sprawl::style::Dimension::Points(0f32),
                    cross_end: sprawl::style::Dimension::Points(0f32),
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
                padding: sprawl::geometry::Rect {
                    main_start: sprawl::style::Dimension::Points(10f32),
                    main_end: sprawl::style::Dimension::Points(10f32),
                    cross_start: sprawl::style::Dimension::Points(10f32),
                    cross_end: sprawl::style::Dimension::Points(10f32),
                    ..Default::default()
                },
                border: sprawl::geometry::Rect {
                    main_start: sprawl::style::Dimension::Points(10f32),
                    main_end: sprawl::style::Dimension::Points(10f32),
                    cross_start: sprawl::style::Dimension::Points(10f32),
                    cross_end: sprawl::style::Dimension::Points(10f32),
                    ..Default::default()
                },
                ..Default::default()
            },
            &[node0, node1, node2, node3],
        )
        .unwrap();
    sprawl.compute_layout(node, sprawl::geometry::Size::undefined()).unwrap();
    assert_eq!(sprawl.layout(node).unwrap().size.width, 100f32);
    assert_eq!(sprawl.layout(node).unwrap().size.height, 100f32);
    assert_eq!(sprawl.layout(node).unwrap().location.x, 0f32);
    assert_eq!(sprawl.layout(node).unwrap().location.y, 0f32);
    assert_eq!(sprawl.layout(node0).unwrap().size.width, 50f32);
    assert_eq!(sprawl.layout(node0).unwrap().size.height, 50f32);
    assert_eq!(sprawl.layout(node0).unwrap().location.x, 10f32);
    assert_eq!(sprawl.layout(node0).unwrap().location.y, 10f32);
    assert_eq!(sprawl.layout(node1).unwrap().size.width, 50f32);
    assert_eq!(sprawl.layout(node1).unwrap().size.height, 50f32);
    assert_eq!(sprawl.layout(node1).unwrap().location.x, 40f32);
    assert_eq!(sprawl.layout(node1).unwrap().location.y, 40f32);
    assert_eq!(sprawl.layout(node2).unwrap().size.width, 50f32);
    assert_eq!(sprawl.layout(node2).unwrap().size.height, 50f32);
    assert_eq!(sprawl.layout(node2).unwrap().location.x, 20f32);
    assert_eq!(sprawl.layout(node2).unwrap().location.y, 20f32);
    assert_eq!(sprawl.layout(node3).unwrap().size.width, 50f32);
    assert_eq!(sprawl.layout(node3).unwrap().size.height, 50f32);
    assert_eq!(sprawl.layout(node3).unwrap().location.x, 30f32);
    assert_eq!(sprawl.layout(node3).unwrap().location.y, 30f32);
}
