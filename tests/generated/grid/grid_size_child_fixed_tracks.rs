#[test]
#[allow(non_snake_case)]
fn grid_size_child_fixed_tracks__border_box_ltr() {
    #[allow(unused_imports)]
    use taffy::{prelude::*, Layout};
    let mut taffy = crate::new_test_tree();
    let node0 = taffy
        .new_leaf_with_context(
            taffy::style::Style {
                align_self: Some(taffy::style::AlignSelf::Start),
                justify_self: Some(taffy::style::JustifySelf::Start),
                ..Default::default()
            },
            crate::TestNodeContext::ahem_text("HH\u{200b}HH\u{200b}HH\u{200b}HH", crate::WritingMode::Horizontal),
        )
        .unwrap();
    let node1 = taffy
        .new_leaf_with_context(
            taffy::style::Style {
                align_self: Some(taffy::style::AlignSelf::Start),
                justify_self: Some(taffy::style::JustifySelf::Start),
                ..Default::default()
            },
            crate::TestNodeContext::ahem_text("HHH\u{200b}HHH", crate::WritingMode::Horizontal),
        )
        .unwrap();
    let node2 = taffy
        .new_leaf_with_context(
            taffy::style::Style {
                align_self: Some(taffy::style::AlignSelf::Start),
                justify_self: Some(taffy::style::JustifySelf::Start),
                ..Default::default()
            },
            crate::TestNodeContext::ahem_text("HH\u{200b}HHHH", crate::WritingMode::Horizontal),
        )
        .unwrap();
    let node3 = taffy
        .new_leaf_with_context(
            taffy::style::Style {
                align_self: Some(taffy::style::AlignSelf::Start),
                justify_self: Some(taffy::style::JustifySelf::Start),
                size: taffy::geometry::Size { width: taffy::style::Dimension::from_length(20f32), height: auto() },
                ..Default::default()
            },
            crate::TestNodeContext::ahem_text("HH\u{200b}HH\u{200b}HH\u{200b}HH", crate::WritingMode::Horizontal),
        )
        .unwrap();
    let node4 = taffy
        .new_leaf_with_context(
            taffy::style::Style {
                align_self: Some(taffy::style::AlignSelf::Start),
                justify_self: Some(taffy::style::JustifySelf::Start),
                max_size: taffy::geometry::Size { width: taffy::style::Dimension::from_length(30f32), height: auto() },
                ..Default::default()
            },
            crate::TestNodeContext::ahem_text("HH\u{200b}HH\u{200b}HH\u{200b}HH", crate::WritingMode::Horizontal),
        )
        .unwrap();
    let node = taffy
        .new_with_children(
            taffy::style::Style {
                display: taffy::style::Display::Grid,
                grid_template_rows: vec![length(40f32), length(40f32), length(40f32)],
                grid_template_columns: vec![length(40f32), length(40f32), length(40f32)],
                size: taffy::geometry::Size {
                    width: taffy::style::Dimension::from_length(120f32),
                    height: taffy::style::Dimension::from_length(120f32),
                },
                ..Default::default()
            },
            &[node0, node1, node2, node3, node4],
        )
        .unwrap();
    taffy.compute_layout_with_measure(node, taffy::geometry::Size::MAX_CONTENT, crate::test_measure_function).unwrap();
    println!("\nComputed tree:");
    taffy.print_tree(node);
    println!();
    let layout = taffy.layout(node).unwrap();
    let Layout { size, location, .. } = layout;
    assert_eq!(size.width, 120f32, "width of node {:?}. Expected {}. Actual {}", node, 120f32, size.width);
    assert_eq!(size.height, 120f32, "height of node {:?}. Expected {}. Actual {}", node, 120f32, size.height);
    assert_eq!(location.x, 0f32, "x of node {:?}. Expected {}. Actual {}", node, 0f32, location.x);
    assert_eq!(location.y, 0f32, "y of node {:?}. Expected {}. Actual {}", node, 0f32, location.y);
    let layout = taffy.layout(node0).unwrap();
    let Layout { size, location, .. } = layout;
    assert_eq!(size.width, 40f32, "width of node {:?}. Expected {}. Actual {}", node0, 40f32, size.width);
    assert_eq!(size.height, 20f32, "height of node {:?}. Expected {}. Actual {}", node0, 20f32, size.height);
    assert_eq!(location.x, 0f32, "x of node {:?}. Expected {}. Actual {}", node0, 0f32, location.x);
    assert_eq!(location.y, 0f32, "y of node {:?}. Expected {}. Actual {}", node0, 0f32, location.y);
    let layout = taffy.layout(node1).unwrap();
    let Layout { size, location, .. } = layout;
    assert_eq!(size.width, 40f32, "width of node {:?}. Expected {}. Actual {}", node1, 40f32, size.width);
    assert_eq!(size.height, 20f32, "height of node {:?}. Expected {}. Actual {}", node1, 20f32, size.height);
    assert_eq!(location.x, 40f32, "x of node {:?}. Expected {}. Actual {}", node1, 40f32, location.x);
    assert_eq!(location.y, 0f32, "y of node {:?}. Expected {}. Actual {}", node1, 0f32, location.y);
    let layout = taffy.layout(node2).unwrap();
    let Layout { size, location, .. } = layout;
    assert_eq!(size.width, 40f32, "width of node {:?}. Expected {}. Actual {}", node2, 40f32, size.width);
    assert_eq!(size.height, 20f32, "height of node {:?}. Expected {}. Actual {}", node2, 20f32, size.height);
    assert_eq!(location.x, 80f32, "x of node {:?}. Expected {}. Actual {}", node2, 80f32, location.x);
    assert_eq!(location.y, 0f32, "y of node {:?}. Expected {}. Actual {}", node2, 0f32, location.y);
    let layout = taffy.layout(node3).unwrap();
    let Layout { size, location, .. } = layout;
    assert_eq!(size.width, 20f32, "width of node {:?}. Expected {}. Actual {}", node3, 20f32, size.width);
    assert_eq!(size.height, 40f32, "height of node {:?}. Expected {}. Actual {}", node3, 40f32, size.height);
    assert_eq!(location.x, 0f32, "x of node {:?}. Expected {}. Actual {}", node3, 0f32, location.x);
    assert_eq!(location.y, 40f32, "y of node {:?}. Expected {}. Actual {}", node3, 40f32, location.y);
    let layout = taffy.layout(node4).unwrap();
    let Layout { size, location, .. } = layout;
    assert_eq!(size.width, 30f32, "width of node {:?}. Expected {}. Actual {}", node4, 30f32, size.width);
    assert_eq!(size.height, 40f32, "height of node {:?}. Expected {}. Actual {}", node4, 40f32, size.height);
    assert_eq!(location.x, 40f32, "x of node {:?}. Expected {}. Actual {}", node4, 40f32, location.x);
    assert_eq!(location.y, 40f32, "y of node {:?}. Expected {}. Actual {}", node4, 40f32, location.y);
}

