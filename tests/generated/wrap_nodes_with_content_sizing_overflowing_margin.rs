#[test]
fn wrap_nodes_with_content_sizing_overflowing_margin() {
    let mut stretch = stretch::Stretch::new();
    let node000 = stretch.new_node(
        stretch::style::Style {
            size: stretch::geometry::Size {
                width: stretch::style::Dimension::Points(40f32),
                height: stretch::style::Dimension::Points(40f32),
                ..Default::default()
            },
            ..Default::default()
        },
        vec![],
    );
    let node00 = stretch.new_node(
        stretch::style::Style { flex_direction: stretch::style::FlexDirection::Column, ..Default::default() },
        vec![node000],
    );
    let node010 = stretch.new_node(
        stretch::style::Style {
            size: stretch::geometry::Size {
                width: stretch::style::Dimension::Points(40f32),
                height: stretch::style::Dimension::Points(40f32),
                ..Default::default()
            },
            ..Default::default()
        },
        vec![],
    );
    let node01 = stretch.new_node(
        stretch::style::Style {
            flex_direction: stretch::style::FlexDirection::Column,
            margin: stretch::geometry::Rect { end: stretch::style::Dimension::Points(10f32), ..Default::default() },
            ..Default::default()
        },
        vec![node010],
    );
    let node0 = stretch.new_node(
        stretch::style::Style {
            flex_wrap: stretch::style::FlexWrap::Wrap,
            size: stretch::geometry::Size { width: stretch::style::Dimension::Points(85f32), ..Default::default() },
            ..Default::default()
        },
        vec![node00, node01],
    );
    let node = stretch.new_node(
        stretch::style::Style {
            flex_direction: stretch::style::FlexDirection::Column,
            size: stretch::geometry::Size {
                width: stretch::style::Dimension::Points(500f32),
                height: stretch::style::Dimension::Points(500f32),
                ..Default::default()
            },
            ..Default::default()
        },
        vec![node0],
    );
    stretch.compute_layout(node, stretch::geometry::Size::undefined()).unwrap();
    assert_eq!(stretch.layout(node).size.width, 500f32);
    assert_eq!(stretch.layout(node).size.height, 500f32);
    assert_eq!(stretch.layout(node).location.x, 0f32);
    assert_eq!(stretch.layout(node).location.y, 0f32);
    assert_eq!(stretch.layout(node0).size.width, 85f32);
    assert_eq!(stretch.layout(node0).size.height, 80f32);
    assert_eq!(stretch.layout(node0).location.x, 0f32);
    assert_eq!(stretch.layout(node0).location.y, 0f32);
    assert_eq!(stretch.layout(node00).size.width, 40f32);
    assert_eq!(stretch.layout(node00).size.height, 40f32);
    assert_eq!(stretch.layout(node00).location.x, 0f32);
    assert_eq!(stretch.layout(node00).location.y, 0f32);
    assert_eq!(stretch.layout(node000).size.width, 40f32);
    assert_eq!(stretch.layout(node000).size.height, 40f32);
    assert_eq!(stretch.layout(node000).location.x, 0f32);
    assert_eq!(stretch.layout(node000).location.y, 0f32);
    assert_eq!(stretch.layout(node01).size.width, 40f32);
    assert_eq!(stretch.layout(node01).size.height, 40f32);
    assert_eq!(stretch.layout(node01).location.x, 0f32);
    assert_eq!(stretch.layout(node01).location.y, 40f32);
    assert_eq!(stretch.layout(node010).size.width, 40f32);
    assert_eq!(stretch.layout(node010).size.height, 40f32);
    assert_eq!(stretch.layout(node010).location.x, 0f32);
    assert_eq!(stretch.layout(node010).location.y, 0f32);
}
