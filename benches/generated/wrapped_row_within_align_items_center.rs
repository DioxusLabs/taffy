pub fn compute() {
    let mut taffy = taffy::Taffy::new();
    let node00 = taffy.new_with_children(
        taffy::style::FlexboxLayout {
            size: taffy::geometry::Size {
                width: taffy::style::Dimension::Points(150f32),
                height: taffy::style::Dimension::Points(80f32),
                ..Default::default()
            },
            ..Default::default()
        },
        &[],
    );
    let node01 = taffy.new_with_children(
        taffy::style::FlexboxLayout {
            size: taffy::geometry::Size {
                width: taffy::style::Dimension::Points(80f32),
                height: taffy::style::Dimension::Points(80f32),
                ..Default::default()
            },
            ..Default::default()
        },
        &[],
    );
    let node0 = taffy.new_with_children(
        taffy::style::FlexboxLayout { flex_wrap: taffy::style::FlexWrap::Wrap, ..Default::default() },
        &[node00, node01],
    );
    let node = taffy.new_with_children(
        taffy::style::FlexboxLayout {
            flex_direction: taffy::style::FlexDirection::Column,
            align_items: taffy::style::AlignItems::Center,
            size: taffy::geometry::Size {
                width: taffy::style::Dimension::Points(200f32),
                height: taffy::style::Dimension::Points(200f32),
                ..Default::default()
            },
            ..Default::default()
        },
        &[node0],
    );
    taffy.compute_layout(node, taffy::geometry::Size::undefined()).unwrap();
}
