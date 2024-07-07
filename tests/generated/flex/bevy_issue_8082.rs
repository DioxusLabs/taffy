#[test]
#[allow(non_snake_case)]
fn bevy_issue_8082__border_box() {
    #[allow(unused_imports)]
    use taffy::{prelude::*, tree::Layout, TaffyTree};
    let mut taffy: TaffyTree<crate::TextMeasure> = TaffyTree::new();
    let node00 = taffy
        .new_leaf(taffy::style::Style {
            size: taffy::geometry::Size {
                width: taffy::style::Dimension::Length(50f32),
                height: taffy::style::Dimension::Length(50f32),
            },
            margin: taffy::geometry::Rect {
                left: taffy::style::LengthPercentageAuto::Length(10f32),
                right: taffy::style::LengthPercentageAuto::Length(10f32),
                top: taffy::style::LengthPercentageAuto::Length(10f32),
                bottom: taffy::style::LengthPercentageAuto::Length(10f32),
            },
            ..Default::default()
        })
        .unwrap();
    let node01 = taffy
        .new_leaf(taffy::style::Style {
            size: taffy::geometry::Size {
                width: taffy::style::Dimension::Length(50f32),
                height: taffy::style::Dimension::Length(50f32),
            },
            margin: taffy::geometry::Rect {
                left: taffy::style::LengthPercentageAuto::Length(10f32),
                right: taffy::style::LengthPercentageAuto::Length(10f32),
                top: taffy::style::LengthPercentageAuto::Length(10f32),
                bottom: taffy::style::LengthPercentageAuto::Length(10f32),
            },
            ..Default::default()
        })
        .unwrap();
    let node02 = taffy
        .new_leaf(taffy::style::Style {
            size: taffy::geometry::Size {
                width: taffy::style::Dimension::Length(50f32),
                height: taffy::style::Dimension::Length(50f32),
            },
            margin: taffy::geometry::Rect {
                left: taffy::style::LengthPercentageAuto::Length(10f32),
                right: taffy::style::LengthPercentageAuto::Length(10f32),
                top: taffy::style::LengthPercentageAuto::Length(10f32),
                bottom: taffy::style::LengthPercentageAuto::Length(10f32),
            },
            ..Default::default()
        })
        .unwrap();
    let node03 = taffy
        .new_leaf(taffy::style::Style {
            size: taffy::geometry::Size {
                width: taffy::style::Dimension::Length(50f32),
                height: taffy::style::Dimension::Length(50f32),
            },
            margin: taffy::geometry::Rect {
                left: taffy::style::LengthPercentageAuto::Length(10f32),
                right: taffy::style::LengthPercentageAuto::Length(10f32),
                top: taffy::style::LengthPercentageAuto::Length(10f32),
                bottom: taffy::style::LengthPercentageAuto::Length(10f32),
            },
            ..Default::default()
        })
        .unwrap();
    let node0 = taffy
        .new_with_children(
            taffy::style::Style {
                flex_wrap: taffy::style::FlexWrap::Wrap,
                align_items: Some(taffy::style::AlignItems::FlexStart),
                align_content: Some(taffy::style::AlignContent::Center),
                justify_content: Some(taffy::style::JustifyContent::Center),
                ..Default::default()
            },
            &[node00, node01, node02, node03],
        )
        .unwrap();
    let node = taffy
        .new_with_children(
            taffy::style::Style {
                flex_direction: taffy::style::FlexDirection::Column,
                align_items: Some(taffy::style::AlignItems::Stretch),
                align_content: Some(taffy::style::AlignContent::Center),
                justify_content: Some(taffy::style::JustifyContent::FlexStart),
                size: taffy::geometry::Size {
                    width: taffy::style::Dimension::Length(200f32),
                    height: taffy::style::Dimension::Length(400f32),
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
    assert_eq!(size.height, 400f32, "height of node {:?}. Expected {}. Actual {}", node, 400f32, size.height);
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
    assert_eq!(size.width, 200f32, "width of node {:?}. Expected {}. Actual {}", node0, 200f32, size.width);
    assert_eq!(size.height, 140f32, "height of node {:?}. Expected {}. Actual {}", node0, 140f32, size.height);
    assert_eq!(location.x, 0f32, "x of node {:?}. Expected {}. Actual {}", node0, 0f32, location.x);
    assert_eq!(location.y, 0f32, "y of node {:?}. Expected {}. Actual {}", node0, 0f32, location.y);
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
    assert_eq!(size.width, 50f32, "width of node {:?}. Expected {}. Actual {}", node00, 50f32, size.width);
    assert_eq!(size.height, 50f32, "height of node {:?}. Expected {}. Actual {}", node00, 50f32, size.height);
    assert_eq!(location.x, 40f32, "x of node {:?}. Expected {}. Actual {}", node00, 40f32, location.x);
    assert_eq!(location.y, 10f32, "y of node {:?}. Expected {}. Actual {}", node00, 10f32, location.y);
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
    let layout @ Layout { size, location, .. } = taffy.layout(node01).unwrap();
    assert_eq!(size.width, 50f32, "width of node {:?}. Expected {}. Actual {}", node01, 50f32, size.width);
    assert_eq!(size.height, 50f32, "height of node {:?}. Expected {}. Actual {}", node01, 50f32, size.height);
    assert_eq!(location.x, 110f32, "x of node {:?}. Expected {}. Actual {}", node01, 110f32, location.x);
    assert_eq!(location.y, 10f32, "y of node {:?}. Expected {}. Actual {}", node01, 10f32, location.y);
    #[cfg(feature = "content_size")]
    assert_eq!(
        layout.scroll_width(),
        0f32,
        "scroll_width of node {:?}. Expected {}. Actual {}",
        node01,
        0f32,
        layout.scroll_width()
    );
    #[cfg(feature = "content_size")]
    assert_eq!(
        layout.scroll_height(),
        0f32,
        "scroll_height of node {:?}. Expected {}. Actual {}",
        node01,
        0f32,
        layout.scroll_height()
    );
    #[cfg_attr(not(feature = "content_size"), allow(unused_variables))]
    let layout @ Layout { size, location, .. } = taffy.layout(node02).unwrap();
    assert_eq!(size.width, 50f32, "width of node {:?}. Expected {}. Actual {}", node02, 50f32, size.width);
    assert_eq!(size.height, 50f32, "height of node {:?}. Expected {}. Actual {}", node02, 50f32, size.height);
    assert_eq!(location.x, 40f32, "x of node {:?}. Expected {}. Actual {}", node02, 40f32, location.x);
    assert_eq!(location.y, 80f32, "y of node {:?}. Expected {}. Actual {}", node02, 80f32, location.y);
    #[cfg(feature = "content_size")]
    assert_eq!(
        layout.scroll_width(),
        0f32,
        "scroll_width of node {:?}. Expected {}. Actual {}",
        node02,
        0f32,
        layout.scroll_width()
    );
    #[cfg(feature = "content_size")]
    assert_eq!(
        layout.scroll_height(),
        0f32,
        "scroll_height of node {:?}. Expected {}. Actual {}",
        node02,
        0f32,
        layout.scroll_height()
    );
    #[cfg_attr(not(feature = "content_size"), allow(unused_variables))]
    let layout @ Layout { size, location, .. } = taffy.layout(node03).unwrap();
    assert_eq!(size.width, 50f32, "width of node {:?}. Expected {}. Actual {}", node03, 50f32, size.width);
    assert_eq!(size.height, 50f32, "height of node {:?}. Expected {}. Actual {}", node03, 50f32, size.height);
    assert_eq!(location.x, 110f32, "x of node {:?}. Expected {}. Actual {}", node03, 110f32, location.x);
    assert_eq!(location.y, 80f32, "y of node {:?}. Expected {}. Actual {}", node03, 80f32, location.y);
    #[cfg(feature = "content_size")]
    assert_eq!(
        layout.scroll_width(),
        0f32,
        "scroll_width of node {:?}. Expected {}. Actual {}",
        node03,
        0f32,
        layout.scroll_width()
    );
    #[cfg(feature = "content_size")]
    assert_eq!(
        layout.scroll_height(),
        0f32,
        "scroll_height of node {:?}. Expected {}. Actual {}",
        node03,
        0f32,
        layout.scroll_height()
    );
}

#[test]
#[allow(non_snake_case)]
fn bevy_issue_8082__content_box() {
    #[allow(unused_imports)]
    use taffy::{prelude::*, tree::Layout, TaffyTree};
    let mut taffy: TaffyTree<crate::TextMeasure> = TaffyTree::new();
    let node00 = taffy
        .new_leaf(taffy::style::Style {
            box_sizing: taffy::style::BoxSizing::ContentBox,
            size: taffy::geometry::Size {
                width: taffy::style::Dimension::Length(50f32),
                height: taffy::style::Dimension::Length(50f32),
            },
            margin: taffy::geometry::Rect {
                left: taffy::style::LengthPercentageAuto::Length(10f32),
                right: taffy::style::LengthPercentageAuto::Length(10f32),
                top: taffy::style::LengthPercentageAuto::Length(10f32),
                bottom: taffy::style::LengthPercentageAuto::Length(10f32),
            },
            ..Default::default()
        })
        .unwrap();
    let node01 = taffy
        .new_leaf(taffy::style::Style {
            box_sizing: taffy::style::BoxSizing::ContentBox,
            size: taffy::geometry::Size {
                width: taffy::style::Dimension::Length(50f32),
                height: taffy::style::Dimension::Length(50f32),
            },
            margin: taffy::geometry::Rect {
                left: taffy::style::LengthPercentageAuto::Length(10f32),
                right: taffy::style::LengthPercentageAuto::Length(10f32),
                top: taffy::style::LengthPercentageAuto::Length(10f32),
                bottom: taffy::style::LengthPercentageAuto::Length(10f32),
            },
            ..Default::default()
        })
        .unwrap();
    let node02 = taffy
        .new_leaf(taffy::style::Style {
            box_sizing: taffy::style::BoxSizing::ContentBox,
            size: taffy::geometry::Size {
                width: taffy::style::Dimension::Length(50f32),
                height: taffy::style::Dimension::Length(50f32),
            },
            margin: taffy::geometry::Rect {
                left: taffy::style::LengthPercentageAuto::Length(10f32),
                right: taffy::style::LengthPercentageAuto::Length(10f32),
                top: taffy::style::LengthPercentageAuto::Length(10f32),
                bottom: taffy::style::LengthPercentageAuto::Length(10f32),
            },
            ..Default::default()
        })
        .unwrap();
    let node03 = taffy
        .new_leaf(taffy::style::Style {
            box_sizing: taffy::style::BoxSizing::ContentBox,
            size: taffy::geometry::Size {
                width: taffy::style::Dimension::Length(50f32),
                height: taffy::style::Dimension::Length(50f32),
            },
            margin: taffy::geometry::Rect {
                left: taffy::style::LengthPercentageAuto::Length(10f32),
                right: taffy::style::LengthPercentageAuto::Length(10f32),
                top: taffy::style::LengthPercentageAuto::Length(10f32),
                bottom: taffy::style::LengthPercentageAuto::Length(10f32),
            },
            ..Default::default()
        })
        .unwrap();
    let node0 = taffy
        .new_with_children(
            taffy::style::Style {
                box_sizing: taffy::style::BoxSizing::ContentBox,
                flex_wrap: taffy::style::FlexWrap::Wrap,
                align_items: Some(taffy::style::AlignItems::FlexStart),
                align_content: Some(taffy::style::AlignContent::Center),
                justify_content: Some(taffy::style::JustifyContent::Center),
                ..Default::default()
            },
            &[node00, node01, node02, node03],
        )
        .unwrap();
    let node = taffy
        .new_with_children(
            taffy::style::Style {
                box_sizing: taffy::style::BoxSizing::ContentBox,
                flex_direction: taffy::style::FlexDirection::Column,
                align_items: Some(taffy::style::AlignItems::Stretch),
                align_content: Some(taffy::style::AlignContent::Center),
                justify_content: Some(taffy::style::JustifyContent::FlexStart),
                size: taffy::geometry::Size {
                    width: taffy::style::Dimension::Length(200f32),
                    height: taffy::style::Dimension::Length(400f32),
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
    assert_eq!(size.height, 400f32, "height of node {:?}. Expected {}. Actual {}", node, 400f32, size.height);
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
    assert_eq!(size.width, 200f32, "width of node {:?}. Expected {}. Actual {}", node0, 200f32, size.width);
    assert_eq!(size.height, 140f32, "height of node {:?}. Expected {}. Actual {}", node0, 140f32, size.height);
    assert_eq!(location.x, 0f32, "x of node {:?}. Expected {}. Actual {}", node0, 0f32, location.x);
    assert_eq!(location.y, 0f32, "y of node {:?}. Expected {}. Actual {}", node0, 0f32, location.y);
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
    assert_eq!(size.width, 50f32, "width of node {:?}. Expected {}. Actual {}", node00, 50f32, size.width);
    assert_eq!(size.height, 50f32, "height of node {:?}. Expected {}. Actual {}", node00, 50f32, size.height);
    assert_eq!(location.x, 40f32, "x of node {:?}. Expected {}. Actual {}", node00, 40f32, location.x);
    assert_eq!(location.y, 10f32, "y of node {:?}. Expected {}. Actual {}", node00, 10f32, location.y);
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
    let layout @ Layout { size, location, .. } = taffy.layout(node01).unwrap();
    assert_eq!(size.width, 50f32, "width of node {:?}. Expected {}. Actual {}", node01, 50f32, size.width);
    assert_eq!(size.height, 50f32, "height of node {:?}. Expected {}. Actual {}", node01, 50f32, size.height);
    assert_eq!(location.x, 110f32, "x of node {:?}. Expected {}. Actual {}", node01, 110f32, location.x);
    assert_eq!(location.y, 10f32, "y of node {:?}. Expected {}. Actual {}", node01, 10f32, location.y);
    #[cfg(feature = "content_size")]
    assert_eq!(
        layout.scroll_width(),
        0f32,
        "scroll_width of node {:?}. Expected {}. Actual {}",
        node01,
        0f32,
        layout.scroll_width()
    );
    #[cfg(feature = "content_size")]
    assert_eq!(
        layout.scroll_height(),
        0f32,
        "scroll_height of node {:?}. Expected {}. Actual {}",
        node01,
        0f32,
        layout.scroll_height()
    );
    #[cfg_attr(not(feature = "content_size"), allow(unused_variables))]
    let layout @ Layout { size, location, .. } = taffy.layout(node02).unwrap();
    assert_eq!(size.width, 50f32, "width of node {:?}. Expected {}. Actual {}", node02, 50f32, size.width);
    assert_eq!(size.height, 50f32, "height of node {:?}. Expected {}. Actual {}", node02, 50f32, size.height);
    assert_eq!(location.x, 40f32, "x of node {:?}. Expected {}. Actual {}", node02, 40f32, location.x);
    assert_eq!(location.y, 80f32, "y of node {:?}. Expected {}. Actual {}", node02, 80f32, location.y);
    #[cfg(feature = "content_size")]
    assert_eq!(
        layout.scroll_width(),
        0f32,
        "scroll_width of node {:?}. Expected {}. Actual {}",
        node02,
        0f32,
        layout.scroll_width()
    );
    #[cfg(feature = "content_size")]
    assert_eq!(
        layout.scroll_height(),
        0f32,
        "scroll_height of node {:?}. Expected {}. Actual {}",
        node02,
        0f32,
        layout.scroll_height()
    );
    #[cfg_attr(not(feature = "content_size"), allow(unused_variables))]
    let layout @ Layout { size, location, .. } = taffy.layout(node03).unwrap();
    assert_eq!(size.width, 50f32, "width of node {:?}. Expected {}. Actual {}", node03, 50f32, size.width);
    assert_eq!(size.height, 50f32, "height of node {:?}. Expected {}. Actual {}", node03, 50f32, size.height);
    assert_eq!(location.x, 110f32, "x of node {:?}. Expected {}. Actual {}", node03, 110f32, location.x);
    assert_eq!(location.y, 80f32, "y of node {:?}. Expected {}. Actual {}", node03, 80f32, location.y);
    #[cfg(feature = "content_size")]
    assert_eq!(
        layout.scroll_width(),
        0f32,
        "scroll_width of node {:?}. Expected {}. Actual {}",
        node03,
        0f32,
        layout.scroll_width()
    );
    #[cfg(feature = "content_size")]
    assert_eq!(
        layout.scroll_height(),
        0f32,
        "scroll_height of node {:?}. Expected {}. Actual {}",
        node03,
        0f32,
        layout.scroll_height()
    );
}
