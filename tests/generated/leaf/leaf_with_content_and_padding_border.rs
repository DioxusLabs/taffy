#[test]
fn leaf_with_content_and_padding_border() {
    #[allow(unused_imports)]
    use taffy::{
        prelude::*,
        tree::{Layout, MeasureFunc},
        Taffy,
    };
    let mut taffy: Taffy<MeasureFunc<()>> = Taffy::new();
    let node = taffy
        .new_leaf_with_measure(
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
            taffy::tree::MeasureFunc::Raw(|known_dimensions, available_space, _context| {
                const TEXT: &str = "HHHH";
                crate::measure_standard_text(
                    known_dimensions,
                    available_space,
                    TEXT,
                    crate::WritingMode::Horizontal,
                    None,
                )
            }),
        )
        .unwrap();
    taffy.compute_layout(node, taffy::geometry::Size::MAX_CONTENT).unwrap();
    println!("\nComputed tree:");
    taffy::util::print_tree(&taffy, node);
    println!();
    let Layout { size, location, .. } = taffy.layout(node).unwrap();
    assert_eq!(size.width, 62f32, "width of node {:?}. Expected {}. Actual {}", node, 62f32, size.width);
    assert_eq!(size.height, 24f32, "height of node {:?}. Expected {}. Actual {}", node, 24f32, size.height);
    assert_eq!(location.x, 0f32, "x of node {:?}. Expected {}. Actual {}", node, 0f32, location.x);
    assert_eq!(location.y, 0f32, "y of node {:?}. Expected {}. Actual {}", node, 0f32, location.y);
}
