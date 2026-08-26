//! Tests for the detailed grid info exposed behind the `detailed_layout_info` feature:
//! per-track positions, per-line names, and resolved track list serialization.
#[cfg(all(feature = "detailed_layout_info", feature = "grid"))]
mod detailed_grid_info {
    use taffy::prelude::*;
    use taffy::style::{GridTemplateArea, GridTemplateAreas, GridTemplateComponent, GridTemplateRepetition};
    use taffy::tree::DetailedLayoutInfo;
    use taffy::{Point, RepetitionCount};
    use taffy_test_helpers::new_test_tree;

    fn definite(width: f32, height: f32) -> Size<AvailableSpace> {
        Size { width: AvailableSpace::Definite(width), height: AvailableSpace::Definite(height) }
    }

    fn get_detailed_grid_info(
        tree: &TaffyTree<taffy_test_helpers::TestNodeContext>,
        node: NodeId,
    ) -> &taffy::DetailedGridInfo {
        match tree.detailed_layout_info(node) {
            DetailedLayoutInfo::Grid(info) => info,
            _ => panic!("expected detailed grid info"),
        }
    }

    #[test]
    fn no_line_names() {
        let mut tree = new_test_tree();
        let child = tree.new_leaf(Style::default()).unwrap();
        let root = tree
            .new_with_children(
                Style {
                    display: Display::Grid,
                    size: Size { width: Dimension::from_length(100.0), height: Dimension::from_length(50.0) },
                    grid_template_columns: vec![length(40.0), length(60.0)],
                    grid_template_rows: vec![length(50.0)],
                    ..Default::default()
                },
                &[child],
            )
            .unwrap();
        tree.compute_layout(root, definite(100.0, 50.0)).unwrap();

        let info = get_detailed_grid_info(&tree, root);
        assert!(info.columns.line_names.is_empty());
        assert!(info.rows.line_names.is_empty());
        assert_eq!(info.columns.iter_line_names().count(), 0);
        assert_eq!(info.grid_template_columns(), "40px 60px");
        assert_eq!(info.grid_template_rows(), "50px");
    }

    #[test]
    fn implicit_track_before_explicit_grid() {
        // With `grid-template-columns: none` and a child placed at `grid-column: auto / 1`,
        // the grid has exactly one (negative) implicit column track. The resolved track list
        // must not contain a phantom zero-sized positive implicit track.
        // See WPT css/css-grid/parsing/grid-template-columns-computed-implicit-track.html
        let mut tree = new_test_tree();
        let child = tree
            .new_leaf(Style {
                size: Size { width: Dimension::from_length(10.0), height: Dimension::from_length(10.0) },
                grid_column: Line { start: GridPlacement::Auto, end: GridPlacement::from_line_index(1) },
                ..Default::default()
            })
            .unwrap();
        let root = tree.new_with_children(Style { display: Display::Grid, ..Default::default() }, &[child]).unwrap();
        tree.compute_layout(root, Size::MAX_CONTENT).unwrap();

        let info = get_detailed_grid_info(&tree, root);
        assert_eq!(info.columns.negative_implicit_tracks, 1);
        assert_eq!(info.columns.explicit_tracks, 0);
        assert_eq!(info.columns.positive_implicit_tracks, 0);
        assert_eq!(info.grid_template_columns(), "10px");
        assert_eq!(info.rows.negative_implicit_tracks, 0);
        assert_eq!(info.rows.explicit_tracks, 0);
        assert_eq!(info.rows.positive_implicit_tracks, 1);
        assert_eq!(info.grid_template_rows(), "10px");
    }

    #[test]
    fn implicit_track_before_explicit_grid_rows() {
        // Rows analogue: child placed at `grid-row: auto / 1`
        // See WPT css/css-grid/parsing/grid-template-rows-computed-implicit-track.html
        let mut tree = new_test_tree();
        let child = tree
            .new_leaf(Style {
                size: Size { width: Dimension::from_length(10.0), height: Dimension::from_length(10.0) },
                grid_row: Line { start: GridPlacement::Auto, end: GridPlacement::from_line_index(1) },
                ..Default::default()
            })
            .unwrap();
        let root = tree.new_with_children(Style { display: Display::Grid, ..Default::default() }, &[child]).unwrap();
        tree.compute_layout(root, Size::MAX_CONTENT).unwrap();

        let info = get_detailed_grid_info(&tree, root);
        assert_eq!(info.rows.negative_implicit_tracks, 1);
        assert_eq!(info.rows.explicit_tracks, 0);
        assert_eq!(info.rows.positive_implicit_tracks, 0);
        assert_eq!(info.grid_template_rows(), "10px");
        assert_eq!(info.columns.negative_implicit_tracks, 0);
        assert_eq!(info.columns.explicit_tracks, 0);
        assert_eq!(info.columns.positive_implicit_tracks, 1);
        assert_eq!(info.grid_template_columns(), "10px");
    }