#[test]
#[allow(non_snake_case)]
fn grid_size_child_fixed_tracks__content_box_ltr() {
    #[allow(unused_imports)]
    use taffy::{prelude::*, Layout};
    let mut taffy = crate::new_test_tree();
    let node0 = taffy
        .new_leaf_with_context(
            taffy::style::Style {
                box_sizing: taffy::style::BoxSizing::ContentBox,
                align_self: Some(taffy::style::AlignSelf::Start),
                justify_self: Some(taffy::style::JustifySelf::Start),
                ..Default::default()
            },
            crate::TestNodeContext::ahem_text("HH\u{200b}HH\u{200b}HH\u{200b}HH", crate::WritingMode::Horizontal),
        )
        .unwrap();
    let node1 = taffy
        .new_leaf_with_context(
            taffy::style::Style {
                box_sizing: taffy::style::BoxSizing::ContentBox,
                align_self: Some(taffy::style::AlignSelf::Start),
                justify_self: Some(taffy::style::JustifySelf::Start),
                ..Default::default()
            },
            crate::TestNodeContext::ahem_text("HHH\u{200b}HHH", crate::WritingMode::Horizontal),
        )
        .unwrap();
    let node2 = taffy
        .new_leaf_with_context(
            taffy::style::Style {
                box_sizing: taffy::style::BoxSizing::ContentBox,
                align_self: Some(taffy::style::AlignSelf::Start),
                justify_self: Some(taffy::style::JustifySelf::Start),
                ..Default::default()
            },
            crate::TestNodeContext::ahem_text("HH\u{200b}HHHH", crate::WritingMode::Horizontal),
        )
        .unwrap();
    let node3 = taffy
        .new_leaf_with_context(
            taffy::style::Style {
                box_sizing: taffy::style::BoxSizing::ContentBox,
                align_self: Some(taffy::style::AlignSelf::Start),
                justify_self: Some(taffy::style::JustifySelf::Start),
                size: taffy::geometry::Size { width: taffy::style::Dimension::from_length(20f32), height: auto() },
                ..Default::default()
            },
            crate::TestNodeContext::ahem_text("HH\u{200b}HH\u{200b}HH\u{200b}HH", crate::WritingMode::Horizontal),
        )
        .unwrap();
    let node4 = taffy
        .new_leaf_with_context(
            taffy::style::Style {
                box_sizing: taffy::style::BoxSizing::ContentBox,
                align_self: Some(taffy::style::AlignSelf::Start),
                justify_self: Some(taffy::style::JustifySelf::Start),
                max_size: taffy::geometry::Size { width: taffy::style::Dimension::from_length(30f32), height: auto() },
                ..Default::default()
            },
            crate::TestNodeContext::ahem_text("HH\u{200b}HH\u{200b}HH\u{200b}HH", crate::WritingMode::Horizontal),
        )
        .unwrap();
    let node = taffy
        .new_with_children(
            taffy::style::Style {
                display: taffy::style::Display::Grid,
                box_sizing: taffy::style::BoxSizing::ContentBox,
                grid_template_rows: vec![length(40f32), length(40f32), length(40f32)],
                grid_template_columns: vec![length(40f32), length(40f32), length(40f32)],
                size: taffy::geometry::Size {
                    width: taffy::style::Dimension::from_length(120f32),
                    height: taffy::style::Dimension::from_length(120f32),
                },
                ..Default::default()
            },
            &[node0, node1, node2, node3, node4],
        )
        .unwrap();
    taffy.compute_layout_with_measure(node, taffy::geometry::Size::MAX_CONTENT, crate::test_measure_function).unwrap();
    println!("\nComputed tree:");
    taffy.print_tree(node);
    println!();
    let layout = taffy.layout(node).unwrap();
    let Layout { size, location, .. } = layout;
    assert_eq!(size.width, 120f32, "width of node {:?}. Expected {}. Actual {}", node, 120f32, size.width);
    assert_eq!(size.height, 120f32, "height of node {:?}. Expected {}. Actual {}", node, 120f32, size.height);
    assert_eq!(location.x, 0f32, "x of node {:?}. Expected {}. Actual {}", node, 0f32, location.x);
    assert_eq!(location.y, 0f32, "y of node {:?}. Expected {}. Actual {}", node, 0f32, location.y);
    let layout = taffy.layout(node0).unwrap();
    let Layout { size, location, .. } = layout;
    assert_eq!(size.width, 40f32, "width of node {:?}. Expected {}. Actual {}", node0, 40f32, size.width);
    assert_eq!(size.height, 20f32, "height of node {:?}. Expected {}. Actual {}", node0, 20f32, size.height);
    assert_eq!(location.x, 0f32, "x of node {:?}. Expected {}. Actual {}", node0, 0f32, location.x);
    assert_eq!(location.y, 0f32, "y of node {:?}. Expected {}. Actual {}", node0, 0f32, location.y);
    let layout = taffy.layout(node1).unwrap();
    let Layout { size, location, .. } = layout;
    assert_eq!(size.width, 40f32, "width of node {:?}. Expected {}. Actual {}", node1, 40f32, size.width);
    assert_eq!(size.height, 20f32, "height of node {:?}. Expected {}. Actual {}", node1, 20f32, size.height);
    assert_eq!(location.x, 40f32, "x of node {:?}. Expected {}. Actual {}", node1, 40f32, location.x);
    assert_eq!(location.y, 0f32, "y of node {:?}. Expected {}. Actual {}", node1, 0f32, location.y);
    let layout = taffy.layout(node2).unwrap();
    let Layout { size, location, .. } = layout;
    assert_eq!(size.width, 40f32, "width of node {:?}. Expected {}. Actual {}", node2, 40f32, size.width);
    assert_eq!(size.height, 20f32, "height of node {:?}. Expected {}. Actual {}", node2, 20f32, size.height);
    assert_eq!(location.x, 80f32, "x of node {:?}. Expected {}. Actual {}", node2, 80f32, location.x);
    assert_eq!(location.y, 0f32, "y of node {:?}. Expected {}. Actual {}", node2, 0f32, location.y);
    let layout = taffy.layout(node3).unwrap();
    let Layout { size, location, .. } = layout;
    assert_eq!(size.width, 20f32, "width of node {:?}. Expected {}. Actual {}", node3, 20f32, size.width);
    assert_eq!(size.height, 40f32, "height of node {:?}. Expected {}. Actual {}", node3, 40f32, size.height);
    assert_eq!(location.x, 0f32, "x of node {:?}. Expected {}. Actual {}", node3, 0f32, location.x);
    assert_eq!(location.y, 40f32, "y of node {:?}. Expected {}. Actual {}", node3, 40f32, location.y);
    let layout = taffy.layout(node4).unwrap();
    let Layout { size, location, .. } = layout;
    assert_eq!(size.width, 30f32, "width of node {:?}. Expected {}. Actual {}", node4, 30f32, size.width);
    assert_eq!(size.height, 40f32, "height of node {:?}. Expected {}. Actual {}", node4, 40f32, size.height);
    assert_eq!(location.x, 40f32, "x of node {:?}. Expected {}. Actual {}", node4, 40f32, location.x);
    assert_eq!(location.y, 40f32, "y of node {:?}. Expected {}. Actual {}", node4, 40f32, location.y);
}

