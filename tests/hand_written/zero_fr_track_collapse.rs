//! A `0fr` track holding an item with an explicit zero minimum collapses to
//! zero under every sizing mode, as it does in browsers. This is what makes
//! the `grid-template-rows: 0fr` to `1fr` collapse animation pattern work:
//! the track's auto minimum takes the item's `min-height: 0` rather than
//! being inflated to the item's content size while the container is being
//! content-sized.

use taffy::prelude::*;
use taffy_test_helpers::new_test_tree;

fn collapse_container(template: Vec<GridTemplateComponent<String>>) -> f32 {
    let mut taffy = new_test_tree();
    let content =
        taffy.new_leaf(Style { size: Size { width: auto(), height: length(100.0) }, ..Default::default() }).unwrap();
    let item = taffy
        .new_with_children(
            Style { min_size: Size { width: auto(), height: length(0.0) }, ..Default::default() },
            &[content],
        )
        .unwrap();
    let root = taffy
        .new_with_children(
            Style { display: Display::Grid, grid_template_rows: template, ..Default::default() },
            &[item],
        )
        .unwrap();

    taffy
        .compute_layout(root, Size { width: AvailableSpace::Definite(200.0), height: AvailableSpace::MaxContent })
        .unwrap();
    taffy.layout(root).unwrap().size.height
}

#[test]
fn a_zero_fr_track_collapses_to_zero() {
    assert_eq!(collapse_container(vec![fr(0.0)]), 0.0);
}

#[test]
fn a_fractional_fr_track_takes_its_share_of_the_content() {
    // matches Chrome: fr size = contribution / max(factor sum, 1)
    assert_eq!(collapse_container(vec![fr(0.5)]), 50.0);
}

#[test]
fn a_two_fr_track_caps_at_the_content_size() {
    assert_eq!(collapse_container(vec![fr(2.0)]), 100.0);
}

#[test]
fn a_one_fr_track_keeps_its_content_size() {
    assert_eq!(collapse_container(vec![fr(1.0)]), 100.0);
}

#[test]
fn an_auto_track_keeps_its_content_size() {
    assert_eq!(collapse_container(vec![auto()]), 100.0);
}
