pub fn compute() {
    let mut sprawl = sprawl::Sprawl::new();
    let node000 = sprawl
        .new_node(
            sprawl::style::FlexboxLayout {
                size: sprawl::geometry::Size {
                    width: sprawl::style::Dimension::Points(20f32),
                    height: sprawl::style::Dimension::Points(20f32),
                    ..Default::default()
                },
                ..Default::default()
            },
            &[],
        )
        .unwrap();
    let node00 = sprawl
        .new_node(sprawl::style::FlexboxLayout { flex_grow: 1f32, flex_shrink: 1f32, ..Default::default() }, &[node000])
        .unwrap();
    let node0 = sprawl
        .new_node(
            sprawl::style::FlexboxLayout {
                justify_content: sprawl::style::JustifyContent::Center,
                flex_grow: 0f32,
                flex_shrink: 1f32,
                ..Default::default()
            },
            &[node00],
        )
        .unwrap();
    let node = sprawl
        .new_node(
            sprawl::style::FlexboxLayout {
                size: sprawl::geometry::Size {
                    width: sprawl::style::Dimension::Points(100f32),
                    height: sprawl::style::Dimension::Points(100f32),
                    ..Default::default()
                },
                ..Default::default()
            },
            &[node0],
        )
        .unwrap();
    sprawl.compute_layout(node, sprawl::geometry::Size::undefined()).unwrap();
}
