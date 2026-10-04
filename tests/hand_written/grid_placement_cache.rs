//! Tests for the caching of the result of the grid item placement algorithm (see `GridPlacementCache`).
//!
//! In debug builds, Taffy checks every use of a cached placement against the result of running the placement
//! algorithm (and panics if they differ), so these tests also fail if `TaffyTree` fails to invalidate a cache.
use taffy::prelude::*;

/// The number of times that the placement algorithm has run on this thread (only tracked in debug builds)
fn placement_runs() -> Option<usize> {
    #[cfg(debug_assertions)]
    return Some(taffy::compute::grid_placement_runs());
    #[cfg(not(debug_assertions))]
    return None;
}

/// Assert that the placement algorithm has run `expected` times since `before` was returned by `placement_runs`
#[track_caller]
fn assert_placement_runs_since(before: Option<usize>, expected: usize) {
    if let (Some(before), Some(after)) = (before, placement_runs()) {
        assert_eq!(after - before, expected, "unexpected number of runs of the grid placement algorithm");
    }
}

fn item(width: f32) -> Style {
    Style { size: Size { width: length(width), height: length(10.0) }, ..Default::default() }
}

fn placed_item(column: i16, row: i16) -> Style {
    Style { grid_column: line(column), grid_row: line(row), ..item(20.0) }
}

/// Three auto-sized columns: sizing these requires measuring the items
fn grid_style() -> Style {
    Style { display: Display::Grid, grid_template_columns: vec![auto(), auto(), auto()], ..Default::default() }
}

/// A tree consisting of a grid (with the passed style and child styles) which is the sole item of an outer grid
/// with an auto-sized column, so that the inner grid is sized multiple times in each layout pass.
struct TestTree {
    taffy: TaffyTree,
    root: NodeId,
    grid: NodeId,
    items: Vec<NodeId>,
}

impl TestTree {
    fn new(grid_style: Style, item_styles: Vec<Style>) -> Self {
        let mut taffy: TaffyTree = TaffyTree::new();
        let items: Vec<NodeId> = item_styles.into_iter().map(|style| taffy.new_leaf(style).unwrap()).collect();
        let grid = taffy.new_with_children(grid_style, &items).unwrap();
        let root_style = Style {
            display: Display::Grid,
            grid_template_columns: vec![auto()],
            justify_items: Some(JustifyItems::START),
            ..Default::default()
        };
        let root = taffy.new_with_children(root_style, &[grid]).unwrap();
        Self { taffy, root, grid, items }
    }

    fn layout(&mut self) {
        self.taffy.compute_layout(self.root, Size { width: length(400.0), height: max_content() }).unwrap();
    }

    /// Lay the tree out, then check that the result is the same as that of laying out a newly created
    /// tree with the same structure and styles (which has no cached state).
    #[track_caller]
    fn layout_and_check(&mut self) {
        self.layout();

        let item_styles = self.item_styles();
        let mut fresh = TestTree::new(self.taffy.style(self.grid).unwrap().clone(), item_styles);
        fresh.layout();

        assert_eq!(self.taffy.layout(self.root).unwrap(), fresh.taffy.layout(fresh.root).unwrap(), "root layout");
        assert_eq!(self.taffy.layout(self.grid).unwrap(), fresh.taffy.layout(fresh.grid).unwrap(), "grid layout");
        let children = self.taffy.children(self.grid).unwrap();
        assert_eq!(children.len(), fresh.items.len());
        for (index, (child, fresh_child)) in children.iter().zip(fresh.items.iter()).enumerate() {
            let layout = self.taffy.layout(*child).unwrap();
            let fresh_layout = fresh.taffy.layout(*fresh_child).unwrap();
            assert_eq!(layout, fresh_layout, "layout of child {index}");
        }
    }

    fn item_styles(&self) -> Vec<Style> {
        let children = self.taffy.children(self.grid).unwrap();
        children.iter().map(|child| self.taffy.style(*child).unwrap().clone()).collect()
    }

    /// The (x, y) location of each child of the grid
    fn item_locations(&self) -> Vec<(f32, f32)> {
        let children = self.taffy.children(self.grid).unwrap();
        children.iter().map(|child| self.taffy.layout(*child).unwrap().location).map(|loc| (loc.x, loc.y)).collect()
    }
}

fn five_items() -> Vec<Style> {
    vec![item(10.0), item(20.0), item(30.0), item(40.0), item(50.0)]
}

/// Lay out a tree, apply `mutation`, and check that the next layout runs placement `expected_runs` times and
/// gives the same result as a tree without any cached state.
#[track_caller]
fn check_mutation(expected_runs: usize, mutation: impl FnOnce(&mut TestTree)) {
    let mut tree = TestTree::new(grid_style(), five_items());
    tree.layout_and_check();

    mutation(&mut tree);

    let before = placement_runs();
    tree.layout();
    assert_placement_runs_since(before, expected_runs);
    tree.layout_and_check();
}

