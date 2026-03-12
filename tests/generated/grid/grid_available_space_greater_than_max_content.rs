#[test]
#[allow(non_snake_case)]
fn grid_available_space_greater_than_max_content__border_box_ltr() {
    #[allow(unused_imports)]
    use taffy::{prelude::*, Layout};
    let mut taffy = crate::new_test_tree();
    let node0 = taffy
        .new_leaf_with_context(
            taffy::style::Style { ..Default::default() },
            crate::TestNodeContext::ahem_text("HH\u{200b}HH\u{200b}HH\u{200b}HH", crate::WritingMode::Horizontal),
        )
        .unwrap();
    let node1 = taffy
        .new_leaf_with_context(
            taffy::style::Style { ..Default::default() },
            crate::TestNodeContext::ahem_text("HH\u{200b}HH\u{200b}HH\u{200b}HH", crate::WritingMode::Horizontal),
        )
        .unwrap();
    let node = taffy
        .new_with_children(
            taffy::style::Style {
                display: taffy::style::Display::Grid,
                grid_template_columns: vec![auto(), auto()],
                ..Default::default()
            },
            &[node0, node1],
        )
        .unwrap();
    taffy
        .compute_layout_with_measure(
            node,
            taffy::geometry::Size {
                width: taffy::style::AvailableSpace::Definite(400f32),
                height: taffy::style::AvailableSpace::MaxContent,
            },
            crate::test_measure_function,
        )
        .unwrap();
    println!("\nComputed tree:");
    taffy.print_tree(node);
    println!();
    let layout = taffy.layout(node).unwrap();
    let Layout { size, location, .. } = layout;
    assert_eq!(size.width, 160f32, "width of node {:?}. Expected {}. Actual {}", node, 160f32, size.width);
    assert_eq!(size.height, 10f32, "height of node {:?}. Expected {}. Actual {}", node, 10f32, size.height);
    assert_eq!(location.x, 0f32, "x of node {:?}. Expected {}. Actual {}", node, 0f32, location.x);
    assert_eq!(location.y, 0f32, "y of node {:?}. Expected {}. Actual {}", node, 0f32, location.y);
    let layout = taffy.layout(node0).unwrap();
    let Layout { size, location, .. } = layout;
    assert_eq!(size.width, 80f32, "width of node {:?}. Expected {}. Actual {}", node0, 80f32, size.width);
    assert_eq!(size.height, 10f32, "height of node {:?}. Expected {}. Actual {}", node0, 10f32, size.height);
    assert_eq!(location.x, 0f32, "x of node {:?}. Expected {}. Actual {}", node0, 0f32, location.x);
    assert_eq!(location.y, 0f32, "y of node {:?}. Expected {}. Actual {}", node0, 0f32, location.y);
    let layout = taffy.layout(node1).unwrap();
    let Layout { size, location, .. } = layout;
    assert_eq!(size.width, 80f32, "width of node {:?}. Expected {}. Actual {}", node1, 80f32, size.width);
    assert_eq!(size.height, 10f32, "height of node {:?}. Expected {}. Actual {}", node1, 10f32, size.height);
    assert_eq!(location.x, 80f32, "x of node {:?}. Expected {}. Actual {}", node1, 80f32, location.x);
    assert_eq!(location.y, 0f32, "y of node {:?}. Expected {}. Actual {}", node1, 0f32, location.y);
}

