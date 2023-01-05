#[test]
fn single_flex_child_after_absolute_child() {
    use slotmap::Key;
    #[allow(unused_imports)]
    use taffy::{layout::Layout, prelude::*};
    let mut taffy = taffy::Taffy::new();
    let node0 = taffy
        .new_leaf(taffy::style::Style {
            position: taffy::style::Position::Absolute,
            size: taffy::geometry::Size {
                width: taffy::style::Dimension::Percent(1f32),
                height: taffy::style::Dimension::Percent(1f32),
            },
            ..Default::default()
        })
        .unwrap();
    let node1 =
        taffy.new_leaf(taffy::style::Style { flex_grow: 1f32, flex_shrink: 1f32, ..Default::default() }).unwrap();
    let node2 = taffy
        .new_leaf(taffy::style::Style {
            flex_shrink: 0f32,
            flex_basis: taffy::style::Dimension::Points(174f32),
            ..Default::default()
        })
        .unwrap();
    let node = taffy
        .new_with_children(
            taffy::style::Style {
                size: taffy::geometry::Size {
                    width: taffy::style::Dimension::Points(428f32),
                    height: taffy::style::Dimension::Points(845f32),
                },
                ..Default::default()
            },
            &[node0, node1, node2],
        )
        .unwrap();
    taffy.compute_layout(node, taffy::geometry::Size::MAX_CONTENT).unwrap();
    println!("\nComputed tree:");
    taffy::debug::print_tree(&taffy, node);
    println!();
    let Layout { size, location, .. } = taffy.layout(node).unwrap();
    assert_eq!(size.width, 428f32, "width of node {:?}. Expected {}. Actual {}", node.data(), 428f32, size.width);
    assert_eq!(size.height, 845f32, "height of node {:?}. Expected {}. Actual {}", node.data(), 845f32, size.height);
    assert_eq!(location.x, 0f32, "x of node {:?}. Expected {}. Actual {}", node.data(), 0f32, location.x);
    assert_eq!(location.y, 0f32, "y of node {:?}. Expected {}. Actual {}", node.data(), 0f32, location.y);
    let Layout { size, location, .. } = taffy.layout(node0).unwrap();
    assert_eq!(size.width, 428f32, "width of node {:?}. Expected {}. Actual {}", node0.data(), 428f32, size.width);
    assert_eq!(size.height, 845f32, "height of node {:?}. Expected {}. Actual {}", node0.data(), 845f32, size.height);
    assert_eq!(location.x, 0f32, "x of node {:?}. Expected {}. Actual {}", node0.data(), 0f32, location.x);
    assert_eq!(location.y, 0f32, "y of node {:?}. Expected {}. Actual {}", node0.data(), 0f32, location.y);
    let Layout { size, location, .. } = taffy.layout(node1).unwrap();
    assert_eq!(size.width, 254f32, "width of node {:?}. Expected {}. Actual {}", node1.data(), 254f32, size.width);
    assert_eq!(size.height, 845f32, "height of node {:?}. Expected {}. Actual {}", node1.data(), 845f32, size.height);
    assert_eq!(location.x, 0f32, "x of node {:?}. Expected {}. Actual {}", node1.data(), 0f32, location.x);
    assert_eq!(location.y, 0f32, "y of node {:?}. Expected {}. Actual {}", node1.data(), 0f32, location.y);
    let Layout { size, location, .. } = taffy.layout(node2).unwrap();
    assert_eq!(size.width, 174f32, "width of node {:?}. Expected {}. Actual {}", node2.data(), 174f32, size.width);
    assert_eq!(size.height, 845f32, "height of node {:?}. Expected {}. Actual {}", node2.data(), 845f32, size.height);
    assert_eq!(location.x, 254f32, "x of node {:?}. Expected {}. Actual {}", node2.data(), 254f32, location.x);
    assert_eq!(location.y, 0f32, "y of node {:?}. Expected {}. Actual {}", node2.data(), 0f32, location.y);
}
