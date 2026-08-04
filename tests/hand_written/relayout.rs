use taffy::prelude::*;
use taffy_test_helpers::new_test_tree;

#[test]
fn relayout() {
    let mut taffy = new_test_tree();
    let node1 = taffy
        .new_leaf(taffy::style::Style {
            size: taffy::geometry::Size { width: length(8.0), height: length(80.0) },
            ..Default::default()
        })
        .unwrap();
    let node0 = taffy
        .new_with_children(
            taffy::style::Style {
                align_self: Some(taffy::prelude::AlignSelf::CENTER),
                size: taffy::geometry::Size { width: Dimension::AUTO, height: Dimension::AUTO },
                // size: taffy::geometry::Size { width: Dimension::Percent(1.0), height: Dimension::Percent(1.0) },
                ..Default::default()
            },
            &[node1],
        )
        .unwrap();
    let node = taffy
        .new_with_children(
            taffy::style::Style {
                size: taffy::geometry::Size {
                    width: Dimension::from_percent(1f32),
                    height: Dimension::from_percent(1f32),
                },
                ..Default::default()
            },
            &[node0],
        )
        .unwrap();
    taffy
        .compute_layout(
            node,
            taffy::geometry::Size { width: AvailableSpace::Definite(100f32), height: AvailableSpace::Definite(100f32) },
        )
        .unwrap();
    let initial = taffy.layout(node).unwrap().location;
    let initial0 = taffy.layout(node0).unwrap().location;
    let initial1 = taffy.layout(node1).unwrap().location;
    for _ in 1..10 {
        taffy
            .compute_layout(
                node,
                taffy::geometry::Size {
                    width: AvailableSpace::Definite(100f32),
                    height: AvailableSpace::Definite(100f32),
                },
            )
            .unwrap();
        assert_eq!(taffy.layout(node).unwrap().location, initial);
        assert_eq!(taffy.layout(node0).unwrap().location, initial0);
        assert_eq!(taffy.layout(node1).unwrap().location, initial1);
    }
}

#[test]
fn toggle_root_display_none() {
    let hidden_style = Style {
        display: Display::None,
        size: Size { width: length(100.0), height: length(100.0) },
        ..Default::default()
    };

    let flex_style = Style {
        display: Display::Flex,
        size: Size { width: length(100.0), height: length(100.0) },
        ..Default::default()
    };

    // Setup
    let mut taffy = new_test_tree();
    let node = taffy.new_leaf(hidden_style.clone()).unwrap();

    // Layout 1 (None)
    taffy.compute_layout(node, Size::MAX_CONTENT).unwrap();
    let layout = taffy.layout(node).unwrap();
    assert_eq!(layout.location.x, 0.0);
    assert_eq!(layout.location.y, 0.0);
    assert_eq!(layout.size.width, 0.0);
    assert_eq!(layout.size.height, 0.0);

    // Layout 2 (Flex)
    taffy.set_style(node, flex_style).unwrap();
    taffy.compute_layout(node, Size::MAX_CONTENT).unwrap();
    let layout = taffy.layout(node).unwrap();
    assert_eq!(layout.location.x, 0.0);
    assert_eq!(layout.location.y, 0.0);
    assert_eq!(layout.size.width, 100.0);
    assert_eq!(layout.size.height, 100.0);

    // Layout 3 (None)
    taffy.set_style(node, hidden_style).unwrap();
    taffy.compute_layout(node, Size::MAX_CONTENT).unwrap();
    let layout = taffy.layout(node).unwrap();
    assert_eq!(layout.location.x, 0.0);
    assert_eq!(layout.location.y, 0.0);
    assert_eq!(layout.size.width, 0.0);
    assert_eq!(layout.size.height, 0.0);
}

