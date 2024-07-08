#[test]
#[allow(non_snake_case)]
fn flex_grow_0_min_size__border_box() {
    #[allow(unused_imports)]
    use taffy::{prelude::*, tree::Layout, TaffyTree};
    let mut taffy: TaffyTree<crate::TextMeasure> = TaffyTree::new();
    let node0 = taffy
        .new_leaf_with_context(
            taffy::style::Style {
                flex_grow: 0f32,
                flex_shrink: 0f32,
                flex_basis: taffy::style::Dimension::Percent(0f32),
                ..Default::default()
            },
            crate::TextMeasure {
                text_content: "one",
                writing_mode: crate::WritingMode::Horizontal,
                _aspect_ratio: None,
            },
        )
        .unwrap();
    let node1 = taffy
        .new_leaf_with_context(
            taffy::style::Style {
                flex_grow: 0f32,
                flex_shrink: 0f32,
                flex_basis: taffy::style::Dimension::Percent(0f32),
                ..Default::default()
            },
            crate::TextMeasure {
                text_content: "two",
                writing_mode: crate::WritingMode::Horizontal,
                _aspect_ratio: None,
            },
        )
        .unwrap();
    let node2 = taffy
        .new_leaf_with_context(
            taffy::style::Style {
                flex_grow: 0f32,
                flex_shrink: 0f32,
                flex_basis: taffy::style::Dimension::Percent(0f32),
                ..Default::default()
            },
            crate::TextMeasure {
                text_content: "three",
                writing_mode: crate::WritingMode::Horizontal,
                _aspect_ratio: None,
            },
        )
        .unwrap();
    let node3 = taffy
        .new_leaf_with_context(
            taffy::style::Style {
                flex_grow: 0f32,
                flex_shrink: 0f32,
                flex_basis: taffy::style::Dimension::Percent(0f32),
                ..Default::default()
            },
            crate::TextMeasure {
                text_content: "four",
                writing_mode: crate::WritingMode::Horizontal,
                _aspect_ratio: None,
            },
        )
        .unwrap();
    let node = taffy
        .new_with_children(
            taffy::style::Style {
                display: taffy::style::Display::Flex,
                size: taffy::geometry::Size {
                    width: taffy::style::Dimension::Length(400f32),
                    height: taffy::style::Dimension::Length(100f32),
                },
                border: taffy::geometry::Rect {
                    left: taffy::style::LengthPercentage::Length(1f32),
                    right: taffy::style::LengthPercentage::Length(1f32),
                    top: taffy::style::LengthPercentage::Length(1f32),
                    bottom: taffy::style::LengthPercentage::Length(1f32),
                },
                ..Default::default()
            },
            &[node0, node1, node2, node3],
        )
        .unwrap();
    taffy.compute_layout_with_measure(node, taffy::geometry::Size::MAX_CONTENT, crate::test_measure_function).unwrap();
    println!("\nComputed tree:");
    taffy.print_tree(node);
    println!();
    #[cfg_attr(not(feature = "content_size"), allow(unused_variables))]
    let layout @ Layout { size, location, .. } = taffy.layout(node).unwrap();
    assert_eq!(size.width, 400f32, "width of node {:?}. Expected {}. Actual {}", node, 400f32, size.width);
    assert_eq!(size.height, 100f32, "height of node {:?}. Expected {}. Actual {}", node, 100f32, size.height);
    assert_eq!(location.x, 0f32, "x of node {:?}. Expected {}. Actual {}", node, 0f32, location.x);
    assert_eq!(location.y, 0f32, "y of node {:?}. Expected {}. Actual {}", node, 0f32, location.y);
    #[cfg(feature = "content_size")]
    assert_eq!(
        layout.scroll_width(),
        0f32,
        "scroll_width of node {:?}. Expected {}. Actual {}",
        node,
        0f32,
        layout.scroll_width()
    );
    #[cfg(feature = "content_size")]
    assert_eq!(
        layout.scroll_height(),
        0f32,
        "scroll_height of node {:?}. Expected {}. Actual {}",
        node,
        0f32,
        layout.scroll_height()
    );
    #[cfg_attr(not(feature = "content_size"), allow(unused_variables))]
    let layout @ Layout { size, location, .. } = taffy.layout(node0).unwrap();
    assert_eq!(size.width, 30f32, "width of node {:?}. Expected {}. Actual {}", node0, 30f32, size.width);
    assert_eq!(size.height, 98f32, "height of node {:?}. Expected {}. Actual {}", node0, 98f32, size.height);
    assert_eq!(location.x, 1f32, "x of node {:?}. Expected {}. Actual {}", node0, 1f32, location.x);
    assert_eq!(location.y, 1f32, "y of node {:?}. Expected {}. Actual {}", node0, 1f32, location.y);
    #[cfg(feature = "content_size")]
    assert_eq!(
        layout.scroll_width(),
        0f32,
        "scroll_width of node {:?}. Expected {}. Actual {}",
        node0,
        0f32,
        layout.scroll_width()
    );
    #[cfg(feature = "content_size")]
    assert_eq!(
        layout.scroll_height(),
        0f32,
        "scroll_height of node {:?}. Expected {}. Actual {}",
        node0,
        0f32,
        layout.scroll_height()
    );
    #[cfg_attr(not(feature = "content_size"), allow(unused_variables))]
    let layout @ Layout { size, location, .. } = taffy.layout(node1).unwrap();
    assert_eq!(size.width, 30f32, "width of node {:?}. Expected {}. Actual {}", node1, 30f32, size.width);
    assert_eq!(size.height, 98f32, "height of node {:?}. Expected {}. Actual {}", node1, 98f32, size.height);
    assert_eq!(location.x, 31f32, "x of node {:?}. Expected {}. Actual {}", node1, 31f32, location.x);
    assert_eq!(location.y, 1f32, "y of node {:?}. Expected {}. Actual {}", node1, 1f32, location.y);
    #[cfg(feature = "content_size")]
    assert_eq!(
        layout.scroll_width(),
        0f32,
        "scroll_width of node {:?}. Expected {}. Actual {}",
        node1,
        0f32,
        layout.scroll_width()
    );
    #[cfg(feature = "content_size")]
    assert_eq!(
        layout.scroll_height(),
        0f32,
        "scroll_height of node {:?}. Expected {}. Actual {}",
        node1,
        0f32,
        layout.scroll_height()
    );
    #[cfg_attr(not(feature = "content_size"), allow(unused_variables))]
    let layout @ Layout { size, location, .. } = taffy.layout(node2).unwrap();
    assert_eq!(size.width, 50f32, "width of node {:?}. Expected {}. Actual {}", node2, 50f32, size.width);
    assert_eq!(size.height, 98f32, "height of node {:?}. Expected {}. Actual {}", node2, 98f32, size.height);
    assert_eq!(location.x, 61f32, "x of node {:?}. Expected {}. Actual {}", node2, 61f32, location.x);
    assert_eq!(location.y, 1f32, "y of node {:?}. Expected {}. Actual {}", node2, 1f32, location.y);
    #[cfg(feature = "content_size")]
    assert_eq!(
        layout.scroll_width(),
        0f32,
        "scroll_width of node {:?}. Expected {}. Actual {}",
        node2,
        0f32,
        layout.scroll_width()
    );
    #[cfg(feature = "content_size")]
    assert_eq!(
        layout.scroll_height(),
        0f32,
        "scroll_height of node {:?}. Expected {}. Actual {}",
        node2,
        0f32,
        layout.scroll_height()
    );
    #[cfg_attr(not(feature = "content_size"), allow(unused_variables))]
    let layout @ Layout { size, location, .. } = taffy.layout(node3).unwrap();
    assert_eq!(size.width, 40f32, "width of node {:?}. Expected {}. Actual {}", node3, 40f32, size.width);
    assert_eq!(size.height, 98f32, "height of node {:?}. Expected {}. Actual {}", node3, 98f32, size.height);
    assert_eq!(location.x, 111f32, "x of node {:?}. Expected {}. Actual {}", node3, 111f32, location.x);
    assert_eq!(location.y, 1f32, "y of node {:?}. Expected {}. Actual {}", node3, 1f32, location.y);
    #[cfg(feature = "content_size")]
    assert_eq!(
        layout.scroll_width(),
        0f32,
        "scroll_width of node {:?}. Expected {}. Actual {}",
        node3,
        0f32,
        layout.scroll_width()
    );
    #[cfg(feature = "content_size")]
    assert_eq!(
        layout.scroll_height(),
        0f32,
        "scroll_height of node {:?}. Expected {}. Actual {}",
        node3,
        0f32,
        layout.scroll_height()
    );
}

