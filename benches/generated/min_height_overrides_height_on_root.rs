pub fn compute() {
    #[allow(unused_imports)]
    use taffy::prelude::*;
    let mut taffy = taffy::Taffy::new();
    let node = taffy
        .new_leaf(taffy::style::Style {
            size: taffy::geometry::Size { height: taffy::style::Dimension::Points(50f32), ..Size::auto() },
            min_size: taffy::geometry::Size { height: taffy::style::Dimension::Points(100f32), ..Size::auto() },
            ..Default::default()
        })
        .unwrap();
    taffy.compute_layout(node, taffy::geometry::Size::MAX_CONTENT).unwrap();
}
