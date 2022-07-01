pub fn compute() {
    let mut taffy = taffy::Taffy::new();
    let node0 = taffy.new_with_children(
        taffy::style::FlexboxLayout {
            display: taffy::style::Display::None,
            size: taffy::geometry::Size {
                width: taffy::style::Dimension::Points(20f32),
                height: taffy::style::Dimension::Points(20f32),
                ..Default::default()
            },
            margin: taffy::geometry::Rect {
                start: taffy::style::Dimension::Points(10f32),
                end: taffy::style::Dimension::Points(10f32),
                top: taffy::style::Dimension::Points(10f32),
                bottom: taffy::style::Dimension::Points(10f32),
                ..Default::default()
            },
            ..Default::default()
        },
        &[],
    );
    let node1 = taffy.new_with_children(taffy::style::FlexboxLayout { flex_grow: 1f32, ..Default::default() }, &[]);
    let node = taffy.new_with_children(
        taffy::style::FlexboxLayout {
            size: taffy::geometry::Size {
                width: taffy::style::Dimension::Points(100f32),
                height: taffy::style::Dimension::Points(100f32),
                ..Default::default()
            },
            ..Default::default()
        },
        &[node0, node1],
    );
    taffy.compute_layout(node, taffy::geometry::Size::undefined()).unwrap();
}