#[test]
#[allow(non_snake_case)]
fn grid_available_space_greater_than_max_content__content_box_ltr() {
    #[allow(unused_imports)]
    use taffy::{prelude::*, Layout};
    let mut taffy = crate::new_test_tree();
    let node0 = taffy
        .new_leaf_with_context(
            taffy::style::Style { box_sizing: taffy::style::BoxSizing::ContentBox, ..Default::default() },
            crate::TestNodeContext::ahem_text("HH\u{200b}HH\u{200b}HH\u{200b}HH", crate::WritingMode::Horizontal),
        )
        .unwrap();
    let node1 = taffy
        .new_leaf_with_context(
            taffy::style::Style { box_sizing: taffy::style::BoxSizing::ContentBox, ..Default::default() },
            crate::TestNodeContext::ahem_text("HH\u{200b}HH\u{200b}HH\u{200b}HH", crate::WritingMode::Horizontal),
        )
        .unwrap();
    let node = taffy
        .new_with_children(
            taffy::style::Style {
                display: taffy::style::Display::Grid,
                box_sizing: taffy::style::BoxSizing::ContentBox,
                grid_template_columns: vec![auto(), auto()],
                ..Default::default()
            },
            &[node0, node1],
        )
        .unwrap();
    taffy
        .compute_layout_with_measure(
            node,
            taffy::geometry::Size {
                width: taffy::style::AvailableSpace::Definite(400f32),
                height: taffy::style::AvailableSpace::MaxContent,
            },
            crate::test_measure_function,
        )
        .unwrap();
    println!("\nComputed tree:");
    taffy.print_tree(node);
    println!();
    let layout = taffy.layout(node).unwrap();
    let Layout { size, location, .. } = layout;
    assert_eq!(size.width, 160f32, "width of node {:?}. Expected {}. Actual {}", node, 160f32, size.width);
    assert_eq!(size.height, 10f32, "height of node {:?}. Expected {}. Actual {}", node, 10f32, size.height);
    assert_eq!(location.x, 0f32, "x of node {:?}. Expected {}. Actual {}", node, 0f32, location.x);
    assert_eq!(location.y, 0f32, "y of node {:?}. Expected {}. Actual {}", node, 0f32, location.y);
    let layout = taffy.layout(node0).unwrap();
    let Layout { size, location, .. } = layout;
    assert_eq!(size.width, 80f32, "width of node {:?}. Expected {}. Actual {}", node0, 80f32, size.width);
    assert_eq!(size.height, 10f32, "height of node {:?}. Expected {}. Actual {}", node0, 10f32, size.height);
    assert_eq!(location.x, 0f32, "x of node {:?}. Expected {}. Actual {}", node0, 0f32, location.x);
    assert_eq!(location.y, 0f32, "y of node {:?}. Expected {}. Actual {}", node0, 0f32, location.y);
    let layout = taffy.layout(node1).unwrap();
    let Layout { size, location, .. } = layout;
    assert_eq!(size.width, 80f32, "width of node {:?}. Expected {}. Actual {}", node1, 80f32, size.width);
    assert_eq!(size.height, 10f32, "height of node {:?}. Expected {}. Actual {}", node1, 10f32, size.height);
    assert_eq!(location.x, 80f32, "x of node {:?}. Expected {}. Actual {}", node1, 80f32, location.x);
    assert_eq!(location.y, 0f32, "y of node {:?}. Expected {}. Actual {}", node1, 0f32, location.y);
}

#[test]
#[allow(non_snake_case)]
fn grid_available_space_greater_than_max_content__border_box_rtl() {
    #[allow(unused_imports)]
    use taffy::{prelude::*, Layout};
    let mut taffy = crate::new_test_tree();
    let node0 = taffy
        .new_leaf_with_context(
            taffy::style::Style { direction: taffy::style::Direction::Rtl, ..Default::default() },
            crate::TestNodeContext::ahem_text("HH\u{200b}HH\u{200b}HH\u{200b}HH", crate::WritingMode::Horizontal),
        )
        .unwrap();
    let node1 = taffy
        .new_leaf_with_context(
            taffy::style::Style { direction: taffy::style::Direction::Rtl, ..Default::default() },
            crate::TestNodeContext::ahem_text("HH\u{200b}HH\u{200b}HH\u{200b}HH", crate::WritingMode::Horizontal),
        )
        .unwrap();
    let node = taffy
        .new_with_children(
            taffy::style::Style {
                display: taffy::style::Display::Grid,
                direction: taffy::style::Direction::Rtl,
                grid_template_columns: vec![auto(), auto()],
                ..Default::default()
            },
            &[node0, node1],
        )
        .unwrap();
    taffy
        .compute_layout_with_measure(
            node,
            taffy::geometry::Size {
                width: taffy::style::AvailableSpace::Definite(400f32),
                height: taffy::style::AvailableSpace::MaxContent,
            },
            crate::test_measure_function,
        )
        .unwrap();
    println!("\nComputed tree:");
    taffy.print_tree(node);
    println!();
    let layout = taffy.layout(node).unwrap();
    let Layout { size, location, .. } = layout;
    assert_eq!(size.width, 160f32, "width of node {:?}. Expected {}. Actual {}", node, 160f32, size.width);
    assert_eq!(size.height, 10f32, "height of node {:?}. Expected {}. Actual {}", node, 10f32, size.height);
    assert_eq!(location.x, 240f32, "x of node {:?}. Expected {}. Actual {}", node, 240f32, location.x);
    assert_eq!(location.y, 0f32, "y of node {:?}. Expected {}. Actual {}", node, 0f32, location.y);
    let layout = taffy.layout(node0).unwrap();
    let Layout { size, location, .. } = layout;
    assert_eq!(size.width, 80f32, "width of node {:?}. Expected {}. Actual {}", node0, 80f32, size.width);
    assert_eq!(size.height, 10f32, "height of node {:?}. Expected {}. Actual {}", node0, 10f32, size.height);
    assert_eq!(location.x, 80f32, "x of node {:?}. Expected {}. Actual {}", node0, 80f32, location.x);
    assert_eq!(location.y, 0f32, "y of node {:?}. Expected {}. Actual {}", node0, 0f32, location.y);
    let layout = taffy.layout(node1).unwrap();
    let Layout { size, location, .. } = layout;
    assert_eq!(size.width, 80f32, "width of node {:?}. Expected {}. Actual {}", node1, 80f32, size.width);
    assert_eq!(size.height, 10f32, "height of node {:?}. Expected {}. Actual {}", node1, 10f32, size.height);
    assert_eq!(location.x, 0f32, "x of node {:?}. Expected {}. Actual {}", node1, 0f32, location.x);
    assert_eq!(location.y, 0f32, "y of node {:?}. Expected {}. Actual {}", node1, 0f32, location.y);
}

