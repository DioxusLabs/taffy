#[test]
fn absolute_layout_within_border() {
    let mut stretch = stretch::Stretch::new();
    let node0 = stretch.new_node(
        stretch::style::Style {
            position_type: stretch::style::PositionType::Absolute,
            size: stretch::geometry::Size {
                width: stretch::style::Dimension::Points(50f32),
                height: stretch::style::Dimension::Points(50f32),
                ..Default::default()
            },
            position: stretch::geometry::Rect {
                start: stretch::style::Dimension::Points(0f32),
                top: stretch::style::Dimension::Points(0f32),
                ..Default::default()
            },
            ..Default::default()
        },
        vec![],
    );
    let node1 = stretch.new_node(
        stretch::style::Style {
            position_type: stretch::style::PositionType::Absolute,
            size: stretch::geometry::Size {
                width: stretch::style::Dimension::Points(50f32),
                height: stretch::style::Dimension::Points(50f32),
                ..Default::default()
            },
            position: stretch::geometry::Rect {
                end: stretch::style::Dimension::Points(0f32),
                bottom: stretch::style::Dimension::Points(0f32),
                ..Default::default()
            },
            ..Default::default()
        },
        vec![],
    );
    let node2 = stretch.new_node(
        stretch::style::Style {
            position_type: stretch::style::PositionType::Absolute,
            size: stretch::geometry::Size {
                width: stretch::style::Dimension::Points(50f32),
                height: stretch::style::Dimension::Points(50f32),
                ..Default::default()
            },
            margin: stretch::geometry::Rect {
                start: stretch::style::Dimension::Points(10f32),
                end: stretch::style::Dimension::Points(10f32),
                top: stretch::style::Dimension::Points(10f32),
                bottom: stretch::style::Dimension::Points(10f32),
                ..Default::default()
            },
            position: stretch::geometry::Rect {
                start: stretch::style::Dimension::Points(0f32),
                top: stretch::style::Dimension::Points(0f32),
                ..Default::default()
            },
            ..Default::default()
        },
        vec![],
    );
    let node3 = stretch.new_node(
        stretch::style::Style {
            position_type: stretch::style::PositionType::Absolute,
            size: stretch::geometry::Size {
                width: stretch::style::Dimension::Points(50f32),
                height: stretch::style::Dimension::Points(50f32),
                ..Default::default()
            },
            margin: stretch::geometry::Rect {
                start: stretch::style::Dimension::Points(10f32),
                end: stretch::style::Dimension::Points(10f32),
                top: stretch::style::Dimension::Points(10f32),
                bottom: stretch::style::Dimension::Points(10f32),
                ..Default::default()
            },
            position: stretch::geometry::Rect {
                end: stretch::style::Dimension::Points(0f32),
                bottom: stretch::style::Dimension::Points(0f32),
                ..Default::default()
            },
            ..Default::default()
        },
        vec![],
    );
    let node = stretch.new_node(
        stretch::style::Style {
            size: stretch::geometry::Size {
                width: stretch::style::Dimension::Points(100f32),
                height: stretch::style::Dimension::Points(100f32),
                ..Default::default()
            },
            padding: stretch::geometry::Rect {
                start: stretch::style::Dimension::Points(10f32),
                end: stretch::style::Dimension::Points(10f32),
                top: stretch::style::Dimension::Points(10f32),
                bottom: stretch::style::Dimension::Points(10f32),
                ..Default::default()
            },
            border: stretch::geometry::Rect {
                start: stretch::style::Dimension::Points(10f32),
                end: stretch::style::Dimension::Points(10f32),
                top: stretch::style::Dimension::Points(10f32),
                bottom: stretch::style::Dimension::Points(10f32),
                ..Default::default()
            },
            ..Default::default()
        },
        vec![node0, node1, node2, node3],
    );
    stretch.compute_layout(node, stretch::geometry::Size::undefined()).unwrap();
    assert_eq!(stretch.layout(node).size.width, 100f32);
    assert_eq!(stretch.layout(node).size.height, 100f32);
    assert_eq!(stretch.layout(node).location.x, 0f32);
    assert_eq!(stretch.layout(node).location.y, 0f32);
    assert_eq!(stretch.layout(node0).size.width, 50f32);
    assert_eq!(stretch.layout(node0).size.height, 50f32);
    assert_eq!(stretch.layout(node0).location.x, 10f32);
    assert_eq!(stretch.layout(node0).location.y, 10f32);
    assert_eq!(stretch.layout(node1).size.width, 50f32);
    assert_eq!(stretch.layout(node1).size.height, 50f32);
    assert_eq!(stretch.layout(node1).location.x, 40f32);
    assert_eq!(stretch.layout(node1).location.y, 40f32);
    assert_eq!(stretch.layout(node2).size.width, 50f32);
    assert_eq!(stretch.layout(node2).size.height, 50f32);
    assert_eq!(stretch.layout(node2).location.x, 20f32);
    assert_eq!(stretch.layout(node2).location.y, 20f32);
    assert_eq!(stretch.layout(node3).size.width, 50f32);
    assert_eq!(stretch.layout(node3).size.height, 50f32);
    assert_eq!(stretch.layout(node3).location.x, 30f32);
    assert_eq!(stretch.layout(node3).location.y, 30f32);
}