    #[test]
    fn template_and_area_line_names() {
        let mut tree = new_test_tree();
        let child = tree.new_leaf(Style::default()).unwrap();
        let root = tree
            .new_with_children(
                Style {
                    display: Display::Grid,
                    size: Size { width: Dimension::from_length(100.0), height: Dimension::from_length(50.0) },
                    grid_template_columns: vec![length(40.0), length(60.0)],
                    grid_template_column_names: vec![
                        vec!["full-start".into()],
                        vec!["main-start".into()],
                        vec!["main-end".into(), "full-end".into()],
                    ],
                    grid_template_rows: vec![length(50.0)],
                    grid_template_areas: Some(GridTemplateAreas {
                        areas: vec![GridTemplateArea {
                            name: "hero".into(),
                            row_start: 1,
                            row_end: 2,
                            column_start: 1,
                            column_end: 3,
                        }],
                        row_count: 1,
                        column_count: 2,
                    }),
                    ..Default::default()
                },
                &[child],
            )
            .unwrap();
        tree.compute_layout(root, definite(100.0, 50.0)).unwrap();

        let info = get_detailed_grid_info(&tree, root);
        let column_lines: Vec<&[String]> = info.columns.iter_line_names().collect();
        assert_eq!(column_lines.len(), 3);
        assert_eq!(column_lines[0], ["full-start".to_string(), "hero-start".to_string()]);
        assert_eq!(column_lines[1], ["main-start".to_string()]);
        assert_eq!(column_lines[2], ["main-end".to_string(), "full-end".to_string(), "hero-end".to_string()]);
        assert_eq!(info.columns.names_for_line(1), ["main-start".to_string()]);

        let row_lines: Vec<&[String]> = info.rows.iter_line_names().collect();
        assert_eq!(row_lines, [&["hero-start".to_string()][..], &["hero-end".to_string()][..]]);

        assert_eq!(
            info.grid_template_columns(),
            "[full-start hero-start] 40px [main-start] 60px [main-end full-end hero-end]"
        );
        assert_eq!(info.grid_template_rows(), "[hero-start] 50px [hero-end]");
    }

    #[test]
    fn repeat_expansion_line_names() {
        let mut tree = new_test_tree();
        let child = tree.new_leaf(Style::default()).unwrap();
        let root = tree
            .new_with_children(
                Style {
                    display: Display::Grid,
                    size: Size { width: Dimension::from_length(100.0), height: Dimension::from_length(50.0) },
                    grid_template_columns: vec![GridTemplateComponent::Repeat(GridTemplateRepetition {
                        count: RepetitionCount::Count(2),
                        tracks: vec![length(50.0)],
                        line_names: vec![vec!["col-start".into()], vec!["col-end".into()]],
                    })],
                    grid_template_column_names: vec![vec!["outer-start".into()], vec!["outer-end".into()]],
                    grid_template_rows: vec![length(50.0)],
                    ..Default::default()
                },
                &[child],
            )
            .unwrap();
        tree.compute_layout(root, definite(100.0, 50.0)).unwrap();

        let info = get_detailed_grid_info(&tree, root);
        let column_lines: Vec<&[String]> = info.columns.iter_line_names().collect();
        assert_eq!(column_lines.len(), 3);
        // First repetition's end line collapses with the second repetition's start line,
        // and the last repetition's end line collapses with the following template name set
        assert_eq!(column_lines[0], ["outer-start".to_string(), "col-start".to_string()]);
        assert_eq!(column_lines[1], ["col-end".to_string(), "col-start".to_string()]);
        assert_eq!(column_lines[2], ["col-end".to_string(), "outer-end".to_string()]);

        assert_eq!(
            info.grid_template_columns(),
            "[outer-start col-start] 50px [col-end col-start] 50px [col-end outer-end]"
        );
    }

