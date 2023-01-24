pub fn compute() {
    #[allow(unused_imports)]
    use taffy::prelude::*;
    let mut taffy = taffy::Taffy::new();
    let node0 = taffy
        .new_leaf(taffy::style::Style {
            grid_row: taffy::geometry::Line { start: line(1i16), end: taffy::style::GridPlacement::Auto },
            grid_column: taffy::geometry::Line { start: line(1i16), end: taffy::style::GridPlacement::Auto },
            ..Default::default()
        })
        .unwrap();
    let node1 = taffy
        .new_leaf(taffy::style::Style {
            grid_row: taffy::geometry::Line { start: line(1i16), end: taffy::style::GridPlacement::Auto },
            grid_column: taffy::geometry::Line { start: line(2i16), end: taffy::style::GridPlacement::Auto },
            ..Default::default()
        })
        .unwrap();
    let node2 = taffy
        .new_leaf_with_measure(
            taffy::style::Style {
                grid_row: taffy::geometry::Line { start: line(1i16), end: taffy::style::GridPlacement::Auto },
                grid_column: taffy::geometry::Line { start: line(1i16), end: taffy::style::GridPlacement::Span(2u16) },
                ..Default::default()
            },
            taffy::node::MeasureFunc::Raw(|known_dimensions, available_space| {
                const TEXT: &str = "HHHH\u{200b}HHHH";
                super::measure_standard_text(
                    known_dimensions,
                    available_space,
                    TEXT,
                    super::WritingMode::Horizontal,
                    None,
                )
            }),
        )
        .unwrap();
    let node = taffy
        .new_with_children(
            taffy::style::Style {
                display: taffy::style::Display::Grid,
                grid_template_rows: vec![points(40f32)],
                grid_template_columns: vec![min_content(), min_content()],
                ..Default::default()
            },
            &[node0, node1, node2],
        )
        .unwrap();
    taffy.compute_layout(node, taffy::geometry::Size::MAX_CONTENT).unwrap();
}
