#[test]
fn contents_flex_nested2() {
    #[allow(unused_imports)]
    use taffy::{prelude::*, tree::Layout, Taffy};
    let mut taffy: Taffy<crate::TextMeasure> = Taffy::new();
    let node0 = taffy
        .new_leaf(taffy::style::Style {
            size: taffy::geometry::Size { width: taffy::style::Dimension::Length(30f32), height: auto() },
            ..Default::default()
        })
        .unwrap();
    let node1 = taffy
        .new_leaf(taffy::style::Style {
            size: taffy::geometry::Size { width: taffy::style::Dimension::Length(30f32), height: auto() },
            ..Default::default()
        })
        .unwrap();
    let node20 = taffy
        .new_leaf(taffy::style::Style {
            size: taffy::geometry::Size { width: taffy::style::Dimension::Length(30f32), height: auto() },
            ..Default::default()
        })
        .unwrap();
    let node2100 = taffy
        .new_leaf(taffy::style::Style {
            size: taffy::geometry::Size { width: taffy::style::Dimension::Length(30f32), height: auto() },
            ..Default::default()
        })
        .unwrap();
    let node2101 = taffy
        .new_leaf(taffy::style::Style {
            size: taffy::geometry::Size { width: taffy::style::Dimension::Length(30f32), height: auto() },
            ..Default::default()
        })
        .unwrap();
    let node210 = taffy
        .new_with_children(
            taffy::style::Style { display: taffy::style::Display::Contents, ..Default::default() },
            &[node2100, node2101],
        )
        .unwrap();
    let node211 = taffy
        .new_leaf(taffy::style::Style {
            size: taffy::geometry::Size { width: taffy::style::Dimension::Length(30f32), height: auto() },
            ..Default::default()
        })
        .unwrap();
    let node21 = taffy
        .new_with_children(
            taffy::style::Style { display: taffy::style::Display::Contents, ..Default::default() },
            &[node210, node211],
        )
        .unwrap();
    let node2 = taffy
        .new_with_children(
            taffy::style::Style { display: taffy::style::Display::Contents, ..Default::default() },
            &[node20, node21],
        )
        .unwrap();
    let node = taffy
        .new_with_children(
            taffy::style::Style {
                display: taffy::style::Display::Flex,
                justify_content: Some(taffy::style::JustifyContent::SpaceBetween),
                size: taffy::geometry::Size {
                    width: taffy::style::Dimension::Length(400f32),
                    height: taffy::style::Dimension::Length(300f32),
                },
                ..Default::default()
            },
            &[node0, node1, node2],
        )
        .unwrap();
    taffy.compute_layout_with_measure(node, taffy::geometry::Size::MAX_CONTENT, crate::test_measure_function).unwrap();
    println!("\nComputed tree:");
    taffy.print_tree(node);
    println!();
    let Layout { size, location, .. } = taffy.layout(node).unwrap();
    assert_eq!(size.width, 400f32, "width of node {:?}. Expected {}. Actual {}", node, 400f32, size.width);
    assert_eq!(size.height, 300f32, "height of node {:?}. Expected {}. Actual {}", node, 300f32, size.height);
    assert_eq!(location.x, 0f32, "x of node {:?}. Expected {}. Actual {}", node, 0f32, location.x);
    assert_eq!(location.y, 0f32, "y of node {:?}. Expected {}. Actual {}", node, 0f32, location.y);
    let Layout { size, location, .. } = taffy.layout(node0).unwrap();
    assert_eq!(size.width, 30f32, "width of node {:?}. Expected {}. Actual {}", node0, 30f32, size.width);
    assert_eq!(size.height, 300f32, "height of node {:?}. Expected {}. Actual {}", node0, 300f32, size.height);
    assert_eq!(location.x, 0f32, "x of node {:?}. Expected {}. Actual {}", node0, 0f32, location.x);
    assert_eq!(location.y, 0f32, "y of node {:?}. Expected {}. Actual {}", node0, 0f32, location.y);
    let Layout { size, location, .. } = taffy.layout(node1).unwrap();
    assert_eq!(size.width, 30f32, "width of node {:?}. Expected {}. Actual {}", node1, 30f32, size.width);
    assert_eq!(size.height, 300f32, "height of node {:?}. Expected {}. Actual {}", node1, 300f32, size.height);
    assert_eq!(location.x, 74f32, "x of node {:?}. Expected {}. Actual {}", node1, 74f32, location.x);
    assert_eq!(location.y, 0f32, "y of node {:?}. Expected {}. Actual {}", node1, 0f32, location.y);
    let Layout { size, location, .. } = taffy.layout(node2).unwrap();
    assert_eq!(size.width, 0f32, "width of node {:?}. Expected {}. Actual {}", node2, 0f32, size.width);
    assert_eq!(size.height, 0f32, "height of node {:?}. Expected {}. Actual {}", node2, 0f32, size.height);
    assert_eq!(location.x, 0f32, "x of node {:?}. Expected {}. Actual {}", node2, 0f32, location.x);
    assert_eq!(location.y, 0f32, "y of node {:?}. Expected {}. Actual {}", node2, 0f32, location.y);
    let Layout { size, location, .. } = taffy.layout(node20).unwrap();
    assert_eq!(size.width, 30f32, "width of node {:?}. Expected {}. Actual {}", node20, 30f32, size.width);
    assert_eq!(size.height, 300f32, "height of node {:?}. Expected {}. Actual {}", node20, 300f32, size.height);
    assert_eq!(location.x, 148f32, "x of node {:?}. Expected {}. Actual {}", node20, 148f32, location.x);
    assert_eq!(location.y, 0f32, "y of node {:?}. Expected {}. Actual {}", node20, 0f32, location.y);
    let Layout { size, location, .. } = taffy.layout(node21).unwrap();
    assert_eq!(size.width, 0f32, "width of node {:?}. Expected {}. Actual {}", node21, 0f32, size.width);
    assert_eq!(size.height, 0f32, "height of node {:?}. Expected {}. Actual {}", node21, 0f32, size.height);
    assert_eq!(location.x, 0f32, "x of node {:?}. Expected {}. Actual {}", node21, 0f32, location.x);
    assert_eq!(location.y, 0f32, "y of node {:?}. Expected {}. Actual {}", node21, 0f32, location.y);
    let Layout { size, location, .. } = taffy.layout(node210).unwrap();
    assert_eq!(size.width, 0f32, "width of node {:?}. Expected {}. Actual {}", node210, 0f32, size.width);
    assert_eq!(size.height, 0f32, "height of node {:?}. Expected {}. Actual {}", node210, 0f32, size.height);
    assert_eq!(location.x, 0f32, "x of node {:?}. Expected {}. Actual {}", node210, 0f32, location.x);
    assert_eq!(location.y, 0f32, "y of node {:?}. Expected {}. Actual {}", node210, 0f32, location.y);
    let Layout { size, location, .. } = taffy.layout(node2100).unwrap();
    assert_eq!(size.width, 30f32, "width of node {:?}. Expected {}. Actual {}", node2100, 30f32, size.width);
    assert_eq!(size.height, 300f32, "height of node {:?}. Expected {}. Actual {}", node2100, 300f32, size.height);
    assert_eq!(location.x, 222f32, "x of node {:?}. Expected {}. Actual {}", node2100, 222f32, location.x);
    assert_eq!(location.y, 0f32, "y of node {:?}. Expected {}. Actual {}", node2100, 0f32, location.y);
    let Layout { size, location, .. } = taffy.layout(node2101).unwrap();
    assert_eq!(size.width, 30f32, "width of node {:?}. Expected {}. Actual {}", node2101, 30f32, size.width);
    assert_eq!(size.height, 300f32, "height of node {:?}. Expected {}. Actual {}", node2101, 300f32, size.height);
    assert_eq!(location.x, 296f32, "x of node {:?}. Expected {}. Actual {}", node2101, 296f32, location.x);
    assert_eq!(location.y, 0f32, "y of node {:?}. Expected {}. Actual {}", node2101, 0f32, location.y);
    let Layout { size, location, .. } = taffy.layout(node211).unwrap();
    assert_eq!(size.width, 30f32, "width of node {:?}. Expected {}. Actual {}", node211, 30f32, size.width);
    assert_eq!(size.height, 300f32, "height of node {:?}. Expected {}. Actual {}", node211, 300f32, size.height);
    assert_eq!(location.x, 370f32, "x of node {:?}. Expected {}. Actual {}", node211, 370f32, location.x);
    assert_eq!(location.y, 0f32, "y of node {:?}. Expected {}. Actual {}", node211, 0f32, location.y);
}
