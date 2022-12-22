pub fn compute() {
    #[allow(unused_imports)]
    use taffy::prelude::*;
    let mut taffy = taffy::Taffy::new();
    let node0 = taffy
        .new_leaf(taffy::style::Style {
            grid_row: taffy::geometry::Line {
                start: taffy::style::GridPlacement::Line(1i16),
                end: taffy::style::GridPlacement::Auto,
            },
            grid_column: taffy::geometry::Line {
                start: taffy::style::GridPlacement::Line(1i16),
                end: taffy::style::GridPlacement::Auto,
            },
            size: taffy::geometry::Size {
                width: taffy::style::Dimension::Points(20f32),
                height: taffy::style::Dimension::Points(20f32),
            },
            ..Default::default()
        })
        .unwrap();
    let node1 = taffy
        .new_leaf(taffy::style::Style {
            grid_row: taffy::geometry::Line {
                start: taffy::style::GridPlacement::Line(3i16),
                end: taffy::style::GridPlacement::Auto,
            },
            grid_column: taffy::geometry::Line {
                start: taffy::style::GridPlacement::Line(3i16),
                end: taffy::style::GridPlacement::Auto,
            },
            size: taffy::geometry::Size {
                width: taffy::style::Dimension::Points(60f32),
                height: taffy::style::Dimension::Points(60f32),
            },
            ..Default::default()
        })
        .unwrap();
    let node = taffy
        .new_with_children(
            taffy::style::Style {
                display: taffy::style::Display::Grid,
                justify_items: Some(taffy::style::JustifyItems::Center),
                grid_template_rows: vec![points(40f32), points(40f32), points(40f32)],
                grid_template_columns: vec![points(40f32), points(40f32), points(40f32)],
                size: taffy::geometry::Size {
                    width: taffy::style::Dimension::Points(120f32),
                    height: taffy::style::Dimension::Points(120f32),
                },
                ..Default::default()
            },
            &[node0, node1],
        )
        .unwrap();
    taffy.compute_layout(node, taffy::geometry::Size::MAX_CONTENT).unwrap();
}
