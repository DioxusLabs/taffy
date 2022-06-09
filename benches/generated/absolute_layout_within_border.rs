pub fn compute() {
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
}
