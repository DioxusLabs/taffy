pub fn compute() {
    let mut taffy = taffy::Taffy::new();
    let node0 = taffy.new_with_children(
        taffy::style::FlexboxLayout {
            size: taffy::geometry::Size {
                width: taffy::style::Dimension::Points(72f32),
                height: taffy::style::Dimension::Points(72f32),
                ..Default::default()
            },
            margin: taffy::geometry::Rect {
                start: taffy::style::Dimension::Auto,
                end: taffy::style::Dimension::Points(10f32),
                ..Default::default()
            },
            ..Default::default()
        },
        &[],
    );
    let node = taffy.new_with_children(
        taffy::style::FlexboxLayout {
            justify_content: taffy::style::JustifyContent::Center,
            size: taffy::geometry::Size {
                width: taffy::style::Dimension::Points(52f32),
                height: taffy::style::Dimension::Points(52f32),
                ..Default::default()
            },
            ..Default::default()
        },
        &[node0],
    );
    taffy.compute_layout(node, taffy::geometry::Size::undefined()).unwrap();
}
