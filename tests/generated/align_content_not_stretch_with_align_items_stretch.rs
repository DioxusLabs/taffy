#[test]
fn align_content_not_stretch_with_align_items_stretch() {
    use slotmap::Key;
    #[allow(unused_imports)]
    use taffy::{layout::Layout, prelude::*};
    let mut taffy = taffy::Taffy::new();
    let node00 = taffy
        .new_leaf(taffy::style::Style {
            size: taffy::geometry::Size {
                width: taffy::style::Dimension::Points(272f32),
                height: taffy::style::Dimension::Points(44f32),
            },
            ..Default::default()
        })
        .unwrap();
    let node0 = taffy
        .new_with_children(
            taffy::style::Style { flex_direction: taffy::style::FlexDirection::Column, ..Default::default() },
            &[node00],
        )
        .unwrap();
    let node10 = taffy
        .new_leaf(taffy::style::Style {
            size: taffy::geometry::Size {
                width: taffy::style::Dimension::Points(56f32),
                height: taffy::style::Dimension::Points(44f32),
            },
            ..Default::default()
        })
        .unwrap();
    let node1 = taffy
        .new_with_children(
            taffy::style::Style { flex_direction: taffy::style::FlexDirection::Column, ..Default::default() },
            &[node10],
        )
        .unwrap();
    let node = taffy
        .new_with_children(
            taffy::style::Style {
                flex_wrap: taffy::style::FlexWrap::Wrap,
                align_content: Some(taffy::style::AlignContent::Start),
                size: taffy::geometry::Size {
                    width: taffy::style::Dimension::Points(328f32),
                    height: taffy::style::Dimension::Points(52f32),
                },
                ..Default::default()
            },
            &[node0, node1],
        )
        .unwrap();
    taffy.compute_layout(node, taffy::geometry::Size::MAX_CONTENT).unwrap();
    println!("\nComputed tree:");
    taffy::debug::print_tree(&taffy, node);
    println!();
    let Layout { size, location, .. } = taffy.layout(node).unwrap();
    assert_eq!(size.width, 328f32, "width of node {:?}. Expected {}. Actual {}", node.data(), 328f32, size.width);
    assert_eq!(size.height, 52f32, "height of node {:?}. Expected {}. Actual {}", node.data(), 52f32, size.height);
    assert_eq!(location.x, 0f32, "x of node {:?}. Expected {}. Actual {}", node.data(), 0f32, location.x);
    assert_eq!(location.y, 0f32, "y of node {:?}. Expected {}. Actual {}", node.data(), 0f32, location.y);
    let Layout { size, location, .. } = taffy.layout(node0).unwrap();
    assert_eq!(size.width, 272f32, "width of node {:?}. Expected {}. Actual {}", node0.data(), 272f32, size.width);
    assert_eq!(size.height, 44f32, "height of node {:?}. Expected {}. Actual {}", node0.data(), 44f32, size.height);
    assert_eq!(location.x, 0f32, "x of node {:?}. Expected {}. Actual {}", node0.data(), 0f32, location.x);
    assert_eq!(location.y, 0f32, "y of node {:?}. Expected {}. Actual {}", node0.data(), 0f32, location.y);
    let Layout { size, location, .. } = taffy.layout(node00).unwrap();
    assert_eq!(size.width, 272f32, "width of node {:?}. Expected {}. Actual {}", node00.data(), 272f32, size.width);
    assert_eq!(size.height, 44f32, "height of node {:?}. Expected {}. Actual {}", node00.data(), 44f32, size.height);
    assert_eq!(location.x, 0f32, "x of node {:?}. Expected {}. Actual {}", node00.data(), 0f32, location.x);
    assert_eq!(location.y, 0f32, "y of node {:?}. Expected {}. Actual {}", node00.data(), 0f32, location.y);
    let Layout { size, location, .. } = taffy.layout(node1).unwrap();
    assert_eq!(size.width, 56f32, "width of node {:?}. Expected {}. Actual {}", node1.data(), 56f32, size.width);
    assert_eq!(size.height, 44f32, "height of node {:?}. Expected {}. Actual {}", node1.data(), 44f32, size.height);
    assert_eq!(location.x, 272f32, "x of node {:?}. Expected {}. Actual {}", node1.data(), 272f32, location.x);
    assert_eq!(location.y, 0f32, "y of node {:?}. Expected {}. Actual {}", node1.data(), 0f32, location.y);
    let Layout { size, location, .. } = taffy.layout(node10).unwrap();
    assert_eq!(size.width, 56f32, "width of node {:?}. Expected {}. Actual {}", node10.data(), 56f32, size.width);
    assert_eq!(size.height, 44f32, "height of node {:?}. Expected {}. Actual {}", node10.data(), 44f32, size.height);
    assert_eq!(location.x, 0f32, "x of node {:?}. Expected {}. Actual {}", node10.data(), 0f32, location.x);
    assert_eq!(location.y, 0f32, "y of node {:?}. Expected {}. Actual {}", node10.data(), 0f32, location.y);
}
