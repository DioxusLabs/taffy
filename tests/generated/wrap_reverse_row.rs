#[test]
fn wrap_reverse_row() {
    #[allow(unused_imports)]
    use taffy::{prelude::*, tree::Layout};
    let mut taffy = taffy::Taffy::new();
    let node0 = taffy
        .new_leaf(taffy::style::Style {
            size: taffy::geometry::Size {
                width: taffy::style::Dimension::Points(31f32),
                height: taffy::style::Dimension::Points(30f32),
            },
            ..Default::default()
        })
        .unwrap();
    let node1 = taffy
        .new_leaf(taffy::style::Style {
            size: taffy::geometry::Size {
                width: taffy::style::Dimension::Points(32f32),
                height: taffy::style::Dimension::Points(30f32),
            },
            ..Default::default()
        })
        .unwrap();
    let node2 = taffy
        .new_leaf(taffy::style::Style {
            size: taffy::geometry::Size {
                width: taffy::style::Dimension::Points(33f32),
                height: taffy::style::Dimension::Points(30f32),
            },
            ..Default::default()
        })
        .unwrap();
    let node3 = taffy
        .new_leaf(taffy::style::Style {
            size: taffy::geometry::Size {
                width: taffy::style::Dimension::Points(34f32),
                height: taffy::style::Dimension::Points(30f32),
            },
            ..Default::default()
        })
        .unwrap();
    let node = taffy
        .new_with_children(
            taffy::style::Style {
                flex_wrap: taffy::style::FlexWrap::WrapReverse,
                size: taffy::geometry::Size { width: taffy::style::Dimension::Points(100f32), height: auto() },
                ..Default::default()
            },
            &[node0, node1, node2, node3],
        )
        .unwrap();
    taffy.compute_layout(node, taffy::geometry::Size::MAX_CONTENT).unwrap();
    println!("\nComputed tree:");
    taffy::util::print_tree(&taffy, node);
    println!();
    let Layout { size, location, .. } = taffy.layout(node).unwrap();
    assert_eq!(size.width, 100f32, "width of node {:?}. Expected {}. Actual {}", node, 100f32, size.width);
    assert_eq!(size.height, 60f32, "height of node {:?}. Expected {}. Actual {}", node, 60f32, size.height);
    assert_eq!(location.x, 0f32, "x of node {:?}. Expected {}. Actual {}", node, 0f32, location.x);
    assert_eq!(location.y, 0f32, "y of node {:?}. Expected {}. Actual {}", node, 0f32, location.y);
    let Layout { size, location, .. } = taffy.layout(node0).unwrap();
    assert_eq!(size.width, 31f32, "width of node {:?}. Expected {}. Actual {}", node0, 31f32, size.width);
    assert_eq!(size.height, 30f32, "height of node {:?}. Expected {}. Actual {}", node0, 30f32, size.height);
    assert_eq!(location.x, 0f32, "x of node {:?}. Expected {}. Actual {}", node0, 0f32, location.x);
    assert_eq!(location.y, 30f32, "y of node {:?}. Expected {}. Actual {}", node0, 30f32, location.y);
    let Layout { size, location, .. } = taffy.layout(node1).unwrap();
    assert_eq!(size.width, 32f32, "width of node {:?}. Expected {}. Actual {}", node1, 32f32, size.width);
    assert_eq!(size.height, 30f32, "height of node {:?}. Expected {}. Actual {}", node1, 30f32, size.height);
    assert_eq!(location.x, 31f32, "x of node {:?}. Expected {}. Actual {}", node1, 31f32, location.x);
    assert_eq!(location.y, 30f32, "y of node {:?}. Expected {}. Actual {}", node1, 30f32, location.y);
    let Layout { size, location, .. } = taffy.layout(node2).unwrap();
    assert_eq!(size.width, 33f32, "width of node {:?}. Expected {}. Actual {}", node2, 33f32, size.width);
    assert_eq!(size.height, 30f32, "height of node {:?}. Expected {}. Actual {}", node2, 30f32, size.height);
    assert_eq!(location.x, 63f32, "x of node {:?}. Expected {}. Actual {}", node2, 63f32, location.x);
    assert_eq!(location.y, 30f32, "y of node {:?}. Expected {}. Actual {}", node2, 30f32, location.y);
    let Layout { size, location, .. } = taffy.layout(node3).unwrap();
    assert_eq!(size.width, 34f32, "width of node {:?}. Expected {}. Actual {}", node3, 34f32, size.width);
    assert_eq!(size.height, 30f32, "height of node {:?}. Expected {}. Actual {}", node3, 30f32, size.height);
    assert_eq!(location.x, 0f32, "x of node {:?}. Expected {}. Actual {}", node3, 0f32, location.x);
    assert_eq!(location.y, 0f32, "y of node {:?}. Expected {}. Actual {}", node3, 0f32, location.y);
}
