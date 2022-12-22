pub fn compute() {
    #[allow(unused_imports)]
    use taffy::prelude::*;
    let mut taffy = taffy::Taffy::new();
    let node = taffy
        .new_leaf(taffy::style::Style {
            size: taffy::geometry::Size { width: taffy::style::Dimension::Points(50f32), height: auto() },
            min_size: taffy::geometry::Size { width: taffy::style::Dimension::Points(100f32), height: auto() },
            ..Default::default()
        })
        .unwrap();
    taffy.compute_layout(node, taffy::geometry::Size::MAX_CONTENT).unwrap();
}
