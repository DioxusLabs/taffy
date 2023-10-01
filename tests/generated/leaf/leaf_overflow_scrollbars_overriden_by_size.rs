#[test]
fn leaf_overflow_scrollbars_overriden_by_size() {
    #[allow(unused_imports)]
    use taffy::{
        prelude::*,
        tree::{Layout, MeasureFunc},
        Taffy,
    };
    let mut taffy: Taffy<crate::TextMeasure> = Taffy::new();
    let node = taffy
        .new_leaf(taffy::style::Style {
            overflow: taffy::geometry::Point { x: taffy::style::Overflow::Scroll, y: taffy::style::Overflow::Scroll },
            scrollbar_width: 15f32,
            size: taffy::geometry::Size {
                width: taffy::style::Dimension::Length(2f32),
                height: taffy::style::Dimension::Length(4f32),
            },
            ..Default::default()
        })
        .unwrap();
    taffy.compute_layout_with_measure(node, taffy::geometry::Size::MAX_CONTENT, crate::test_measure_function).unwrap();
    println!("\nComputed tree:");
    taffy::util::print_tree(&taffy, node);
    println!();
    let Layout { size, location, .. } = taffy.layout(node).unwrap();
    assert_eq!(size.width, 2f32, "width of node {:?}. Expected {}. Actual {}", node, 2f32, size.width);
    assert_eq!(size.height, 4f32, "height of node {:?}. Expected {}. Actual {}", node, 4f32, size.height);
    assert_eq!(location.x, 0f32, "x of node {:?}. Expected {}. Actual {}", node, 0f32, location.x);
    assert_eq!(location.y, 0f32, "y of node {:?}. Expected {}. Actual {}", node, 0f32, location.y);
}
