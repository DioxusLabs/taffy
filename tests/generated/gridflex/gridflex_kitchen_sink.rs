#[test]
#[allow(non_snake_case)]
fn gridflex_kitchen_sink__border_box_ltr() {
    #[allow(unused_imports)]
    use taffy::{prelude::*, Layout};
    let mut taffy = crate::new_test_tree();
    let node000 = taffy
        .new_leaf(taffy::style::Style {
            size: taffy::geometry::Size { width: taffy::style::Dimension::from_length(20f32), height: auto() },
            ..Default::default()
        })
        .unwrap();
    let node00100 = taffy
        .new_leaf(taffy::style::Style {
            size: taffy::geometry::Size { width: taffy::style::Dimension::from_length(20f32), height: auto() },
            ..Default::default()
        })
        .unwrap();
    let node00101 = taffy.new_leaf(taffy::style::Style { ..Default::default() }).unwrap();
    let node00102 = taffy.new_leaf(taffy::style::Style { ..Default::default() }).unwrap();
    let node00103 = taffy.new_leaf(taffy::style::Style { ..Default::default() }).unwrap();
    let node0010 = taffy
        .new_with_children(
            taffy::style::Style {
                display: taffy::style::Display::Grid,
                grid_template_rows: vec![percent(0.3f32), percent(0.1f32)],
                grid_template_columns: vec![auto(), percent(0.1f32)],
                ..Default::default()
            },
            &[node00100, node00101, node00102, node00103],
        )
        .unwrap();
    let node001 = taffy
        .new_with_children(
            taffy::style::Style {
                flex_grow: 1f32,
                size: taffy::geometry::Size { width: taffy::style::Dimension::from_length(50f32), height: auto() },
                ..Default::default()
            },
            &[node0010],
        )
        .unwrap();
    let node00 = taffy.new_with_children(taffy::style::Style { ..Default::default() }, &[node000, node001]).unwrap();
    let node01 = taffy
        .new_leaf_with_context(
            taffy::style::Style { ..Default::default() },
            crate::TestNodeContext::ahem_text("HH", crate::WritingMode::Horizontal),
        )
        .unwrap();
    let node02 = taffy
        .new_leaf_with_context(
            taffy::style::Style { ..Default::default() },
            crate::TestNodeContext::ahem_text("HH", crate::WritingMode::Horizontal),
        )
        .unwrap();
    let node03 = taffy
        .new_leaf_with_context(
            taffy::style::Style { ..Default::default() },
            crate::TestNodeContext::ahem_text("HH", crate::WritingMode::Horizontal),
        )
        .unwrap();
    let node0 = taffy
        .new_with_children(
            taffy::style::Style {
                display: taffy::style::Display::Grid,
                grid_template_rows: vec![fr(1f32), fr(1f32)],
                grid_template_columns: vec![fr(1f32), fr(1f32)],
                ..Default::default()
            },
            &[node00, node01, node02, node03],
        )
        .unwrap();
    let node = taffy.new_with_children(taffy::style::Style { ..Default::default() }, &[node0]).unwrap();
    taffy.compute_layout_with_measure(node, taffy::geometry::Size::MAX_CONTENT, crate::test_measure_function).unwrap();
    println!("\nComputed tree:");
    taffy.print_tree(node);
    println!();
    let layout = taffy.layout(node).unwrap();
    let Layout { size, location, .. } = layout;
    assert_eq!(size.width, 140f32, "width of node {:?}. Expected {}. Actual {}", node, 140f32, size.width);
    assert_eq!(size.height, 20f32, "height of node {:?}. Expected {}. Actual {}", node, 20f32, size.height);
    assert_eq!(location.x, 0f32, "x of node {:?}. Expected {}. Actual {}", node, 0f32, location.x);
    assert_eq!(location.y, 0f32, "y of node {:?}. Expected {}. Actual {}", node, 0f32, location.y);
    let layout = taffy.layout(node0).unwrap();
    let Layout { size, location, .. } = layout;
    assert_eq!(size.width, 140f32, "width of node {:?}. Expected {}. Actual {}", node0, 140f32, size.width);
    assert_eq!(size.height, 20f32, "height of node {:?}. Expected {}. Actual {}", node0, 20f32, size.height);
    assert_eq!(location.x, 0f32, "x of node {:?}. Expected {}. Actual {}", node0, 0f32, location.x);
    assert_eq!(location.y, 0f32, "y of node {:?}. Expected {}. Actual {}", node0, 0f32, location.y);
    let layout = taffy.layout(node00).unwrap();
    let Layout { size, location, .. } = layout;
    assert_eq!(size.width, 70f32, "width of node {:?}. Expected {}. Actual {}", node00, 70f32, size.width);
    assert_eq!(size.height, 10f32, "height of node {:?}. Expected {}. Actual {}", node00, 10f32, size.height);
    assert_eq!(location.x, 0f32, "x of node {:?}. Expected {}. Actual {}", node00, 0f32, location.x);
    assert_eq!(location.y, 0f32, "y of node {:?}. Expected {}. Actual {}", node00, 0f32, location.y);
    let layout = taffy.layout(node000).unwrap();
    let Layout { size, location, .. } = layout;
    assert_eq!(size.width, 20f32, "width of node {:?}. Expected {}. Actual {}", node000, 20f32, size.width);
    assert_eq!(size.height, 10f32, "height of node {:?}. Expected {}. Actual {}", node000, 10f32, size.height);
    assert_eq!(location.x, 0f32, "x of node {:?}. Expected {}. Actual {}", node000, 0f32, location.x);
    assert_eq!(location.y, 0f32, "y of node {:?}. Expected {}. Actual {}", node000, 0f32, location.y);
    let layout = taffy.layout(node001).unwrap();
    let Layout { size, location, .. } = layout;
    assert_eq!(size.width, 50f32, "width of node {:?}. Expected {}. Actual {}", node001, 50f32, size.width);
    assert_eq!(size.height, 10f32, "height of node {:?}. Expected {}. Actual {}", node001, 10f32, size.height);
    assert_eq!(location.x, 20f32, "x of node {:?}. Expected {}. Actual {}", node001, 20f32, location.x);
    assert_eq!(location.y, 0f32, "y of node {:?}. Expected {}. Actual {}", node001, 0f32, location.y);
    let layout = taffy.layout(node0010).unwrap();
    let Layout { size, location, .. } = layout;
    assert_eq!(size.width, 20f32, "width of node {:?}. Expected {}. Actual {}", node0010, 20f32, size.width);
    assert_eq!(size.height, 10f32, "height of node {:?}. Expected {}. Actual {}", node0010, 10f32, size.height);
    assert_eq!(location.x, 0f32, "x of node {:?}. Expected {}. Actual {}", node0010, 0f32, location.x);
    assert_eq!(location.y, 0f32, "y of node {:?}. Expected {}. Actual {}", node0010, 0f32, location.y);
    let layout = taffy.layout(node00100).unwrap();
    let Layout { size, location, .. } = layout;
    assert_eq!(size.width, 20f32, "width of node {:?}. Expected {}. Actual {}", node00100, 20f32, size.width);
    assert_eq!(size.height, 3f32, "height of node {:?}. Expected {}. Actual {}", node00100, 3f32, size.height);
    assert_eq!(location.x, 0f32, "x of node {:?}. Expected {}. Actual {}", node00100, 0f32, location.x);
    assert_eq!(location.y, 0f32, "y of node {:?}. Expected {}. Actual {}", node00100, 0f32, location.y);
    let layout = taffy.layout(node00101).unwrap();
    let Layout { size, location, .. } = layout;
    assert_eq!(size.width, 2f32, "width of node {:?}. Expected {}. Actual {}", node00101, 2f32, size.width);
    assert_eq!(size.height, 3f32, "height of node {:?}. Expected {}. Actual {}", node00101, 3f32, size.height);
    assert_eq!(location.x, 20f32, "x of node {:?}. Expected {}. Actual {}", node00101, 20f32, location.x);
    assert_eq!(location.y, 0f32, "y of node {:?}. Expected {}. Actual {}", node00101, 0f32, location.y);
    let layout = taffy.layout(node00102).unwrap();
    let Layout { size, location, .. } = layout;
    assert_eq!(size.width, 20f32, "width of node {:?}. Expected {}. Actual {}", node00102, 20f32, size.width);
    assert_eq!(size.height, 1f32, "height of node {:?}. Expected {}. Actual {}", node00102, 1f32, size.height);
    assert_eq!(location.x, 0f32, "x of node {:?}. Expected {}. Actual {}", node00102, 0f32, location.x);
    assert_eq!(location.y, 3f32, "y of node {:?}. Expected {}. Actual {}", node00102, 3f32, location.y);
    let layout = taffy.layout(node00103).unwrap();
    let Layout { size, location, .. } = layout;
    assert_eq!(size.width, 2f32, "width of node {:?}. Expected {}. Actual {}", node00103, 2f32, size.width);
    assert_eq!(size.height, 1f32, "height of node {:?}. Expected {}. Actual {}", node00103, 1f32, size.height);
    assert_eq!(location.x, 20f32, "x of node {:?}. Expected {}. Actual {}", node00103, 20f32, location.x);
    assert_eq!(location.y, 3f32, "y of node {:?}. Expected {}. Actual {}", node00103, 3f32, location.y);
    let layout = taffy.layout(node01).unwrap();
    let Layout { size, location, .. } = layout;
    assert_eq!(size.width, 70f32, "width of node {:?}. Expected {}. Actual {}", node01, 70f32, size.width);
    assert_eq!(size.height, 10f32, "height of node {:?}. Expected {}. Actual {}", node01, 10f32, size.height);
    assert_eq!(location.x, 70f32, "x of node {:?}. Expected {}. Actual {}", node01, 70f32, location.x);
    assert_eq!(location.y, 0f32, "y of node {:?}. Expected {}. Actual {}", node01, 0f32, location.y);
    let layout = taffy.layout(node02).unwrap();
    let Layout { size, location, .. } = layout;
    assert_eq!(size.width, 70f32, "width of node {:?}. Expected {}. Actual {}", node02, 70f32, size.width);
    assert_eq!(size.height, 10f32, "height of node {:?}. Expected {}. Actual {}", node02, 10f32, size.height);
    assert_eq!(location.x, 0f32, "x of node {:?}. Expected {}. Actual {}", node02, 0f32, location.x);
    assert_eq!(location.y, 10f32, "y of node {:?}. Expected {}. Actual {}", node02, 10f32, location.y);
    let layout = taffy.layout(node03).unwrap();
    let Layout { size, location, .. } = layout;
    assert_eq!(size.width, 70f32, "width of node {:?}. Expected {}. Actual {}", node03, 70f32, size.width);
    assert_eq!(size.height, 10f32, "height of node {:?}. Expected {}. Actual {}", node03, 10f32, size.height);
    assert_eq!(location.x, 70f32, "x of node {:?}. Expected {}. Actual {}", node03, 70f32, location.x);
    assert_eq!(location.y, 10f32, "y of node {:?}. Expected {}. Actual {}", node03, 10f32, location.y);
}