#[test]
#[allow(non_snake_case)]
fn grid_available_space_greater_than_max_content__content_box_rtl() {
    #[allow(unused_imports)]
    use taffy::{prelude::*, Layout};
    let mut taffy = crate::new_test_tree();
    let node0 = taffy
        .new_leaf_with_context(
            taffy::style::Style {
                box_sizing: taffy::style::BoxSizing::ContentBox,
                direction: taffy::style::Direction::Rtl,
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
                grid_template_columns: vec![auto(), auto()],
                ..Default::default()
            },
            &[node0, node1],
        )
        .unwrap();
    taffy
        .compute_layout_with_measure(
            node,
            taffy::geometry::Size {
                width: taffy::style::AvailableSpace::Definite(400f32),
                height: taffy::style::AvailableSpace::MaxContent,
            },
            crate::test_measure_function,
        )
        .unwrap();
    println!("\nComputed tree:");
    taffy.print_tree(node);
    println!();
    let layout = taffy.layout(node).unwrap();
    let Layout { size, location, .. } = layout;
    assert_eq!(size.width, 160f32, "width of node {:?}. Expected {}. Actual {}", node, 160f32, size.width);
    assert_eq!(size.height, 10f32, "height of node {:?}. Expected {}. Actual {}", node, 10f32, size.height);
    assert_eq!(location.x, 240f32, "x of node {:?}. Expected {}. Actual {}", node, 240f32, location.x);
    assert_eq!(location.y, 0f32, "y of node {:?}. Expected {}. Actual {}", node, 0f32, location.y);
    let layout = taffy.layout(node0).unwrap();
    let Layout { size, location, .. } = layout;
    assert_eq!(size.width, 80f32, "width of node {:?}. Expected {}. Actual {}", node0, 80f32, size.width);
    assert_eq!(size.height, 10f32, "height of node {:?}. Expected {}. Actual {}", node0, 10f32, size.height);
    assert_eq!(location.x, 80f32, "x of node {:?}. Expected {}. Actual {}", node0, 80f32, location.x);
    assert_eq!(location.y, 0f32, "y of node {:?}. Expected {}. Actual {}", node0, 0f32, location.y);
    let layout = taffy.layout(node1).unwrap();
    let Layout { size, location, .. } = layout;
    assert_eq!(size.width, 80f32, "width of node {:?}. Expected {}. Actual {}", node1, 80f32, size.width);
    assert_eq!(size.height, 10f32, "height of node {:?}. Expected {}. Actual {}", node1, 10f32, size.height);
    assert_eq!(location.x, 0f32, "x of node {:?}. Expected {}. Actual {}", node1, 0f32, location.x);
    assert_eq!(location.y, 0f32, "y of node {:?}. Expected {}. Actual {}", node1, 0f32, location.y);
}