#[test]
#[allow(non_snake_case)]
fn flex_grow_0_min_size__content_box() {
    #[allow(unused_imports)]
    use taffy::{prelude::*, tree::Layout, TaffyTree};
    let mut taffy: TaffyTree<crate::TextMeasure> = TaffyTree::new();
    let node0 = taffy
        .new_leaf_with_context(
            taffy::style::Style {
                box_sizing: taffy::style::BoxSizing::ContentBox,
                flex_grow: 0f32,
                flex_shrink: 0f32,
                flex_basis: taffy::style::Dimension::Percent(0f32),
                ..Default::default()
            },
            crate::TextMeasure {
                text_content: "one",
                writing_mode: crate::WritingMode::Horizontal,
                _aspect_ratio: None,
            },
        )
        .unwrap();
    let node1 = taffy
        .new_leaf_with_context(
            taffy::style::Style {
                box_sizing: taffy::style::BoxSizing::ContentBox,
                flex_grow: 0f32,
                flex_shrink: 0f32,
                flex_basis: taffy::style::Dimension::Percent(0f32),
                ..Default::default()
            },
            crate::TextMeasure {
                text_content: "two",
                writing_mode: crate::WritingMode::Horizontal,
                _aspect_ratio: None,
            },
        )
        .unwrap();
    let node2 = taffy
        .new_leaf_with_context(
            taffy::style::Style {
                box_sizing: taffy::style::BoxSizing::ContentBox,
                flex_grow: 0f32,
                flex_shrink: 0f32,
                flex_basis: taffy::style::Dimension::Percent(0f32),
                ..Default::default()
            },
            crate::TextMeasure {
                text_content: "three",
                writing_mode: crate::WritingMode::Horizontal,
                _aspect_ratio: None,
            },
        )
        .unwrap();
    let node3 = taffy
        .new_leaf_with_context(
            taffy::style::Style {
                box_sizing: taffy::style::BoxSizing::ContentBox,
                flex_grow: 0f32,
                flex_shrink: 0f32,
                flex_basis: taffy::style::Dimension::Percent(0f32),
                ..Default::default()
            },
            crate::TextMeasure {
                text_content: "four",
                writing_mode: crate::WritingMode::Horizontal,
                _aspect_ratio: None,
            },
        )
        .unwrap();
    let node = taffy
        .new_with_children(
            taffy::style::Style {
                display: taffy::style::Display::Flex,
                box_sizing: taffy::style::BoxSizing::ContentBox,
                size: taffy::geometry::Size {
                    width: taffy::style::Dimension::Length(400f32),
                    height: taffy::style::Dimension::Length(100f32),
                },
                border: taffy::geometry::Rect {
                    left: taffy::style::LengthPercentage::Length(1f32),
                    right: taffy::style::LengthPercentage::Length(1f32),
                    top: taffy::style::LengthPercentage::Length(1f32),
                    bottom: taffy::style::LengthPercentage::Length(1f32),
                },
                ..Default::default()
            },
            &[node0, node1, node2, node3],
        )
        .unwrap();
    taffy.compute_layout_with_measure(node, taffy::geometry::Size::MAX_CONTENT, crate::test_measure_function).unwrap();
    println!("\nComputed tree:");
    taffy.print_tree(node);
    println!();
    #[cfg_attr(not(feature = "content_size"), allow(unused_variables))]
    let layout @ Layout { size, location, .. } = taffy.layout(node).unwrap();
    assert_eq!(size.width, 402f32, "width of node {:?}. Expected {}. Actual {}", node, 402f32, size.width);
    assert_eq!(size.height, 102f32, "height of node {:?}. Expected {}. Actual {}", node, 102f32, size.height);
    assert_eq!(location.x, 0f32, "x of node {:?}. Expected {}. Actual {}", node, 0f32, location.x);
    assert_eq!(location.y, 0f32, "y of node {:?}. Expected {}. Actual {}", node, 0f32, location.y);
    #[cfg(feature = "content_size")]
    assert_eq!(
        layout.scroll_width(),
        0f32,
        "scroll_width of node {:?}. Expected {}. Actual {}",
        node,
        0f32,
        layout.scroll_width()
    );
    #[cfg(feature = "content_size")]
    assert_eq!(
        layout.scroll_height(),
        0f32,
        "scroll_height of node {:?}. Expected {}. Actual {}",
        node,
        0f32,
        layout.scroll_height()
    );
    #[cfg_attr(not(feature = "content_size"), allow(unused_variables))]
    let layout @ Layout { size, location, .. } = taffy.layout(node0).unwrap();
    assert_eq!(size.width, 30f32, "width of node {:?}. Expected {}. Actual {}", node0, 30f32, size.width);
    assert_eq!(size.height, 100f32, "height of node {:?}. Expected {}. Actual {}", node0, 100f32, size.height);
    assert_eq!(location.x, 1f32, "x of node {:?}. Expected {}. Actual {}", node0, 1f32, location.x);
    assert_eq!(location.y, 1f32, "y of node {:?}. Expected {}. Actual {}", node0, 1f32, location.y);
    #[cfg(feature = "content_size")]
    assert_eq!(
        layout.scroll_width(),
        0f32,
        "scroll_width of node {:?}. Expected {}. Actual {}",
        node0,
        0f32,
        layout.scroll_width()
    );
    #[cfg(feature = "content_size")]
    assert_eq!(
        layout.scroll_height(),
        0f32,
        "scroll_height of node {:?}. Expected {}. Actual {}",
        node0,
        0f32,
        layout.scroll_height()
    );
    #[cfg_attr(not(feature = "content_size"), allow(unused_variables))]
    let layout @ Layout { size, location, .. } = taffy.layout(node1).unwrap();
    assert_eq!(size.width, 30f32, "width of node {:?}. Expected {}. Actual {}", node1, 30f32, size.width);
    assert_eq!(size.height, 100f32, "height of node {:?}. Expected {}. Actual {}", node1, 100f32, size.height);
    assert_eq!(location.x, 31f32, "x of node {:?}. Expected {}. Actual {}", node1, 31f32, location.x);
    assert_eq!(location.y, 1f32, "y of node {:?}. Expected {}. Actual {}", node1, 1f32, location.y);
    #[cfg(feature = "content_size")]
    assert_eq!(
        layout.scroll_width(),
        0f32,
        "scroll_width of node {:?}. Expected {}. Actual {}",
        node1,
        0f32,
        layout.scroll_width()
    );
    #[cfg(feature = "content_size")]
    assert_eq!(
        layout.scroll_height(),
        0f32,
        "scroll_height of node {:?}. Expected {}. Actual {}",
        node1,
        0f32,
        layout.scroll_height()
    );
    #[cfg_attr(not(feature = "content_size"), allow(unused_variables))]
    let layout @ Layout { size, location, .. } = taffy.layout(node2).unwrap();
    assert_eq!(size.width, 50f32, "width of node {:?}. Expected {}. Actual {}", node2, 50f32, size.width);
    assert_eq!(size.height, 100f32, "height of node {:?}. Expected {}. Actual {}", node2, 100f32, size.height);
    assert_eq!(location.x, 61f32, "x of node {:?}. Expected {}. Actual {}", node2, 61f32, location.x);
    assert_eq!(location.y, 1f32, "y of node {:?}. Expected {}. Actual {}", node2, 1f32, location.y);
    #[cfg(feature = "content_size")]
    assert_eq!(
        layout.scroll_width(),
        0f32,
        "scroll_width of node {:?}. Expected {}. Actual {}",
        node2,
        0f32,
        layout.scroll_width()
    );
    #[cfg(feature = "content_size")]
    assert_eq!(
        layout.scroll_height(),
        0f32,
        "scroll_height of node {:?}. Expected {}. Actual {}",
        node2,
        0f32,
        layout.scroll_height()
    );
    #[cfg_attr(not(feature = "content_size"), allow(unused_variables))]
    let layout @ Layout { size, location, .. } = taffy.layout(node3).unwrap();
    assert_eq!(size.width, 40f32, "width of node {:?}. Expected {}. Actual {}", node3, 40f32, size.width);
    assert_eq!(size.height, 100f32, "height of node {:?}. Expected {}. Actual {}", node3, 100f32, size.height);
    assert_eq!(location.x, 111f32, "x of node {:?}. Expected {}. Actual {}", node3, 111f32, location.x);
    assert_eq!(location.y, 1f32, "y of node {:?}. Expected {}. Actual {}", node3, 1f32, location.y);
    #[cfg(feature = "content_size")]
    assert_eq!(
        layout.scroll_width(),
        0f32,
        "scroll_width of node {:?}. Expected {}. Actual {}",
        node3,
        0f32,
        layout.scroll_width()
    );
    #[cfg(feature = "content_size")]
    assert_eq!(
        layout.scroll_height(),
        0f32,
        "scroll_height of node {:?}. Expected {}. Actual {}",
        node3,
        0f32,
        layout.scroll_height()
    );
}