#[test]
#[allow(non_snake_case)]
fn gridflex_kitchen_sink__content_box_ltr() {
    #[allow(unused_imports)]
    use taffy::{prelude::*, Layout};
    let mut taffy = crate::new_test_tree();
    let node000 = taffy
        .new_leaf(taffy::style::Style {
            box_sizing: taffy::style::BoxSizing::ContentBox,
            size: taffy::geometry::Size { width: taffy::style::Dimension::from_length(20f32), height: auto() },
            ..Default::default()
        })
        .unwrap();
    let node00100 = taffy
        .new_leaf(taffy::style::Style {
            box_sizing: taffy::style::BoxSizing::ContentBox,
            size: taffy::geometry::Size { width: taffy::style::Dimension::from_length(20f32), height: auto() },
            ..Default::default()
        })
        .unwrap();
    let node00101 = taffy
        .new_leaf(taffy::style::Style { box_sizing: taffy::style::BoxSizing::ContentBox, ..Default::default() })
        .unwrap();
    let node00102 = taffy
        .new_leaf(taffy::style::Style { box_sizing: taffy::style::BoxSizing::ContentBox, ..Default::default() })
        .unwrap();
    let node00103 = taffy
        .new_leaf(taffy::style::Style { box_sizing: taffy::style::BoxSizing::ContentBox, ..Default::default() })
        .unwrap();
    let node0010 = taffy
        .new_with_children(
            taffy::style::Style {
                display: taffy::style::Display::Grid,
                box_sizing: taffy::style::BoxSizing::ContentBox,
                grid_template_rows: vec![percent(0.3f32), percent(0.1f32)],
                grid_template_columns: vec![auto(), percent(0.1f32)],
                ..Default::default()
            },
            &[node00100, node00101, node00102, node00103],
        )
        .unwrap();
    let node001 = taffy
        .new_with_children(
            taffy::style::Style {
                box_sizing: taffy::style::BoxSizing::ContentBox,
                flex_grow: 1f32,
                size: taffy::geometry::Size { width: taffy::style::Dimension::from_length(50f32), height: auto() },
                ..Default::default()
            },
            &[node0010],
        )
        .unwrap();
    let node00 = taffy
        .new_with_children(
            taffy::style::Style { box_sizing: taffy::style::BoxSizing::ContentBox, ..Default::default() },
            &[node000, node001],
        )
        .unwrap();
    let node01 = taffy
        .new_leaf_with_context(
            taffy::style::Style { box_sizing: taffy::style::BoxSizing::ContentBox, ..Default::default() },
            crate::TestNodeContext::ahem_text("HH", crate::WritingMode::Horizontal),
        )
        .unwrap();
    let node02 = taffy
        .new_leaf_with_context(
            taffy::style::Style { box_sizing: taffy::style::BoxSizing::ContentBox, ..Default::default() },
            crate::TestNodeContext::ahem_text("HH", crate::WritingMode::Horizontal),
        )
        .unwrap();
    let node03 = taffy
        .new_leaf_with_context(
            taffy::style::Style { box_sizing: taffy::style::BoxSizing::ContentBox, ..Default::default() },
            crate::TestNodeContext::ahem_text("HH", crate::WritingMode::Horizontal),
        )
        .unwrap();
    let node0 = taffy
        .new_with_children(
            taffy::style::Style {
                display: taffy::style::Display::Grid,
                box_sizing: taffy::style::BoxSizing::ContentBox,
                grid_template_rows: vec![fr(1f32), fr(1f32)],
                grid_template_columns: vec![fr(1f32), fr(1f32)],
                ..Default::default()
            },
            &[node00, node01, node02, node03],
        )
        .unwrap();
    let node = taffy
        .new_with_children(
            taffy::style::Style { box_sizing: taffy::style::BoxSizing::ContentBox, ..Default::default() },
            &[node0],
        )
        .unwrap();
    taffy.compute_layout_with_measure(node, taffy::geometry::Size::MAX_CONTENT, crate::test_measure_function).unwrap();
    println!("\nComputed tree:");
    taffy.print_tree(node);
    println!();
    let layout = taffy.layout(node).unwrap();
    let Layout { size, location, .. } = layout;
    assert_eq!(size.width, 140f32, "width of node {:?}. Expected {}. Actual {}", node, 140f32, size.width);
    assert_eq!(size.height, 20f32, "height of node {:?}. Expected {}. Actual {}", node, 20f32, size.height);
    assert_eq!(location.x, 0f32, "x of node {:?}. Expected {}. Actual {}", node, 0f32, location.x);
    assert_eq!(location.y, 0f32, "y of node {:?}. Expected {}. Actual {}", node, 0f32, location.y);
    let layout = taffy.layout(node0).unwrap();
    let Layout { size, location, .. } = layout;
    assert_eq!(size.width, 140f32, "width of node {:?}. Expected {}. Actual {}", node0, 140f32, size.width);
    assert_eq!(size.height, 20f32, "height of node {:?}. Expected {}. Actual {}", node0, 20f32, size.height);
    assert_eq!(location.x, 0f32, "x of node {:?}. Expected {}. Actual {}", node0, 0f32, location.x);
    assert_eq!(location.y, 0f32, "y of node {:?}. Expected {}. Actual {}", node0, 0f32, location.y);
    let layout = taffy.layout(node00).unwrap();
    let Layout { size, location, .. } = layout;
    assert_eq!(size.width, 70f32, "width of node {:?}. Expected {}. Actual {}", node00, 70f32, size.width);
    assert_eq!(size.height, 10f32, "height of node {:?}. Expected {}. Actual {}", node00, 10f32, size.height);
    assert_eq!(location.x, 0f32, "x of node {:?}. Expected {}. Actual {}", node00, 0f32, location.x);
    assert_eq!(location.y, 0f32, "y of node {:?}. Expected {}. Actual {}", node00, 0f32, location.y);
    let layout = taffy.layout(node000).unwrap();
    let Layout { size, location, .. } = layout;
    assert_eq!(size.width, 20f32, "width of node {:?}. Expected {}. Actual {}", node000, 20f32, size.width);
    assert_eq!(size.height, 10f32, "height of node {:?}. Expected {}. Actual {}", node000, 10f32, size.height);
    assert_eq!(location.x, 0f32, "x of node {:?}. Expected {}. Actual {}", node000, 0f32, location.x);
    assert_eq!(location.y, 0f32, "y of node {:?}. Expected {}. Actual {}", node000, 0f32, location.y);
    let layout = taffy.layout(node001).unwrap();
    let Layout { size, location, .. } = layout;
    assert_eq!(size.width, 50f32, "width of node {:?}. Expected {}. Actual {}", node001, 50f32, size.width);
    assert_eq!(size.height, 10f32, "height of node {:?}. Expected {}. Actual {}", node001, 10f32, size.height);
    assert_eq!(location.x, 20f32, "x of node {:?}. Expected {}. Actual {}", node001, 20f32, location.x);
    assert_eq!(location.y, 0f32, "y of node {:?}. Expected {}. Actual {}", node001, 0f32, location.y);
    let layout = taffy.layout(node0010).unwrap();
    let Layout { size, location, .. } = layout;
    assert_eq!(size.width, 20f32, "width of node {:?}. Expected {}. Actual {}", node0010, 20f32, size.width);
    assert_eq!(size.height, 10f32, "height of node {:?}. Expected {}. Actual {}", node0010, 10f32, size.height);
    assert_eq!(location.x, 0f32, "x of node {:?}. Expected {}. Actual {}", node0010, 0f32, location.x);
    assert_eq!(location.y, 0f32, "y of node {:?}. Expected {}. Actual {}", node0010, 0f32, location.y);
    let layout = taffy.layout(node00100).unwrap();
    let Layout { size, location, .. } = layout;
    assert_eq!(size.width, 20f32, "width of node {:?}. Expected {}. Actual {}", node00100, 20f32, size.width);
    assert_eq!(size.height, 3f32, "height of node {:?}. Expected {}. Actual {}", node00100, 3f32, size.height);
    assert_eq!(location.x, 0f32, "x of node {:?}. Expected {}. Actual {}", node00100, 0f32, location.x);
    assert_eq!(location.y, 0f32, "y of node {:?}. Expected {}. Actual {}", node00100, 0f32, location.y);
    let layout = taffy.layout(node00101).unwrap();
    let Layout { size, location, .. } = layout;
    assert_eq!(size.width, 2f32, "width of node {:?}. Expected {}. Actual {}", node00101, 2f32, size.width);
    assert_eq!(size.height, 3f32, "height of node {:?}. Expected {}. Actual {}", node00101, 3f32, size.height);
    assert_eq!(location.x, 20f32, "x of node {:?}. Expected {}. Actual {}", node00101, 20f32, location.x);
    assert_eq!(location.y, 0f32, "y of node {:?}. Expected {}. Actual {}", node00101, 0f32, location.y);
    let layout = taffy.layout(node00102).unwrap();
    let Layout { size, location, .. } = layout;
    assert_eq!(size.width, 20f32, "width of node {:?}. Expected {}. Actual {}", node00102, 20f32, size.width);
    assert_eq!(size.height, 1f32, "height of node {:?}. Expected {}. Actual {}", node00102, 1f32, size.height);
    assert_eq!(location.x, 0f32, "x of node {:?}. Expected {}. Actual {}", node00102, 0f32, location.x);
    assert_eq!(location.y, 3f32, "y of node {:?}. Expected {}. Actual {}", node00102, 3f32, location.y);
    let layout = taffy.layout(node00103).unwrap();
    let Layout { size, location, .. } = layout;
    assert_eq!(size.width, 2f32, "width of node {:?}. Expected {}. Actual {}", node00103, 2f32, size.width);
    assert_eq!(size.height, 1f32, "height of node {:?}. Expected {}. Actual {}", node00103, 1f32, size.height);
    assert_eq!(location.x, 20f32, "x of node {:?}. Expected {}. Actual {}", node00103, 20f32, location.x);
    assert_eq!(location.y, 3f32, "y of node {:?}. Expected {}. Actual {}", node00103, 3f32, location.y);
    let layout = taffy.layout(node01).unwrap();
    let Layout { size, location, .. } = layout;
    assert_eq!(size.width, 70f32, "width of node {:?}. Expected {}. Actual {}", node01, 70f32, size.width);
    assert_eq!(size.height, 10f32, "height of node {:?}. Expected {}. Actual {}", node01, 10f32, size.height);
    assert_eq!(location.x, 70f32, "x of node {:?}. Expected {}. Actual {}", node01, 70f32, location.x);
    assert_eq!(location.y, 0f32, "y of node {:?}. Expected {}. Actual {}", node01, 0f32, location.y);
    let layout = taffy.layout(node02).unwrap();
    let Layout { size, location, .. } = layout;
    assert_eq!(size.width, 70f32, "width of node {:?}. Expected {}. Actual {}", node02, 70f32, size.width);
    assert_eq!(size.height, 10f32, "height of node {:?}. Expected {}. Actual {}", node02, 10f32, size.height);
    assert_eq!(location.x, 0f32, "x of node {:?}. Expected {}. Actual {}", node02, 0f32, location.x);
    assert_eq!(location.y, 10f32, "y of node {:?}. Expected {}. Actual {}", node02, 10f32, location.y);
    let layout = taffy.layout(node03).unwrap();
    let Layout { size, location, .. } = layout;
    assert_eq!(size.width, 70f32, "width of node {:?}. Expected {}. Actual {}", node03, 70f32, size.width);
    assert_eq!(size.height, 10f32, "height of node {:?}. Expected {}. Actual {}", node03, 10f32, size.height);
    assert_eq!(location.x, 70f32, "x of node {:?}. Expected {}. Actual {}", node03, 70f32, location.x);
    assert_eq!(location.y, 10f32, "y of node {:?}. Expected {}. Actual {}", node03, 10f32, location.y);
}

