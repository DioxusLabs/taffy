#[test]
fn grid_span_13_most_non_flex_with_minmax_indefinite() {
    use slotmap::Key;
    #[allow(unused_imports)]
    use taffy::{layout::Layout, prelude::*};
    let mut taffy = taffy::Taffy::new();
    let node0 = taffy
        .new_leaf_with_measure(
            taffy::style::Style {
                grid_column: taffy::geometry::Line { start: line(1i16), end: taffy::style::GridPlacement::Span(13u16) },
                ..Default::default()
            },
            taffy::node::MeasureFunc::Raw(|known_dimensions, available_space| {
                const TEXT: &str = "HHHHHHHHHHHHHHHH\u{200b}HHHHHHHHHHHHHHHH";
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
    let node1 = taffy.new_leaf(taffy::style::Style { ..Default::default() }).unwrap();
    let node2 = taffy.new_leaf(taffy::style::Style { ..Default::default() }).unwrap();
    let node3 = taffy.new_leaf(taffy::style::Style { ..Default::default() }).unwrap();
    let node4 = taffy.new_leaf(taffy::style::Style { ..Default::default() }).unwrap();
    let node5 = taffy.new_leaf(taffy::style::Style { ..Default::default() }).unwrap();
    let node6 = taffy.new_leaf(taffy::style::Style { ..Default::default() }).unwrap();
    let node7 = taffy.new_leaf(taffy::style::Style { ..Default::default() }).unwrap();
    let node8 = taffy.new_leaf(taffy::style::Style { ..Default::default() }).unwrap();
    let node9 = taffy.new_leaf(taffy::style::Style { ..Default::default() }).unwrap();
    let node10 = taffy.new_leaf(taffy::style::Style { ..Default::default() }).unwrap();
    let node11 = taffy.new_leaf(taffy::style::Style { ..Default::default() }).unwrap();
    let node12 = taffy.new_leaf(taffy::style::Style { ..Default::default() }).unwrap();
    let node13 = taffy.new_leaf(taffy::style::Style { ..Default::default() }).unwrap();
    let node = taffy
        .new_with_children(
            taffy::style::Style {
                display: taffy::style::Display::Grid,
                grid_template_rows: vec![points(40f32), points(40f32)],
                grid_template_columns: vec![
                    min_content(),
                    max_content(),
                    fit_content(points(20f32)),
                    auto(),
                    points(10f32),
                    percent(0.2f32),
                    minmax(points(2f32), auto()),
                    minmax(points(2f32), points(4f32)),
                    minmax(points(2f32), min_content()),
                    minmax(points(2f32), max_content()),
                    minmax(min_content(), max_content()),
                    minmax(min_content(), auto()),
                    minmax(max_content(), auto()),
                ],
                ..Default::default()
            },
            &[node0, node1, node2, node3, node4, node5, node6, node7, node8, node9, node10, node11, node12, node13],
        )
        .unwrap();
    taffy.compute_layout(node, taffy::geometry::Size::MAX_CONTENT).unwrap();
    println!("\nComputed tree:");
    taffy::debug::print_tree(&taffy, node);
    println!();
    let Layout { size, location, .. } = taffy.layout(node).unwrap();
    assert_eq!(size.width, 322f32, "width of node {:?}. Expected {}. Actual {}", node.data(), 322f32, size.width);
    assert_eq!(size.height, 80f32, "height of node {:?}. Expected {}. Actual {}", node.data(), 80f32, size.height);
    assert_eq!(location.x, 0f32, "x of node {:?}. Expected {}. Actual {}", node.data(), 0f32, location.x);
    assert_eq!(location.y, 0f32, "y of node {:?}. Expected {}. Actual {}", node.data(), 0f32, location.y);
    let Layout { size, location, .. } = taffy.layout(node0).unwrap();
    assert_eq!(size.width, 322f32, "width of node {:?}. Expected {}. Actual {}", node0.data(), 322f32, size.width);
    assert_eq!(size.height, 40f32, "height of node {:?}. Expected {}. Actual {}", node0.data(), 40f32, size.height);
    assert_eq!(location.x, 0f32, "x of node {:?}. Expected {}. Actual {}", node0.data(), 0f32, location.x);
    assert_eq!(location.y, 0f32, "y of node {:?}. Expected {}. Actual {}", node0.data(), 0f32, location.y);
    let Layout { size, location, .. } = taffy.layout(node1).unwrap();
    assert_eq!(size.width, 11f32, "width of node {:?}. Expected {}. Actual {}", node1.data(), 11f32, size.width);
    assert_eq!(size.height, 40f32, "height of node {:?}. Expected {}. Actual {}", node1.data(), 40f32, size.height);
    assert_eq!(location.x, 0f32, "x of node {:?}. Expected {}. Actual {}", node1.data(), 0f32, location.x);
    assert_eq!(location.y, 40f32, "y of node {:?}. Expected {}. Actual {}", node1.data(), 40f32, location.y);
    let Layout { size, location, .. } = taffy.layout(node2).unwrap();
    assert_eq!(size.width, 91f32, "width of node {:?}. Expected {}. Actual {}", node2.data(), 91f32, size.width);
    assert_eq!(size.height, 40f32, "height of node {:?}. Expected {}. Actual {}", node2.data(), 40f32, size.height);
    assert_eq!(location.x, 11f32, "x of node {:?}. Expected {}. Actual {}", node2.data(), 11f32, location.x);
    assert_eq!(location.y, 40f32, "y of node {:?}. Expected {}. Actual {}", node2.data(), 40f32, location.y);
    let Layout { size, location, .. } = taffy.layout(node3).unwrap();
    assert_eq!(size.width, 11f32, "width of node {:?}. Expected {}. Actual {}", node3.data(), 11f32, size.width);
    assert_eq!(size.height, 40f32, "height of node {:?}. Expected {}. Actual {}", node3.data(), 40f32, size.height);
    assert_eq!(location.x, 102f32, "x of node {:?}. Expected {}. Actual {}", node3.data(), 102f32, location.x);
    assert_eq!(location.y, 40f32, "y of node {:?}. Expected {}. Actual {}", node3.data(), 40f32, location.y);
    let Layout { size, location, .. } = taffy.layout(node4).unwrap();
    assert_eq!(size.width, 11f32, "width of node {:?}. Expected {}. Actual {}", node4.data(), 11f32, size.width);
    assert_eq!(size.height, 40f32, "height of node {:?}. Expected {}. Actual {}", node4.data(), 40f32, size.height);
    assert_eq!(location.x, 113f32, "x of node {:?}. Expected {}. Actual {}", node4.data(), 113f32, location.x);
    assert_eq!(location.y, 40f32, "y of node {:?}. Expected {}. Actual {}", node4.data(), 40f32, location.y);
    let Layout { size, location, .. } = taffy.layout(node5).unwrap();
    assert_eq!(size.width, 10f32, "width of node {:?}. Expected {}. Actual {}", node5.data(), 10f32, size.width);
    assert_eq!(size.height, 40f32, "height of node {:?}. Expected {}. Actual {}", node5.data(), 40f32, size.height);
    assert_eq!(location.x, 124f32, "x of node {:?}. Expected {}. Actual {}", node5.data(), 124f32, location.x);
    assert_eq!(location.y, 40f32, "y of node {:?}. Expected {}. Actual {}", node5.data(), 40f32, location.y);
    let Layout { size, location, .. } = taffy.layout(node6).unwrap();
    assert_eq!(size.width, 65f32, "width of node {:?}. Expected {}. Actual {}", node6.data(), 65f32, size.width);
    assert_eq!(size.height, 40f32, "height of node {:?}. Expected {}. Actual {}", node6.data(), 40f32, size.height);
    assert_eq!(location.x, 134f32, "x of node {:?}. Expected {}. Actual {}", node6.data(), 134f32, location.x);
    assert_eq!(location.y, 40f32, "y of node {:?}. Expected {}. Actual {}", node6.data(), 40f32, location.y);
    let Layout { size, location, .. } = taffy.layout(node7).unwrap();
    assert_eq!(size.width, 2f32, "width of node {:?}. Expected {}. Actual {}", node7.data(), 2f32, size.width);
    assert_eq!(size.height, 40f32, "height of node {:?}. Expected {}. Actual {}", node7.data(), 40f32, size.height);
    assert_eq!(location.x, 199f32, "x of node {:?}. Expected {}. Actual {}", node7.data(), 199f32, location.x);
    assert_eq!(location.y, 40f32, "y of node {:?}. Expected {}. Actual {}", node7.data(), 40f32, location.y);
    let Layout { size, location, .. } = taffy.layout(node8).unwrap();
    assert_eq!(size.width, 4f32, "width of node {:?}. Expected {}. Actual {}", node8.data(), 4f32, size.width);
    assert_eq!(size.height, 40f32, "height of node {:?}. Expected {}. Actual {}", node8.data(), 40f32, size.height);
    assert_eq!(location.x, 201f32, "x of node {:?}. Expected {}. Actual {}", node8.data(), 201f32, location.x);
    assert_eq!(location.y, 40f32, "y of node {:?}. Expected {}. Actual {}", node8.data(), 40f32, location.y);
    let Layout { size, location, .. } = taffy.layout(node9).unwrap();
    assert_eq!(size.width, 2f32, "width of node {:?}. Expected {}. Actual {}", node9.data(), 2f32, size.width);
    assert_eq!(size.height, 40f32, "height of node {:?}. Expected {}. Actual {}", node9.data(), 40f32, size.height);
    assert_eq!(location.x, 205f32, "x of node {:?}. Expected {}. Actual {}", node9.data(), 205f32, location.x);
    assert_eq!(location.y, 40f32, "y of node {:?}. Expected {}. Actual {}", node9.data(), 40f32, location.y);
    let Layout { size, location, .. } = taffy.layout(node10).unwrap();
    assert_eq!(size.width, 2f32, "width of node {:?}. Expected {}. Actual {}", node10.data(), 2f32, size.width);
    assert_eq!(size.height, 40f32, "height of node {:?}. Expected {}. Actual {}", node10.data(), 40f32, size.height);
    assert_eq!(location.x, 207f32, "x of node {:?}. Expected {}. Actual {}", node10.data(), 207f32, location.x);
    assert_eq!(location.y, 40f32, "y of node {:?}. Expected {}. Actual {}", node10.data(), 40f32, location.y);
    let Layout { size, location, .. } = taffy.layout(node11).unwrap();
    assert_eq!(size.width, 11f32, "width of node {:?}. Expected {}. Actual {}", node11.data(), 11f32, size.width);
    assert_eq!(size.height, 40f32, "height of node {:?}. Expected {}. Actual {}", node11.data(), 40f32, size.height);
    assert_eq!(location.x, 209f32, "x of node {:?}. Expected {}. Actual {}", node11.data(), 209f32, location.x);
    assert_eq!(location.y, 40f32, "y of node {:?}. Expected {}. Actual {}", node11.data(), 40f32, location.y);
    let Layout { size, location, .. } = taffy.layout(node12).unwrap();
    assert_eq!(size.width, 11f32, "width of node {:?}. Expected {}. Actual {}", node12.data(), 11f32, size.width);
    assert_eq!(size.height, 40f32, "height of node {:?}. Expected {}. Actual {}", node12.data(), 40f32, size.height);
    assert_eq!(location.x, 220f32, "x of node {:?}. Expected {}. Actual {}", node12.data(), 220f32, location.x);
    assert_eq!(location.y, 40f32, "y of node {:?}. Expected {}. Actual {}", node12.data(), 40f32, location.y);
    let Layout { size, location, .. } = taffy.layout(node13).unwrap();
    assert_eq!(size.width, 91f32, "width of node {:?}. Expected {}. Actual {}", node13.data(), 91f32, size.width);
    assert_eq!(size.height, 40f32, "height of node {:?}. Expected {}. Actual {}", node13.data(), 40f32, size.height);
    assert_eq!(location.x, 231f32, "x of node {:?}. Expected {}. Actual {}", node13.data(), 231f32, location.x);
    assert_eq!(location.y, 40f32, "y of node {:?}. Expected {}. Actual {}", node13.data(), 40f32, location.y);
}
