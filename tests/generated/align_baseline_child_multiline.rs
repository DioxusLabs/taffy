#[test]
fn align_baseline_child_multiline() {
    let mut stretch = stretch::Stretch::new();
    let node0 = stretch.new_node(
        stretch::style::Style {
            size: stretch::geometry::Size {
                width: stretch::style::Dimension::Points(50f32),
                height: stretch::style::Dimension::Points(60f32),
                ..Default::default()
            },
            ..Default::default()
        },
        vec![],
    );
    let node10 = stretch.new_node(
        stretch::style::Style {
            size: stretch::geometry::Size {
                width: stretch::style::Dimension::Points(25f32),
                height: stretch::style::Dimension::Points(20f32),
                ..Default::default()
            },
            ..Default::default()
        },
        vec![],
    );
    let node11 = stretch.new_node(
        stretch::style::Style {
            size: stretch::geometry::Size {
                width: stretch::style::Dimension::Points(25f32),
                height: stretch::style::Dimension::Points(10f32),
                ..Default::default()
            },
            ..Default::default()
        },
        vec![],
    );
    let node12 = stretch.new_node(
        stretch::style::Style {
            size: stretch::geometry::Size {
                width: stretch::style::Dimension::Points(25f32),
                height: stretch::style::Dimension::Points(20f32),
                ..Default::default()
            },
            ..Default::default()
        },
        vec![],
    );
    let node13 = stretch.new_node(
        stretch::style::Style {
            size: stretch::geometry::Size {
                width: stretch::style::Dimension::Points(25f32),
                height: stretch::style::Dimension::Points(10f32),
                ..Default::default()
            },
            ..Default::default()
        },
        vec![],
    );
    let node1 = stretch.new_node(
        stretch::style::Style {
            flex_wrap: stretch::style::FlexWrap::Wrap,
            size: stretch::geometry::Size { width: stretch::style::Dimension::Points(50f32), ..Default::default() },
            ..Default::default()
        },
        vec![node10, node11, node12, node13],
    );
    let node = stretch.new_node(
        stretch::style::Style {
            align_items: stretch::style::AlignItems::Baseline,
            size: stretch::geometry::Size { width: stretch::style::Dimension::Points(100f32), ..Default::default() },
            ..Default::default()
        },
        vec![node0, node1],
    );
    stretch.compute_layout(node, stretch::geometry::Size::undefined()).unwrap();
    assert_eq!(stretch.layout(node).size.width, 100f32);
    assert_eq!(stretch.layout(node).size.height, 80f32);
    assert_eq!(stretch.layout(node).location.x, 0f32);
    assert_eq!(stretch.layout(node).location.y, 0f32);
    assert_eq!(stretch.layout(node0).size.width, 50f32);
    assert_eq!(stretch.layout(node0).size.height, 60f32);
    assert_eq!(stretch.layout(node0).location.x, 0f32);
    assert_eq!(stretch.layout(node0).location.y, 0f32);
    assert_eq!(stretch.layout(node1).size.width, 50f32);
    assert_eq!(stretch.layout(node1).size.height, 40f32);
    assert_eq!(stretch.layout(node1).location.x, 50f32);
    assert_eq!(stretch.layout(node1).location.y, 40f32);
    assert_eq!(stretch.layout(node10).size.width, 25f32);
    assert_eq!(stretch.layout(node10).size.height, 20f32);
    assert_eq!(stretch.layout(node10).location.x, 0f32);
    assert_eq!(stretch.layout(node10).location.y, 0f32);
    assert_eq!(stretch.layout(node11).size.width, 25f32);
    assert_eq!(stretch.layout(node11).size.height, 10f32);
    assert_eq!(stretch.layout(node11).location.x, 25f32);
    assert_eq!(stretch.layout(node11).location.y, 0f32);
    assert_eq!(stretch.layout(node12).size.width, 25f32);
    assert_eq!(stretch.layout(node12).size.height, 20f32);
    assert_eq!(stretch.layout(node12).location.x, 0f32);
    assert_eq!(stretch.layout(node12).location.y, 20f32);
    assert_eq!(stretch.layout(node13).size.width, 25f32);
    assert_eq!(stretch.layout(node13).size.height, 10f32);
    assert_eq!(stretch.layout(node13).location.x, 25f32);
    assert_eq!(stretch.layout(node13).location.y, 20f32);
}
