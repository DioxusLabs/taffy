#[test]
fn flex_wrap_wrap_to_child_height() {
    let mut stretch = stretch::Stretch::new();
    let node000 = stretch.new_node(
        stretch::style::Style {
            size: stretch::geometry::Size {
                width: stretch::style::Dimension::Points(100f32),
                height: stretch::style::Dimension::Points(100f32),
                ..Default::default()
            },
            ..Default::default()
        },
        vec![],
    );
    let node00 = stretch.new_node(
        stretch::style::Style {
            flex_direction: stretch::style::FlexDirection::Column,
            size: stretch::geometry::Size { width: stretch::style::Dimension::Points(100f32), ..Default::default() },
            ..Default::default()
        },
        vec![node000],
    );
    let node0 = stretch.new_node(
        stretch::style::Style {
            flex_wrap: stretch::style::FlexWrap::Wrap,
            align_items: stretch::style::AlignItems::FlexStart,
            ..Default::default()
        },
        vec![node00],
    );
    let node1 = stretch.new_node(
        stretch::style::Style {
            size: stretch::geometry::Size {
                width: stretch::style::Dimension::Points(100f32),
                height: stretch::style::Dimension::Points(100f32),
                ..Default::default()
            },
            ..Default::default()
        },
        vec![],
    );
    let node = stretch.new_node(
        stretch::style::Style { flex_direction: stretch::style::FlexDirection::Column, ..Default::default() },
        vec![node0, node1],
    );
    stretch.compute_layout(node, stretch::geometry::Size::undefined()).unwrap();
    assert_eq!(stretch.layout(node).size.width, 100f32);
    assert_eq!(stretch.layout(node).size.height, 200f32);
    assert_eq!(stretch.layout(node).location.x, 0f32);
    assert_eq!(stretch.layout(node).location.y, 0f32);
    assert_eq!(stretch.layout(node0).size.width, 100f32);
    assert_eq!(stretch.layout(node0).size.height, 100f32);
    assert_eq!(stretch.layout(node0).location.x, 0f32);
    assert_eq!(stretch.layout(node0).location.y, 0f32);
    assert_eq!(stretch.layout(node00).size.width, 100f32);
    assert_eq!(stretch.layout(node00).size.height, 100f32);
    assert_eq!(stretch.layout(node00).location.x, 0f32);
    assert_eq!(stretch.layout(node00).location.y, 0f32);
    assert_eq!(stretch.layout(node000).size.width, 100f32);
    assert_eq!(stretch.layout(node000).size.height, 100f32);
    assert_eq!(stretch.layout(node000).location.x, 0f32);
    assert_eq!(stretch.layout(node000).location.y, 0f32);
    assert_eq!(stretch.layout(node1).size.width, 100f32);
    assert_eq!(stretch.layout(node1).size.height, 100f32);
    assert_eq!(stretch.layout(node1).location.x, 0f32);
    assert_eq!(stretch.layout(node1).location.y, 100f32);
}