#[test]
#[allow(non_snake_case)]
fn gridflex_kitchen_sink__border_box_rtl() {
    #[allow(unused_imports)]
    use taffy::{prelude::*, Layout};
    let mut taffy = crate::new_test_tree();
    let node000 = taffy
        .new_leaf(taffy::style::Style {
            direction: taffy::style::Direction::Rtl,
            size: taffy::geometry::Size { width: taffy::style::Dimension::from_length(20f32), height: auto() },
            ..Default::default()
        })
        .unwrap();
    let node00100 = taffy
        .new_leaf(taffy::style::Style {
            direction: taffy::style::Direction::Rtl,
            size: taffy::geometry::Size { width: taffy::style::Dimension::from_length(20f32), height: auto() },
            ..Default::default()
        })
        .unwrap();
    let node00101 =
        taffy.new_leaf(taffy::style::Style { direction: taffy::style::Direction::Rtl, ..Default::default() }).unwrap();
    let node00102 =
        taffy.new_leaf(taffy::style::Style { direction: taffy::style::Direction::Rtl, ..Default::default() }).unwrap();
    let node00103 =
        taffy.new_leaf(taffy::style::Style { direction: taffy::style::Direction::Rtl, ..Default::default() }).unwrap();
    let node0010 = taffy
        .new_with_children(
            taffy::style::Style {
                display: taffy::style::Display::Grid,
                direction: taffy::style::Direction::Rtl,
                grid_template_rows: vec![percent(0.3f32), percent(0.1f32)],
                grid_template_columns: vec![auto(), percent(0.1f32)],
                ..Default::default()
            },
            &[node00100, node00101, node00102, node00103],
        )
        .unwrap();
    let node001 = taffy
        .new_with_children(
            taffy::style::Style {
                direction: taffy::style::Direction::Rtl,
                flex_grow: 1f32,
                size: taffy::geometry::Size { width: taffy::style::Dimension::from_length(50f32), height: auto() },
                ..Default::default()
            },
            &[node0010],
        )
        .unwrap();
    let node00 = taffy
        .new_with_children(
            taffy::style::Style { direction: taffy::style::Direction::Rtl, ..Default::default() },
            &[node000, node001],
        )
        .unwrap();
    let node01 = taffy
        .new_leaf_with_context(
            taffy::style::Style { direction: taffy::style::Direction::Rtl, ..Default::default() },
            crate::TestNodeContext::ahem_text("HH", crate::WritingMode::Horizontal),
        )
        .unwrap();
    let node02 = taffy
        .new_leaf_with_context(
            taffy::style::Style { direction: taffy::style::Direction::Rtl, ..Default::default() },
            crate::TestNodeContext::ahem_text("HH", crate::WritingMode::Horizontal),
        )
        .unwrap();
    let node03 = taffy
        .new_leaf_with_context(
            taffy::style::Style { direction: taffy::style::Direction::Rtl, ..Default::default() },
            crate::TestNodeContext::ahem_text("HH", crate::WritingMode::Horizontal),
        )
        .unwrap();
    let node0 = taffy
        .new_with_children(
            taffy::style::Style {
                display: taffy::style::Display::Grid,
                direction: taffy::style::Direction::Rtl,
                grid_template_rows: vec![fr(1f32), fr(1f32)],
                grid_template_columns: vec![fr(1f32), fr(1f32)],
                ..Default::default()
            },
            &[node00, node01, node02, node03],
        )
        .unwrap();
    let node = taffy
        .new_with_children(
            taffy::style::Style { direction: taffy::style::Direction::Rtl, ..Default::default() },
            &[node0],
        )
        .unwrap();
    taffy.compute_layout_with_measure(node, taffy::geometry::Size::MAX_CONTENT, crate::test_measure_function).unwrap();
    println!("\nComputed tree:");
    taffy.print_tree(node);
    println!();
    let layout = taffy.layout(node).unwrap();
    let Layout { size, location, .. } = layout;
    assert_eq!(size.width, 140f32, "width of node {:?}. Expected {}. Actual {}", node, 140f32, size.width);
    assert_eq!(size.height, 20f32, "height of node {:?}. Expected {}. Actual {}", node, 20f32, size.height);
    assert_eq!(location.x, 0f32, "x of node {:?}. Expected {}. Actual {}", node, 0f32, location.x);
    assert_eq!(location.y, 0f32, "y of node {:?}. Expected {}. Actual {}", node, 0f32, location.y);
    let layout = taffy.layout(node0).unwrap();
    let Layout { size, location, .. } = layout;
    assert_eq!(size.width, 140f32, "width of node {:?}. Expected {}. Actual {}", node0, 140f32, size.width);
    assert_eq!(size.height, 20f32, "height of node {:?}. Expected {}. Actual {}", node0, 20f32, size.height);
    assert_eq!(location.x, 0f32, "x of node {:?}. Expected {}. Actual {}", node0, 0f32, location.x);
    assert_eq!(location.y, 0f32, "y of node {:?}. Expected {}. Actual {}", node0, 0f32, location.y);
    let layout = taffy.layout(node00).unwrap();
    let Layout { size, location, .. } = layout;
    assert_eq!(size.width, 70f32, "width of node {:?}. Expected {}. Actual {}", node00, 70f32, size.width);
    assert_eq!(size.height, 10f32, "height of node {:?}. Expected {}. Actual {}", node00, 10f32, size.height);
    assert_eq!(location.x, 70f32, "x of node {:?}. Expected {}. Actual {}", node00, 70f32, location.x);
    assert_eq!(location.y, 0f32, "y of node {:?}. Expected {}. Actual {}", node00, 0f32, location.y);
    let layout = taffy.layout(node000).unwrap();
    let Layout { size, location, .. } = layout;
    assert_eq!(size.width, 20f32, "width of node {:?}. Expected {}. Actual {}", node000, 20f32, size.width);
    assert_eq!(size.height, 10f32, "height of node {:?}. Expected {}. Actual {}", node000, 10f32, size.height);
    assert_eq!(location.x, 50f32, "x of node {:?}. Expected {}. Actual {}", node000, 50f32, location.x);
    assert_eq!(location.y, 0f32, "y of node {:?}. Expected {}. Actual {}", node000, 0f32, location.y);
    let layout = taffy.layout(node001).unwrap();
    let Layout { size, location, .. } = layout;
    assert_eq!(size.width, 50f32, "width of node {:?}. Expected {}. Actual {}", node001, 50f32, size.width);
    assert_eq!(size.height, 10f32, "height of node {:?}. Expected {}. Actual {}", node001, 10f32, size.height);
    assert_eq!(location.x, 0f32, "x of node {:?}. Expected {}. Actual {}", node001, 0f32, location.x);
    assert_eq!(location.y, 0f32, "y of node {:?}. Expected {}. Actual {}", node001, 0f32, location.y);
    let layout = taffy.layout(node0010).unwrap();
    let Layout { size, location, .. } = layout;
    assert_eq!(size.width, 20f32, "width of node {:?}. Expected {}. Actual {}", node0010, 20f32, size.width);
    assert_eq!(size.height, 10f32, "height of node {:?}. Expected {}. Actual {}", node0010, 10f32, size.height);
    assert_eq!(location.x, 30f32, "x of node {:?}. Expected {}. Actual {}", node0010, 30f32, location.x);
    assert_eq!(location.y, 0f32, "y of node {:?}. Expected {}. Actual {}", node0010, 0f32, location.y);
    let layout = taffy.layout(node00100).unwrap();
    let Layout { size, location, .. } = layout;
    assert_eq!(size.width, 20f32, "width of node {:?}. Expected {}. Actual {}", node00100, 20f32, size.width);
    assert_eq!(size.height, 3f32, "height of node {:?}. Expected {}. Actual {}", node00100, 3f32, size.height);
    assert_eq!(location.x, 0f32, "x of node {:?}. Expected {}. Actual {}", node00100, 0f32, location.x);
    assert_eq!(location.y, 0f32, "y of node {:?}. Expected {}. Actual {}", node00100, 0f32, location.y);
    let layout = taffy.layout(node00101).unwrap();
    let Layout { size, location, .. } = layout;
    assert_eq!(size.width, 2f32, "width of node {:?}. Expected {}. Actual {}", node00101, 2f32, size.width);
    assert_eq!(size.height, 3f32, "height of node {:?}. Expected {}. Actual {}", node00101, 3f32, size.height);
    assert_eq!(location.x, -2f32, "x of node {:?}. Expected {}. Actual {}", node00101, -2f32, location.x);
    assert_eq!(location.y, 0f32, "y of node {:?}. Expected {}. Actual {}", node00101, 0f32, location.y);
    let layout = taffy.layout(node00102).unwrap();
    let Layout { size, location, .. } = layout;
    assert_eq!(size.width, 20f32, "width of node {:?}. Expected {}. Actual {}", node00102, 20f32, size.width);
    assert_eq!(size.height, 1f32, "height of node {:?}. Expected {}. Actual {}", node00102, 1f32, size.height);
    assert_eq!(location.x, 0f32, "x of node {:?}. Expected {}. Actual {}", node00102, 0f32, location.x);
    assert_eq!(location.y, 3f32, "y of node {:?}. Expected {}. Actual {}", node00102, 3f32, location.y);
    let layout = taffy.layout(node00103).unwrap();
    let Layout { size, location, .. } = layout;
    assert_eq!(size.width, 2f32, "width of node {:?}. Expected {}. Actual {}", node00103, 2f32, size.width);
    assert_eq!(size.height, 1f32, "height of node {:?}. Expected {}. Actual {}", node00103, 1f32, size.height);
    assert_eq!(location.x, -2f32, "x of node {:?}. Expected {}. Actual {}", node00103, -2f32, location.x);
    assert_eq!(location.y, 3f32, "y of node {:?}. Expected {}. Actual {}", node00103, 3f32, location.y);
    let layout = taffy.layout(node01).unwrap();
    let Layout { size, location, .. } = layout;
    assert_eq!(size.width, 70f32, "width of node {:?}. Expected {}. Actual {}", node01, 70f32, size.width);
    assert_eq!(size.height, 10f32, "height of node {:?}. Expected {}. Actual {}", node01, 10f32, size.height);
    assert_eq!(location.x, 0f32, "x of node {:?}. Expected {}. Actual {}", node01, 0f32, location.x);
    assert_eq!(location.y, 0f32, "y of node {:?}. Expected {}. Actual {}", node01, 0f32, location.y);
    let layout = taffy.layout(node02).unwrap();
    let Layout { size, location, .. } = layout;
    assert_eq!(size.width, 70f32, "width of node {:?}. Expected {}. Actual {}", node02, 70f32, size.width);
    assert_eq!(size.height, 10f32, "height of node {:?}. Expected {}. Actual {}", node02, 10f32, size.height);
    assert_eq!(location.x, 70f32, "x of node {:?}. Expected {}. Actual {}", node02, 70f32, location.x);
    assert_eq!(location.y, 10f32, "y of node {:?}. Expected {}. Actual {}", node02, 10f32, location.y);
    let layout = taffy.layout(node03).unwrap();
    let Layout { size, location, .. } = layout;
    assert_eq!(size.width, 70f32, "width of node {:?}. Expected {}. Actual {}", node03, 70f32, size.width);
    assert_eq!(size.height, 10f32, "height of node {:?}. Expected {}. Actual {}", node03, 10f32, size.height);
    assert_eq!(location.x, 0f32, "x of node {:?}. Expected {}. Actual {}", node03, 0f32, location.x);
    assert_eq!(location.y, 10f32, "y of node {:?}. Expected {}. Actual {}", node03, 10f32, location.y);
}