#[test]
fn toggle_root_display_none_with_children() {
    use taffy::prelude::*;

    let mut taffy = new_test_tree();

    let child = taffy
        .new_leaf(Style { size: Size { width: length(800.0), height: length(100.0) }, ..Default::default() })
        .unwrap();

    let parent = taffy
        .new_with_children(
            Style { size: Size { width: length(800.0), height: length(100.0) }, ..Default::default() },
            &[child],
        )
        .unwrap();

    let root = taffy.new_with_children(Style::default(), &[parent]).unwrap();
    taffy.compute_layout(root, Size::MAX_CONTENT).unwrap();
    assert_eq!(taffy.layout(child).unwrap().size.width, 800.0);
    assert_eq!(taffy.layout(child).unwrap().size.height, 100.0);

    taffy.set_style(root, Style { display: Display::None, ..Default::default() }).unwrap();
    taffy.compute_layout(root, Size::MAX_CONTENT).unwrap();
    assert_eq!(taffy.layout(child).unwrap().size.width, 0.0);
    assert_eq!(taffy.layout(child).unwrap().size.height, 0.0);

    taffy.set_style(root, Style::default()).unwrap();
    taffy.compute_layout(root, Size::MAX_CONTENT).unwrap();
    assert_eq!(taffy.layout(parent).unwrap().size.width, 800.0);
    assert_eq!(taffy.layout(parent).unwrap().size.height, 100.0);
    assert_eq!(taffy.layout(child).unwrap().size.width, 800.0);
    assert_eq!(taffy.layout(child).unwrap().size.height, 100.0);
}

#[test]
fn toggle_flex_child_display_none() {
    let hidden_style = Style {
        display: Display::None,
        size: Size { width: length(100.0), height: length(100.0) },
        ..Default::default()
    };

    let flex_style = Style {
        display: Display::Flex,
        size: Size { width: length(100.0), height: length(100.0) },
        ..Default::default()
    };

    // Setup
    let mut taffy = new_test_tree();
    let node = taffy.new_leaf(hidden_style.clone()).unwrap();
    let root = taffy.new_with_children(flex_style.clone(), &[node]).unwrap();

    // Layout 1 (None)
    taffy.compute_layout(root, Size::MAX_CONTENT).unwrap();
    let layout = taffy.layout(node).unwrap();
    assert_eq!(layout.location.x, 0.0);
    assert_eq!(layout.location.y, 0.0);
    assert_eq!(layout.size.width, 0.0);
    assert_eq!(layout.size.height, 0.0);

    // Layout 2 (Flex)
    taffy.set_style(node, flex_style).unwrap();
    taffy.compute_layout(root, Size::MAX_CONTENT).unwrap();
    let layout = taffy.layout(node).unwrap();
    assert_eq!(layout.location.x, 0.0);
    assert_eq!(layout.location.y, 0.0);
    assert_eq!(layout.size.width, 100.0);
    assert_eq!(layout.size.height, 100.0);

    // Layout 3 (None)
    taffy.set_style(node, hidden_style).unwrap();
    taffy.compute_layout(root, Size::MAX_CONTENT).unwrap();
    let layout = taffy.layout(node).unwrap();
    assert_eq!(layout.location.x, 0.0);
    assert_eq!(layout.location.y, 0.0);
    assert_eq!(layout.size.width, 0.0);
    assert_eq!(layout.size.height, 0.0);
}

#[test]
fn toggle_flex_container_display_none() {
    let hidden_style = Style {
        display: Display::None,
        size: Size { width: length(100.0), height: length(100.0) },
        ..Default::default()
    };

    let flex_style = Style {
        display: Display::Flex,
        size: Size { width: length(100.0), height: length(100.0) },
        ..Default::default()
    };

    // Setup
    let mut taffy = new_test_tree();
    let node = taffy.new_leaf(hidden_style.clone()).unwrap();
    let root = taffy.new_with_children(hidden_style.clone(), &[node]).unwrap();

    // Layout 1 (None)
    taffy.compute_layout(root, Size::MAX_CONTENT).unwrap();
    let layout = taffy.layout(root).unwrap();
    assert_eq!(layout.location.x, 0.0);
    assert_eq!(layout.location.y, 0.0);
    assert_eq!(layout.size.width, 0.0);
    assert_eq!(layout.size.height, 0.0);

    // Layout 2 (Flex)
    taffy.set_style(root, flex_style).unwrap();
    taffy.compute_layout(root, Size::MAX_CONTENT).unwrap();
    let layout = taffy.layout(root).unwrap();
    assert_eq!(layout.location.x, 0.0);
    assert_eq!(layout.location.y, 0.0);
    assert_eq!(layout.size.width, 100.0);
    assert_eq!(layout.size.height, 100.0);

    // Layout 3 (None)
    taffy.set_style(root, hidden_style).unwrap();
    taffy.compute_layout(root, Size::MAX_CONTENT).unwrap();
    let layout = taffy.layout(root).unwrap();
    assert_eq!(layout.location.x, 0.0);
    assert_eq!(layout.location.y, 0.0);
    assert_eq!(layout.size.width, 0.0);
    assert_eq!(layout.size.height, 0.0);
}

