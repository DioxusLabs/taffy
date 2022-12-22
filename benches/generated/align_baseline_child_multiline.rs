pub fn compute() {
    #[allow(unused_imports)]
    use taffy::prelude::*;
    let mut taffy = taffy::Taffy::new();
    let node0 = taffy
        .new_leaf(taffy::style::Style {
            size: taffy::geometry::Size {
                width: taffy::style::Dimension::Points(50f32),
                height: taffy::style::Dimension::Points(60f32),
            },
            ..Default::default()
        })
        .unwrap();
    let node10 = taffy
        .new_leaf(taffy::style::Style {
            size: taffy::geometry::Size {
                width: taffy::style::Dimension::Points(25f32),
                height: taffy::style::Dimension::Points(20f32),
            },
            ..Default::default()
        })
        .unwrap();
    let node11 = taffy
        .new_leaf(taffy::style::Style {
            size: taffy::geometry::Size {
                width: taffy::style::Dimension::Points(25f32),
                height: taffy::style::Dimension::Points(10f32),
            },
            ..Default::default()
        })
        .unwrap();
    let node12 = taffy
        .new_leaf(taffy::style::Style {
            size: taffy::geometry::Size {
                width: taffy::style::Dimension::Points(25f32),
                height: taffy::style::Dimension::Points(20f32),
            },
            ..Default::default()
        })
        .unwrap();
    let node13 = taffy
        .new_leaf(taffy::style::Style {
            size: taffy::geometry::Size {
                width: taffy::style::Dimension::Points(25f32),
                height: taffy::style::Dimension::Points(10f32),
            },
            ..Default::default()
        })
        .unwrap();
    let node1 = taffy
        .new_with_children(
            taffy::style::Style {
                flex_wrap: taffy::style::FlexWrap::Wrap,
                size: taffy::geometry::Size { width: taffy::style::Dimension::Points(50f32), height: auto() },
                ..Default::default()
            },
            &[node10, node11, node12, node13],
        )
        .unwrap();
    let node = taffy
        .new_with_children(
            taffy::style::Style {
                align_items: Some(taffy::style::AlignItems::Baseline),
                size: taffy::geometry::Size { width: taffy::style::Dimension::Points(100f32), height: auto() },
                ..Default::default()
            },
            &[node0, node1],
        )
        .unwrap();
    taffy.compute_layout(node, taffy::geometry::Size::MAX_CONTENT).unwrap();
}
