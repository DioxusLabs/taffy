pub fn compute() {
    #[allow(unused_imports)]
    use taffy::prelude::*;
    let mut taffy = taffy::Taffy::new();
    let node0 = taffy
        .new_leaf(taffy::style::Style {
            flex_grow: 1f32,
            flex_shrink: 1f32,
            flex_basis: taffy::style::Dimension::Percent(0f32),
            margin: taffy::geometry::Rect {
                left: taffy::style::LengthPercentageAuto::Points(2f32),
                right: taffy::style::LengthPercentageAuto::Points(2f32),
                top: zero(),
                bottom: zero(),
            },
            ..Default::default()
        })
        .unwrap();
    let node1 = taffy
        .new_leaf(taffy::style::Style {
            flex_grow: 1f32,
            flex_shrink: 1f32,
            flex_basis: taffy::style::Dimension::Percent(0f32),
            margin: taffy::geometry::Rect {
                left: taffy::style::LengthPercentageAuto::Points(10f32),
                right: taffy::style::LengthPercentageAuto::Points(10f32),
                top: zero(),
                bottom: zero(),
            },
            ..Default::default()
        })
        .unwrap();
    let node2 = taffy
        .new_leaf(taffy::style::Style {
            flex_grow: 1f32,
            flex_shrink: 1f32,
            flex_basis: taffy::style::Dimension::Percent(0f32),
            margin: taffy::geometry::Rect {
                left: taffy::style::LengthPercentageAuto::Points(15f32),
                right: taffy::style::LengthPercentageAuto::Points(15f32),
                top: zero(),
                bottom: zero(),
            },
            ..Default::default()
        })
        .unwrap();
    let node = taffy
        .new_with_children(
            taffy::style::Style {
                gap: taffy::geometry::Size { width: taffy::style::LengthPercentage::Points(10f32), height: zero() },
                size: taffy::geometry::Size {
                    width: taffy::style::Dimension::Points(80f32),
                    height: taffy::style::Dimension::Points(100f32),
                },
                ..Default::default()
            },
            &[node0, node1, node2],
        )
        .unwrap();
    taffy.compute_layout(node, taffy::geometry::Size::MAX_CONTENT).unwrap();
}
