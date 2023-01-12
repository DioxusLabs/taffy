pub fn compute() {
    #[allow(unused_imports)]
    use taffy::prelude::*;
    let mut taffy = taffy::Taffy::new();
    let node0 = taffy
        .new_leaf(taffy::style::Style {
            flex_grow: 1f32,
            flex_basis: taffy::style::Dimension::Points(5f32),
            ..Default::default()
        })
        .unwrap();
    let node1 = taffy
        .new_leaf_with_measure(
            taffy::style::Style {
                flex_grow: 1f32,
                flex_basis: taffy::style::Dimension::Points(5f32),
                ..Default::default()
            },
            taffy::node::MeasureFunc::Raw(|known_dimensions, available_space| {
                const TEXT: &str = "H";
                super::measure_standard_text(
                    known_dimensions,
                    available_space,
                    TEXT,
                    super::WritingMode::Horizontal,
                    None,
                )
            }),
        )
        .unwrap();
    let node = taffy
        .new_with_children(
            taffy::style::Style {
                size: taffy::geometry::Size {
                    width: taffy::style::Dimension::Points(20f32),
                    height: taffy::style::Dimension::Points(10f32),
                },
                ..Default::default()
            },
            &[node0, node1],
        )
        .unwrap();
    taffy.compute_layout(node, taffy::geometry::Size::MAX_CONTENT).unwrap();
}