#[test]
fn toggle_grid_child_display_none() {
    let hidden_style = Style {
        display: Display::None,
        size: Size { width: length(100.0), height: length(100.0) },
        ..Default::default()
    };

    let grid_style = Style {
        display: Display::Grid,
        size: Size { width: length(100.0), height: length(100.0) },
        ..Default::default()
    };

    // Setup
    let mut taffy = new_test_tree();
    let node = taffy.new_leaf(hidden_style.clone()).unwrap();
    let root = taffy.new_with_children(grid_style.clone(), &[node]).unwrap();

    // Layout 1 (None)
    taffy.compute_layout(root, Size::MAX_CONTENT).unwrap();
    let layout = taffy.layout(node).unwrap();
    assert_eq!(layout.location.x, 0.0);
    assert_eq!(layout.location.y, 0.0);
    assert_eq!(layout.size.width, 0.0);
    assert_eq!(layout.size.height, 0.0);

    // Layout 2 (Flex)
    taffy.set_style(node, grid_style).unwrap();
    taffy.compute_layout(root, Size::MAX_CONTENT).unwrap();
    let layout = taffy.layout(node).unwrap();
    assert_eq!(layout.location.x, 0.0);
    assert_eq!(layout.location.y, 0.0);
    assert_eq!(layout.size.width, 100.0);
    assert_eq!(layout.size.height, 100.0);

    // Layout 3 (None)
    taffy.set_style(node, hidden_style).unwrap();
    taffy.compute_layout(root, Size::MAX_CONTENT).unwrap();
    let layout = taffy.layout(node).unwrap();
    assert_eq!(layout.location.x, 0.0);
    assert_eq!(layout.location.y, 0.0);
    assert_eq!(layout.size.width, 0.0);
    assert_eq!(layout.size.height, 0.0);
}

#[test]
fn toggle_grid_container_display_none() {
    let hidden_style = Style {
        display: Display::None,
        size: Size { width: length(100.0), height: length(100.0) },
        ..Default::default()
    };

    let grid_style = Style {
        display: Display::Grid,
        size: Size { width: length(100.0), height: length(100.0) },
        ..Default::default()
    };

    // Setup
    let mut taffy = new_test_tree();
    let node = taffy.new_leaf(hidden_style.clone()).unwrap();
    let root = taffy.new_with_children(hidden_style.clone(), &[node]).unwrap();

    // Layout 1 (None)
    taffy.compute_layout(root, Size::MAX_CONTENT).unwrap();
    let layout = taffy.layout(root).unwrap();
    assert_eq!(layout.location.x, 0.0);
    assert_eq!(layout.location.y, 0.0);
    assert_eq!(layout.size.width, 0.0);
    assert_eq!(layout.size.height, 0.0);

    // Layout 2 (Flex)
    taffy.set_style(root, grid_style).unwrap();
    taffy.compute_layout(root, Size::MAX_CONTENT).unwrap();
    let layout = taffy.layout(root).unwrap();
    assert_eq!(layout.location.x, 0.0);
    assert_eq!(layout.location.y, 0.0);
    assert_eq!(layout.size.width, 100.0);
    assert_eq!(layout.size.height, 100.0);

    // Layout 3 (None)
    taffy.set_style(root, hidden_style).unwrap();
    taffy.compute_layout(root, Size::MAX_CONTENT).unwrap();
    let layout = taffy.layout(root).unwrap();
    assert_eq!(layout.location.x, 0.0);
    assert_eq!(layout.location.y, 0.0);
    assert_eq!(layout.size.width, 0.0);
    assert_eq!(layout.size.height, 0.0);
}

