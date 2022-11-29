pub fn compute() {
    #[allow(unused_imports)]
    use taffy::prelude::*;
    let mut taffy = taffy::Taffy::new();
    let node = taffy
        .new_with_children(
            taffy::style::Style {
                size: taffy::geometry::Size { height: taffy::style::Dimension::Points(200f32), ..Size::auto() },
                max_size: taffy::geometry::Size { height: taffy::style::Dimension::Points(100f32), ..Size::auto() },
                ..Default::default()
            },
            &[],
        )
        .unwrap();
    taffy.compute_layout(node, taffy::geometry::Size::MAX_CONTENT).unwrap();
}
