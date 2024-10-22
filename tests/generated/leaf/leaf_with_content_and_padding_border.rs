#[test]
#[allow(non_snake_case)]
fn leaf_with_content_and_padding_border__border_box() {
    #[allow(unused_imports)]
    use taffy::{prelude::*, tree::Layout, TaffyTree};
    let mut taffy: TaffyTree<crate::TextMeasure> = TaffyTree::new();
    let node = taffy
        .new_leaf_with_context(
            taffy::style::Style {
                padding: taffy::geometry::Rect {
                    left: taffy::style::LengthPercentage::Length(8f32),
                    right: taffy::style::LengthPercentage::Length(4f32),
                    top: taffy::style::LengthPercentage::Length(2f32),
                    bottom: taffy::style::LengthPercentage::Length(6f32),
                },
                border: taffy::geometry::Rect {
                    left: taffy::style::LengthPercentage::Length(7f32),
                    right: taffy::style::LengthPercentage::Length(3f32),
                    top: taffy::style::LengthPercentage::Length(1f32),
                    bottom: taffy::style::LengthPercentage::Length(5f32),
                },
                ..Default::default()
            },
            crate::TextMeasure {
                text_content: "HHHH",
                writing_mode: crate::WritingMode::Horizontal,
                _aspect_ratio: None,
            },
        )
        .unwrap();
    taffy.compute_layout_with_measure(node, taffy::geometry::Size::MAX_CONTENT, crate::test_measure_function).unwrap();
    println!("\nComputed tree:");
    taffy.print_tree(node);
    println!();
    let layout = taffy.layout(node).unwrap();
    let Layout { size, location, .. } = layout;
    assert_eq!(size.width, 62f32, "width of node {:?}. Expected {}. Actual {}", node, 62f32, size.width);
    assert_eq!(size.height, 24f32, "height of node {:?}. Expected {}. Actual {}", node, 24f32, size.height);
    assert_eq!(location.x, 0f32, "x of node {:?}. Expected {}. Actual {}", node, 0f32, location.x);
    assert_eq!(location.y, 0f32, "y of node {:?}. Expected {}. Actual {}", node, 0f32, location.y);
}

#[test]
#[allow(non_snake_case)]
fn leaf_with_content_and_padding_border__content_box() {
    #[allow(unused_imports)]
    use taffy::{prelude::*, tree::Layout, TaffyTree};
    let mut taffy: TaffyTree<crate::TextMeasure> = TaffyTree::new();
    let node = taffy
        .new_leaf_with_context(
            taffy::style::Style {
                box_sizing: taffy::style::BoxSizing::ContentBox,
                padding: taffy::geometry::Rect {
                    left: taffy::style::LengthPercentage::Length(8f32),
                    right: taffy::style::LengthPercentage::Length(4f32),
                    top: taffy::style::LengthPercentage::Length(2f32),
                    bottom: taffy::style::LengthPercentage::Length(6f32),
                },
                border: taffy::geometry::Rect {
                    left: taffy::style::LengthPercentage::Length(7f32),
                    right: taffy::style::LengthPercentage::Length(3f32),
                    top: taffy::style::LengthPercentage::Length(1f32),
                    bottom: taffy::style::LengthPercentage::Length(5f32),
                },
                ..Default::default()
            },
            crate::TextMeasure {
                text_content: "HHHH",
                writing_mode: crate::WritingMode::Horizontal,
                _aspect_ratio: None,
            },
        )
        .unwrap();
    taffy.compute_layout_with_measure(node, taffy::geometry::Size::MAX_CONTENT, crate::test_measure_function).unwrap();
    println!("\nComputed tree:");
    taffy.print_tree(node);
    println!();
    let layout = taffy.layout(node).unwrap();
    let Layout { size, location, .. } = layout;
    assert_eq!(size.width, 62f32, "width of node {:?}. Expected {}. Actual {}", node, 62f32, size.width);
    assert_eq!(size.height, 24f32, "height of node {:?}. Expected {}. Actual {}", node, 24f32, size.height);
    assert_eq!(location.x, 0f32, "x of node {:?}. Expected {}. Actual {}", node, 0f32, location.x);
    assert_eq!(location.y, 0f32, "y of node {:?}. Expected {}. Actual {}", node, 0f32, location.y);
}