/// Port of the 3rd test case of the WPT test
/// css/css-grid/placement/grid-container-change-grid-tracks-recompute-child-positions-001.html
///
/// Checks that changing grid-template-{rows,columns,areas} on an already-laid-out grid container
/// recomputes the positions of automatically placed grid items.
///
/// ```css
/// .grid {
///     grid-auto-flow: row dense;
///     grid-auto-rows: 5px;
///     grid-auto-columns: 5px;
/// }
/// #firstGridItem { grid-row: auto; grid-column: 1; }
/// #secondGridItem { grid-row: 1; grid-column: auto; }
/// #thirdGridItem { grid-row: auto; grid-column: auto; }
/// ```
///
/// The 3rd case sets `grid-template-rows: 10px; grid-template-columns: 10px;
/// grid-template-areas: "a ."` (the previous case had `grid-template-areas: "a"`).
#[test]
fn grid_track_changes_recompute_auto_placed_item_positions() {
    use taffy::{GridTemplateArea, GridTemplateComponent};

    fn container_style(
        template_rows: Vec<GridTemplateComponent<String>>,
        template_columns: Vec<GridTemplateComponent<String>>,
        template_areas: Vec<GridTemplateArea<String>>,
    ) -> Style {
        Style {
            display: Display::Grid,
            grid_auto_flow: GridAutoFlow::RowDense,
            grid_auto_rows: vec![length(5.0)],
            grid_auto_columns: vec![length(5.0)],
            grid_template_rows: template_rows,
            grid_template_columns: template_columns,
            grid_template_areas: template_areas,
            ..Default::default()
        }
    }

    let mut taffy = new_test_tree();

    // #firstGridItem: grid-row: auto; grid-column: 1;
    let first =
        taffy.new_leaf(Style { grid_column: Line { start: line(1), end: auto() }, ..Default::default() }).unwrap();
    // #secondGridItem: grid-row: 1; grid-column: auto;
    let second =
        taffy.new_leaf(Style { grid_row: Line { start: line(1), end: auto() }, ..Default::default() }).unwrap();
    // #thirdGridItem: grid-row: auto; grid-column: auto;
    let third = taffy.new_leaf(Style::default()).unwrap();

    // 2nd case: grid-template-rows: 10px; grid-template-columns: 10px; grid-template-areas: "a";
    let container = taffy
        .new_with_children(
            container_style(
                vec![length(10.0)],
                vec![length(10.0)],
                vec![GridTemplateArea { name: "a".into(), row_start: 1, row_end: 2, column_start: 1, column_end: 2 }],
            ),
            &[first, second, third],
        )
        .unwrap();
    taffy.compute_layout(container, Size::MAX_CONTENT).unwrap();

    // 3rd case: grid-template-rows: 10px; grid-template-columns: 10px; grid-template-areas: "a .";
    // The "." cell implies a second explicit column (sized by grid-auto-columns: 5px).
    //
    // NOTE: Taffy's `GridTemplateArea` only records named areas, so the unnamed "." cell cannot be
    // represented directly and the implied explicit grid size is derived from the extents of the
    // named areas. As no grid item references area "a", we encode the 2-column-wide template by
    // extending area "a" to `column_end: 3` (with `column_end: 2` — as a stylo-based conversion
    // would produce — Taffy only infers 1 explicit column and places the third item in row 3).
    taffy
        .set_style(
            container,
            container_style(
                vec![length(10.0)],
                vec![length(10.0)],
                vec![GridTemplateArea { name: "a".into(), row_start: 1, row_end: 2, column_start: 1, column_end: 3 }],
            ),
        )
        .unwrap();
    taffy.compute_layout(container, Size::MAX_CONTENT).unwrap();

    // Expected: first { w: 10, h: 5, x: 0, y: 10 }, second { w: 10, h: 10, x: 0, y: 0 },
    // third { w: 5, h: 10, x: 10, y: 0 }
    let layout = taffy.layout(first).unwrap();
    assert_eq!(layout.location.x, 0.0);
    assert_eq!(layout.location.y, 10.0);
    assert_eq!(layout.size.width, 10.0);
    assert_eq!(layout.size.height, 5.0);

    let layout = taffy.layout(second).unwrap();
    assert_eq!(layout.location.x, 0.0);
    assert_eq!(layout.location.y, 0.0);
    assert_eq!(layout.size.width, 10.0);
    assert_eq!(layout.size.height, 10.0);

    let layout = taffy.layout(third).unwrap();
    assert_eq!(layout.location.x, 10.0);
    assert_eq!(layout.location.y, 0.0);
    assert_eq!(layout.size.width, 5.0);
    assert_eq!(layout.size.height, 10.0);
}

