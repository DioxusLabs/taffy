#[test]
fn grid_fr_span_2_proportion_zero_sum() {
    use slotmap::Key;
    #[allow(unused_imports)]
    use taffy::{layout::Layout, prelude::*};
    let mut taffy = taffy::Taffy::new();
    let node0 = taffy
        .new_leaf(taffy::style::Style {
            grid_column: taffy::geometry::Line {
                start: taffy::style::GridPlacement::Span(2u16),
                end: taffy::style::GridPlacement::Auto,
            },
            size: taffy::geometry::Size { width: taffy::style::Dimension::Points(60f32), height: auto() },
            ..Default::default()
        })
        .unwrap();
    let node1 = taffy.new_leaf(taffy::style::Style { ..Default::default() }).unwrap();
    let node2 = taffy.new_leaf(taffy::style::Style { ..Default::default() }).unwrap();
    let node = taffy
        .new_with_children(
            taffy::style::Style {
                display: taffy::style::Display::Grid,
                grid_template_rows: vec![points(40f32), points(40f32)],
                grid_template_columns: vec![flex(0f32), flex(0f32)],
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
    assert_eq!(size.width, 60f32, "width of node {:?}. Expected {}. Actual {}", node.data(), 60f32, size.width);
    assert_eq!(size.height, 80f32, "height of node {:?}. Expected {}. Actual {}", node.data(), 80f32, size.height);
    assert_eq!(location.x, 0f32, "x of node {:?}. Expected {}. Actual {}", node.data(), 0f32, location.x);
    assert_eq!(location.y, 0f32, "y of node {:?}. Expected {}. Actual {}", node.data(), 0f32, location.y);
    let Layout { size, location, .. } = taffy.layout(node0).unwrap();
    assert_eq!(size.width, 60f32, "width of node {:?}. Expected {}. Actual {}", node0.data(), 60f32, size.width);
    assert_eq!(size.height, 40f32, "height of node {:?}. Expected {}. Actual {}", node0.data(), 40f32, size.height);
    assert_eq!(location.x, 0f32, "x of node {:?}. Expected {}. Actual {}", node0.data(), 0f32, location.x);
    assert_eq!(location.y, 0f32, "y of node {:?}. Expected {}. Actual {}", node0.data(), 0f32, location.y);
    let Layout { size, location, .. } = taffy.layout(node1).unwrap();
    assert_eq!(size.width, 30f32, "width of node {:?}. Expected {}. Actual {}", node1.data(), 30f32, size.width);
    assert_eq!(size.height, 40f32, "height of node {:?}. Expected {}. Actual {}", node1.data(), 40f32, size.height);
    assert_eq!(location.x, 0f32, "x of node {:?}. Expected {}. Actual {}", node1.data(), 0f32, location.x);
    assert_eq!(location.y, 40f32, "y of node {:?}. Expected {}. Actual {}", node1.data(), 40f32, location.y);
    let Layout { size, location, .. } = taffy.layout(node2).unwrap();
    assert_eq!(size.width, 30f32, "width of node {:?}. Expected {}. Actual {}", node2.data(), 30f32, size.width);
    assert_eq!(size.height, 40f32, "height of node {:?}. Expected {}. Actual {}", node2.data(), 40f32, size.height);
    assert_eq!(location.x, 30f32, "x of node {:?}. Expected {}. Actual {}", node2.data(), 30f32, location.x);
    assert_eq!(location.y, 40f32, "y of node {:?}. Expected {}. Actual {}", node2.data(), 40f32, location.y);
}
