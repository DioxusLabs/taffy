#[test]
#[allow(non_snake_case)]
fn grid_align_content_end_with_padding_border__border_box() {
    #[allow(unused_imports)]
    use taffy::{prelude::*, Layout};
    let mut taffy = crate::new_test_tree();
    let node0 = taffy.new_leaf(taffy::style::Style { ..Default::default() }).unwrap();
    let node1 = taffy.new_leaf(taffy::style::Style { ..Default::default() }).unwrap();
    let node2 = taffy.new_leaf(taffy::style::Style { ..Default::default() }).unwrap();
    let node3 = taffy.new_leaf(taffy::style::Style { ..Default::default() }).unwrap();
    let node4 = taffy.new_leaf(taffy::style::Style { ..Default::default() }).unwrap();
    let node5 = taffy.new_leaf(taffy::style::Style { ..Default::default() }).unwrap();
    let node6 = taffy.new_leaf(taffy::style::Style { ..Default::default() }).unwrap();
    let node7 = taffy.new_leaf(taffy::style::Style { ..Default::default() }).unwrap();
    let node8 = taffy.new_leaf(taffy::style::Style { ..Default::default() }).unwrap();
    let node = taffy
        .new_with_children(
            taffy::style::Style {
                display: taffy::style::Display::Grid,
                align_content: Some(taffy::style::AlignContent::End),
                grid_template_rows: vec![length(40f32), length(40f32), length(40f32)],
                grid_template_columns: vec![length(40f32), length(40f32), length(40f32)],
                size: taffy::geometry::Size {
                    width: taffy::style::Dimension::from_length(200f32),
                    height: taffy::style::Dimension::from_length(200f32),
                },
                padding: taffy::geometry::Rect {
                    left: length(40f32),
                    right: length(20f32),
                    top: length(10f32),
                    bottom: length(30f32),
                },
                border: taffy::geometry::Rect {
                    left: length(8f32),
                    right: length(4f32),
                    top: length(2f32),
                    bottom: length(6f32),
                },
                ..Default::default()
            },
            &[node0, node1, node2, node3, node4, node5, node6, node7, node8],
        )
        .unwrap();
    taffy.compute_layout_with_measure(node, taffy::geometry::Size::MAX_CONTENT, crate::test_measure_function).unwrap();
    println!("\nComputed tree:");
    taffy.print_tree(node);
    println!();
    let layout = taffy.layout(node).unwrap();
    let Layout { size, location, .. } = layout;
    assert_eq!(size.width, 200f32, "width of node {:?}. Expected {}. Actual {}", node, 200f32, size.width);
    assert_eq!(size.height, 200f32, "height of node {:?}. Expected {}. Actual {}", node, 200f32, size.height);
    assert_eq!(location.x, 0f32, "x of node {:?}. Expected {}. Actual {}", node, 0f32, location.x);
    assert_eq!(location.y, 0f32, "y of node {:?}. Expected {}. Actual {}", node, 0f32, location.y);
    let layout = taffy.layout(node0).unwrap();
    let Layout { size, location, .. } = layout;
    assert_eq!(size.width, 40f32, "width of node {:?}. Expected {}. Actual {}", node0, 40f32, size.width);
    assert_eq!(size.height, 40f32, "height of node {:?}. Expected {}. Actual {}", node0, 40f32, size.height);
    assert_eq!(location.x, 48f32, "x of node {:?}. Expected {}. Actual {}", node0, 48f32, location.x);
    assert_eq!(location.y, 44f32, "y of node {:?}. Expected {}. Actual {}", node0, 44f32, location.y);
    let layout = taffy.layout(node1).unwrap();
    let Layout { size, location, .. } = layout;
    assert_eq!(size.width, 40f32, "width of node {:?}. Expected {}. Actual {}", node1, 40f32, size.width);
    assert_eq!(size.height, 40f32, "height of node {:?}. Expected {}. Actual {}", node1, 40f32, size.height);
    assert_eq!(location.x, 88f32, "x of node {:?}. Expected {}. Actual {}", node1, 88f32, location.x);
    assert_eq!(location.y, 44f32, "y of node {:?}. Expected {}. Actual {}", node1, 44f32, location.y);
    let layout = taffy.layout(node2).unwrap();
    let Layout { size, location, .. } = layout;
    assert_eq!(size.width, 40f32, "width of node {:?}. Expected {}. Actual {}", node2, 40f32, size.width);
    assert_eq!(size.height, 40f32, "height of node {:?}. Expected {}. Actual {}", node2, 40f32, size.height);
    assert_eq!(location.x, 128f32, "x of node {:?}. Expected {}. Actual {}", node2, 128f32, location.x);
    assert_eq!(location.y, 44f32, "y of node {:?}. Expected {}. Actual {}", node2, 44f32, location.y);
    let layout = taffy.layout(node3).unwrap();
    let Layout { size, location, .. } = layout;
    assert_eq!(size.width, 40f32, "width of node {:?}. Expected {}. Actual {}", node3, 40f32, size.width);
    assert_eq!(size.height, 40f32, "height of node {:?}. Expected {}. Actual {}", node3, 40f32, size.height);
    assert_eq!(location.x, 48f32, "x of node {:?}. Expected {}. Actual {}", node3, 48f32, location.x);
    assert_eq!(location.y, 84f32, "y of node {:?}. Expected {}. Actual {}", node3, 84f32, location.y);
    let layout = taffy.layout(node4).unwrap();
    let Layout { size, location, .. } = layout;
    assert_eq!(size.width, 40f32, "width of node {:?}. Expected {}. Actual {}", node4, 40f32, size.width);
    assert_eq!(size.height, 40f32, "height of node {:?}. Expected {}. Actual {}", node4, 40f32, size.height);
    assert_eq!(location.x, 88f32, "x of node {:?}. Expected {}. Actual {}", node4, 88f32, location.x);
    assert_eq!(location.y, 84f32, "y of node {:?}. Expected {}. Actual {}", node4, 84f32, location.y);
    let layout = taffy.layout(node5).unwrap();
    let Layout { size, location, .. } = layout;
    assert_eq!(size.width, 40f32, "width of node {:?}. Expected {}. Actual {}", node5, 40f32, size.width);
    assert_eq!(size.height, 40f32, "height of node {:?}. Expected {}. Actual {}", node5, 40f32, size.height);
    assert_eq!(location.x, 128f32, "x of node {:?}. Expected {}. Actual {}", node5, 128f32, location.x);
    assert_eq!(location.y, 84f32, "y of node {:?}. Expected {}. Actual {}", node5, 84f32, location.y);
    let layout = taffy.layout(node6).unwrap();
    let Layout { size, location, .. } = layout;
    assert_eq!(size.width, 40f32, "width of node {:?}. Expected {}. Actual {}", node6, 40f32, size.width);
    assert_eq!(size.height, 40f32, "height of node {:?}. Expected {}. Actual {}", node6, 40f32, size.height);
    assert_eq!(location.x, 48f32, "x of node {:?}. Expected {}. Actual {}", node6, 48f32, location.x);
    assert_eq!(location.y, 124f32, "y of node {:?}. Expected {}. Actual {}", node6, 124f32, location.y);
    let layout = taffy.layout(node7).unwrap();
    let Layout { size, location, .. } = layout;
    assert_eq!(size.width, 40f32, "width of node {:?}. Expected {}. Actual {}", node7, 40f32, size.width);
    assert_eq!(size.height, 40f32, "height of node {:?}. Expected {}. Actual {}", node7, 40f32, size.height);
    assert_eq!(location.x, 88f32, "x of node {:?}. Expected {}. Actual {}", node7, 88f32, location.x);
    assert_eq!(location.y, 124f32, "y of node {:?}. Expected {}. Actual {}", node7, 124f32, location.y);
    let layout = taffy.layout(node8).unwrap();
    let Layout { size, location, .. } = layout;
    assert_eq!(size.width, 40f32, "width of node {:?}. Expected {}. Actual {}", node8, 40f32, size.width);
    assert_eq!(size.height, 40f32, "height of node {:?}. Expected {}. Actual {}", node8, 40f32, size.height);
    assert_eq!(location.x, 128f32, "x of node {:?}. Expected {}. Actual {}", node8, 128f32, location.x);
    assert_eq!(location.y, 124f32, "y of node {:?}. Expected {}. Actual {}", node8, 124f32, location.y);
}