#[test]
fn relayout_is_stable_with_rounding() {
    let mut taffy = new_test_tree();
    taffy.enable_rounding();

    // <div style="width: 1920px; height: 1080px">
    //     <div style="width: 100%; left: 1.5px">
    //         <div style="width: 150px; justify-content: end">
    //             <div style="min-width: 300px" />
    //         </div>
    //     </div>
    // </div>

    let inner =
        taffy.new_leaf(Style { min_size: Size { width: length(300.), height: auto() }, ..Default::default() }).unwrap();
    let wrapper = taffy
        .new_with_children(
            Style {
                size: Size { width: length(150.), height: auto() },
                justify_content: Some(JustifyContent::END),
                ..Default::default()
            },
            &[inner],
        )
        .unwrap();
    let outer = taffy
        .new_with_children(
            Style {
                size: Size { width: percent(1.), height: auto() },
                inset: Rect { left: length(1.5), right: auto(), top: auto(), bottom: auto() },
                ..Default::default()
            },
            &[wrapper],
        )
        .unwrap();
    let root = taffy
        .new_with_children(
            Style { size: Size { width: length(1920.), height: length(1080.) }, ..Default::default() },
            &[outer],
        )
        .unwrap();

    // Compute and assert initial layout.

    taffy.compute_layout(root, Size::MAX_CONTENT).ok();
    taffy.print_tree(root);

    let initial_root_layout = taffy.layout(root).unwrap().clone();
    assert_eq!(initial_root_layout.location.x, 0.0);
    assert_eq!(initial_root_layout.location.y, 0.0);
    assert_eq!(initial_root_layout.size.width, 1920.0);
    assert_eq!(initial_root_layout.size.height, 1080.0);

    let initial_outer_layout = taffy.layout(outer).unwrap().clone();
    assert_eq!(initial_outer_layout.location.x, 2.0);
    assert_eq!(initial_outer_layout.location.y, 0.0);
    assert_eq!(initial_outer_layout.size.width, 1920.0);
    assert_eq!(initial_outer_layout.size.height, 1080.0);

    let initial_wrapper_layout = taffy.layout(wrapper).unwrap().clone();
    assert_eq!(initial_wrapper_layout.location.x, 0.0);
    assert_eq!(initial_wrapper_layout.location.y, 0.0);
    assert_eq!(initial_wrapper_layout.size.width, 150.0);
    assert_eq!(initial_wrapper_layout.size.height, 1080.0);

    let initial_inner_layout = taffy.layout(inner).unwrap().clone();
    assert_eq!(initial_inner_layout.location.x, -150.0);
    assert_eq!(initial_inner_layout.location.y, 0.0);
    assert_eq!(initial_inner_layout.size.width, 300.0);
    assert_eq!(initial_inner_layout.size.height, 1080.0);

    // Recompute and assert that new layout marks initial layout each time
    for _ in 0..5 {
        taffy.mark_dirty(root).ok();
        taffy.compute_layout(root, Size::MAX_CONTENT).ok();
        taffy.print_tree(root);

        let root_layout = taffy.layout(root).unwrap();
        assert_eq!(initial_root_layout.location.x, root_layout.location.x);
        assert_eq!(initial_root_layout.location.y, root_layout.location.y);
        assert_eq!(initial_root_layout.size.width, root_layout.size.width);
        assert_eq!(initial_root_layout.size.height, root_layout.size.height);
        let outer_layout = taffy.layout(outer).unwrap();
        assert_eq!(initial_outer_layout.location.x, outer_layout.location.x);
        assert_eq!(initial_outer_layout.location.y, outer_layout.location.y);
        assert_eq!(initial_outer_layout.size.width, outer_layout.size.width);
        assert_eq!(initial_outer_layout.size.height, outer_layout.size.height);
        let wrapper_layout = taffy.layout(wrapper).unwrap();
        assert_eq!(initial_wrapper_layout.location.x, wrapper_layout.location.x);
        assert_eq!(initial_wrapper_layout.location.x, wrapper_layout.location.y);
        assert_eq!(initial_wrapper_layout.size.width, wrapper_layout.size.width);
        assert_eq!(initial_wrapper_layout.size.height, wrapper_layout.size.height);
        let inner_layout = taffy.layout(inner).unwrap();
        assert_eq!(initial_inner_layout.location.x, inner_layout.location.x);
        assert_eq!(initial_inner_layout.location.y, inner_layout.location.y);
        assert_eq!(initial_inner_layout.size.width, inner_layout.size.width);
        assert_eq!(initial_inner_layout.size.height, inner_layout.size.height);
    }
}