#[test]
fn placement_runs_once_per_grid_in_a_layout_pass() {
    let mut tree = TestTree::new(grid_style(), five_items());
    let before = placement_runs();
    tree.layout();
    // Once for the outer grid and once for the inner grid (even though the inner grid is sized more than once)
    assert_placement_runs_since(before, 2);
    assert_eq!(tree.item_locations(), [(0.0, 0.0), (40.0, 0.0), (90.0, 0.0), (0.0, 10.0), (40.0, 10.0)]);
}

#[test]
fn placement_is_reused_when_nothing_changed() {
    check_mutation(0, |_| {});
}

#[test]
fn placement_is_reused_after_mark_dirty() {
    check_mutation(0, |tree| tree.taffy.mark_dirty(tree.items[2]).unwrap());
    check_mutation(0, |tree| tree.taffy.mark_dirty(tree.grid).unwrap());
}

#[test]
fn placement_is_reused_when_an_item_is_resized() {
    check_mutation(0, |tree| tree.taffy.set_style(tree.items[1], item(100.0)).unwrap());
}

#[test]
fn placement_is_reused_when_available_space_changes() {
    let mut tree = TestTree::new(grid_style(), five_items());
    tree.layout();
    let before = placement_runs();
    tree.taffy.compute_layout(tree.root, Size { width: length(50.0), height: max_content() }).unwrap();
    tree.taffy.compute_layout(tree.root, Size::MAX_CONTENT).unwrap();
    assert_placement_runs_since(before, 0);
}

#[test]
fn item_grid_placement_change_invalidates_placement() {
    check_mutation(1, |tree| tree.taffy.set_style(tree.items[1], placed_item(3, 2)).unwrap());
    check_mutation(1, |tree| {
        let style = Style { grid_row: Line { start: span(2), end: auto() }, ..item(10.0) };
        tree.taffy.set_style(tree.items[0], style).unwrap()
    });
}

#[test]
fn item_display_none_invalidates_placement() {
    check_mutation(1, |tree| {
        tree.taffy.set_style(tree.items[1], Style { display: Display::None, ..item(20.0) }).unwrap()
    });
}

#[test]
fn item_position_change_invalidates_placement() {
    check_mutation(1, |tree| {
        tree.taffy.set_style(tree.items[1], Style { position: Position::Absolute, ..item(20.0) }).unwrap()
    });
}

#[test]
fn container_style_change_invalidates_placement() {
    check_mutation(1, |tree| {
        let style = Style { grid_template_columns: vec![auto(), auto()], ..grid_style() };
        tree.taffy.set_style(tree.grid, style).unwrap()
    });
    check_mutation(1, |tree| {
        let style = Style { grid_auto_flow: GridAutoFlow::Column, grid_template_rows: vec![auto(); 2], ..grid_style() };
        tree.taffy.set_style(tree.grid, style).unwrap()
    });
}

#[test]
fn adding_a_child_invalidates_placement() {
    check_mutation(1, |tree| {
        let child = tree.taffy.new_leaf(item(60.0)).unwrap();
        tree.taffy.add_child(tree.grid, child).unwrap();
    });
    check_mutation(1, |tree| {
        let child = tree.taffy.new_leaf(item(60.0)).unwrap();
        tree.taffy.insert_child_at_index(tree.grid, 1, child).unwrap();
    });
}

#[test]
fn removing_a_child_invalidates_placement() {
    check_mutation(1, |tree| {
        tree.taffy.remove_child(tree.grid, tree.items[1]).unwrap();
    });
    check_mutation(1, |tree| {
        tree.taffy.remove_child_at_index(tree.grid, 0).unwrap();
    });
    check_mutation(1, |tree| {
        tree.taffy.remove_children_range(tree.grid, 1..3).unwrap();
    });
    check_mutation(1, |tree| {
        tree.taffy.remove(tree.items[3]).unwrap();
    });
}

#[test]
fn replacing_a_child_invalidates_placement() {
    check_mutation(1, |tree| {
        let child = tree.taffy.new_leaf(placed_item(2, 3)).unwrap();
        tree.taffy.replace_child_at_index(tree.grid, 1, child).unwrap();
    });
}

#[test]
fn reordering_children_invalidates_placement() {
    check_mutation(1, |tree| {
        let mut children = tree.items.clone();
        children.reverse();
        tree.taffy.set_children(tree.grid, &children).unwrap();
    });
}

#[test]
fn moving_a_child_to_another_grid_invalidates_both_placements() {
    let mut tree = TestTree::new(grid_style(), five_items());
    let other_items: Vec<NodeId> = (0..2).map(|_| tree.taffy.new_leaf(item(15.0)).unwrap()).collect();
    let other_grid = tree.taffy.new_with_children(grid_style(), &other_items).unwrap();
    tree.taffy.add_child(tree.root, other_grid).unwrap();
    tree.layout();

    // `set_children` removes the moved child from its previous parent
    tree.taffy.set_children(other_grid, &[other_items[0], tree.items[0], other_items[1]]).unwrap();

    let before = placement_runs();
    tree.layout();
    assert_placement_runs_since(before, 2);
    let child_x = |tree: &TestTree, node: NodeId| tree.taffy.layout(node).unwrap().location.x;
    assert_eq!(tree.item_locations(), [(0.0, 0.0), (50.0, 0.0), (80.0, 0.0), (0.0, 10.0)]);
    assert_eq!(child_x(&tree, other_items[0]), 0.0);
    assert_eq!(child_x(&tree, tree.items[0]), 15.0);
    assert_eq!(child_x(&tree, other_items[1]), 25.0);
}

