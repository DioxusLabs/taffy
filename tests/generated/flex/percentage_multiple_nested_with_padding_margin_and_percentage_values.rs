#[test]
#[allow(non_snake_case)]
fn percentage_multiple_nested_with_padding_margin_and_percentage_values__border_box() {
    #[allow(unused_imports)]
    use taffy::{prelude::*, tree::Layout, TaffyTree};
    let mut taffy: TaffyTree<crate::TextMeasure> = TaffyTree::new();
    let node000 = taffy
        .new_leaf(taffy::style::Style {
            size: taffy::geometry::Size { width: taffy::style::Dimension::Percent(0.45f32), height: auto() },
            margin: taffy::geometry::Rect {
                left: taffy::style::LengthPercentageAuto::Percent(0.05f32),
                right: taffy::style::LengthPercentageAuto::Percent(0.05f32),
                top: taffy::style::LengthPercentageAuto::Percent(0.05f32),
                bottom: taffy::style::LengthPercentageAuto::Percent(0.05f32),
            },
            padding: taffy::geometry::Rect {
                left: taffy::style::LengthPercentage::Length(3f32),
                right: taffy::style::LengthPercentage::Length(3f32),
                top: taffy::style::LengthPercentage::Length(3f32),
                bottom: taffy::style::LengthPercentage::Length(3f32),
            },
            ..Default::default()
        })
        .unwrap();
    let node00 = taffy
        .new_with_children(
            taffy::style::Style {
                flex_direction: taffy::style::FlexDirection::Column,
                size: taffy::geometry::Size { width: taffy::style::Dimension::Percent(0.5f32), height: auto() },
                margin: taffy::geometry::Rect {
                    left: taffy::style::LengthPercentageAuto::Length(5f32),
                    right: taffy::style::LengthPercentageAuto::Length(5f32),
                    top: taffy::style::LengthPercentageAuto::Length(5f32),
                    bottom: taffy::style::LengthPercentageAuto::Length(5f32),
                },
                padding: taffy::geometry::Rect {
                    left: taffy::style::LengthPercentage::Percent(0.03f32),
                    right: taffy::style::LengthPercentage::Percent(0.03f32),
                    top: taffy::style::LengthPercentage::Percent(0.03f32),
                    bottom: taffy::style::LengthPercentage::Percent(0.03f32),
                },
                ..Default::default()
            },
            &[node000],
        )
        .unwrap();
    let node0 = taffy
        .new_with_children(
            taffy::style::Style {
                flex_direction: taffy::style::FlexDirection::Column,
                flex_grow: 1f32,
                flex_basis: taffy::style::Dimension::Percent(0.1f32),
                min_size: taffy::geometry::Size { width: taffy::style::Dimension::Percent(0.6f32), height: auto() },
                margin: taffy::geometry::Rect {
                    left: taffy::style::LengthPercentageAuto::Length(5f32),
                    right: taffy::style::LengthPercentageAuto::Length(5f32),
                    top: taffy::style::LengthPercentageAuto::Length(5f32),
                    bottom: taffy::style::LengthPercentageAuto::Length(5f32),
                },
                padding: taffy::geometry::Rect {
                    left: taffy::style::LengthPercentage::Length(3f32),
                    right: taffy::style::LengthPercentage::Length(3f32),
                    top: taffy::style::LengthPercentage::Length(3f32),
                    bottom: taffy::style::LengthPercentage::Length(3f32),
                },
                ..Default::default()
            },
            &[node00],
        )
        .unwrap();
    let node1 = taffy
        .new_leaf(taffy::style::Style {
            flex_grow: 4f32,
            flex_basis: taffy::style::Dimension::Percent(0.15f32),
            min_size: taffy::geometry::Size { width: taffy::style::Dimension::Percent(0.2f32), height: auto() },
            ..Default::default()
        })
        .unwrap();
    let node = taffy
        .new_with_children(
            taffy::style::Style {
                flex_direction: taffy::style::FlexDirection::Column,
                size: taffy::geometry::Size {
                    width: taffy::style::Dimension::Length(200f32),
                    height: taffy::style::Dimension::Length(200f32),
                },
                ..Default::default()
            },
            &[node0, node1],
        )
        .unwrap();
    taffy.compute_layout_with_measure(node, taffy::geometry::Size::MAX_CONTENT, crate::test_measure_function).unwrap();
    println!("\nComputed tree:");
    taffy.print_tree(node);
    println!();
    #[cfg_attr(not(feature = "content_size"), allow(unused_variables))]
    let layout @ Layout { size, location, .. } = taffy.layout(node).unwrap();
    assert_eq!(size.width, 200f32, "width of node {:?}. Expected {}. Actual {}", node, 200f32, size.width);
    assert_eq!(size.height, 200f32, "height of node {:?}. Expected {}. Actual {}", node, 200f32, size.height);
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
    assert_eq!(size.width, 190f32, "width of node {:?}. Expected {}. Actual {}", node0, 190f32, size.width);
    assert_eq!(size.height, 48f32, "height of node {:?}. Expected {}. Actual {}", node0, 48f32, size.height);
    assert_eq!(location.x, 5f32, "x of node {:?}. Expected {}. Actual {}", node0, 5f32, location.x);
    assert_eq!(location.y, 5f32, "y of node {:?}. Expected {}. Actual {}", node0, 5f32, location.y);
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
    let layout @ Layout { size, location, .. } = taffy.layout(node00).unwrap();
    assert_eq!(size.width, 92f32, "width of node {:?}. Expected {}. Actual {}", node00, 92f32, size.width);
    assert_eq!(size.height, 25f32, "height of node {:?}. Expected {}. Actual {}", node00, 25f32, size.height);
    assert_eq!(location.x, 8f32, "x of node {:?}. Expected {}. Actual {}", node00, 8f32, location.x);
    assert_eq!(location.y, 8f32, "y of node {:?}. Expected {}. Actual {}", node00, 8f32, location.y);
    #[cfg(feature = "content_size")]
    assert_eq!(
        layout.scroll_width(),
        0f32,
        "scroll_width of node {:?}. Expected {}. Actual {}",
        node00,
        0f32,
        layout.scroll_width()
    );
    #[cfg(feature = "content_size")]
    assert_eq!(
        layout.scroll_height(),
        0f32,
        "scroll_height of node {:?}. Expected {}. Actual {}",
        node00,
        0f32,
        layout.scroll_height()
    );
    #[cfg_attr(not(feature = "content_size"), allow(unused_variables))]
    let layout @ Layout { size, location, .. } = taffy.layout(node000).unwrap();
    assert_eq!(size.width, 36f32, "width of node {:?}. Expected {}. Actual {}", node000, 36f32, size.width);
    assert_eq!(size.height, 6f32, "height of node {:?}. Expected {}. Actual {}", node000, 6f32, size.height);
    assert_eq!(location.x, 10f32, "x of node {:?}. Expected {}. Actual {}", node000, 10f32, location.x);
    assert_eq!(location.y, 10f32, "y of node {:?}. Expected {}. Actual {}", node000, 10f32, location.y);
    #[cfg(feature = "content_size")]
    assert_eq!(
        layout.scroll_width(),
        0f32,
        "scroll_width of node {:?}. Expected {}. Actual {}",
        node000,
        0f32,
        layout.scroll_width()
    );
    #[cfg(feature = "content_size")]
    assert_eq!(
        layout.scroll_height(),
        0f32,
        "scroll_height of node {:?}. Expected {}. Actual {}",
        node000,
        0f32,
        layout.scroll_height()
    );
    #[cfg_attr(not(feature = "content_size"), allow(unused_variables))]
    let layout @ Layout { size, location, .. } = taffy.layout(node1).unwrap();
    assert_eq!(size.width, 200f32, "width of node {:?}. Expected {}. Actual {}", node1, 200f32, size.width);
    assert_eq!(size.height, 142f32, "height of node {:?}. Expected {}. Actual {}", node1, 142f32, size.height);
    assert_eq!(location.x, 0f32, "x of node {:?}. Expected {}. Actual {}", node1, 0f32, location.x);
    assert_eq!(location.y, 58f32, "y of node {:?}. Expected {}. Actual {}", node1, 58f32, location.y);
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
}

