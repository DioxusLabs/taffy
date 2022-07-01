pub fn compute() {
    let mut taffy = taffy::Taffy::new();
    let node000 = taffy.new_with_children(
        taffy::style::FlexboxLayout {
            size: taffy::geometry::Size {
                width: taffy::style::Dimension::Points(40f32),
                height: taffy::style::Dimension::Points(40f32),
                ..Default::default()
            },
            ..Default::default()
        },
        &[],
    );
    let node00 = taffy.new_with_children(
        taffy::style::FlexboxLayout { flex_direction: taffy::style::FlexDirection::Column, ..Default::default() },
        &[node000],
    );
    let node010 = taffy.new_with_children(
        taffy::style::FlexboxLayout {
            size: taffy::geometry::Size {
                width: taffy::style::Dimension::Points(40f32),
                height: taffy::style::Dimension::Points(40f32),
                ..Default::default()
            },
            ..Default::default()
        },
        &[],
    );
    let node01 = taffy.new_with_children(
        taffy::style::FlexboxLayout {
            flex_direction: taffy::style::FlexDirection::Column,
            margin: taffy::geometry::Rect { end: taffy::style::Dimension::Points(10f32), ..Default::default() },
            ..Default::default()
        },
        &[node010],
    );
    let node0 = taffy.new_with_children(
        taffy::style::FlexboxLayout {
            flex_wrap: taffy::style::FlexWrap::Wrap,
            size: taffy::geometry::Size { width: taffy::style::Dimension::Points(85f32), ..Default::default() },
            ..Default::default()
        },
        &[node00, node01],
    );
    let node = taffy.new_with_children(
        taffy::style::FlexboxLayout {
            flex_direction: taffy::style::FlexDirection::Column,
            size: taffy::geometry::Size {
                width: taffy::style::Dimension::Points(500f32),
                height: taffy::style::Dimension::Points(500f32),
                ..Default::default()
            },
            ..Default::default()
        },
        &[node0],
    );
    taffy.compute_layout(node, taffy::geometry::Size::undefined()).unwrap();
}
