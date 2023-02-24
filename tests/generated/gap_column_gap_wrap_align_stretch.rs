#[test]
fn gap_column_gap_wrap_align_stretch() {
    use slotmap::Key;
    #[allow(unused_imports)]
    use taffy::{layout::Layout, prelude::*};
    let mut taffy = taffy::Taffy::new();
    let node0 = taffy
        .new_leaf(taffy::style::Style {
            flex_grow: 1f32,
            min_size: taffy::geometry::Size { width: taffy::style::Dimension::Points(60f32), height: auto() },
            ..Default::default()
        })
        .unwrap();
    let node1 = taffy
        .new_leaf(taffy::style::Style {
            flex_grow: 1f32,
            min_size: taffy::geometry::Size { width: taffy::style::Dimension::Points(60f32), height: auto() },
            ..Default::default()
        })
        .unwrap();
    let node2 = taffy
        .new_leaf(taffy::style::Style {
            flex_grow: 1f32,
            min_size: taffy::geometry::Size { width: taffy::style::Dimension::Points(60f32), height: auto() },
            ..Default::default()
        })
        .unwrap();
    let node3 = taffy
        .new_leaf(taffy::style::Style {
            flex_grow: 1f32,
            min_size: taffy::geometry::Size { width: taffy::style::Dimension::Points(60f32), height: auto() },
            ..Default::default()
        })
        .unwrap();
    let node4 = taffy
        .new_leaf(taffy::style::Style {
            flex_grow: 1f32,
            min_size: taffy::geometry::Size { width: taffy::style::Dimension::Points(60f32), height: auto() },
            ..Default::default()
        })
        .unwrap();
    let node = taffy
        .new_with_children(
            taffy::style::Style {
                flex_wrap: taffy::style::FlexWrap::Wrap,
                align_content: Some(taffy::style::AlignContent::Stretch),
                gap: taffy::geometry::Size { width: taffy::style::LengthPercentage::Points(5f32), height: zero() },
                size: taffy::geometry::Size {
                    width: taffy::style::Dimension::Points(300f32),
                    height: taffy::style::Dimension::Points(300f32),
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
    let Layout { size, location, .. } = taffy.layout(node).unwrap();
    assert_eq!(size.width, 300f32, "width of node {:?}. Expected {}. Actual {}", node.data(), 300f32, size.width);
    assert_eq!(size.height, 300f32, "height of node {:?}. Expected {}. Actual {}", node.data(), 300f32, size.height);
    assert_eq!(location.x, 0f32, "x of node {:?}. Expected {}. Actual {}", node.data(), 0f32, location.x);
    assert_eq!(location.y, 0f32, "y of node {:?}. Expected {}. Actual {}", node.data(), 0f32, location.y);
    let Layout { size, location, .. } = taffy.layout(node0).unwrap();
    assert_eq!(size.width, 71f32, "width of node {:?}. Expected {}. Actual {}", node0.data(), 71f32, size.width);
    assert_eq!(size.height, 150f32, "height of node {:?}. Expected {}. Actual {}", node0.data(), 150f32, size.height);
    assert_eq!(location.x, 0f32, "x of node {:?}. Expected {}. Actual {}", node0.data(), 0f32, location.x);
    assert_eq!(location.y, 0f32, "y of node {:?}. Expected {}. Actual {}", node0.data(), 0f32, location.y);
    let Layout { size, location, .. } = taffy.layout(node1).unwrap();
    assert_eq!(size.width, 72f32, "width of node {:?}. Expected {}. Actual {}", node1.data(), 72f32, size.width);
    assert_eq!(size.height, 150f32, "height of node {:?}. Expected {}. Actual {}", node1.data(), 150f32, size.height);
    assert_eq!(location.x, 76f32, "x of node {:?}. Expected {}. Actual {}", node1.data(), 76f32, location.x);
    assert_eq!(location.y, 0f32, "y of node {:?}. Expected {}. Actual {}", node1.data(), 0f32, location.y);
    let Layout { size, location, .. } = taffy.layout(node2).unwrap();
    assert_eq!(size.width, 71f32, "width of node {:?}. Expected {}. Actual {}", node2.data(), 71f32, size.width);
    assert_eq!(size.height, 150f32, "height of node {:?}. Expected {}. Actual {}", node2.data(), 150f32, size.height);
    assert_eq!(location.x, 153f32, "x of node {:?}. Expected {}. Actual {}", node2.data(), 153f32, location.x);
    assert_eq!(location.y, 0f32, "y of node {:?}. Expected {}. Actual {}", node2.data(), 0f32, location.y);
    let Layout { size, location, .. } = taffy.layout(node3).unwrap();
    assert_eq!(size.width, 71f32, "width of node {:?}. Expected {}. Actual {}", node3.data(), 71f32, size.width);
    assert_eq!(size.height, 150f32, "height of node {:?}. Expected {}. Actual {}", node3.data(), 150f32, size.height);
    assert_eq!(location.x, 229f32, "x of node {:?}. Expected {}. Actual {}", node3.data(), 229f32, location.x);
    assert_eq!(location.y, 0f32, "y of node {:?}. Expected {}. Actual {}", node3.data(), 0f32, location.y);
    let Layout { size, location, .. } = taffy.layout(node4).unwrap();
    assert_eq!(size.width, 300f32, "width of node {:?}. Expected {}. Actual {}", node4.data(), 300f32, size.width);
    assert_eq!(size.height, 150f32, "height of node {:?}. Expected {}. Actual {}", node4.data(), 150f32, size.height);
    assert_eq!(location.x, 0f32, "x of node {:?}. Expected {}. Actual {}", node4.data(), 0f32, location.x);
    assert_eq!(location.y, 150f32, "y of node {:?}. Expected {}. Actual {}", node4.data(), 150f32, location.y);
}
