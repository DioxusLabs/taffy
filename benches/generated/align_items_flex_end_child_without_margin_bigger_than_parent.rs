pub fn compute() {
    let mut taffy = taffy::Taffy::new();
    let node00 = taffy
        .new_with_children(
            taffy::style::FlexboxLayout {
                size: taffy::geometry::Size {
                    width: Some(taffy::style::Dimension::Points(70f32)),
                    height: Some(taffy::style::Dimension::Points(70f32)),
                    ..Default::default()
                },
                ..Default::default()
            },
            &[],
        )
        .unwrap();
    let node0 = taffy
        .new_with_children(
            taffy::style::FlexboxLayout { align_items: taffy::style::AlignItems::FlexEnd, ..Default::default() },
            &[node00],
        )
        .unwrap();
    let node = taffy
        .new_with_children(
            taffy::style::FlexboxLayout {
                align_items: taffy::style::AlignItems::Center,
                justify_content: taffy::style::JustifyContent::Center,
                size: taffy::geometry::Size {
                    width: Some(taffy::style::Dimension::Points(50f32)),
                    height: Some(taffy::style::Dimension::Points(50f32)),
                    ..Default::default()
                },
                ..Default::default()
            },
            &[node0],
        )
        .unwrap();
    taffy.compute_layout(node, taffy::geometry::Size::undefined()).unwrap();
}
