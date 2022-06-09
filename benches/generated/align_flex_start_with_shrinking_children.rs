pub fn compute() {
    let mut sprawl = sprawl::Sprawl::new();
    let node000 = sprawl
        .new_with_children(sprawl::style::Style { flex_grow: 1f32, flex_shrink: 1f32, ..Default::default() }, &[])
        .unwrap();
    let node00 = sprawl
        .new_with_children(
            sprawl::style::Style { flex_grow: 1f32, flex_shrink: 1f32, ..Default::default() },
            &[node000],
        )
        .unwrap();
    let node0 = sprawl
        .new_with_children(
            sprawl::style::Style { align_items: sprawl::style::AlignItems::FlexStart, ..Default::default() },
            &[node00],
        )
        .unwrap();
    let node = sprawl
        .new_with_children(
            sprawl::style::Style {
                size: sprawl::geometry::Size {
                    width: sprawl::style::Dimension::Points(500f32),
                    height: sprawl::style::Dimension::Points(500f32),
                    ..Default::default()
                },
                ..Default::default()
            },
            &[node0],
        )
        .unwrap();
    sprawl.compute_layout(node, sprawl::geometry::Size::undefined()).unwrap();
}
