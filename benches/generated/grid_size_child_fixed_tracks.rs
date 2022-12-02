pub fn compute() {
    #[allow(unused_imports)]
    use taffy::prelude::*;
    let mut taffy = taffy::Taffy::new();
    let node0 = taffy
        .new_leaf_with_measure(
            taffy::style::Style {
                align_self: Some(taffy::style::AlignSelf::Start),
                justify_self: Some(taffy::style::JustifySelf::Start),
                ..Default::default()
            },
            taffy::node::MeasureFunc::Raw(|known_dimensions, available_space| {
                const TEXT: &'static str = "HH\u{200b}HH\u{200b}HH\u{200b}HH";
                super::measure_standard_text(known_dimensions, available_space, TEXT)
            }),
        )
        .unwrap();
    let node1 = taffy
        .new_leaf_with_measure(
            taffy::style::Style {
                align_self: Some(taffy::style::AlignSelf::Start),
                justify_self: Some(taffy::style::JustifySelf::Start),
                ..Default::default()
            },
            taffy::node::MeasureFunc::Raw(|known_dimensions, available_space| {
                const TEXT: &'static str = "HHH\u{200b}HHH";
                super::measure_standard_text(known_dimensions, available_space, TEXT)
            }),
        )
        .unwrap();
    let node2 = taffy
        .new_leaf_with_measure(
            taffy::style::Style {
                align_self: Some(taffy::style::AlignSelf::Start),
                justify_self: Some(taffy::style::JustifySelf::Start),
                ..Default::default()
            },
            taffy::node::MeasureFunc::Raw(|known_dimensions, available_space| {
                const TEXT: &'static str = "HH\u{200b}HHHH";
                super::measure_standard_text(known_dimensions, available_space, TEXT)
            }),
        )
        .unwrap();
    let node3 = taffy
        .new_leaf_with_measure(
            taffy::style::Style {
                align_self: Some(taffy::style::AlignSelf::Start),
                justify_self: Some(taffy::style::JustifySelf::Start),
                size: taffy::geometry::Size { width: taffy::style::Dimension::Points(20f32), ..Size::auto() },
                ..Default::default()
            },
            taffy::node::MeasureFunc::Raw(|known_dimensions, available_space| {
                const TEXT: &'static str = "HH\u{200b}HH\u{200b}HH\u{200b}HH";
                super::measure_standard_text(known_dimensions, available_space, TEXT)
            }),
        )
        .unwrap();
    let node4 = taffy
        .new_leaf_with_measure(
            taffy::style::Style {
                align_self: Some(taffy::style::AlignSelf::Start),
                justify_self: Some(taffy::style::JustifySelf::Start),
                max_size: taffy::geometry::Size { width: taffy::style::Dimension::Points(30f32), ..Size::auto() },
                ..Default::default()
            },
            taffy::node::MeasureFunc::Raw(|known_dimensions, available_space| {
                const TEXT: &'static str = "HH\u{200b}HH\u{200b}HH\u{200b}HH";
                super::measure_standard_text(known_dimensions, available_space, TEXT)
            }),
        )
        .unwrap();
    let node = taffy
        .new_with_children(
            taffy::style::Style {
                display: taffy::style::Display::Grid,
                grid_template_rows: vec![
                    taffy::style::TrackSizingFunction::Fixed(taffy::style::Dimension::Points(40f32)),
                    taffy::style::TrackSizingFunction::Fixed(taffy::style::Dimension::Points(40f32)),
                    taffy::style::TrackSizingFunction::Fixed(taffy::style::Dimension::Points(40f32)),
                ],
                grid_template_columns: vec![
                    taffy::style::TrackSizingFunction::Fixed(taffy::style::Dimension::Points(40f32)),
                    taffy::style::TrackSizingFunction::Fixed(taffy::style::Dimension::Points(40f32)),
                    taffy::style::TrackSizingFunction::Fixed(taffy::style::Dimension::Points(40f32)),
                ],
                size: taffy::geometry::Size {
                    width: taffy::style::Dimension::Points(120f32),
                    height: taffy::style::Dimension::Points(120f32),
                    ..Size::auto()
                },
                ..Default::default()
            },
            &[node0, node1, node2, node3, node4],
        )
        .unwrap();
    taffy.compute_layout(node, taffy::geometry::Size::MAX_CONTENT).unwrap();
}
