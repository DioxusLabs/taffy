pub fn compute() {
    let mut taffy = taffy::Taffy::new();
    let node00 = taffy.new_with_children(
        taffy::style::FlexboxLayout {
            size: taffy::geometry::Size {
                width: taffy::style::Dimension::Points(50f32),
                height: taffy::style::Dimension::Points(50f32),
                ..Default::default()
            },
            margin: taffy::geometry::Rect {
                start: taffy::style::Dimension::Points(10f32),
                end: taffy::style::Dimension::Points(10f32),
                ..Default::default()
            },
            ..Default::default()
        },
        &[],
    );
    let node0 = taffy.new_with_children(
        taffy::style::FlexboxLayout { align_items: taffy::style::AlignItems::Center, ..Default::default() },
        &[node00],
    );
    let node = taffy.new_with_children(
        taffy::style::FlexboxLayout {
            align_items: taffy::style::AlignItems::Center,
            justify_content: taffy::style::JustifyContent::Center,
            size: taffy::geometry::Size {
                width: taffy::style::Dimension::Points(50f32),
                height: taffy::style::Dimension::Points(50f32),
                ..Default::default()
            },
            ..Default::default()
        },
        &[node0],
    );
    taffy.compute_layout(node, taffy::geometry::Size::undefined()).unwrap();
}
