#[test]
fn android_news_feed() {
    use slotmap::Key;
    #[allow(unused_imports)]
    use taffy::{layout::Layout, prelude::*};
    let mut taffy = taffy::Taffy::new();
    let node000000 = taffy
        .new_leaf(taffy::style::Style {
            align_content: Some(taffy::style::AlignContent::Stretch),
            flex_shrink: 0f32,
            size: taffy::geometry::Size {
                width: taffy::style::Dimension::Points(120f32),
                height: taffy::style::Dimension::Points(120f32),
            },
            ..Default::default()
        })
        .unwrap();
    let node00000 = taffy
        .new_with_children(
            taffy::style::Style {
                align_content: Some(taffy::style::AlignContent::Stretch),
                flex_shrink: 0f32,
                ..Default::default()
            },
            &[node000000],
        )
        .unwrap();
    let node000010 = taffy
        .new_leaf(taffy::style::Style {
            align_content: Some(taffy::style::AlignContent::Stretch),
            flex_shrink: 1f32,
            ..Default::default()
        })
        .unwrap();
    let node000011 = taffy
        .new_leaf(taffy::style::Style {
            align_content: Some(taffy::style::AlignContent::Stretch),
            flex_shrink: 1f32,
            ..Default::default()
        })
        .unwrap();
    let node00001 = taffy
        .new_with_children(
            taffy::style::Style {
                align_content: Some(taffy::style::AlignContent::Stretch),
                flex_shrink: 1f32,
                margin: taffy::geometry::Rect {
                    left: zero(),
                    right: taffy::style::LengthPercentageAuto::Points(36f32),
                    top: zero(),
                    bottom: zero(),
                },
                padding: taffy::geometry::Rect {
                    left: taffy::style::LengthPercentage::Points(36f32),
                    right: taffy::style::LengthPercentage::Points(36f32),
                    top: taffy::style::LengthPercentage::Points(21f32),
                    bottom: taffy::style::LengthPercentage::Points(18f32),
                },
                ..Default::default()
            },
            &[node000010, node000011],
        )
        .unwrap();
    let node0000 = taffy
        .new_with_children(
            taffy::style::Style {
                align_items: Some(taffy::style::AlignItems::Start),
                align_content: Some(taffy::style::AlignContent::Stretch),
                margin: taffy::geometry::Rect {
                    left: zero(),
                    right: zero(),
                    top: taffy::style::LengthPercentageAuto::Points(24f32),
                    bottom: zero(),
                },
                ..Default::default()
            },
            &[node00000, node00001],
        )
        .unwrap();
    let node000 = taffy
        .new_with_children(
            taffy::style::Style { align_content: Some(taffy::style::AlignContent::Stretch), ..Default::default() },
            &[node0000],
        )
        .unwrap();
    let node001000 = taffy
        .new_leaf(taffy::style::Style {
            align_content: Some(taffy::style::AlignContent::Stretch),
            flex_shrink: 0f32,
            size: taffy::geometry::Size {
                width: taffy::style::Dimension::Points(72f32),
                height: taffy::style::Dimension::Points(72f32),
            },
            ..Default::default()
        })
        .unwrap();
    let node00100 = taffy
        .new_with_children(
            taffy::style::Style {
                align_content: Some(taffy::style::AlignContent::Stretch),
                flex_shrink: 0f32,
                ..Default::default()
            },
            &[node001000],
        )
        .unwrap();
    let node001010 = taffy
        .new_leaf(taffy::style::Style {
            align_content: Some(taffy::style::AlignContent::Stretch),
            flex_shrink: 1f32,
            ..Default::default()
        })
        .unwrap();
    let node001011 = taffy
        .new_leaf(taffy::style::Style {
            align_content: Some(taffy::style::AlignContent::Stretch),
            flex_shrink: 1f32,
            ..Default::default()
        })
        .unwrap();
    let node00101 = taffy
        .new_with_children(
            taffy::style::Style {
                align_content: Some(taffy::style::AlignContent::Stretch),
                flex_shrink: 1f32,
                margin: taffy::geometry::Rect {
                    left: zero(),
                    right: taffy::style::LengthPercentageAuto::Points(36f32),
                    top: zero(),
                    bottom: zero(),
                },
                padding: taffy::geometry::Rect {
                    left: taffy::style::LengthPercentage::Points(36f32),
                    right: taffy::style::LengthPercentage::Points(36f32),
                    top: taffy::style::LengthPercentage::Points(21f32),
                    bottom: taffy::style::LengthPercentage::Points(18f32),
                },
                ..Default::default()
            },
            &[node001010, node001011],
        )
        .unwrap();
    let node0010 = taffy
        .new_with_children(
            taffy::style::Style {
                align_items: Some(taffy::style::AlignItems::Start),
                align_content: Some(taffy::style::AlignContent::Stretch),
                margin: taffy::geometry::Rect {
                    left: zero(),
                    right: zero(),
                    top: taffy::style::LengthPercentageAuto::Points(24f32),
                    bottom: zero(),
                },
                ..Default::default()
            },
            &[node00100, node00101],
        )
        .unwrap();
    let node001 = taffy
        .new_with_children(
            taffy::style::Style { align_content: Some(taffy::style::AlignContent::Stretch), ..Default::default() },
            &[node0010],
        )
        .unwrap();
    let node00 = taffy
        .new_with_children(
            taffy::style::Style { align_content: Some(taffy::style::AlignContent::Stretch), ..Default::default() },
            &[node000, node001],
        )
        .unwrap();
    let node0 =
        taffy.new_with_children(taffy::style::Style { flex_shrink: 0f32, ..Default::default() }, &[node00]).unwrap();
    let node = taffy
        .new_with_children(
            taffy::style::Style {
                align_content: Some(taffy::style::AlignContent::Stretch),
                flex_shrink: 0f32,
                size: taffy::geometry::Size { width: taffy::style::Dimension::Points(1080f32), height: auto() },
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
    assert_eq!(size.width, 1080f32, "width of node {:?}. Expected {}. Actual {}", node.data(), 1080f32, size.width);
    assert_eq!(size.height, 144f32, "height of node {:?}. Expected {}. Actual {}", node.data(), 144f32, size.height);
    assert_eq!(location.x, 0f32, "x of node {:?}. Expected {}. Actual {}", node.data(), 0f32, location.x);
    assert_eq!(location.y, 0f32, "y of node {:?}. Expected {}. Actual {}", node.data(), 0f32, location.y);
    let Layout { size, location, .. } = taffy.layout(node0).unwrap();
    assert_eq!(size.width, 408f32, "width of node {:?}. Expected {}. Actual {}", node0.data(), 408f32, size.width);
    assert_eq!(size.height, 144f32, "height of node {:?}. Expected {}. Actual {}", node0.data(), 144f32, size.height);
    assert_eq!(location.x, 0f32, "x of node {:?}. Expected {}. Actual {}", node0.data(), 0f32, location.x);
    assert_eq!(location.y, 0f32, "y of node {:?}. Expected {}. Actual {}", node0.data(), 0f32, location.y);
    let Layout { size, location, .. } = taffy.layout(node00).unwrap();
    assert_eq!(size.width, 408f32, "width of node {:?}. Expected {}. Actual {}", node00.data(), 408f32, size.width);
    assert_eq!(size.height, 144f32, "height of node {:?}. Expected {}. Actual {}", node00.data(), 144f32, size.height);
    assert_eq!(location.x, 0f32, "x of node {:?}. Expected {}. Actual {}", node00.data(), 0f32, location.x);
    assert_eq!(location.y, 0f32, "y of node {:?}. Expected {}. Actual {}", node00.data(), 0f32, location.y);
    let Layout { size, location, .. } = taffy.layout(node000).unwrap();
    assert_eq!(size.width, 228f32, "width of node {:?}. Expected {}. Actual {}", node000.data(), 228f32, size.width);
    assert_eq!(size.height, 144f32, "height of node {:?}. Expected {}. Actual {}", node000.data(), 144f32, size.height);
    assert_eq!(location.x, 0f32, "x of node {:?}. Expected {}. Actual {}", node000.data(), 0f32, location.x);
    assert_eq!(location.y, 0f32, "y of node {:?}. Expected {}. Actual {}", node000.data(), 0f32, location.y);
    let Layout { size, location, .. } = taffy.layout(node0000).unwrap();
    assert_eq!(size.width, 228f32, "width of node {:?}. Expected {}. Actual {}", node0000.data(), 228f32, size.width);
    assert_eq!(
        size.height,
        120f32,
        "height of node {:?}. Expected {}. Actual {}",
        node0000.data(),
        120f32,
        size.height
    );
    assert_eq!(location.x, 0f32, "x of node {:?}. Expected {}. Actual {}", node0000.data(), 0f32, location.x);
    assert_eq!(location.y, 24f32, "y of node {:?}. Expected {}. Actual {}", node0000.data(), 24f32, location.y);
    let Layout { size, location, .. } = taffy.layout(node00000).unwrap();
    assert_eq!(size.width, 120f32, "width of node {:?}. Expected {}. Actual {}", node00000.data(), 120f32, size.width);
    assert_eq!(
        size.height,
        120f32,
        "height of node {:?}. Expected {}. Actual {}",
        node00000.data(),
        120f32,
        size.height
    );
    assert_eq!(location.x, 0f32, "x of node {:?}. Expected {}. Actual {}", node00000.data(), 0f32, location.x);
    assert_eq!(location.y, 0f32, "y of node {:?}. Expected {}. Actual {}", node00000.data(), 0f32, location.y);
    let Layout { size, location, .. } = taffy.layout(node000000).unwrap();
    assert_eq!(size.width, 120f32, "width of node {:?}. Expected {}. Actual {}", node000000.data(), 120f32, size.width);
    assert_eq!(
        size.height,
        120f32,
        "height of node {:?}. Expected {}. Actual {}",
        node000000.data(),
        120f32,
        size.height
    );
    assert_eq!(location.x, 0f32, "x of node {:?}. Expected {}. Actual {}", node000000.data(), 0f32, location.x);
    assert_eq!(location.y, 0f32, "y of node {:?}. Expected {}. Actual {}", node000000.data(), 0f32, location.y);
    let Layout { size, location, .. } = taffy.layout(node00001).unwrap();
    assert_eq!(size.width, 72f32, "width of node {:?}. Expected {}. Actual {}", node00001.data(), 72f32, size.width);
    assert_eq!(size.height, 39f32, "height of node {:?}. Expected {}. Actual {}", node00001.data(), 39f32, size.height);
    assert_eq!(location.x, 120f32, "x of node {:?}. Expected {}. Actual {}", node00001.data(), 120f32, location.x);
    assert_eq!(location.y, 0f32, "y of node {:?}. Expected {}. Actual {}", node00001.data(), 0f32, location.y);
    let Layout { size, location, .. } = taffy.layout(node000010).unwrap();
    assert_eq!(size.width, 0f32, "width of node {:?}. Expected {}. Actual {}", node000010.data(), 0f32, size.width);
    assert_eq!(size.height, 0f32, "height of node {:?}. Expected {}. Actual {}", node000010.data(), 0f32, size.height);
    assert_eq!(location.x, 36f32, "x of node {:?}. Expected {}. Actual {}", node000010.data(), 36f32, location.x);
    assert_eq!(location.y, 21f32, "y of node {:?}. Expected {}. Actual {}", node000010.data(), 21f32, location.y);
    let Layout { size, location, .. } = taffy.layout(node000011).unwrap();
    assert_eq!(size.width, 0f32, "width of node {:?}. Expected {}. Actual {}", node000011.data(), 0f32, size.width);
    assert_eq!(size.height, 0f32, "height of node {:?}. Expected {}. Actual {}", node000011.data(), 0f32, size.height);
    assert_eq!(location.x, 36f32, "x of node {:?}. Expected {}. Actual {}", node000011.data(), 36f32, location.x);
    assert_eq!(location.y, 21f32, "y of node {:?}. Expected {}. Actual {}", node000011.data(), 21f32, location.y);
    let Layout { size, location, .. } = taffy.layout(node001).unwrap();
    assert_eq!(size.width, 180f32, "width of node {:?}. Expected {}. Actual {}", node001.data(), 180f32, size.width);
    assert_eq!(size.height, 144f32, "height of node {:?}. Expected {}. Actual {}", node001.data(), 144f32, size.height);
    assert_eq!(location.x, 228f32, "x of node {:?}. Expected {}. Actual {}", node001.data(), 228f32, location.x);
    assert_eq!(location.y, 0f32, "y of node {:?}. Expected {}. Actual {}", node001.data(), 0f32, location.y);
    let Layout { size, location, .. } = taffy.layout(node0010).unwrap();
    assert_eq!(size.width, 180f32, "width of node {:?}. Expected {}. Actual {}", node0010.data(), 180f32, size.width);
    assert_eq!(
        size.height,
        120f32,
        "height of node {:?}. Expected {}. Actual {}",
        node0010.data(),
        120f32,
        size.height
    );
    assert_eq!(location.x, 0f32, "x of node {:?}. Expected {}. Actual {}", node0010.data(), 0f32, location.x);
    assert_eq!(location.y, 24f32, "y of node {:?}. Expected {}. Actual {}", node0010.data(), 24f32, location.y);
    let Layout { size, location, .. } = taffy.layout(node00100).unwrap();
    assert_eq!(size.width, 72f32, "width of node {:?}. Expected {}. Actual {}", node00100.data(), 72f32, size.width);
    assert_eq!(size.height, 72f32, "height of node {:?}. Expected {}. Actual {}", node00100.data(), 72f32, size.height);
    assert_eq!(location.x, 0f32, "x of node {:?}. Expected {}. Actual {}", node00100.data(), 0f32, location.x);
    assert_eq!(location.y, 0f32, "y of node {:?}. Expected {}. Actual {}", node00100.data(), 0f32, location.y);
    let Layout { size, location, .. } = taffy.layout(node001000).unwrap();
    assert_eq!(size.width, 72f32, "width of node {:?}. Expected {}. Actual {}", node001000.data(), 72f32, size.width);
    assert_eq!(
        size.height,
        72f32,
        "height of node {:?}. Expected {}. Actual {}",
        node001000.data(),
        72f32,
        size.height
    );
    assert_eq!(location.x, 0f32, "x of node {:?}. Expected {}. Actual {}", node001000.data(), 0f32, location.x);
    assert_eq!(location.y, 0f32, "y of node {:?}. Expected {}. Actual {}", node001000.data(), 0f32, location.y);
    let Layout { size, location, .. } = taffy.layout(node00101).unwrap();
    assert_eq!(size.width, 72f32, "width of node {:?}. Expected {}. Actual {}", node00101.data(), 72f32, size.width);
    assert_eq!(size.height, 39f32, "height of node {:?}. Expected {}. Actual {}", node00101.data(), 39f32, size.height);
    assert_eq!(location.x, 72f32, "x of node {:?}. Expected {}. Actual {}", node00101.data(), 72f32, location.x);
    assert_eq!(location.y, 0f32, "y of node {:?}. Expected {}. Actual {}", node00101.data(), 0f32, location.y);
    let Layout { size, location, .. } = taffy.layout(node001010).unwrap();
    assert_eq!(size.width, 0f32, "width of node {:?}. Expected {}. Actual {}", node001010.data(), 0f32, size.width);
    assert_eq!(size.height, 0f32, "height of node {:?}. Expected {}. Actual {}", node001010.data(), 0f32, size.height);
    assert_eq!(location.x, 36f32, "x of node {:?}. Expected {}. Actual {}", node001010.data(), 36f32, location.x);
    assert_eq!(location.y, 21f32, "y of node {:?}. Expected {}. Actual {}", node001010.data(), 21f32, location.y);
    let Layout { size, location, .. } = taffy.layout(node001011).unwrap();
    assert_eq!(size.width, 0f32, "width of node {:?}. Expected {}. Actual {}", node001011.data(), 0f32, size.width);
    assert_eq!(size.height, 0f32, "height of node {:?}. Expected {}. Actual {}", node001011.data(), 0f32, size.height);
    assert_eq!(location.x, 36f32, "x of node {:?}. Expected {}. Actual {}", node001011.data(), 36f32, location.x);
    assert_eq!(location.y, 21f32, "y of node {:?}. Expected {}. Actual {}", node001011.data(), 21f32, location.y);
}
