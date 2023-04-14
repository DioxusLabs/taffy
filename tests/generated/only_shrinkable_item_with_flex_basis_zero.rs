#[test]
fn only_shrinkable_item_with_flex_basis_zero() {
    #[allow(unused_imports)]
    use taffy::{prelude::*, tree::Layout};
    let mut taffy = taffy::Taffy::new();
    let node0 = taffy
        .new_leaf(taffy::style::Style {
            flex_shrink: 1f32,
            flex_basis: taffy::style::Dimension::Points(0f32),
            ..Default::default()
        })
        .unwrap();
    let node1 = taffy
        .new_leaf(taffy::style::Style {
            flex_shrink: 0f32,
            flex_basis: taffy::style::Dimension::Points(93f32),
            margin: taffy::geometry::Rect {
                left: zero(),
                right: zero(),
                top: zero(),
                bottom: taffy::style::LengthPercentageAuto::Points(6f32),
            },
            ..Default::default()
        })
        .unwrap();
    let node2 = taffy
        .new_leaf(taffy::style::Style {
            flex_shrink: 0f32,
            flex_basis: taffy::style::Dimension::Points(764f32),
            ..Default::default()
        })
        .unwrap();
    let node = taffy
        .new_with_children(
            taffy::style::Style {
                flex_direction: taffy::style::FlexDirection::Column,
                size: taffy::geometry::Size { width: taffy::style::Dimension::Points(480f32), height: auto() },
                max_size: taffy::geometry::Size { width: auto(), height: taffy::style::Dimension::Points(764f32) },
                ..Default::default()
            },
            &[node0, node1, node2],
        )
        .unwrap();
    taffy.compute_layout(node, taffy::geometry::Size::MAX_CONTENT).unwrap();
    println!("\nComputed tree:");
    taffy::util::print_tree(&taffy, node);
    println!();
    let Layout { size, location, .. } = taffy.layout(node).unwrap();
    assert_eq!(size.width, 480f32, "width of node {:?}. Expected {}. Actual {}", node, 480f32, size.width);
    assert_eq!(size.height, 764f32, "height of node {:?}. Expected {}. Actual {}", node, 764f32, size.height);
    assert_eq!(location.x, 0f32, "x of node {:?}. Expected {}. Actual {}", node, 0f32, location.x);
    assert_eq!(location.y, 0f32, "y of node {:?}. Expected {}. Actual {}", node, 0f32, location.y);
    let Layout { size, location, .. } = taffy.layout(node0).unwrap();
    assert_eq!(size.width, 480f32, "width of node {:?}. Expected {}. Actual {}", node0, 480f32, size.width);
    assert_eq!(size.height, 0f32, "height of node {:?}. Expected {}. Actual {}", node0, 0f32, size.height);
    assert_eq!(location.x, 0f32, "x of node {:?}. Expected {}. Actual {}", node0, 0f32, location.x);
    assert_eq!(location.y, 0f32, "y of node {:?}. Expected {}. Actual {}", node0, 0f32, location.y);
    let Layout { size, location, .. } = taffy.layout(node1).unwrap();
    assert_eq!(size.width, 480f32, "width of node {:?}. Expected {}. Actual {}", node1, 480f32, size.width);
    assert_eq!(size.height, 93f32, "height of node {:?}. Expected {}. Actual {}", node1, 93f32, size.height);
    assert_eq!(location.x, 0f32, "x of node {:?}. Expected {}. Actual {}", node1, 0f32, location.x);
    assert_eq!(location.y, 0f32, "y of node {:?}. Expected {}. Actual {}", node1, 0f32, location.y);
    let Layout { size, location, .. } = taffy.layout(node2).unwrap();
    assert_eq!(size.width, 480f32, "width of node {:?}. Expected {}. Actual {}", node2, 480f32, size.width);
    assert_eq!(size.height, 764f32, "height of node {:?}. Expected {}. Actual {}", node2, 764f32, size.height);
    assert_eq!(location.x, 0f32, "x of node {:?}. Expected {}. Actual {}", node2, 0f32, location.x);
    assert_eq!(location.y, 99f32, "y of node {:?}. Expected {}. Actual {}", node2, 99f32, location.y);
}