#[test]
#[allow(non_snake_case)]
fn grid_size_child_fixed_tracks__border_box_rtl() {
    #[allow(unused_imports)]
    use taffy::{prelude::*, Layout};
    let mut taffy = crate::new_test_tree();
    let node0 = taffy
        .new_leaf_with_context(
            taffy::style::Style {
                direction: taffy::style::Direction::Rtl,
                align_self: Some(taffy::style::AlignSelf::Start),
                justify_self: Some(taffy::style::JustifySelf::Start),
                ..Default::default()
            },
            crate::TestNodeContext::ahem_text("HH\u{200b}HH\u{200b}HH\u{200b}HH", crate::WritingMode::Horizontal),
        )
        .unwrap();
    let node1 = taffy
        .new_leaf_with_context(
            taffy::style::Style {
                direction: taffy::style::Direction::Rtl,
                align_self: Some(taffy::style::AlignSelf::Start),
                justify_self: Some(taffy::style::JustifySelf::Start),
                ..Default::default()
            },
            crate::TestNodeContext::ahem_text("HHH\u{200b}HHH", crate::WritingMode::Horizontal),
        )
        .unwrap();
    let node2 = taffy
        .new_leaf_with_context(
            taffy::style::Style {
                direction: taffy::style::Direction::Rtl,
                align_self: Some(taffy::style::AlignSelf::Start),
                justify_self: Some(taffy::style::JustifySelf::Start),
                ..Default::default()
            },
            crate::TestNodeContext::ahem_text("HH\u{200b}HHHH", crate::WritingMode::Horizontal),
        )
        .unwrap();
    let node3 = taffy
        .new_leaf_with_context(
            taffy::style::Style {
                direction: taffy::style::Direction::Rtl,
                align_self: Some(taffy::style::AlignSelf::Start),
                justify_self: Some(taffy::style::JustifySelf::Start),
                size: taffy::geometry::Size { width: taffy::style::Dimension::from_length(20f32), height: auto() },
                ..Default::default()
            },
            crate::TestNodeContext::ahem_text("HH\u{200b}HH\u{200b}HH\u{200b}HH", crate::WritingMode::Horizontal),
        )
        .unwrap();
    let node4 = taffy
        .new_leaf_with_context(
            taffy::style::Style {
                direction: taffy::style::Direction::Rtl,
                align_self: Some(taffy::style::AlignSelf::Start),
                justify_self: Some(taffy::style::JustifySelf::Start),
                max_size: taffy::geometry::Size { width: taffy::style::Dimension::from_length(30f32), height: auto() },
                ..Default::default()
            },
            crate::TestNodeContext::ahem_text("HH\u{200b}HH\u{200b}HH\u{200b}HH", crate::WritingMode::Horizontal),
        )
        .unwrap();
    let node = taffy
        .new_with_children(
            taffy::style::Style {
                display: taffy::style::Display::Grid,
                direction: taffy::style::Direction::Rtl,
                grid_template_rows: vec![length(40f32), length(40f32), length(40f32)],
                grid_template_columns: vec![length(40f32), length(40f32), length(40f32)],
                size: taffy::geometry::Size {
                    width: taffy::style::Dimension::from_length(120f32),
                    height: taffy::style::Dimension::from_length(120f32),
                },
                ..Default::default()
            },
            &[node0, node1, node2, node3, node4],
        )
        .unwrap();
    taffy.compute_layout_with_measure(node, taffy::geometry::Size::MAX_CONTENT, crate::test_measure_function).unwrap();
    println!("\nComputed tree:");
    taffy.print_tree(node);
    println!();
    let layout = taffy.layout(node).unwrap();
    let Layout { size, location, .. } = layout;
    assert_eq!(size.width, 120f32, "width of node {:?}. Expected {}. Actual {}", node, 120f32, size.width);
    assert_eq!(size.height, 120f32, "height of node {:?}. Expected {}. Actual {}", node, 120f32, size.height);
    assert_eq!(location.x, 0f32, "x of node {:?}. Expected {}. Actual {}", node, 0f32, location.x);
    assert_eq!(location.y, 0f32, "y of node {:?}. Expected {}. Actual {}", node, 0f32, location.y);
    let layout = taffy.layout(node0).unwrap();
    let Layout { size, location, .. } = layout;
    assert_eq!(size.width, 40f32, "width of node {:?}. Expected {}. Actual {}", node0, 40f32, size.width);
    assert_eq!(size.height, 20f32, "height of node {:?}. Expected {}. Actual {}", node0, 20f32, size.height);
    assert_eq!(location.x, 80f32, "x of node {:?}. Expected {}. Actual {}", node0, 80f32, location.x);
    assert_eq!(location.y, 0f32, "y of node {:?}. Expected {}. Actual {}", node0, 0f32, location.y);
    let layout = taffy.layout(node1).unwrap();
    let Layout { size, location, .. } = layout;
    assert_eq!(size.width, 40f32, "width of node {:?}. Expected {}. Actual {}", node1, 40f32, size.width);
    assert_eq!(size.height, 20f32, "height of node {:?}. Expected {}. Actual {}", node1, 20f32, size.height);
    assert_eq!(location.x, 40f32, "x of node {:?}. Expected {}. Actual {}", node1, 40f32, location.x);
    assert_eq!(location.y, 0f32, "y of node {:?}. Expected {}. Actual {}", node1, 0f32, location.y);
    let layout = taffy.layout(node2).unwrap();
    let Layout { size, location, .. } = layout;
    assert_eq!(size.width, 40f32, "width of node {:?}. Expected {}. Actual {}", node2, 40f32, size.width);
    assert_eq!(size.height, 20f32, "height of node {:?}. Expected {}. Actual {}", node2, 20f32, size.height);
    assert_eq!(location.x, 0f32, "x of node {:?}. Expected {}. Actual {}", node2, 0f32, location.x);
    assert_eq!(location.y, 0f32, "y of node {:?}. Expected {}. Actual {}", node2, 0f32, location.y);
    let layout = taffy.layout(node3).unwrap();
    let Layout { size, location, .. } = layout;
    assert_eq!(size.width, 20f32, "width of node {:?}. Expected {}. Actual {}", node3, 20f32, size.width);
    assert_eq!(size.height, 40f32, "height of node {:?}. Expected {}. Actual {}", node3, 40f32, size.height);
    assert_eq!(location.x, 100f32, "x of node {:?}. Expected {}. Actual {}", node3, 100f32, location.x);
    assert_eq!(location.y, 40f32, "y of node {:?}. Expected {}. Actual {}", node3, 40f32, location.y);
    let layout = taffy.layout(node4).unwrap();
    let Layout { size, location, .. } = layout;
    assert_eq!(size.width, 30f32, "width of node {:?}. Expected {}. Actual {}", node4, 30f32, size.width);
    assert_eq!(size.height, 40f32, "height of node {:?}. Expected {}. Actual {}", node4, 40f32, size.height);
    assert_eq!(location.x, 50f32, "x of node {:?}. Expected {}. Actual {}", node4, 50f32, location.x);
    assert_eq!(location.y, 40f32, "y of node {:?}. Expected {}. Actual {}", node4, 40f32, location.y);
}

