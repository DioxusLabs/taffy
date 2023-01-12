pub fn compute() {
    #[allow(unused_imports)]
    use taffy::prelude::*;
    let mut taffy = taffy::Taffy::new();
    let node0 = taffy
        .new_leaf(taffy::style::Style {
            size: taffy::geometry::Size {
                width: taffy::style::Dimension::Points(62f32),
                height: taffy::style::Dimension::Points(62f32),
            },
            ..Default::default()
        })
        .unwrap();
    let node = taffy
        .new_with_children(
            taffy::style::Style {
                align_items: Some(taffy::style::AlignItems::Center),
                min_size: taffy::geometry::Size {
                    width: taffy::style::Dimension::Points(320f32),
                    height: taffy::style::Dimension::Points(72f32),
                },
                max_size: taffy::geometry::Size {
                    width: taffy::style::Dimension::Points(320f32),
                    height: taffy::style::Dimension::Points(504f32),
                },
                padding: taffy::geometry::Rect {
                    left: zero(),
                    right: zero(),
                    top: taffy::style::LengthPercentage::Points(8f32),
                    bottom: taffy::style::LengthPercentage::Points(8f32),
                },
                ..Default::default()
            },
            &[node0],
        )
        .unwrap();
    taffy.compute_layout(node, taffy::geometry::Size::MAX_CONTENT).unwrap();
}
