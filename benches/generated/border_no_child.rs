pub fn compute() {
    #[allow(unused_imports)]
    use taffy::prelude::*;
    let mut taffy = taffy::Taffy::new();
    let node = taffy
        .new_leaf(taffy::style::Style {
            border: taffy::geometry::Rect {
                left: taffy::style::LengthPercentage::Points(10f32),
                right: taffy::style::LengthPercentage::Points(10f32),
                top: taffy::style::LengthPercentage::Points(10f32),
                bottom: taffy::style::LengthPercentage::Points(10f32),
            },
            ..Default::default()
        })
        .unwrap();
    taffy.compute_layout(node, taffy::geometry::Size::MAX_CONTENT).unwrap();
}