#[test]
#[allow(non_snake_case)]
fn grid_size_child_fixed_tracks__content_box_rtl() {
    #[allow(unused_imports)]
    use taffy::{prelude::*, Layout};
    let mut taffy = crate::new_test_tree();
    let node0 = taffy
        .new_leaf_with_context(
            taffy::style::Style {
                box_sizing: taffy::style::BoxSizing::ContentBox,
                direction: taffy::style::Direction::Rtl,
                align_self: Some(taffy::style::AlignSelf::Start),
                justify_self: Some(taffy::style::JustifySelf::Start),
                ..Default::default()
            },
            crate::TestNodeContext::ahem_text("HH\u{200b}HH\u{200b}HH\u{200b}HH", crate::WritingMode::Horizontal),
        )
        .unwrap();
    let node1 = taffy
        .new_leaf_with_context(
            taffy::style::Style {
                box_sizing: taffy::style::BoxSizing::ContentBox,
                direction: taffy::style::Direction::Rtl,
                align_self: Some(taffy::style::AlignSelf::Start),
                justify_self: Some(taffy::style::JustifySelf::Start),
                ..Default::default()
            },
            crate::TestNodeContext::ahem_text("HHH\u{200b}HHH", crate::WritingMode::Horizontal),
        )
        .unwrap();
    let node2 = taffy
        .new_leaf_with_context(
            taffy::style::Style {
                box_sizing: taffy::style::BoxSizing::ContentBox,
                direction: taffy::style::Direction::Rtl,
                align_self: Some(taffy::style::AlignSelf::Start),
                justify_self: Some(taffy::style::JustifySelf::Start),
                ..Default::default()
            },
            crate::TestNodeContext::ahem_text("HH\u{200b}HHHH", crate::WritingMode::Horizontal),
        )
        .unwrap();
    let node3 = taffy
        .new_leaf_with_context(
            taffy::style::Style {
                box_sizing: taffy::style::BoxSizing::ContentBox,
                direction: taffy::style::Direction::Rtl,
                align_self: Some(taffy::style::AlignSelf::Start),
                justify_self: Some(taffy::style::JustifySelf::Start),
                size: taffy::geometry::Size { width: taffy::style::Dimension::from_length(20f32), height: auto() },
                ..Default::default()
            },
            crate::TestNodeContext::ahem_text("HH\u{200b}HH\u{200b}HH\u{200b}HH", crate::WritingMode::Horizontal),
        )
        .unwrap();
    let node4 = taffy
        .new_leaf_with_context(
            taffy::style::Style {
                box_sizing: taffy::style::BoxSizing::ContentBox,
                direction: taffy::style::Direction::Rtl,
                align_self: Some(taffy::style::AlignSelf::Start),
                justify_self: Some(taffy::style::JustifySelf::Start),
                max_size: taffy::geometry::Size { width: taffy::style::Dimension::from_length(30f32), height: auto() },
                ..Default::default()
            },
            crate::TestNodeContext::ahem_text("HH\u{200b}HH\u{200b}HH\u{200b}HH", crate::WritingMode::Horizontal),
        )
        .unwrap();
    let node = taffy
        .new_with_children(
            taffy::style::Style {
                display: taffy::style::Display::Grid,
                box_sizing: taffy::style::BoxSizing::ContentBox,
                direction: taffy::style::Direction::Rtl,
                grid_template_rows: vec![length(40f32), length(40f32), length(40f32)],
                grid_template_columns: vec![length(40f32), length(40f32), length(40f32)],
                size: taffy::geometry::Size {
                    width: taffy::style::Dimension::from_length(120f32),
                    height: taffy::style::Dimension::from_length(120f32),
                },
                ..Default::default()
            },
            &[node0, node1, node2, node3, node4],
        )
        .unwrap();
    taffy.compute_layout_with_measure(node, taffy::geometry::Size::MAX_CONTENT, crate::test_measure_function).unwrap();
    println!("\nComputed tree:");
    taffy.print_tree(node);
    println!();
    let layout = taffy.layout(node).unwrap();
    let Layout { size, location, .. } = layout;
    assert_eq!(size.width, 120f32, "width of node {:?}. Expected {}. Actual {}", node, 120f32, size.width);
    assert_eq!(size.height, 120f32, "height of node {:?}. Expected {}. Actual {}", node, 120f32, size.height);
    assert_eq!(location.x, 0f32, "x of node {:?}. Expected {}. Actual {}", node, 0f32, location.x);
    assert_eq!(location.y, 0f32, "y of node {:?}. Expected {}. Actual {}", node, 0f32, location.y);
    let layout = taffy.layout(node0).unwrap();
    let Layout { size, location, .. } = layout;
    assert_eq!(size.width, 40f32, "width of node {:?}. Expected {}. Actual {}", node0, 40f32, size.width);
    assert_eq!(size.height, 20f32, "height of node {:?}. Expected {}. Actual {}", node0, 20f32, size.height);
    assert_eq!(location.x, 80f32, "x of node {:?}. Expected {}. Actual {}", node0, 80f32, location.x);
    assert_eq!(location.y, 0f32, "y of node {:?}. Expected {}. Actual {}", node0, 0f32, location.y);
    let layout = taffy.layout(node1).unwrap();
    let Layout { size, location, .. } = layout;
    assert_eq!(size.width, 40f32, "width of node {:?}. Expected {}. Actual {}", node1, 40f32, size.width);
    assert_eq!(size.height, 20f32, "height of node {:?}. Expected {}. Actual {}", node1, 20f32, size.height);
    assert_eq!(location.x, 40f32, "x of node {:?}. Expected {}. Actual {}", node1, 40f32, location.x);
    assert_eq!(location.y, 0f32, "y of node {:?}. Expected {}. Actual {}", node1, 0f32, location.y);
    let layout = taffy.layout(node2).unwrap();
    let Layout { size, location, .. } = layout;
    assert_eq!(size.width, 40f32, "width of node {:?}. Expected {}. Actual {}", node2, 40f32, size.width);
    assert_eq!(size.height, 20f32, "height of node {:?}. Expected {}. Actual {}", node2, 20f32, size.height);
    assert_eq!(location.x, 0f32, "x of node {:?}. Expected {}. Actual {}", node2, 0f32, location.x);
    assert_eq!(location.y, 0f32, "y of node {:?}. Expected {}. Actual {}", node2, 0f32, location.y);
    let layout = taffy.layout(node3).unwrap();
    let Layout { size, location, .. } = layout;
    assert_eq!(size.width, 20f32, "width of node {:?}. Expected {}. Actual {}", node3, 20f32, size.width);
    assert_eq!(size.height, 40f32, "height of node {:?}. Expected {}. Actual {}", node3, 40f32, size.height);
    assert_eq!(location.x, 100f32, "x of node {:?}. Expected {}. Actual {}", node3, 100f32, location.x);
    assert_eq!(location.y, 40f32, "y of node {:?}. Expected {}. Actual {}", node3, 40f32, location.y);
    let layout = taffy.layout(node4).unwrap();
    let Layout { size, location, .. } = layout;
    assert_eq!(size.width, 30f32, "width of node {:?}. Expected {}. Actual {}", node4, 30f32, size.width);
    assert_eq!(size.height, 40f32, "height of node {:?}. Expected {}. Actual {}", node4, 40f32, size.height);
    assert_eq!(location.x, 50f32, "x of node {:?}. Expected {}. Actual {}", node4, 50f32, location.x);
    assert_eq!(location.y, 40f32, "y of node {:?}. Expected {}. Actual {}", node4, 40f32, location.y);
}
