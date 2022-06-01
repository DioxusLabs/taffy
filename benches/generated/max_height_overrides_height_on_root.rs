pub fn compute() {
    let mut stretch = sprawl::Sprawl::new();
    let node = stretch
        .new_node(
            sprawl::style::Style {
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
    stretch.compute_layout(node, sprawl::geometry::Size::undefined()).unwrap();
}
