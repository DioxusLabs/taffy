pub fn compute() {
    let mut sprawl = sprawl::Sprawl::new();
    let node = sprawl
        .new_node(
            sprawl::style::FlexboxLayout {
                size: sprawl::geometry::Size { height: sprawl::style::Dimension::Points(200f32), ..Default::default() },
                max_size: sprawl::geometry::Size {
                    height: sprawl::style::Dimension::Points(100f32),
                    ..Default::default()
                },
                ..Default::default()
            },
            &[],
        )
        .unwrap();
    sprawl.compute_layout(node, sprawl::geometry::Size::undefined()).unwrap();
}
