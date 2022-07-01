pub fn compute() {
    let mut taffy = taffy::Taffy::new();
    let node00 = taffy.new_with_children(
        taffy::style::FlexboxLayout {
            size: taffy::geometry::Size {
                width: taffy::style::Dimension::Points(70f32),
                height: taffy::style::Dimension::Points(100f32),
                ..Default::default()
            },
            ..Default::default()
        },
        &[],
    );
    let node0 = taffy.new_with_children(
        taffy::style::FlexboxLayout {
            flex_direction: taffy::style::FlexDirection::Column,
            flex_grow: 1f32,
            size: taffy::geometry::Size { width: taffy::style::Dimension::Points(0f32), ..Default::default() },
            ..Default::default()
        },
        &[node00],
    );
    let node10 = taffy.new_with_children(
        taffy::style::FlexboxLayout {
            size: taffy::geometry::Size {
                width: taffy::style::Dimension::Points(20f32),
                height: taffy::style::Dimension::Points(100f32),
                ..Default::default()
            },
            ..Default::default()
        },
        &[],
    );
    let node1 = taffy.new_with_children(
        taffy::style::FlexboxLayout {
            flex_direction: taffy::style::FlexDirection::Column,
            flex_grow: 1f32,
            size: taffy::geometry::Size { width: taffy::style::Dimension::Points(0f32), ..Default::default() },
            ..Default::default()
        },
        &[node10],
    );
    let node = taffy.new_with_children(
        taffy::style::FlexboxLayout {
            size: taffy::geometry::Size { width: taffy::style::Dimension::Points(10f32), ..Default::default() },
            ..Default::default()
        },
        &[node0, node1],
    );
    taffy.compute_layout(node, taffy::geometry::Size::undefined()).unwrap();
}