#[test]
#[allow(non_snake_case)]
fn gridflex_kitchen_sink__content_box_rtl() {
    #[allow(unused_imports)]
    use taffy::{prelude::*, Layout};
    let mut taffy = crate::new_test_tree();
    let node000 = taffy
        .new_leaf(taffy::style::Style {
            box_sizing: taffy::style::BoxSizing::ContentBox,
            direction: taffy::style::Direction::Rtl,
            size: taffy::geometry::Size { width: taffy::style::Dimension::from_length(20f32), height: auto() },
            ..Default::default()
        })
        .unwrap();
    let node00100 = taffy
        .new_leaf(taffy::style::Style {
            box_sizing: taffy::style::BoxSizing::ContentBox,
            direction: taffy::style::Direction::Rtl,
            size: taffy::geometry::Size { width: taffy::style::Dimension::from_length(20f32), height: auto() },
            ..Default::default()
        })
        .unwrap();
    let node00101 = taffy
        .new_leaf(taffy::style::Style {
            box_sizing: taffy::style::BoxSizing::ContentBox,
            direction: taffy::style::Direction::Rtl,
            ..Default::default()
        })
        .unwrap();
    let node00102 = taffy
        .new_leaf(taffy::style::Style {
            box_sizing: taffy::style::BoxSizing::ContentBox,
            direction: taffy::style::Direction::Rtl,
            ..Default::default()
        })
        .unwrap();
    let node00103 = taffy
        .new_leaf(taffy::style::Style {
            box_sizing: taffy::style::BoxSizing::ContentBox,
            direction: taffy::style::Direction::Rtl,
            ..Default::default()
        })
        .unwrap();
    let node0010 = taffy
        .new_with_children(
            taffy::style::Style {
                display: taffy::style::Display::Grid,
                box_sizing: taffy::style::BoxSizing::ContentBox,
                direction: taffy::style::Direction::Rtl,
                grid_template_rows: vec![percent(0.3f32), percent(0.1f32)],
                grid_template_columns: vec![auto(), percent(0.1f32)],
                ..Default::default()
            },
            &[node00100, node00101, node00102, node00103],
        )
        .unwrap();
    let node001 = taffy
        .new_with_children(
            taffy::style::Style {
                box_sizing: taffy::style::BoxSizing::ContentBox,
                direction: taffy::style::Direction::Rtl,
                flex_grow: 1f32,
                size: taffy::geometry::Size { width: taffy::style::Dimension::from_length(50f32), height: auto() },
                ..Default::default()
            },
            &[node0010],
        )
        .unwrap();
    let node00 = taffy
        .new_with_children(
            taffy::style::Style {
                box_sizing: taffy::style::BoxSizing::ContentBox,
                direction: taffy::style::Direction::Rtl,
                ..Default::default()
            },
            &[node000, node001],
        )
        .unwrap();
    let node01 = taffy
        .new_leaf_with_context(
            taffy::style::Style {
                box_sizing: taffy::style::BoxSizing::ContentBox,
                direction: taffy::style::Direction::Rtl,
                ..Default::default()
            },
            crate::TestNodeContext::ahem_text("HH", crate::WritingMode::Horizontal),
        )
        .unwrap();
    let node02 = taffy
        .new_leaf_with_context(
            taffy::style::Style {
                box_sizing: taffy::style::BoxSizing::ContentBox,
                direction: taffy::style::Direction::Rtl,
                ..Default::default()
            },
            crate::TestNodeContext::ahem_text("HH", crate::WritingMode::Horizontal),
        )
        .unwrap();
    let node03 = taffy
        .new_leaf_with_context(
            taffy::style::Style {
                box_sizing: taffy::style::BoxSizing::ContentBox,
                direction: taffy::style::Direction::Rtl,
                ..Default::default()
            },
            crate::TestNodeContext::ahem_text("HH", crate::WritingMode::Horizontal),
        )
        .unwrap();
    let node0 = taffy
        .new_with_children(
            taffy::style::Style {
                display: taffy::style::Display::Grid,
                box_sizing: taffy::style::BoxSizing::ContentBox,
                direction: taffy::style::Direction::Rtl,
                grid_template_rows: vec![fr(1f32), fr(1f32)],
                grid_template_columns: vec![fr(1f32), fr(1f32)],
                ..Default::default()
            },
            &[node00, node01, node02, node03],
        )
        .unwrap();
    let node = taffy
        .new_with_children(
            taffy::style::Style {
                box_sizing: taffy::style::BoxSizing::ContentBox,
                direction: taffy::style::Direction::Rtl,
                ..Default::default()
            },
            &[node0],
        )
        .unwrap();
    taffy.compute_layout_with_measure(node, taffy::geometry::Size::MAX_CONTENT, crate::test_measure_function).unwrap();
    println!("\nComputed tree:");
    taffy.print_tree(node);
    println!();
    let layout = taffy.layout(node).unwrap();
    let Layout { size, location, .. } = layout;
    assert_eq!(size.width, 140f32, "width of node {:?}. Expected {}. Actual {}", node, 140f32, size.width);
    assert_eq!(size.height, 20f32, "height of node {:?}. Expected {}. Actual {}", node, 20f32, size.height);
    assert_eq!(location.x, 0f32, "x of node {:?}. Expected {}. Actual {}", node, 0f32, location.x);
    assert_eq!(location.y, 0f32, "y of node {:?}. Expected {}. Actual {}", node, 0f32, location.y);
    let layout = taffy.layout(node0).unwrap();
    let Layout { size, location, .. } = layout;
    assert_eq!(size.width, 140f32, "width of node {:?}. Expected {}. Actual {}", node0, 140f32, size.width);
    assert_eq!(size.height, 20f32, "height of node {:?}. Expected {}. Actual {}", node0, 20f32, size.height);
    assert_eq!(location.x, 0f32, "x of node {:?}. Expected {}. Actual {}", node0, 0f32, location.x);
    assert_eq!(location.y, 0f32, "y of node {:?}. Expected {}. Actual {}", node0, 0f32, location.y);
    let layout = taffy.layout(node00).unwrap();
    let Layout { size, location, .. } = layout;
    assert_eq!(size.width, 70f32, "width of node {:?}. Expected {}. Actual {}", node00, 70f32, size.width);
    assert_eq!(size.height, 10f32, "height of node {:?}. Expected {}. Actual {}", node00, 10f32, size.height);
    assert_eq!(location.x, 70f32, "x of node {:?}. Expected {}. Actual {}", node00, 70f32, location.x);
    assert_eq!(location.y, 0f32, "y of node {:?}. Expected {}. Actual {}", node00, 0f32, location.y);
    let layout = taffy.layout(node000).unwrap();
    let Layout { size, location, .. } = layout;
    assert_eq!(size.width, 20f32, "width of node {:?}. Expected {}. Actual {}", node000, 20f32, size.width);
    assert_eq!(size.height, 10f32, "height of node {:?}. Expected {}. Actual {}", node000, 10f32, size.height);
    assert_eq!(location.x, 50f32, "x of node {:?}. Expected {}. Actual {}", node000, 50f32, location.x);
    assert_eq!(location.y, 0f32, "y of node {:?}. Expected {}. Actual {}", node000, 0f32, location.y);
    let layout = taffy.layout(node001).unwrap();
    let Layout { size, location, .. } = layout;
    assert_eq!(size.width, 50f32, "width of node {:?}. Expected {}. Actual {}", node001, 50f32, size.width);
    assert_eq!(size.height, 10f32, "height of node {:?}. Expected {}. Actual {}", node001, 10f32, size.height);
    assert_eq!(location.x, 0f32, "x of node {:?}. Expected {}. Actual {}", node001, 0f32, location.x);
    assert_eq!(location.y, 0f32, "y of node {:?}. Expected {}. Actual {}", node001, 0f32, location.y);
    let layout = taffy.layout(node0010).unwrap();
    let Layout { size, location, .. } = layout;
    assert_eq!(size.width, 20f32, "width of node {:?}. Expected {}. Actual {}", node0010, 20f32, size.width);
    assert_eq!(size.height, 10f32, "height of node {:?}. Expected {}. Actual {}", node0010, 10f32, size.height);
    assert_eq!(location.x, 30f32, "x of node {:?}. Expected {}. Actual {}", node0010, 30f32, location.x);
    assert_eq!(location.y, 0f32, "y of node {:?}. Expected {}. Actual {}", node0010, 0f32, location.y);
    let layout = taffy.layout(node00100).unwrap();
    let Layout { size, location, .. } = layout;
    assert_eq!(size.width, 20f32, "width of node {:?}. Expected {}. Actual {}", node00100, 20f32, size.width);
    assert_eq!(size.height, 3f32, "height of node {:?}. Expected {}. Actual {}", node00100, 3f32, size.height);
    assert_eq!(location.x, 0f32, "x of node {:?}. Expected {}. Actual {}", node00100, 0f32, location.x);
    assert_eq!(location.y, 0f32, "y of node {:?}. Expected {}. Actual {}", node00100, 0f32, location.y);
    let layout = taffy.layout(node00101).unwrap();
    let Layout { size, location, .. } = layout;
    assert_eq!(size.width, 2f32, "width of node {:?}. Expected {}. Actual {}", node00101, 2f32, size.width);
    assert_eq!(size.height, 3f32, "height of node {:?}. Expected {}. Actual {}", node00101, 3f32, size.height);
    assert_eq!(location.x, -2f32, "x of node {:?}. Expected {}. Actual {}", node00101, -2f32, location.x);
    assert_eq!(location.y, 0f32, "y of node {:?}. Expected {}. Actual {}", node00101, 0f32, location.y);
    let layout = taffy.layout(node00102).unwrap();
    let Layout { size, location, .. } = layout;
    assert_eq!(size.width, 20f32, "width of node {:?}. Expected {}. Actual {}", node00102, 20f32, size.width);
    assert_eq!(size.height, 1f32, "height of node {:?}. Expected {}. Actual {}", node00102, 1f32, size.height);
    assert_eq!(location.x, 0f32, "x of node {:?}. Expected {}. Actual {}", node00102, 0f32, location.x);
    assert_eq!(location.y, 3f32, "y of node {:?}. Expected {}. Actual {}", node00102, 3f32, location.y);
    let layout = taffy.layout(node00103).unwrap();
    let Layout { size, location, .. } = layout;
    assert_eq!(size.width, 2f32, "width of node {:?}. Expected {}. Actual {}", node00103, 2f32, size.width);
    assert_eq!(size.height, 1f32, "height of node {:?}. Expected {}. Actual {}", node00103, 1f32, size.height);
    assert_eq!(location.x, -2f32, "x of node {:?}. Expected {}. Actual {}", node00103, -2f32, location.x);
    assert_eq!(location.y, 3f32, "y of node {:?}. Expected {}. Actual {}", node00103, 3f32, location.y);
    let layout = taffy.layout(node01).unwrap();
    let Layout { size, location, .. } = layout;
    assert_eq!(size.width, 70f32, "width of node {:?}. Expected {}. Actual {}", node01, 70f32, size.width);
    assert_eq!(size.height, 10f32, "height of node {:?}. Expected {}. Actual {}", node01, 10f32, size.height);
    assert_eq!(location.x, 0f32, "x of node {:?}. Expected {}. Actual {}", node01, 0f32, location.x);
    assert_eq!(location.y, 0f32, "y of node {:?}. Expected {}. Actual {}", node01, 0f32, location.y);
    let layout = taffy.layout(node02).unwrap();
    let Layout { size, location, .. } = layout;
    assert_eq!(size.width, 70f32, "width of node {:?}. Expected {}. Actual {}", node02, 70f32, size.width);
    assert_eq!(size.height, 10f32, "height of node {:?}. Expected {}. Actual {}", node02, 10f32, size.height);
    assert_eq!(location.x, 70f32, "x of node {:?}. Expected {}. Actual {}", node02, 70f32, location.x);
    assert_eq!(location.y, 10f32, "y of node {:?}. Expected {}. Actual {}", node02, 10f32, location.y);
    let layout = taffy.layout(node03).unwrap();
    let Layout { size, location, .. } = layout;
    assert_eq!(size.width, 70f32, "width of node {:?}. Expected {}. Actual {}", node03, 70f32, size.width);
    assert_eq!(size.height, 10f32, "height of node {:?}. Expected {}. Actual {}", node03, 10f32, size.height);
    assert_eq!(location.x, 0f32, "x of node {:?}. Expected {}. Actual {}", node03, 0f32, location.x);
    assert_eq!(location.y, 10f32, "y of node {:?}. Expected {}. Actual {}", node03, 10f32, location.y);
}