#[test]
#[allow(non_snake_case)]
fn percentage_multiple_nested_with_padding_margin_and_percentage_values__content_box() {
    #[allow(unused_imports)]
    use taffy::{prelude::*, tree::Layout, TaffyTree};
    let mut taffy: TaffyTree<crate::TextMeasure> = TaffyTree::new();
    let node000 = taffy
        .new_leaf(taffy::style::Style {
            box_sizing: taffy::style::BoxSizing::ContentBox,
            size: taffy::geometry::Size { width: taffy::style::Dimension::Percent(0.45f32), height: auto() },
            margin: taffy::geometry::Rect {
                left: taffy::style::LengthPercentageAuto::Percent(0.05f32),
                right: taffy::style::LengthPercentageAuto::Percent(0.05f32),
                top: taffy::style::LengthPercentageAuto::Percent(0.05f32),
                bottom: taffy::style::LengthPercentageAuto::Percent(0.05f32),
            },
            padding: taffy::geometry::Rect {
                left: taffy::style::LengthPercentage::Length(3f32),
                right: taffy::style::LengthPercentage::Length(3f32),
                top: taffy::style::LengthPercentage::Length(3f32),
                bottom: taffy::style::LengthPercentage::Length(3f32),
            },
            ..Default::default()
        })
        .unwrap();
    let node00 = taffy
        .new_with_children(
            taffy::style::Style {
                box_sizing: taffy::style::BoxSizing::ContentBox,
                flex_direction: taffy::style::FlexDirection::Column,
                size: taffy::geometry::Size { width: taffy::style::Dimension::Percent(0.5f32), height: auto() },
                margin: taffy::geometry::Rect {
                    left: taffy::style::LengthPercentageAuto::Length(5f32),
                    right: taffy::style::LengthPercentageAuto::Length(5f32),
                    top: taffy::style::LengthPercentageAuto::Length(5f32),
                    bottom: taffy::style::LengthPercentageAuto::Length(5f32),
                },
                padding: taffy::geometry::Rect {
                    left: taffy::style::LengthPercentage::Percent(0.03f32),
                    right: taffy::style::LengthPercentage::Percent(0.03f32),
                    top: taffy::style::LengthPercentage::Percent(0.03f32),
                    bottom: taffy::style::LengthPercentage::Percent(0.03f32),
                },
                ..Default::default()
            },
            &[node000],
        )
        .unwrap();
    let node0 = taffy
        .new_with_children(
            taffy::style::Style {
                box_sizing: taffy::style::BoxSizing::ContentBox,
                flex_direction: taffy::style::FlexDirection::Column,
                flex_grow: 1f32,
                flex_basis: taffy::style::Dimension::Percent(0.1f32),
                min_size: taffy::geometry::Size { width: taffy::style::Dimension::Percent(0.6f32), height: auto() },
                margin: taffy::geometry::Rect {
                    left: taffy::style::LengthPercentageAuto::Length(5f32),
                    right: taffy::style::LengthPercentageAuto::Length(5f32),
                    top: taffy::style::LengthPercentageAuto::Length(5f32),
                    bottom: taffy::style::LengthPercentageAuto::Length(5f32),
                },
                padding: taffy::geometry::Rect {
                    left: taffy::style::LengthPercentage::Length(3f32),
                    right: taffy::style::LengthPercentage::Length(3f32),
                    top: taffy::style::LengthPercentage::Length(3f32),
                    bottom: taffy::style::LengthPercentage::Length(3f32),
                },
                ..Default::default()
            },
            &[node00],
        )
        .unwrap();
    let node1 = taffy
        .new_leaf(taffy::style::Style {
            box_sizing: taffy::style::BoxSizing::ContentBox,
            flex_grow: 4f32,
            flex_basis: taffy::style::Dimension::Percent(0.15f32),
            min_size: taffy::geometry::Size { width: taffy::style::Dimension::Percent(0.2f32), height: auto() },
            ..Default::default()
        })
        .unwrap();
    let node = taffy
        .new_with_children(
            taffy::style::Style {
                box_sizing: taffy::style::BoxSizing::ContentBox,
                flex_direction: taffy::style::FlexDirection::Column,
                size: taffy::geometry::Size {
                    width: taffy::style::Dimension::Length(200f32),
                    height: taffy::style::Dimension::Length(200f32),
                },
                ..Default::default()
            },
            &[node0, node1],
        )
        .unwrap();
    taffy.compute_layout_with_measure(node, taffy::geometry::Size::MAX_CONTENT, crate::test_measure_function).unwrap();
    println!("\nComputed tree:");
    taffy.print_tree(node);
    println!();
    #[cfg_attr(not(feature = "content_size"), allow(unused_variables))]
    let layout @ Layout { size, location, .. } = taffy.layout(node).unwrap();
    assert_eq!(size.width, 200f32, "width of node {:?}. Expected {}. Actual {}", node, 200f32, size.width);
    assert_eq!(size.height, 200f32, "height of node {:?}. Expected {}. Actual {}", node, 200f32, size.height);
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
    assert_eq!(size.width, 190f32, "width of node {:?}. Expected {}. Actual {}", node0, 190f32, size.width);
    assert_eq!(size.height, 53f32, "height of node {:?}. Expected {}. Actual {}", node0, 53f32, size.height);
    assert_eq!(location.x, 5f32, "x of node {:?}. Expected {}. Actual {}", node0, 5f32, location.x);
    assert_eq!(location.y, 5f32, "y of node {:?}. Expected {}. Actual {}", node0, 5f32, location.y);
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
    let layout @ Layout { size, location, .. } = taffy.layout(node00).unwrap();
    assert_eq!(size.width, 103f32, "width of node {:?}. Expected {}. Actual {}", node00, 103f32, size.width);
    assert_eq!(size.height, 26f32, "height of node {:?}. Expected {}. Actual {}", node00, 26f32, size.height);
    assert_eq!(location.x, 8f32, "x of node {:?}. Expected {}. Actual {}", node00, 8f32, location.x);
    assert_eq!(location.y, 8f32, "y of node {:?}. Expected {}. Actual {}", node00, 8f32, location.y);
    #[cfg(feature = "content_size")]
    assert_eq!(
        layout.scroll_width(),
        0f32,
        "scroll_width of node {:?}. Expected {}. Actual {}",
        node00,
        0f32,
        layout.scroll_width()
    );
    #[cfg(feature = "content_size")]
    assert_eq!(
        layout.scroll_height(),
        0f32,
        "scroll_height of node {:?}. Expected {}. Actual {}",
        node00,
        0f32,
        layout.scroll_height()
    );
    #[cfg_attr(not(feature = "content_size"), allow(unused_variables))]
    let layout @ Layout { size, location, .. } = taffy.layout(node000).unwrap();
    assert_eq!(size.width, 48f32, "width of node {:?}. Expected {}. Actual {}", node000, 48f32, size.width);
    assert_eq!(size.height, 6f32, "height of node {:?}. Expected {}. Actual {}", node000, 6f32, size.height);
    assert_eq!(location.x, 10f32, "x of node {:?}. Expected {}. Actual {}", node000, 10f32, location.x);
    assert_eq!(location.y, 10f32, "y of node {:?}. Expected {}. Actual {}", node000, 10f32, location.y);
    #[cfg(feature = "content_size")]
    assert_eq!(
        layout.scroll_width(),
        0f32,
        "scroll_width of node {:?}. Expected {}. Actual {}",
        node000,
        0f32,
        layout.scroll_width()
    );
    #[cfg(feature = "content_size")]
    assert_eq!(
        layout.scroll_height(),
        0f32,
        "scroll_height of node {:?}. Expected {}. Actual {}",
        node000,
        0f32,
        layout.scroll_height()
    );
    #[cfg_attr(not(feature = "content_size"), allow(unused_variables))]
    let layout @ Layout { size, location, .. } = taffy.layout(node1).unwrap();
    assert_eq!(size.width, 200f32, "width of node {:?}. Expected {}. Actual {}", node1, 200f32, size.width);
    assert_eq!(size.height, 137f32, "height of node {:?}. Expected {}. Actual {}", node1, 137f32, size.height);
    assert_eq!(location.x, 0f32, "x of node {:?}. Expected {}. Actual {}", node1, 0f32, location.x);
    assert_eq!(location.y, 63f32, "y of node {:?}. Expected {}. Actual {}", node1, 63f32, location.y);
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
}
