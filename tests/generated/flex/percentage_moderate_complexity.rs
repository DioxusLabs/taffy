#[test]
fn percentage_moderate_complexity() {
    #[allow(unused_imports)]
    use taffy::{prelude::*, tree::Layout, Taffy};
    let mut taffy: Taffy<crate::TextMeasure> = Taffy::new();
    taffy.disable_rounding();
    let node00 = taffy
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
    let node0 = taffy
        .new_with_children(
            taffy::style::Style {
                display: taffy::style::Display::Flex,
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
            &[node00],
        )
        .unwrap();
    let node = taffy
        .new_with_children(
            taffy::style::Style {
                display: taffy::style::Display::Flex,
                flex_direction: taffy::style::FlexDirection::Column,
                size: taffy::geometry::Size { width: taffy::style::Dimension::Length(200f32), height: auto() },
                padding: taffy::geometry::Rect {
                    left: taffy::style::LengthPercentage::Length(3f32),
                    right: taffy::style::LengthPercentage::Length(3f32),
                    top: taffy::style::LengthPercentage::Length(3f32),
                    bottom: taffy::style::LengthPercentage::Length(3f32),
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
    assert!(size.width - 200f32 < 0.1, "width of node {:?}. Expected {}. Actual {}", node, 200f32, size.width);
    assert!(
        size.height - 42.15625f32 < 0.1,
        "height of node {:?}. Expected {}. Actual {}",
        node,
        42.15625f32,
        size.height
    );
    assert!(location.x - 0f32 < 0.1, "x of node {:?}. Expected {}. Actual {}", node, 0f32, location.x);
    assert!(location.y - 0f32 < 0.1, "y of node {:?}. Expected {}. Actual {}", node, 0f32, location.y);
    #[cfg(feature = "content_size")]
    assert!(
        layout.scroll_width() - 0f32 < 0.1,
        "scroll_width of node {:?}. Expected {}. Actual {}",
        node,
        0f32,
        layout.scroll_width()
    );
    #[cfg(feature = "content_size")]
    assert!(
        layout.scroll_height() - 0f32 < 0.1,
        "scroll_height of node {:?}. Expected {}. Actual {}",
        node,
        0f32,
        layout.scroll_height()
    );
    #[cfg_attr(not(feature = "content_size"), allow(unused_variables))]
    let layout @ Layout { size, location, .. } = taffy.layout(node0).unwrap();
    assert!(size.width - 97f32 < 0.1, "width of node {:?}. Expected {}. Actual {}", node0, 97f32, size.width);
    assert!(
        size.height - 26.15625f32 < 0.1,
        "height of node {:?}. Expected {}. Actual {}",
        node0,
        26.15625f32,
        size.height
    );
    assert!(location.x - 8f32 < 0.1, "x of node {:?}. Expected {}. Actual {}", node0, 8f32, location.x);
    assert!(location.y - 8f32 < 0.1, "y of node {:?}. Expected {}. Actual {}", node0, 8f32, location.y);
    #[cfg(feature = "content_size")]
    assert!(
        layout.scroll_width() - 0f32 < 0.1,
        "scroll_width of node {:?}. Expected {}. Actual {}",
        node0,
        0f32,
        layout.scroll_width()
    );
    #[cfg(feature = "content_size")]
    assert!(
        layout.scroll_height() - 0f32 < 0.1,
        "scroll_height of node {:?}. Expected {}. Actual {}",
        node0,
        0f32,
        layout.scroll_height()
    );
    #[cfg_attr(not(feature = "content_size"), allow(unused_variables))]
    let layout @ Layout { size, location, .. } = taffy.layout(node00).unwrap();
    assert!(
        size.width - 38.414063f32 < 0.1,
        "width of node {:?}. Expected {}. Actual {}",
        node00,
        38.414063f32,
        size.width
    );
    assert!(size.height - 6f32 < 0.1, "height of node {:?}. Expected {}. Actual {}", node00, 6f32, size.height);
    assert!(
        location.x - 10.078125f32 < 0.1,
        "x of node {:?}. Expected {}. Actual {}",
        node00,
        10.078125f32,
        location.x
    );
    assert!(
        location.y - 10.078125f32 < 0.1,
        "y of node {:?}. Expected {}. Actual {}",
        node00,
        10.078125f32,
        location.y
    );
    #[cfg(feature = "content_size")]
    assert!(
        layout.scroll_width() - 0f32 < 0.1,
        "scroll_width of node {:?}. Expected {}. Actual {}",
        node00,
        0f32,
        layout.scroll_width()
    );
    #[cfg(feature = "content_size")]
    assert!(
        layout.scroll_height() - 0f32 < 0.1,
        "scroll_height of node {:?}. Expected {}. Actual {}",
        node00,
        0f32,
        layout.scroll_height()
    );
}