/// The number of `auto-fill` repetitions depends on the size of the container, so a cached placement
/// can only be reused if the container's size gives the same number of repetitions.
#[test]
fn auto_fill_repetition_count_change_reruns_placement() {
    let auto_fill_style = |width: f32| Style {
        display: Display::Grid,
        size: Size { width: length(width), height: auto() },
        grid_template_columns: vec![repeat(RepetitionCount::AutoFill, vec![length(50.0)])],
        ..Default::default()
    };
    let mut tree = TestTree::new(auto_fill_style(150.0), five_items());
    tree.layout_and_check();
    assert_eq!(tree.item_locations(), [(0.0, 0.0), (50.0, 0.0), (100.0, 0.0), (0.0, 10.0), (50.0, 10.0)]);

    tree.taffy.set_style(tree.grid, auto_fill_style(100.0)).unwrap();
    tree.layout_and_check();
    assert_eq!(tree.item_locations(), [(0.0, 0.0), (50.0, 0.0), (0.0, 10.0), (50.0, 10.0), (0.0, 20.0)]);
}

/// Like the above, but the container's size changes as a result of a change to the size of its parent
/// (which does not change any style of the container or its items).
#[test]
fn auto_fill_repetition_count_change_due_to_ancestor_reruns_placement() {
    let auto_fill_style = Style {
        display: Display::Grid,
        size: Size { width: percent(1.0), height: auto() },
        grid_template_columns: vec![repeat(RepetitionCount::AutoFill, vec![length(50.0)])],
        ..Default::default()
    };
    let mut taffy: TaffyTree = TaffyTree::new();
    let items: Vec<NodeId> = five_items().into_iter().map(|style| taffy.new_leaf(style).unwrap()).collect();
    let grid = taffy.new_with_children(auto_fill_style, &items).unwrap();
    let root_style = |width: f32| Style { size: Size { width: length(width), height: auto() }, ..Default::default() };
    let root = taffy.new_with_children(root_style(150.0), &[grid]).unwrap();
    let locations = |taffy: &TaffyTree| -> Vec<(f32, f32)> {
        items.iter().map(|item| taffy.layout(*item).unwrap().location).map(|loc| (loc.x, loc.y)).collect()
    };

    taffy.compute_layout(root, Size::MAX_CONTENT).unwrap();
    assert_eq!(locations(&taffy), [(0.0, 0.0), (50.0, 0.0), (100.0, 0.0), (0.0, 10.0), (50.0, 10.0)]);

    taffy.set_style(root, root_style(100.0)).unwrap();
    let before = placement_runs();
    taffy.compute_layout(root, Size::MAX_CONTENT).unwrap();
    assert_placement_runs_since(before, 1);
    assert_eq!(locations(&taffy), [(0.0, 0.0), (50.0, 0.0), (0.0, 10.0), (50.0, 10.0), (0.0, 20.0)]);

    // The same number of repetitions fit in a slightly wider container
    taffy.set_style(root, root_style(120.0)).unwrap();
    let before = placement_runs();
    taffy.compute_layout(root, Size::MAX_CONTENT).unwrap();
    assert_placement_runs_since(before, 0);
    assert_eq!(locations(&taffy), [(0.0, 0.0), (50.0, 0.0), (0.0, 10.0), (50.0, 10.0), (0.0, 20.0)]);
}

/// Empty `auto-fit` tracks are collapsed. Which tracks are empty is determined from the occupancy matrix when
/// placement is run, and must be determined from the cached placement when it is not.
#[test]
fn auto_fit_tracks_are_collapsed_when_placement_is_cached() {
    let auto_fit_style = Style {
        display: Display::Grid,
        size: Size { width: length(250.0), height: auto() },
        grid_template_columns: vec![repeat(RepetitionCount::AutoFit, vec![length(50.0)])],
        gap: Size { width: length(10.0), height: zero() },
        ..Default::default()
    };
    // Columns 1, 3 and 4 are empty
    let mut tree = TestTree::new(auto_fit_style, vec![placed_item(2, 1), placed_item(2, 2)]);
    tree.layout_and_check();
    assert_eq!(tree.item_locations(), [(0.0, 0.0), (0.0, 10.0)]);

    // Resize an item, so that the grid is laid out again using the cached placement
    tree.taffy.set_style(tree.items[1], Style { size: Size::from_lengths(30.0, 30.0), ..placed_item(2, 2) }).unwrap();
    let before = placement_runs();
    tree.layout();
    assert_placement_runs_since(before, 0);
    tree.layout_and_check();
    assert_eq!(tree.item_locations(), [(0.0, 0.0), (0.0, 10.0)]);
    assert_eq!(tree.taffy.layout(tree.items[1]).unwrap().size, Size { width: 30.0, height: 30.0 });
}
