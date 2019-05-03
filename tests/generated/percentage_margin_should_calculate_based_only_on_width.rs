#[test]
fn percentage_margin_should_calculate_based_only_on_width() {
    let mut stretch = stretch::Stretch::new();
    let node00 = stretch.new_node(
        stretch::style::Style {
            size: stretch::geometry::Size {
                width: stretch::style::Dimension::Points(10f32),
                height: stretch::style::Dimension::Points(10f32),
                ..Default::default()
            },
            ..Default::default()
        },
        vec![],
    );
    let node0 = stretch.new_node(
        stretch::style::Style {
            flex_direction: stretch::style::FlexDirection::Column,
            flex_grow: 1f32,
            margin: stretch::geometry::Rect {
                start: stretch::style::Dimension::Percent(0.1f32),
                end: stretch::style::Dimension::Percent(0.1f32),
                top: stretch::style::Dimension::Percent(0.1f32),
                bottom: stretch::style::Dimension::Percent(0.1f32),
                ..Default::default()
            },
            ..Default::default()
        },
        vec![node00],
    );
    let node = stretch.new_node(
        stretch::style::Style {
            flex_direction: stretch::style::FlexDirection::Column,
            size: stretch::geometry::Size {
                width: stretch::style::Dimension::Points(200f32),
                height: stretch::style::Dimension::Points(100f32),
                ..Default::default()
            },
            ..Default::default()
        },
        vec![node0],
    );
    stretch.compute_layout(node, stretch::geometry::Size::undefined()).unwrap();
    assert_eq!(stretch.layout(node).size.width, 200f32);
    assert_eq!(stretch.layout(node).size.height, 100f32);
    assert_eq!(stretch.layout(node).location.x, 0f32);
    assert_eq!(stretch.layout(node).location.y, 0f32);
    assert_eq!(stretch.layout(node0).size.width, 160f32);
    assert_eq!(stretch.layout(node0).size.height, 60f32);
    assert_eq!(stretch.layout(node0).location.x, 20f32);
    assert_eq!(stretch.layout(node0).location.y, 20f32);
    assert_eq!(stretch.layout(node00).size.width, 10f32);
    assert_eq!(stretch.layout(node00).size.height, 10f32);
    assert_eq!(stretch.layout(node00).location.x, 0f32);
    assert_eq!(stretch.layout(node00).location.y, 0f32);
}