    #[test]
    fn implicit_tracks_shift_line_names() {
        let mut tree = new_test_tree();
        let child = tree
            .new_leaf(Style {
                grid_column: Line { start: GridPlacement::from_line_index(-3), end: GridPlacement::Auto },
                ..Default::default()
            })
            .unwrap();
        let root = tree
            .new_with_children(
                Style {
                    display: Display::Grid,
                    size: Size { width: Dimension::from_length(100.0), height: Dimension::from_length(50.0) },
                    grid_template_columns: vec![length(50.0)],
                    grid_template_column_names: vec![vec!["a".into()], vec!["b".into()]],
                    grid_template_rows: vec![length(50.0)],
                    ..Default::default()
                },
                &[child],
            )
            .unwrap();
        tree.compute_layout(root, definite(100.0, 50.0)).unwrap();

        let info = get_detailed_grid_info(&tree, root);
        assert_eq!(info.columns.negative_implicit_tracks, 1);
        let column_lines: Vec<&[String]> = info.columns.iter_line_names().collect();
        // One leading implicit track: its lines are unnamed, explicit names shift by one
        assert_eq!(column_lines.len(), 3);
        assert!(column_lines[0].is_empty());
        assert_eq!(column_lines[1], ["a".to_string()]);
        assert_eq!(column_lines[2], ["b".to_string()]);
        let expected = format!("{}px [a] 50px [b]", info.columns.positions[0].end - info.columns.positions[0].start);
        assert_eq!(info.grid_template_columns(), expected);
    }

    #[test]
    fn item_grid_area() {
        let mut tree = new_test_tree();
        let child_a = tree.new_leaf(Style::default()).unwrap();
        let child_b = tree
            .new_leaf(Style {
                grid_row: line(2),
                grid_column: Line { start: line(1), end: line(3) },
                ..Default::default()
            })
            .unwrap();
        let container = tree
            .new_with_children(
                Style {
                    display: Display::Grid,
                    grid_template_columns: vec![length(40.0), length(60.0)],
                    grid_template_rows: vec![length(50.0), length(30.0)],
                    gap: length(10.0),
                    padding: Rect { left: length(5.0), right: length(5.0), top: length(5.0), bottom: length(5.0) },
                    ..Default::default()
                },
                &[child_a, child_b],
            )
            .unwrap();

        tree.compute_layout(container, Size::MAX_CONTENT).unwrap();
        let info = get_detailed_grid_info(&tree, container);

        // child_a is auto-placed into row 1 / column 1
        assert_eq!(info.item_grid_area(0), Some((Point { x: 5.0, y: 5.0 }, Size { width: 40.0, height: 50.0 })));
        // child_b spans both columns in row 2
        assert_eq!(info.item_grid_area(1), Some((Point { x: 5.0, y: 65.0 }, Size { width: 110.0, height: 30.0 })));
        // out of bounds index
        assert_eq!(info.item_grid_area(2), None);

        assert_eq!(
            info.resolve_absolute_grid_area(
                Line { start: line(1), end: line(3) },
                Line { start: line(1), end: line(3) },
                taffy::style::Direction::Ltr,
                Rect { left: 0.0, right: 120.0, top: 0.0, bottom: 100.0 },
            ),
            Rect { left: 5.0, right: 115.0, top: 5.0, bottom: 95.0 }
        );
        assert_eq!(
            info.resolve_absolute_grid_area(
                Line::AUTO,
                Line { start: GridPlacement::Auto, end: line(2) },
                taffy::style::Direction::Ltr,
                Rect { left: 0.0, right: 120.0, top: 0.0, bottom: 100.0 },
            ),
            Rect { left: 0.0, right: 45.0, top: 0.0, bottom: 100.0 }
        );
        assert_eq!(
            info.resolve_absolute_grid_area(
                Line::AUTO,
                Line { start: line(-1), end: GridPlacement::Auto },
                taffy::style::Direction::Ltr,
                Rect { left: 0.0, right: 120.0, top: 0.0, bottom: 100.0 },
            ),
            Rect { left: 115.0, right: 120.0, top: 0.0, bottom: 100.0 }
        );
        assert_eq!(
            info.resolve_absolute_grid_area(
                Line::AUTO,
                Line { start: GridPlacement::NamedLine("missing".into(), 1), end: line(2) },
                taffy::style::Direction::Ltr,
                Rect { left: 0.0, right: 120.0, top: 0.0, bottom: 100.0 },
            ),
            // An unknown named line resolves to the first implicit line past the explicit grid,
            // which (being outside the grid) falls back to the padding edge. The resulting
            // start/end pair is swapped, so this is equivalent to `2 / <padding-end>`.
            Rect { left: 55.0, right: 120.0, top: 0.0, bottom: 100.0 }
        );
        assert_eq!(
            info.resolve_absolute_grid_area(
                Line::AUTO,
                Line { start: GridPlacement::Span(2), end: GridPlacement::Span(3) },
                taffy::style::Direction::Ltr,
                Rect { left: 0.0, right: 120.0, top: 0.0, bottom: 100.0 },
            ),
            Rect { left: 0.0, right: 120.0, top: 0.0, bottom: 100.0 }
        );
    }

