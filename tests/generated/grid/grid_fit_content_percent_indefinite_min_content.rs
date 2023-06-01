#[test]
fn grid_fit_content_percent_indefinite_min_content() {
    #[allow(unused_imports)]
    use taffy::{
        prelude::*,
        tree::{Layout, MeasureFunc},
        Taffy,
    };
    let mut taffy: Taffy<MeasureFunc<()>> = Taffy::new();
    let node0 = taffy
        .new_leaf_with_measure(
            taffy::style::Style { ..Default::default() },
            taffy::tree::MeasureFunc::Raw(|known_dimensions, available_space, _context| {
                const TEXT: &str = "HHHH\u{200b}HH";
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
    let node = taffy
        .new_with_children(
            taffy::style::Style {
                display: taffy::style::Display::Grid,
                grid_template_rows: vec![length(40f32)],
                grid_template_columns: vec![fit_content(percent(0.5f32))],
                ..Default::default()
            },
            &[node0],
        )
        .unwrap();
    taffy.compute_layout(node, taffy::geometry::Size::MAX_CONTENT).unwrap();
    println!("\nComputed tree:");
    taffy::util::print_tree(&taffy, node);
    println!();
    let Layout { size, location, .. } = taffy.layout(node).unwrap();
    assert_eq!(size.width, 60f32, "width of node {:?}. Expected {}. Actual {}", node, 60f32, size.width);
    assert_eq!(size.height, 40f32, "height of node {:?}. Expected {}. Actual {}", node, 40f32, size.height);
    assert_eq!(location.x, 0f32, "x of node {:?}. Expected {}. Actual {}", node, 0f32, location.x);
    assert_eq!(location.y, 0f32, "y of node {:?}. Expected {}. Actual {}", node, 0f32, location.y);
    let Layout { size, location, .. } = taffy.layout(node0).unwrap();
    assert_eq!(size.width, 40f32, "width of node {:?}. Expected {}. Actual {}", node0, 40f32, size.width);
    assert_eq!(size.height, 40f32, "height of node {:?}. Expected {}. Actual {}", node0, 40f32, size.height);
    assert_eq!(location.x, 0f32, "x of node {:?}. Expected {}. Actual {}", node0, 0f32, location.x);
    assert_eq!(location.y, 0f32, "y of node {:?}. Expected {}. Actual {}", node0, 0f32, location.y);
}
