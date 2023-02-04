#[test]
fn gridflex_kitchen_sink() {
    use slotmap::Key;
    #[allow(unused_imports)]
    use taffy::{layout::Layout, prelude::*};
    let mut taffy = taffy::Taffy::new();
    let node000 = taffy
        .new_leaf(taffy::style::Style {
            size: taffy::geometry::Size { width: taffy::style::Dimension::Points(20f32), height: auto() },
            ..Default::default()
        })
        .unwrap();
    let node00100 = taffy
        .new_leaf(taffy::style::Style {
            size: taffy::geometry::Size { width: taffy::style::Dimension::Points(20f32), height: auto() },
            ..Default::default()
        })
        .unwrap();
    let node00101 = taffy.new_leaf(taffy::style::Style { ..Default::default() }).unwrap();
    let node00102 = taffy.new_leaf(taffy::style::Style { ..Default::default() }).unwrap();
    let node00103 = taffy.new_leaf(taffy::style::Style { ..Default::default() }).unwrap();
    let node0010 = taffy
        .new_with_children(
            taffy::style::Style {
                display: taffy::style::Display::Grid,
                grid_template_rows: vec![percent(0.3f32), percent(0.1f32)],
                grid_template_columns: vec![auto(), percent(0.1f32)],
                ..Default::default()
            },
            &[node00100, node00101, node00102, node00103],
        )
        .unwrap();
    let node001 = taffy
        .new_with_children(
            taffy::style::Style {
                flex_grow: 1f32,
                size: taffy::geometry::Size { width: taffy::style::Dimension::Points(50f32), height: auto() },
                ..Default::default()
            },
            &[node0010],
        )
        .unwrap();
    let node00 = taffy.new_with_children(taffy::style::Style { ..Default::default() }, &[node000, node001]).unwrap();
    let node01 = taffy
        .new_leaf_with_measure(
            taffy::style::Style { ..Default::default() },
            taffy::node::MeasureFunc::Raw(|known_dimensions, available_space| {
                const TEXT: &str = "HH";
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
    let node02 = taffy
        .new_leaf_with_measure(
            taffy::style::Style { ..Default::default() },
            taffy::node::MeasureFunc::Raw(|known_dimensions, available_space| {
                const TEXT: &str = "HH";
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
    let node03 = taffy
        .new_leaf_with_measure(
            taffy::style::Style { ..Default::default() },
            taffy::node::MeasureFunc::Raw(|known_dimensions, available_space| {
                const TEXT: &str = "HH";
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
    let node0 = taffy
        .new_with_children(
            taffy::style::Style {
                display: taffy::style::Display::Grid,
                grid_template_rows: vec![flex(1f32), flex(1f32)],
                grid_template_columns: vec![flex(1f32), flex(1f32)],
                ..Default::default()
            },
            &[node00, node01, node02, node03],
        )
        .unwrap();
    let node = taffy.new_with_children(taffy::style::Style { ..Default::default() }, &[node0]).unwrap();
    taffy.compute_layout(node, taffy::geometry::Size::MAX_CONTENT).unwrap();
    println!("\nComputed tree:");
    taffy::debug::print_tree(&taffy, node);
    println!();
    let Layout { size, location, .. } = taffy.layout(node).unwrap();
    assert_eq!(size.width, 140f32, "width of node {:?}. Expected {}. Actual {}", node.data(), 140f32, size.width);
    assert_eq!(size.height, 20f32, "height of node {:?}. Expected {}. Actual {}", node.data(), 20f32, size.height);
    assert_eq!(location.x, 0f32, "x of node {:?}. Expected {}. Actual {}", node.data(), 0f32, location.x);
    assert_eq!(location.y, 0f32, "y of node {:?}. Expected {}. Actual {}", node.data(), 0f32, location.y);
    let Layout { size, location, .. } = taffy.layout(node0).unwrap();
    assert_eq!(size.width, 140f32, "width of node {:?}. Expected {}. Actual {}", node0.data(), 140f32, size.width);
    assert_eq!(size.height, 20f32, "height of node {:?}. Expected {}. Actual {}", node0.data(), 20f32, size.height);
    assert_eq!(location.x, 0f32, "x of node {:?}. Expected {}. Actual {}", node0.data(), 0f32, location.x);
    assert_eq!(location.y, 0f32, "y of node {:?}. Expected {}. Actual {}", node0.data(), 0f32, location.y);
    let Layout { size, location, .. } = taffy.layout(node00).unwrap();
    assert_eq!(size.width, 70f32, "width of node {:?}. Expected {}. Actual {}", node00.data(), 70f32, size.width);
    assert_eq!(size.height, 10f32, "height of node {:?}. Expected {}. Actual {}", node00.data(), 10f32, size.height);
    assert_eq!(location.x, 0f32, "x of node {:?}. Expected {}. Actual {}", node00.data(), 0f32, location.x);
    assert_eq!(location.y, 0f32, "y of node {:?}. Expected {}. Actual {}", node00.data(), 0f32, location.y);
    let Layout { size, location, .. } = taffy.layout(node000).unwrap();
    assert_eq!(size.width, 20f32, "width of node {:?}. Expected {}. Actual {}", node000.data(), 20f32, size.width);
    assert_eq!(size.height, 10f32, "height of node {:?}. Expected {}. Actual {}", node000.data(), 10f32, size.height);
    assert_eq!(location.x, 0f32, "x of node {:?}. Expected {}. Actual {}", node000.data(), 0f32, location.x);
    assert_eq!(location.y, 0f32, "y of node {:?}. Expected {}. Actual {}", node000.data(), 0f32, location.y);
    let Layout { size, location, .. } = taffy.layout(node001).unwrap();
    assert_eq!(size.width, 50f32, "width of node {:?}. Expected {}. Actual {}", node001.data(), 50f32, size.width);
    assert_eq!(size.height, 10f32, "height of node {:?}. Expected {}. Actual {}", node001.data(), 10f32, size.height);
    assert_eq!(location.x, 20f32, "x of node {:?}. Expected {}. Actual {}", node001.data(), 20f32, location.x);
    assert_eq!(location.y, 0f32, "y of node {:?}. Expected {}. Actual {}", node001.data(), 0f32, location.y);
    let Layout { size, location, .. } = taffy.layout(node0010).unwrap();
    assert_eq!(size.width, 20f32, "width of node {:?}. Expected {}. Actual {}", node0010.data(), 20f32, size.width);
    assert_eq!(size.height, 10f32, "height of node {:?}. Expected {}. Actual {}", node0010.data(), 10f32, size.height);
    assert_eq!(location.x, 0f32, "x of node {:?}. Expected {}. Actual {}", node0010.data(), 0f32, location.x);
    assert_eq!(location.y, 0f32, "y of node {:?}. Expected {}. Actual {}", node0010.data(), 0f32, location.y);
    let Layout { size, location, .. } = taffy.layout(node00100).unwrap();
    assert_eq!(size.width, 20f32, "width of node {:?}. Expected {}. Actual {}", node00100.data(), 20f32, size.width);
    assert_eq!(size.height, 3f32, "height of node {:?}. Expected {}. Actual {}", node00100.data(), 3f32, size.height);
    assert_eq!(location.x, 0f32, "x of node {:?}. Expected {}. Actual {}", node00100.data(), 0f32, location.x);
    assert_eq!(location.y, 0f32, "y of node {:?}. Expected {}. Actual {}", node00100.data(), 0f32, location.y);
    let Layout { size, location, .. } = taffy.layout(node00101).unwrap();
    assert_eq!(size.width, 2f32, "width of node {:?}. Expected {}. Actual {}", node00101.data(), 2f32, size.width);
    assert_eq!(size.height, 3f32, "height of node {:?}. Expected {}. Actual {}", node00101.data(), 3f32, size.height);
    assert_eq!(location.x, 20f32, "x of node {:?}. Expected {}. Actual {}", node00101.data(), 20f32, location.x);
    assert_eq!(location.y, 0f32, "y of node {:?}. Expected {}. Actual {}", node00101.data(), 0f32, location.y);
    let Layout { size, location, .. } = taffy.layout(node00102).unwrap();
    assert_eq!(size.width, 20f32, "width of node {:?}. Expected {}. Actual {}", node00102.data(), 20f32, size.width);
    assert_eq!(size.height, 1f32, "height of node {:?}. Expected {}. Actual {}", node00102.data(), 1f32, size.height);
    assert_eq!(location.x, 0f32, "x of node {:?}. Expected {}. Actual {}", node00102.data(), 0f32, location.x);
    assert_eq!(location.y, 3f32, "y of node {:?}. Expected {}. Actual {}", node00102.data(), 3f32, location.y);
    let Layout { size, location, .. } = taffy.layout(node00103).unwrap();
    assert_eq!(size.width, 2f32, "width of node {:?}. Expected {}. Actual {}", node00103.data(), 2f32, size.width);
    assert_eq!(size.height, 1f32, "height of node {:?}. Expected {}. Actual {}", node00103.data(), 1f32, size.height);
    assert_eq!(location.x, 20f32, "x of node {:?}. Expected {}. Actual {}", node00103.data(), 20f32, location.x);
    assert_eq!(location.y, 3f32, "y of node {:?}. Expected {}. Actual {}", node00103.data(), 3f32, location.y);
    let Layout { size, location, .. } = taffy.layout(node01).unwrap();
    assert_eq!(size.width, 70f32, "width of node {:?}. Expected {}. Actual {}", node01.data(), 70f32, size.width);
    assert_eq!(size.height, 10f32, "height of node {:?}. Expected {}. Actual {}", node01.data(), 10f32, size.height);
    assert_eq!(location.x, 70f32, "x of node {:?}. Expected {}. Actual {}", node01.data(), 70f32, location.x);
    assert_eq!(location.y, 0f32, "y of node {:?}. Expected {}. Actual {}", node01.data(), 0f32, location.y);
    let Layout { size, location, .. } = taffy.layout(node02).unwrap();
    assert_eq!(size.width, 70f32, "width of node {:?}. Expected {}. Actual {}", node02.data(), 70f32, size.width);
    assert_eq!(size.height, 10f32, "height of node {:?}. Expected {}. Actual {}", node02.data(), 10f32, size.height);
    assert_eq!(location.x, 0f32, "x of node {:?}. Expected {}. Actual {}", node02.data(), 0f32, location.x);
    assert_eq!(location.y, 10f32, "y of node {:?}. Expected {}. Actual {}", node02.data(), 10f32, location.y);
    let Layout { size, location, .. } = taffy.layout(node03).unwrap();
    assert_eq!(size.width, 70f32, "width of node {:?}. Expected {}. Actual {}", node03.data(), 70f32, size.width);
    assert_eq!(size.height, 10f32, "height of node {:?}. Expected {}. Actual {}", node03.data(), 10f32, size.height);
    assert_eq!(location.x, 70f32, "x of node {:?}. Expected {}. Actual {}", node03.data(), 70f32, location.x);
    assert_eq!(location.y, 10f32, "y of node {:?}. Expected {}. Actual {}", node03.data(), 10f32, location.y);
}