#[test]
#[allow(non_snake_case)]
fn grid_align_content_end_with_padding_border__content_box() {
    #[allow(unused_imports)]
    use taffy::{prelude::*, Layout};
    let mut taffy = crate::new_test_tree();
    let node0 = taffy
        .new_leaf(taffy::style::Style { box_sizing: taffy::style::BoxSizing::ContentBox, ..Default::default() })
        .unwrap();
    let node1 = taffy
        .new_leaf(taffy::style::Style { box_sizing: taffy::style::BoxSizing::ContentBox, ..Default::default() })
        .unwrap();
    let node2 = taffy
        .new_leaf(taffy::style::Style { box_sizing: taffy::style::BoxSizing::ContentBox, ..Default::default() })
        .unwrap();
    let node3 = taffy
        .new_leaf(taffy::style::Style { box_sizing: taffy::style::BoxSizing::ContentBox, ..Default::default() })
        .unwrap();
    let node4 = taffy
        .new_leaf(taffy::style::Style { box_sizing: taffy::style::BoxSizing::ContentBox, ..Default::default() })
        .unwrap();
    let node5 = taffy
        .new_leaf(taffy::style::Style { box_sizing: taffy::style::BoxSizing::ContentBox, ..Default::default() })
        .unwrap();
    let node6 = taffy
        .new_leaf(taffy::style::Style { box_sizing: taffy::style::BoxSizing::ContentBox, ..Default::default() })
        .unwrap();
    let node7 = taffy
        .new_leaf(taffy::style::Style { box_sizing: taffy::style::BoxSizing::ContentBox, ..Default::default() })
        .unwrap();
    let node8 = taffy
        .new_leaf(taffy::style::Style { box_sizing: taffy::style::BoxSizing::ContentBox, ..Default::default() })
        .unwrap();
    let node = taffy
        .new_with_children(
            taffy::style::Style {
                display: taffy::style::Display::Grid,
                box_sizing: taffy::style::BoxSizing::ContentBox,
                align_content: Some(taffy::style::AlignContent::End),
                grid_template_rows: vec![length(40f32), length(40f32), length(40f32)],
                grid_template_columns: vec![length(40f32), length(40f32), length(40f32)],
                size: taffy::geometry::Size {
                    width: taffy::style::Dimension::from_length(200f32),
                    height: taffy::style::Dimension::from_length(200f32),
                },
                padding: taffy::geometry::Rect {
                    left: length(40f32),
                    right: length(20f32),
                    top: length(10f32),
                    bottom: length(30f32),
                },
                border: taffy::geometry::Rect {
                    left: length(8f32),
                    right: length(4f32),
                    top: length(2f32),
                    bottom: length(6f32),
                },
                ..Default::default()
            },
            &[node0, node1, node2, node3, node4, node5, node6, node7, node8],
        )
        .unwrap();
    taffy.compute_layout_with_measure(node, taffy::geometry::Size::MAX_CONTENT, crate::test_measure_function).unwrap();
    println!("\nComputed tree:");
    taffy.print_tree(node);
    println!();
    let layout = taffy.layout(node).unwrap();
    let Layout { size, location, .. } = layout;
    assert_eq!(size.width, 272f32, "width of node {:?}. Expected {}. Actual {}", node, 272f32, size.width);
    assert_eq!(size.height, 248f32, "height of node {:?}. Expected {}. Actual {}", node, 248f32, size.height);
    assert_eq!(location.x, 0f32, "x of node {:?}. Expected {}. Actual {}", node, 0f32, location.x);
    assert_eq!(location.y, 0f32, "y of node {:?}. Expected {}. Actual {}", node, 0f32, location.y);
    let layout = taffy.layout(node0).unwrap();
    let Layout { size, location, .. } = layout;
    assert_eq!(size.width, 40f32, "width of node {:?}. Expected {}. Actual {}", node0, 40f32, size.width);
    assert_eq!(size.height, 40f32, "height of node {:?}. Expected {}. Actual {}", node0, 40f32, size.height);
    assert_eq!(location.x, 48f32, "x of node {:?}. Expected {}. Actual {}", node0, 48f32, location.x);
    assert_eq!(location.y, 92f32, "y of node {:?}. Expected {}. Actual {}", node0, 92f32, location.y);
    let layout = taffy.layout(node1).unwrap();
    let Layout { size, location, .. } = layout;
    assert_eq!(size.width, 40f32, "width of node {:?}. Expected {}. Actual {}", node1, 40f32, size.width);
    assert_eq!(size.height, 40f32, "height of node {:?}. Expected {}. Actual {}", node1, 40f32, size.height);
    assert_eq!(location.x, 88f32, "x of node {:?}. Expected {}. Actual {}", node1, 88f32, location.x);
    assert_eq!(location.y, 92f32, "y of node {:?}. Expected {}. Actual {}", node1, 92f32, location.y);
    let layout = taffy.layout(node2).unwrap();
    let Layout { size, location, .. } = layout;
    assert_eq!(size.width, 40f32, "width of node {:?}. Expected {}. Actual {}", node2, 40f32, size.width);
    assert_eq!(size.height, 40f32, "height of node {:?}. Expected {}. Actual {}", node2, 40f32, size.height);
    assert_eq!(location.x, 128f32, "x of node {:?}. Expected {}. Actual {}", node2, 128f32, location.x);
    assert_eq!(location.y, 92f32, "y of node {:?}. Expected {}. Actual {}", node2, 92f32, location.y);
    let layout = taffy.layout(node3).unwrap();
    let Layout { size, location, .. } = layout;
    assert_eq!(size.width, 40f32, "width of node {:?}. Expected {}. Actual {}", node3, 40f32, size.width);
    assert_eq!(size.height, 40f32, "height of node {:?}. Expected {}. Actual {}", node3, 40f32, size.height);
    assert_eq!(location.x, 48f32, "x of node {:?}. Expected {}. Actual {}", node3, 48f32, location.x);
    assert_eq!(location.y, 132f32, "y of node {:?}. Expected {}. Actual {}", node3, 132f32, location.y);
    let layout = taffy.layout(node4).unwrap();
    let Layout { size, location, .. } = layout;
    assert_eq!(size.width, 40f32, "width of node {:?}. Expected {}. Actual {}", node4, 40f32, size.width);
    assert_eq!(size.height, 40f32, "height of node {:?}. Expected {}. Actual {}", node4, 40f32, size.height);
    assert_eq!(location.x, 88f32, "x of node {:?}. Expected {}. Actual {}", node4, 88f32, location.x);
    assert_eq!(location.y, 132f32, "y of node {:?}. Expected {}. Actual {}", node4, 132f32, location.y);
    let layout = taffy.layout(node5).unwrap();
    let Layout { size, location, .. } = layout;
    assert_eq!(size.width, 40f32, "width of node {:?}. Expected {}. Actual {}", node5, 40f32, size.width);
    assert_eq!(size.height, 40f32, "height of node {:?}. Expected {}. Actual {}", node5, 40f32, size.height);
    assert_eq!(location.x, 128f32, "x of node {:?}. Expected {}. Actual {}", node5, 128f32, location.x);
    assert_eq!(location.y, 132f32, "y of node {:?}. Expected {}. Actual {}", node5, 132f32, location.y);
    let layout = taffy.layout(node6).unwrap();
    let Layout { size, location, .. } = layout;
    assert_eq!(size.width, 40f32, "width of node {:?}. Expected {}. Actual {}", node6, 40f32, size.width);
    assert_eq!(size.height, 40f32, "height of node {:?}. Expected {}. Actual {}", node6, 40f32, size.height);
    assert_eq!(location.x, 48f32, "x of node {:?}. Expected {}. Actual {}", node6, 48f32, location.x);
    assert_eq!(location.y, 172f32, "y of node {:?}. Expected {}. Actual {}", node6, 172f32, location.y);
    let layout = taffy.layout(node7).unwrap();
    let Layout { size, location, .. } = layout;
    assert_eq!(size.width, 40f32, "width of node {:?}. Expected {}. Actual {}", node7, 40f32, size.width);
    assert_eq!(size.height, 40f32, "height of node {:?}. Expected {}. Actual {}", node7, 40f32, size.height);
    assert_eq!(location.x, 88f32, "x of node {:?}. Expected {}. Actual {}", node7, 88f32, location.x);
    assert_eq!(location.y, 172f32, "y of node {:?}. Expected {}. Actual {}", node7, 172f32, location.y);
    let layout = taffy.layout(node8).unwrap();
    let Layout { size, location, .. } = layout;
    assert_eq!(size.width, 40f32, "width of node {:?}. Expected {}. Actual {}", node8, 40f32, size.width);
    assert_eq!(size.height, 40f32, "height of node {:?}. Expected {}. Actual {}", node8, 40f32, size.height);
    assert_eq!(location.x, 128f32, "x of node {:?}. Expected {}. Actual {}", node8, 128f32, location.x);
    assert_eq!(location.y, 172f32, "y of node {:?}. Expected {}. Actual {}", node8, 172f32, location.y);
}
