pub fn compute() {
    #[allow(unused_imports)]
    use taffy::prelude::*;
    let mut taffy = taffy::Taffy::new();
    let node0 = taffy
        .new_leaf(taffy::style::Style {
            align_content: Some(taffy::style::AlignContent::Stretch),
            size: taffy::geometry::Size {
                width: taffy::style::Dimension::Points(28f32),
                height: taffy::style::Dimension::Points(27f32),
            },
            ..Default::default()
        })
        .unwrap();
    let node1 = taffy
        .new_leaf_with_measure(
            taffy::style::Style {
                position: taffy::style::Position::Absolute,
                align_content: Some(taffy::style::AlignContent::Stretch),
                flex_grow: 0f32,
                flex_shrink: 1f32,
                size: taffy::geometry::Size {
                    width: taffy::style::Dimension::Percent(1f32),
                    height: taffy::style::Dimension::Points(15f32),
                },
                margin: taffy::geometry::Rect {
                    left: zero(),
                    right: zero(),
                    top: taffy::style::LengthPercentageAuto::Points(4f32),
                    bottom: zero(),
                },
                ..Default::default()
            },
            taffy::node::MeasureFunc::Raw(|known_dimensions, available_space| {
                const TEXT: &str = "\n  ";
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
    let node2 = taffy
        .new_leaf(taffy::style::Style {
            align_content: Some(taffy::style::AlignContent::Stretch),
            size: taffy::geometry::Size {
                width: taffy::style::Dimension::Points(25f32),
                height: taffy::style::Dimension::Points(27f32),
            },
            ..Default::default()
        })
        .unwrap();
    let node = taffy
        .new_with_children(
            taffy::style::Style {
                justify_content: Some(taffy::style::JustifyContent::SpaceBetween),
                min_size: taffy::geometry::Size {
                    width: taffy::style::Dimension::Points(311f32),
                    height: taffy::style::Dimension::Points(0f32),
                },
                max_size: taffy::geometry::Size {
                    width: taffy::style::Dimension::Points(311f32),
                    height: taffy::style::Dimension::Points(36893500000000000000f32),
                },
                ..Default::default()
            },
            &[node0, node1, node2],
        )
        .unwrap();
    taffy.compute_layout(node, taffy::geometry::Size::MAX_CONTENT).unwrap();
}
