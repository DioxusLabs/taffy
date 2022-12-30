pub fn compute() {
    #[allow(unused_imports)]
    use taffy::prelude::*;
    let mut taffy = taffy::Taffy::new();
    let node0 = taffy
        .new_leaf(taffy::style::Style {
            size: taffy::geometry::Size { width: taffy::style::Dimension::Percent(0.5f32), height: auto() },
            padding: taffy::geometry::Rect {
                left: taffy::style::LengthPercentage::Percent(0.03f32),
                right: taffy::style::LengthPercentage::Percent(0.03f32),
                top: taffy::style::LengthPercentage::Percent(0.03f32),
                bottom: taffy::style::LengthPercentage::Percent(0.03f32),
            },
            ..Default::default()
        })
        .unwrap();
    let node = taffy
        .new_with_children(
            taffy::style::Style {
                display: taffy::style::Display::Grid,
                size: taffy::geometry::Size { width: taffy::style::Dimension::Points(200f32), height: auto() },
                ..Default::default()
            },
            &[node0],
        )
        .unwrap();
    taffy.compute_layout(node, taffy::geometry::Size::MAX_CONTENT).unwrap();
}
