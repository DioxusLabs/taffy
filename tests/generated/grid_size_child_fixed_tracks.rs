#[test]
fn grid_size_child_fixed_tracks() {
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
    println!("\nComputed tree:");
    taffy::debug::print_tree(&taffy, node);
    println!();
    assert_eq!(taffy.layout(node).unwrap().size.width, 120f32);
    assert_eq!(taffy.layout(node).unwrap().size.height, 120f32);
    assert_eq!(taffy.layout(node).unwrap().location.x, 0f32);
    assert_eq!(taffy.layout(node).unwrap().location.y, 0f32);
    assert_eq!(taffy.layout(node0).unwrap().size.width, 40f32);
    assert_eq!(taffy.layout(node0).unwrap().size.height, 20f32);
    assert_eq!(taffy.layout(node0).unwrap().location.x, 0f32);
    assert_eq!(taffy.layout(node0).unwrap().location.y, 0f32);
    assert_eq!(taffy.layout(node1).unwrap().size.width, 40f32);
    assert_eq!(taffy.layout(node1).unwrap().size.height, 20f32);
    assert_eq!(taffy.layout(node1).unwrap().location.x, 40f32);
    assert_eq!(taffy.layout(node1).unwrap().location.y, 0f32);
    assert_eq!(taffy.layout(node2).unwrap().size.width, 40f32);
    assert_eq!(taffy.layout(node2).unwrap().size.height, 20f32);
    assert_eq!(taffy.layout(node2).unwrap().location.x, 80f32);
    assert_eq!(taffy.layout(node2).unwrap().location.y, 0f32);
    assert_eq!(taffy.layout(node3).unwrap().size.width, 20f32);
    assert_eq!(taffy.layout(node3).unwrap().size.height, 40f32);
    assert_eq!(taffy.layout(node3).unwrap().location.x, 0f32);
    assert_eq!(taffy.layout(node3).unwrap().location.y, 40f32);
    assert_eq!(taffy.layout(node4).unwrap().size.width, 30f32);
    assert_eq!(taffy.layout(node4).unwrap().size.height, 40f32);
    assert_eq!(taffy.layout(node4).unwrap().location.x, 40f32);
    assert_eq!(taffy.layout(node4).unwrap().location.y, 40f32);
}
