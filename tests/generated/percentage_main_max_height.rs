#[test]
fn percentage_main_max_height() {
    use slotmap::Key;
    #[allow(unused_imports)]
    use taffy::{layout::Layout, prelude::*};
    let mut taffy = taffy::Taffy::new();
    let node00 = taffy
        .new_leaf(taffy::style::Style { flex_basis: taffy::style::Dimension::Points(15f32), ..Default::default() })
        .unwrap();
    let node01 = taffy
        .new_leaf(taffy::style::Style {
            flex_basis: taffy::style::Dimension::Points(48f32),
            max_size: taffy::geometry::Size { width: auto(), height: taffy::style::Dimension::Percent(0.33f32) },
            ..Default::default()
        })
        .unwrap();
    let node0 = taffy
        .new_with_children(
            taffy::style::Style {
                flex_direction: taffy::style::FlexDirection::Column,
                align_items: Some(taffy::style::AlignItems::FlexStart),
                size: taffy::geometry::Size { width: auto(), height: taffy::style::Dimension::Points(151f32) },
                ..Default::default()
            },
            &[node00, node01],
        )
        .unwrap();
    let node = taffy
        .new_with_children(
            taffy::style::Style {
                flex_direction: taffy::style::FlexDirection::Column,
                size: taffy::geometry::Size { width: taffy::style::Dimension::Points(71f32), height: auto() },
                ..Default::default()
            },
            &[node0],
        )
        .unwrap();
    taffy.compute_layout(node, taffy::geometry::Size::MAX_CONTENT).unwrap();
    println!("\nComputed tree:");
    taffy::debug::print_tree(&taffy, node);
    println!();
    let Layout { size, location, .. } = taffy.layout(node).unwrap();
    assert_eq!(size.width, 71f32, "width of node {:?}. Expected {}. Actual {}", node.data(), 71f32, size.width);
    assert_eq!(size.height, 151f32, "height of node {:?}. Expected {}. Actual {}", node.data(), 151f32, size.height);
    assert_eq!(location.x, 0f32, "x of node {:?}. Expected {}. Actual {}", node.data(), 0f32, location.x);
    assert_eq!(location.y, 0f32, "y of node {:?}. Expected {}. Actual {}", node.data(), 0f32, location.y);
    let Layout { size, location, .. } = taffy.layout(node0).unwrap();
    assert_eq!(size.width, 71f32, "width of node {:?}. Expected {}. Actual {}", node0.data(), 71f32, size.width);
    assert_eq!(size.height, 151f32, "height of node {:?}. Expected {}. Actual {}", node0.data(), 151f32, size.height);
    assert_eq!(location.x, 0f32, "x of node {:?}. Expected {}. Actual {}", node0.data(), 0f32, location.x);
    assert_eq!(location.y, 0f32, "y of node {:?}. Expected {}. Actual {}", node0.data(), 0f32, location.y);
    let Layout { size, location, .. } = taffy.layout(node00).unwrap();
    assert_eq!(size.width, 0f32, "width of node {:?}. Expected {}. Actual {}", node00.data(), 0f32, size.width);
    assert_eq!(size.height, 15f32, "height of node {:?}. Expected {}. Actual {}", node00.data(), 15f32, size.height);
    assert_eq!(location.x, 0f32, "x of node {:?}. Expected {}. Actual {}", node00.data(), 0f32, location.x);
    assert_eq!(location.y, 0f32, "y of node {:?}. Expected {}. Actual {}", node00.data(), 0f32, location.y);
    let Layout { size, location, .. } = taffy.layout(node01).unwrap();
    assert_eq!(size.width, 0f32, "width of node {:?}. Expected {}. Actual {}", node01.data(), 0f32, size.width);
    assert_eq!(size.height, 48f32, "height of node {:?}. Expected {}. Actual {}", node01.data(), 48f32, size.height);
    assert_eq!(location.x, 0f32, "x of node {:?}. Expected {}. Actual {}", node01.data(), 0f32, location.x);
    assert_eq!(location.y, 15f32, "y of node {:?}. Expected {}. Actual {}", node01.data(), 15f32, location.y);
}
