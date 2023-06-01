#[test]
fn leaf_overflow_scrollbars_affect_available_space_y_axis() {
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
                overflow: taffy::geometry::Point {
                    x: taffy::style::Overflow::Visible,
                    y: taffy::style::Overflow::Scroll,
                },
                scrollbar_width: 15f32,
                size: taffy::geometry::Size {
                    width: taffy::style::Dimension::Length(45f32),
                    height: taffy::style::Dimension::Length(45f32),
                },
                ..Default::default()
            },
            taffy::tree::MeasureFunc::Raw(|known_dimensions, available_space| {
                const TEXT: &str = "H\u{a0}H\u{a0}H\u{a0}H\u{a0}H\u{a0}H\u{a0}H\u{a0}H\u{a0}H\u{a0}H\u{a0}H";
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
    assert_eq!(size.width, 45f32, "width of node {:?}. Expected {}. Actual {}", node, 45f32, size.width);
    assert_eq!(size.height, 45f32, "height of node {:?}. Expected {}. Actual {}", node, 45f32, size.height);
    assert_eq!(location.x, 0f32, "x of node {:?}. Expected {}. Actual {}", node, 0f32, location.x);
    assert_eq!(location.y, 0f32, "y of node {:?}. Expected {}. Actual {}", node, 0f32, location.y);
}
