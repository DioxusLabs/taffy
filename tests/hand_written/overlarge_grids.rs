//! Tests for grids which exceed the limits described at
//! <https://www.w3.org/TR/css-grid-1/#overlarge-grids>
//!
//! Test cases are derived from Servo bug reports:
//! - <https://github.com/servo/servo/issues/45949>
//! - <https://github.com/servo/servo/issues/45939>
//! - <https://github.com/servo/servo/issues/45938>
//! - <https://github.com/servo/servo/issues/46081>
#![cfg(feature = "grid")]

use taffy::prelude::*;
use taffy::style::{GridAutoFlow, RepetitionCount};

/// <https://github.com/servo/servo/issues/45949>
///
/// `grid-template: 90px / repeat(32768, fit-content(512px)) fit-content(100%)`
///
/// An explicit track count greater than `i16::MAX` previously caused a subtract with overflow in
/// `OriginZeroLine::implied_positive_implicit_tracks`.
#[test]
fn explicit_grid_huge_repeat_count() {
    let mut tree: TaffyTree<()> = TaffyTree::new();
    let child = tree.new_leaf(Style::default()).unwrap();
    let node = tree
        .new_with_children(
            Style {
                display: Display::Grid,
                grid_template_rows: vec![length(90.0)],
                grid_template_columns: vec![repeat(32768, vec![fit_content(length(512.0))]), fit_content(percent(1.0))],
                ..Default::default()
            },
            &[child],
        )
        .unwrap();
    tree.compute_layout(node, Size::MAX_CONTENT).unwrap();
}

/// A repeat count and track count whose product overflows `u16`
#[test]
fn explicit_grid_repeat_count_track_count_product_overflow() {
    let mut tree: TaffyTree<()> = TaffyTree::new();
    let child = tree.new_leaf(Style::default()).unwrap();
    let node = tree
        .new_with_children(
            Style {
                display: Display::Grid,
                grid_template_columns: vec![repeat(40000, vec![length(10.0), length(10.0)])],
                ..Default::default()
            },
            &[child],
        )
        .unwrap();
    tree.compute_layout(node, Size::MAX_CONTENT).unwrap();
}

/// <https://github.com/servo/servo/issues/46081>
///
/// `grid-template-columns: repeat(auto-fill, 0em)` with a definite container size.
///
/// Zero-sized tracks previously resulted in an infinite number of repetitions, which caused
/// an add with overflow when computing the repetition count.
#[test]
fn auto_fill_zero_sized_tracks() {
    let mut tree: TaffyTree<()> = TaffyTree::new();
    let child = tree.new_leaf(Style::default()).unwrap();
    let node = tree
        .new_with_children(
            Style {
                display: Display::Grid,
                size: Size { width: length(200.0), height: auto() },
                grid_template_columns: vec![repeat(RepetitionCount::AutoFill, vec![length(0.0)])],
                ..Default::default()
            },
            &[child],
        )
        .unwrap();
    tree.compute_layout(node, Size::MAX_CONTENT).unwrap();
}

/// Tiny (but non-zero) sized tracks in a large container can also produce more repetitions than
/// fit in the 10000 track limit
#[test]
fn auto_fill_tiny_tracks_huge_container() {
    let mut tree: TaffyTree<()> = TaffyTree::new();
    let child = tree.new_leaf(Style::default()).unwrap();
    let node = tree
        .new_with_children(
            Style {
                display: Display::Grid,
                size: Size { width: length(100_000.0), height: auto() },
                grid_template_columns: vec![repeat(RepetitionCount::AutoFill, vec![length(1.0)])],
                ..Default::default()
            },
            &[child],
        )
        .unwrap();
    tree.compute_layout(node, Size::MAX_CONTENT).unwrap();
}

/// <https://github.com/servo/servo/issues/45939>
///
/// A large negative `grid-row-start` (as produced by Servo wrapping `45983` to `i16`) combined
/// with `grid-auto-flow: column` and auto-placed items with definite column positions previously
/// caused a panic in `OriginZeroLine::into_track_vec_index`.
#[test]
fn auto_flow_column_with_large_row_start() {
    let mut tree: TaffyTree<()> = TaffyTree::new();
    let column_start_child = Style { grid_column: Line { start: line(7), end: auto() }, ..Default::default() };
    let row_start_child = Style { grid_row: Line { start: line(-19553), end: auto() }, ..Default::default() };
    let children = [
        tree.new_leaf(column_start_child.clone()).unwrap(),
        tree.new_leaf(column_start_child.clone()).unwrap(),
        tree.new_leaf(column_start_child.clone()).unwrap(),
        tree.new_leaf(row_start_child).unwrap(),
    ];
    let node = tree
        .new_with_children(
            Style { display: Display::Grid, grid_auto_flow: GridAutoFlow::Column, ..Default::default() },
            &children,
        )
        .unwrap();
    tree.compute_layout(node, Size::MAX_CONTENT).unwrap();
}

/// <https://github.com/servo/servo/issues/45938>
///
/// As above, but with explicit grid columns and items with both a large row-start and
/// a definite column position.
#[test]
fn auto_flow_column_with_explicit_columns_and_large_row_start() {
    let mut tree: TaffyTree<()> = TaffyTree::new();
    let column_start = Line { start: line(7), end: auto() };
    let child1 = Style { grid_column: column_start.clone(), ..Default::default() };
    let child2 = Style {
        grid_column: column_start.clone(),
        grid_row: Line { start: line(-19553), end: auto() },
        ..Default::default()
    };
    let children = [
        tree.new_leaf(child1.clone()).unwrap(),
        tree.new_leaf(child1.clone()).unwrap(),
        tree.new_leaf(child1.clone()).unwrap(),
        tree.new_leaf(child2).unwrap(),
    ];
    let node = tree
        .new_with_children(
            Style {
                display: Display::Grid,
                grid_auto_flow: GridAutoFlow::Column,
                grid_template_rows: vec![auto()],
                grid_template_columns: vec![
                    fit_content(length(80.0)),
                    fit_content(length(80.0)),
                    minmax(max_content(), auto()),
                ],
                ..Default::default()
            },
            &children,
        )
        .unwrap();
    tree.compute_layout(node, Size::MAX_CONTENT).unwrap();
}

/// Definitely placed items with extreme positive and negative line numbers
#[test]
fn definite_placement_with_extreme_line_numbers() {
    let mut tree: TaffyTree<()> = TaffyTree::new();
    let children = [
        tree.new_leaf(Style {
            grid_column: Line { start: line(32767), end: auto() },
            grid_row: Line { start: line(-32768), end: auto() },
            ..Default::default()
        })
        .unwrap(),
        tree.new_leaf(Style {
            grid_column: Line { start: span(10000), end: line(-32768) },
            grid_row: Line { start: line(32767), end: span(10000) },
            ..Default::default()
        })
        .unwrap(),
    ];
    let node = tree.new_with_children(Style { display: Display::Grid, ..Default::default() }, &children).unwrap();
    tree.compute_layout(node, Size::MAX_CONTENT).unwrap();
}