    #[test]
    fn absolute_grid_area_named_span() {
        let mut tree = new_test_tree();
        let child = tree.new_leaf(Style::default()).unwrap();
        let root = tree
            .new_with_children(
                Style {
                    display: Display::Grid,
                    size: Size { width: Dimension::from_length(90.0), height: Dimension::from_length(50.0) },
                    grid_template_columns: vec![length(30.0), length(30.0), length(30.0)],
                    grid_template_column_names: vec![
                        vec!["a".into()],
                        vec!["b".into()],
                        vec!["b".into()],
                        vec!["c".into()],
                    ],
                    grid_template_rows: vec![length(50.0)],
                    ..Default::default()
                },
                &[child],
            )
            .unwrap();
        tree.compute_layout(root, definite(90.0, 50.0)).unwrap();

        let info = get_detailed_grid_info(&tree, root);
        assert_eq!(
            info.resolve_absolute_grid_area(
                Line::AUTO,
                Line { start: GridPlacement::NamedLine("a".into(), 1), end: GridPlacement::NamedSpan("b".into(), 2) },
                taffy::style::Direction::Ltr,
                Rect { left: 0.0, right: 90.0, top: 0.0, bottom: 50.0 },
            ),
            Rect { left: 0.0, right: 60.0, top: 0.0, bottom: 50.0 }
        );
        assert_eq!(
            info.resolve_absolute_grid_area(
                Line::AUTO,
                Line { start: GridPlacement::NamedLine("b".into(), 2), end: GridPlacement::NamedLine("c".into(), 1) },
                taffy::style::Direction::Ltr,
                Rect { left: 0.0, right: 90.0, top: 0.0, bottom: 50.0 },
            ),
            Rect { left: 60.0, right: 90.0, top: 0.0, bottom: 50.0 }
        );
    }

    #[test]
    fn rtl_line_names_are_logical_order() {
        let mut tree = new_test_tree();
        let child = tree.new_leaf(Style::default()).unwrap();
        let root = tree
            .new_with_children(
                Style {
                    display: Display::Grid,
                    direction: taffy::style::Direction::Rtl,
                    size: Size { width: Dimension::from_length(100.0), height: Dimension::from_length(50.0) },
                    grid_template_columns: vec![length(40.0), length(60.0)],
                    grid_template_column_names: vec![vec!["a".into()], vec!["b".into()], vec!["c".into()]],
                    grid_template_rows: vec![length(50.0)],
                    ..Default::default()
                },
                &[child],
            )
            .unwrap();
        tree.compute_layout(root, definite(100.0, 50.0)).unwrap();

        let info = get_detailed_grid_info(&tree, root);
        let column_lines: Vec<&[String]> = info.columns.iter_line_names().collect();
        // Tracks and line names are in logical order regardless of direction
        assert_eq!(column_lines[0], ["a".to_string()]);
        assert_eq!(column_lines[1], ["b".to_string()]);
        assert_eq!(column_lines[2], ["c".to_string()]);
        assert_eq!(info.grid_template_columns(), "[a] 40px [b] 60px [c]");

        // Positions hold physical coordinates: logical track 1 is physically rightmost in RTL
        assert_eq!(info.columns.positions[0], Line { start: 60.0, end: 100.0 });
        assert_eq!(info.columns.positions[1], Line { start: 0.0, end: 60.0 });
        // item_grid_area resolves the physical rectangle of the (auto-placed) item's grid area
        assert_eq!(info.item_grid_area(0), Some((Point { x: 60.0, y: 0.0 }, Size { width: 40.0, height: 50.0 })));
        assert_eq!(
            info.resolve_absolute_grid_area(
                Line::AUTO,
                Line { start: line(1), end: line(2) },
                taffy::style::Direction::Rtl,
                Rect { left: 0.0, right: 100.0, top: 0.0, bottom: 50.0 },
            ),
            Rect { left: 60.0, right: 100.0, top: 0.0, bottom: 50.0 }
        );
        assert_eq!(
            info.resolve_absolute_grid_area(
                Line::AUTO,
                Line { start: GridPlacement::NamedLine("a".into(), 1), end: GridPlacement::NamedLine("b".into(), 1) },
                taffy::style::Direction::Rtl,
                Rect { left: 0.0, right: 100.0, top: 0.0, bottom: 50.0 },
            ),
            Rect { left: 60.0, right: 100.0, top: 0.0, bottom: 50.0 }
        );
        assert_eq!(
            info.resolve_absolute_grid_area(
                Line::AUTO,
                Line { start: line(1), end: GridPlacement::Auto },
                taffy::style::Direction::Rtl,
                Rect { left: 0.0, right: 100.0, top: 0.0, bottom: 50.0 },
            ),
            Rect { left: 0.0, right: 100.0, top: 0.0, bottom: 50.0 }
        );
    }
}
