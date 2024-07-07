#[test]
#[allow(non_snake_case)]
fn bevy_issue_7976_4_level__border_box() {
    #[allow(unused_imports)]
    use taffy::{prelude::*, tree::Layout, TaffyTree};
    let mut taffy: TaffyTree<crate::TextMeasure> = TaffyTree::new();
    let node000 = taffy.new_leaf(taffy::style::Style { ..Default::default() }).unwrap();
    let node00 = taffy
        .new_with_children(
            taffy::style::Style {
                size: taffy::geometry::Size {
                    width: taffy::style::Dimension::Percent(1f32),
                    height: taffy::style::Dimension::Percent(1f32),
                },
                padding: taffy::geometry::Rect {
                    left: taffy::style::LengthPercentage::Length(5f32),
                    right: taffy::style::LengthPercentage::Length(5f32),
                    top: taffy::style::LengthPercentage::Length(5f32),
                    bottom: taffy::style::LengthPercentage::Length(5f32),
                },
                ..Default::default()
            },
            &[node000],
        )
        .unwrap();
    let node0 = taffy
        .new_with_children(
            taffy::style::Style {
                min_size: taffy::geometry::Size {
                    width: taffy::style::Dimension::Length(40f32),
                    height: taffy::style::Dimension::Length(40f32),
                },
                margin: taffy::geometry::Rect {
                    left: taffy::style::LengthPercentageAuto::Length(5f32),
                    right: taffy::style::LengthPercentageAuto::Length(5f32),
                    top: taffy::style::LengthPercentageAuto::Length(5f32),
                    bottom: taffy::style::LengthPercentageAuto::Length(5f32),
                },
                padding: taffy::geometry::Rect {
                    left: taffy::style::LengthPercentage::Length(5f32),
                    right: taffy::style::LengthPercentage::Length(5f32),
                    top: taffy::style::LengthPercentage::Length(5f32),
                    bottom: taffy::style::LengthPercentage::Length(5f32),
                },
                ..Default::default()
            },
            &[node00],
        )
        .unwrap();
    let node = taffy
        .new_with_children(
            taffy::style::Style {
                align_content: Some(taffy::style::AlignContent::Start),
                size: taffy::geometry::Size {
                    width: taffy::style::Dimension::Length(200f32),
                    height: taffy::style::Dimension::Length(200f32),
                },
                ..Default::default()
            },
            &[node0],
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
    assert_eq!(size.width, 40f32, "width of node {:?}. Expected {}. Actual {}", node0, 40f32, size.width);
    assert_eq!(size.height, 190f32, "height of node {:?}. Expected {}. Actual {}", node0, 190f32, size.height);
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
    assert_eq!(size.width, 30f32, "width of node {:?}. Expected {}. Actual {}", node00, 30f32, size.width);
    assert_eq!(size.height, 180f32, "height of node {:?}. Expected {}. Actual {}", node00, 180f32, size.height);
    assert_eq!(location.x, 5f32, "x of node {:?}. Expected {}. Actual {}", node00, 5f32, location.x);
    assert_eq!(location.y, 5f32, "y of node {:?}. Expected {}. Actual {}", node00, 5f32, location.y);
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
    assert_eq!(size.width, 0f32, "width of node {:?}. Expected {}. Actual {}", node000, 0f32, size.width);
    assert_eq!(size.height, 170f32, "height of node {:?}. Expected {}. Actual {}", node000, 170f32, size.height);
    assert_eq!(location.x, 5f32, "x of node {:?}. Expected {}. Actual {}", node000, 5f32, location.x);
    assert_eq!(location.y, 5f32, "y of node {:?}. Expected {}. Actual {}", node000, 5f32, location.y);
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
}

#[test]
#[allow(non_snake_case)]
fn bevy_issue_7976_4_level__content_box() {
    #[allow(unused_imports)]
    use taffy::{prelude::*, tree::Layout, TaffyTree};
    let mut taffy: TaffyTree<crate::TextMeasure> = TaffyTree::new();
    let node000 = taffy
        .new_leaf(taffy::style::Style { box_sizing: taffy::style::BoxSizing::ContentBox, ..Default::default() })
        .unwrap();
    let node00 = taffy
        .new_with_children(
            taffy::style::Style {
                box_sizing: taffy::style::BoxSizing::ContentBox,
                size: taffy::geometry::Size {
                    width: taffy::style::Dimension::Percent(1f32),
                    height: taffy::style::Dimension::Percent(1f32),
                },
                padding: taffy::geometry::Rect {
                    left: taffy::style::LengthPercentage::Length(5f32),
                    right: taffy::style::LengthPercentage::Length(5f32),
                    top: taffy::style::LengthPercentage::Length(5f32),
                    bottom: taffy::style::LengthPercentage::Length(5f32),
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
                min_size: taffy::geometry::Size {
                    width: taffy::style::Dimension::Length(40f32),
                    height: taffy::style::Dimension::Length(40f32),
                },
                margin: taffy::geometry::Rect {
                    left: taffy::style::LengthPercentageAuto::Length(5f32),
                    right: taffy::style::LengthPercentageAuto::Length(5f32),
                    top: taffy::style::LengthPercentageAuto::Length(5f32),
                    bottom: taffy::style::LengthPercentageAuto::Length(5f32),
                },
                padding: taffy::geometry::Rect {
                    left: taffy::style::LengthPercentage::Length(5f32),
                    right: taffy::style::LengthPercentage::Length(5f32),
                    top: taffy::style::LengthPercentage::Length(5f32),
                    bottom: taffy::style::LengthPercentage::Length(5f32),
                },
                ..Default::default()
            },
            &[node00],
        )
        .unwrap();
    let node = taffy
        .new_with_children(
            taffy::style::Style {
                box_sizing: taffy::style::BoxSizing::ContentBox,
                align_content: Some(taffy::style::AlignContent::Start),
                size: taffy::geometry::Size {
                    width: taffy::style::Dimension::Length(200f32),
                    height: taffy::style::Dimension::Length(200f32),
                },
                ..Default::default()
            },
            &[node0],
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
    assert_eq!(size.width, 50f32, "width of node {:?}. Expected {}. Actual {}", node0, 50f32, size.width);
    assert_eq!(size.height, 190f32, "height of node {:?}. Expected {}. Actual {}", node0, 190f32, size.height);
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
        5f32,
        "scroll_height of node {:?}. Expected {}. Actual {}",
        node0,
        5f32,
        layout.scroll_height()
    );
    #[cfg_attr(not(feature = "content_size"), allow(unused_variables))]
    let layout @ Layout { size, location, .. } = taffy.layout(node00).unwrap();
    assert_eq!(size.width, 40f32, "width of node {:?}. Expected {}. Actual {}", node00, 40f32, size.width);
    assert_eq!(size.height, 190f32, "height of node {:?}. Expected {}. Actual {}", node00, 190f32, size.height);
    assert_eq!(location.x, 5f32, "x of node {:?}. Expected {}. Actual {}", node00, 5f32, location.x);
    assert_eq!(location.y, 5f32, "y of node {:?}. Expected {}. Actual {}", node00, 5f32, location.y);
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
    assert_eq!(size.width, 0f32, "width of node {:?}. Expected {}. Actual {}", node000, 0f32, size.width);
    assert_eq!(size.height, 180f32, "height of node {:?}. Expected {}. Actual {}", node000, 180f32, size.height);
    assert_eq!(location.x, 5f32, "x of node {:?}. Expected {}. Actual {}", node000, 5f32, location.x);
    assert_eq!(location.y, 5f32, "y of node {:?}. Expected {}. Actual {}", node000, 5f32, location.y);
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
}
