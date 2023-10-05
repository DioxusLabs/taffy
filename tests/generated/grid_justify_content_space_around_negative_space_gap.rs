#[test]
fn grid_justify_content_space_around_negative_space_gap() {
    use slotmap::Key;
    #[allow(unused_imports)]
    use taffy::{layout::Layout, prelude::*};
    let mut taffy = taffy::Taffy::new();
    let node00 = taffy.new_leaf(taffy::style::Style { ..Default::default() }).unwrap();
    let node01 = taffy.new_leaf(taffy::style::Style { ..Default::default() }).unwrap();
    let node02 = taffy.new_leaf(taffy::style::Style { ..Default::default() }).unwrap();
    let node03 = taffy.new_leaf(taffy::style::Style { ..Default::default() }).unwrap();
    let node04 = taffy.new_leaf(taffy::style::Style { ..Default::default() }).unwrap();
    let node05 = taffy.new_leaf(taffy::style::Style { ..Default::default() }).unwrap();
    let node06 = taffy.new_leaf(taffy::style::Style { ..Default::default() }).unwrap();
    let node07 = taffy.new_leaf(taffy::style::Style { ..Default::default() }).unwrap();
    let node08 = taffy.new_leaf(taffy::style::Style { ..Default::default() }).unwrap();
    let node0 = taffy
        .new_with_children(
            taffy::style::Style {
                display: taffy::style::Display::Grid,
                align_content: Some(taffy::style::AlignContent::Center),
                justify_content: Some(taffy::style::JustifyContent::SpaceAround),
                gap: taffy::geometry::Size {
                    width: taffy::style::LengthPercentage::Points(10f32),
                    height: taffy::style::LengthPercentage::Points(10f32),
                },
                grid_template_rows: vec![points(20f32), points(20f32), points(20f32)],
                grid_template_columns: vec![points(40f32), points(40f32), points(40f32)],
                size: taffy::geometry::Size {
                    width: taffy::style::Dimension::Points(120f32),
                    height: taffy::style::Dimension::Points(120f32),
                },
                ..Default::default()
            },
            &[node00, node01, node02, node03, node04, node05, node06, node07, node08],
        )
        .unwrap();
    let node = taffy
        .new_with_children(
            taffy::style::Style {
                display: taffy::style::Display::Grid,
                size: taffy::geometry::Size {
                    width: taffy::style::Dimension::Points(240f32),
                    height: taffy::style::Dimension::Points(240f32),
                },
                border: taffy::geometry::Rect {
                    left: taffy::style::LengthPercentage::Points(60f32),
                    right: taffy::style::LengthPercentage::Points(60f32),
                    top: taffy::style::LengthPercentage::Points(60f32),
                    bottom: taffy::style::LengthPercentage::Points(60f32),
                },
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
    assert_eq!(size.width, 240f32, "width of node {:?}. Expected {}. Actual {}", node.data(), 240f32, size.width);
    assert_eq!(size.height, 240f32, "height of node {:?}. Expected {}. Actual {}", node.data(), 240f32, size.height);
    assert_eq!(location.x, 0f32, "x of node {:?}. Expected {}. Actual {}", node.data(), 0f32, location.x);
    assert_eq!(location.y, 0f32, "y of node {:?}. Expected {}. Actual {}", node.data(), 0f32, location.y);
    let Layout { size, location, .. } = taffy.layout(node0).unwrap();
    assert_eq!(size.width, 120f32, "width of node {:?}. Expected {}. Actual {}", node0.data(), 120f32, size.width);
    assert_eq!(size.height, 120f32, "height of node {:?}. Expected {}. Actual {}", node0.data(), 120f32, size.height);
    assert_eq!(location.x, 60f32, "x of node {:?}. Expected {}. Actual {}", node0.data(), 60f32, location.x);
    assert_eq!(location.y, 60f32, "y of node {:?}. Expected {}. Actual {}", node0.data(), 60f32, location.y);
    let Layout { size, location, .. } = taffy.layout(node00).unwrap();
    assert_eq!(size.width, 40f32, "width of node {:?}. Expected {}. Actual {}", node00.data(), 40f32, size.width);
    assert_eq!(size.height, 20f32, "height of node {:?}. Expected {}. Actual {}", node00.data(), 20f32, size.height);
    assert_eq!(location.x, -10f32, "x of node {:?}. Expected {}. Actual {}", node00.data(), -10f32, location.x);
    assert_eq!(location.y, 20f32, "y of node {:?}. Expected {}. Actual {}", node00.data(), 20f32, location.y);
    let Layout { size, location, .. } = taffy.layout(node01).unwrap();
    assert_eq!(size.width, 40f32, "width of node {:?}. Expected {}. Actual {}", node01.data(), 40f32, size.width);
    assert_eq!(size.height, 20f32, "height of node {:?}. Expected {}. Actual {}", node01.data(), 20f32, size.height);
    assert_eq!(location.x, 40f32, "x of node {:?}. Expected {}. Actual {}", node01.data(), 40f32, location.x);
    assert_eq!(location.y, 20f32, "y of node {:?}. Expected {}. Actual {}", node01.data(), 20f32, location.y);
    let Layout { size, location, .. } = taffy.layout(node02).unwrap();
    assert_eq!(size.width, 40f32, "width of node {:?}. Expected {}. Actual {}", node02.data(), 40f32, size.width);
    assert_eq!(size.height, 20f32, "height of node {:?}. Expected {}. Actual {}", node02.data(), 20f32, size.height);
    assert_eq!(location.x, 90f32, "x of node {:?}. Expected {}. Actual {}", node02.data(), 90f32, location.x);
    assert_eq!(location.y, 20f32, "y of node {:?}. Expected {}. Actual {}", node02.data(), 20f32, location.y);
    let Layout { size, location, .. } = taffy.layout(node03).unwrap();
    assert_eq!(size.width, 40f32, "width of node {:?}. Expected {}. Actual {}", node03.data(), 40f32, size.width);
    assert_eq!(size.height, 20f32, "height of node {:?}. Expected {}. Actual {}", node03.data(), 20f32, size.height);
    assert_eq!(location.x, -10f32, "x of node {:?}. Expected {}. Actual {}", node03.data(), -10f32, location.x);
    assert_eq!(location.y, 50f32, "y of node {:?}. Expected {}. Actual {}", node03.data(), 50f32, location.y);
    let Layout { size, location, .. } = taffy.layout(node04).unwrap();
    assert_eq!(size.width, 40f32, "width of node {:?}. Expected {}. Actual {}", node04.data(), 40f32, size.width);
    assert_eq!(size.height, 20f32, "height of node {:?}. Expected {}. Actual {}", node04.data(), 20f32, size.height);
    assert_eq!(location.x, 40f32, "x of node {:?}. Expected {}. Actual {}", node04.data(), 40f32, location.x);
    assert_eq!(location.y, 50f32, "y of node {:?}. Expected {}. Actual {}", node04.data(), 50f32, location.y);
    let Layout { size, location, .. } = taffy.layout(node05).unwrap();
    assert_eq!(size.width, 40f32, "width of node {:?}. Expected {}. Actual {}", node05.data(), 40f32, size.width);
    assert_eq!(size.height, 20f32, "height of node {:?}. Expected {}. Actual {}", node05.data(), 20f32, size.height);
    assert_eq!(location.x, 90f32, "x of node {:?}. Expected {}. Actual {}", node05.data(), 90f32, location.x);
    assert_eq!(location.y, 50f32, "y of node {:?}. Expected {}. Actual {}", node05.data(), 50f32, location.y);
    let Layout { size, location, .. } = taffy.layout(node06).unwrap();
    assert_eq!(size.width, 40f32, "width of node {:?}. Expected {}. Actual {}", node06.data(), 40f32, size.width);
    assert_eq!(size.height, 20f32, "height of node {:?}. Expected {}. Actual {}", node06.data(), 20f32, size.height);
    assert_eq!(location.x, -10f32, "x of node {:?}. Expected {}. Actual {}", node06.data(), -10f32, location.x);
    assert_eq!(location.y, 80f32, "y of node {:?}. Expected {}. Actual {}", node06.data(), 80f32, location.y);
    let Layout { size, location, .. } = taffy.layout(node07).unwrap();
    assert_eq!(size.width, 40f32, "width of node {:?}. Expected {}. Actual {}", node07.data(), 40f32, size.width);
    assert_eq!(size.height, 20f32, "height of node {:?}. Expected {}. Actual {}", node07.data(), 20f32, size.height);
    assert_eq!(location.x, 40f32, "x of node {:?}. Expected {}. Actual {}", node07.data(), 40f32, location.x);
    assert_eq!(location.y, 80f32, "y of node {:?}. Expected {}. Actual {}", node07.data(), 80f32, location.y);
    let Layout { size, location, .. } = taffy.layout(node08).unwrap();
    assert_eq!(size.width, 40f32, "width of node {:?}. Expected {}. Actual {}", node08.data(), 40f32, size.width);
    assert_eq!(size.height, 20f32, "height of node {:?}. Expected {}. Actual {}", node08.data(), 20f32, size.height);
    assert_eq!(location.x, 90f32, "x of node {:?}. Expected {}. Actual {}", node08.data(), 90f32, location.x);
    assert_eq!(location.y, 80f32, "y of node {:?}. Expected {}. Actual {}", node08.data(), 80f32, location.y);
}
