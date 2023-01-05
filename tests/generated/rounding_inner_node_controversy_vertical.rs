#[test]
fn rounding_inner_node_controversy_vertical() {
    use slotmap::Key;
    #[allow(unused_imports)]
    use taffy::{layout::Layout, prelude::*};
    let mut taffy = taffy::Taffy::new();
    let node0 = taffy
        .new_leaf(taffy::style::Style {
            flex_grow: 1f32,
            size: taffy::geometry::Size { width: taffy::style::Dimension::Points(10f32), height: auto() },
            ..Default::default()
        })
        .unwrap();
    let node10 = taffy
        .new_leaf_with_measure(
            taffy::style::Style {
                flex_grow: 1f32,
                size: taffy::geometry::Size { width: taffy::style::Dimension::Points(10f32), height: auto() },
                ..Default::default()
            },
            taffy::node::MeasureFunc::Raw(|known_dimensions, available_space| {
                const TEXT: &str = "\n    ";
                super::measure_standard_text(
                    known_dimensions,
                    available_space,
                    TEXT,
                    super::WritingMode::Horizontal,
                    None,
                )
            }),
        )
        .unwrap();
    let node1 = taffy
        .new_with_children(
            taffy::style::Style {
                flex_grow: 1f32,
                size: taffy::geometry::Size { width: taffy::style::Dimension::Points(10f32), height: auto() },
                ..Default::default()
            },
            &[node10],
        )
        .unwrap();
    let node2 = taffy
        .new_leaf(taffy::style::Style {
            flex_grow: 1f32,
            size: taffy::geometry::Size { width: taffy::style::Dimension::Points(10f32), height: auto() },
            ..Default::default()
        })
        .unwrap();
    let node = taffy
        .new_with_children(
            taffy::style::Style {
                size: taffy::geometry::Size { width: auto(), height: taffy::style::Dimension::Points(320f32) },
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
    assert_eq!(size.width, 30f32, "width of node {:?}. Expected {}. Actual {}", node.data(), 30f32, size.width);
    assert_eq!(size.height, 320f32, "height of node {:?}. Expected {}. Actual {}", node.data(), 320f32, size.height);
    assert_eq!(location.x, 0f32, "x of node {:?}. Expected {}. Actual {}", node.data(), 0f32, location.x);
    assert_eq!(location.y, 0f32, "y of node {:?}. Expected {}. Actual {}", node.data(), 0f32, location.y);
    let Layout { size, location, .. } = taffy.layout(node0).unwrap();
    assert_eq!(size.width, 10f32, "width of node {:?}. Expected {}. Actual {}", node0.data(), 10f32, size.width);
    assert_eq!(size.height, 320f32, "height of node {:?}. Expected {}. Actual {}", node0.data(), 320f32, size.height);
    assert_eq!(location.x, 0f32, "x of node {:?}. Expected {}. Actual {}", node0.data(), 0f32, location.x);
    assert_eq!(location.y, 0f32, "y of node {:?}. Expected {}. Actual {}", node0.data(), 0f32, location.y);
    let Layout { size, location, .. } = taffy.layout(node1).unwrap();
    assert_eq!(size.width, 10f32, "width of node {:?}. Expected {}. Actual {}", node1.data(), 10f32, size.width);
    assert_eq!(size.height, 320f32, "height of node {:?}. Expected {}. Actual {}", node1.data(), 320f32, size.height);
    assert_eq!(location.x, 10f32, "x of node {:?}. Expected {}. Actual {}", node1.data(), 10f32, location.x);
    assert_eq!(location.y, 0f32, "y of node {:?}. Expected {}. Actual {}", node1.data(), 0f32, location.y);
    let Layout { size, location, .. } = taffy.layout(node10).unwrap();
    assert_eq!(size.width, 10f32, "width of node {:?}. Expected {}. Actual {}", node10.data(), 10f32, size.width);
    assert_eq!(size.height, 320f32, "height of node {:?}. Expected {}. Actual {}", node10.data(), 320f32, size.height);
    assert_eq!(location.x, 0f32, "x of node {:?}. Expected {}. Actual {}", node10.data(), 0f32, location.x);
    assert_eq!(location.y, 0f32, "y of node {:?}. Expected {}. Actual {}", node10.data(), 0f32, location.y);
    let Layout { size, location, .. } = taffy.layout(node2).unwrap();
    assert_eq!(size.width, 10f32, "width of node {:?}. Expected {}. Actual {}", node2.data(), 10f32, size.width);
    assert_eq!(size.height, 320f32, "height of node {:?}. Expected {}. Actual {}", node2.data(), 320f32, size.height);
    assert_eq!(location.x, 20f32, "x of node {:?}. Expected {}. Actual {}", node2.data(), 20f32, location.x);
    assert_eq!(location.y, 0f32, "y of node {:?}. Expected {}. Actual {}", node2.data(), 0f32, location.y);
}
