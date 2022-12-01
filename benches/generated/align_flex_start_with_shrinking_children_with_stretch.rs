pub fn compute() {
    #[allow(unused_imports)]
    use taffy::prelude::*;
    let mut taffy = taffy::Taffy::new();
    let node000 = taffy
        .new_with_children(taffy::style::Style { flex_grow: 1f32, flex_shrink: 1f32, ..Default::default() }, &[])
        .unwrap();
    let node00 = taffy
        .new_with_children(
            taffy::style::Style {
                align_items: taffy::style::AlignItems::Stretch,
                flex_grow: 1f32,
                flex_shrink: 1f32,
                ..Default::default()
            },
            &[node000],
        )
        .unwrap();
    let node0 = taffy
        .new_with_children(
            taffy::style::Style { align_items: taffy::style::AlignItems::FlexStart, ..Default::default() },
            &[node00],
        )
        .unwrap();
    let node = taffy
        .new_with_children(
            taffy::style::Style {
                size: taffy::geometry::Size {
                    width: taffy::style::Dimension::Points(500f32),
                    height: taffy::style::Dimension::Points(500f32),
                    ..Size::auto()
                },
                ..Default::default()
            },
            &[node0],
        )
        .unwrap();
    taffy.compute_layout(node, taffy::geometry::Size::MAX_CONTENT).unwrap();
}
