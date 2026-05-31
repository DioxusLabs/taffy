//! Tests for `align-content` on Block layout containers (taffy issue #709).
//!
//! Spec: <https://www.w3.org/TR/css-align-3/#align-justify-content>
//!
//! Block layout aligns its in-flow children stack along the block axis (vertical in
//! horizontal-tb writing mode) when `align-content` is set and the container has
//! free space beyond the inflow content height.

#![cfg(feature = "block_layout")]

use taffy::prelude::*;
use taffy_test_helpers::{new_test_tree, TestNodeContext};

fn three_children_in_block_container(align_content: Option<AlignContent>) -> (TaffyTree<TestNodeContext>, [NodeId; 3]) {
    let mut taffy = new_test_tree();
    let child_style = || Style { size: Size { width: length(50.0), height: length(20.0) }, ..Default::default() };
    let c0 = taffy.new_leaf(child_style()).unwrap();
    let c1 = taffy.new_leaf(child_style()).unwrap();
    let c2 = taffy.new_leaf(child_style()).unwrap();
    let root = taffy
        .new_with_children(
            Style {
                display: Display::Block,
                size: Size { width: length(100.0), height: length(120.0) },
                align_content,
                ..Default::default()
            },
            &[c0, c1, c2],
        )
        .unwrap();
    taffy.compute_layout(root, Size::MAX_CONTENT).unwrap();
    (taffy, [c0, c1, c2])
}

// V12: align_content == None → behavior identical to pre-change (children stacked from top).
#[test]
fn block_align_content_none_stacks_from_top() {
    let (taffy, [c0, c1, c2]) = three_children_in_block_container(None);
    assert_eq!(taffy.layout(c0).unwrap().location.y, 0.0);
    assert_eq!(taffy.layout(c1).unwrap().location.y, 20.0);
    assert_eq!(taffy.layout(c2).unwrap().location.y, 40.0);
}

// V1: Start + free_space ≥ 0 → identical to None (no regression).
#[test]
fn block_align_content_start_matches_default() {
    let (taffy, [c0, c1, c2]) = three_children_in_block_container(Some(AlignContent::Start));
    assert_eq!(taffy.layout(c0).unwrap().location.y, 0.0);
    assert_eq!(taffy.layout(c1).unwrap().location.y, 20.0);
    assert_eq!(taffy.layout(c2).unwrap().location.y, 40.0);
}

// V2: End → first child shifted by free_space (60), all children pushed to bottom.
#[test]
fn block_align_content_end_shifts_to_bottom() {
    let (taffy, [c0, c1, c2]) = three_children_in_block_container(Some(AlignContent::End));
    // free_space = 120 - 60 = 60.
    assert_eq!(taffy.layout(c0).unwrap().location.y, 60.0);
    assert_eq!(taffy.layout(c1).unwrap().location.y, 80.0);
    assert_eq!(taffy.layout(c2).unwrap().location.y, 100.0);
}

// V3: Center → first child shifted by free_space / 2 (30).
#[test]
fn block_align_content_center_shifts_to_middle() {
    let (taffy, [c0, c1, c2]) = three_children_in_block_container(Some(AlignContent::Center));
    assert_eq!(taffy.layout(c0).unwrap().location.y, 30.0);
    assert_eq!(taffy.layout(c1).unwrap().location.y, 50.0);
    assert_eq!(taffy.layout(c2).unwrap().location.y, 70.0);
}

// V4: SpaceBetween → first at top, last at bottom, gap = free_space / (n - 1) = 30.
#[test]
fn block_align_content_space_between_distributes_gap() {
    let (taffy, [c0, c1, c2]) = three_children_in_block_container(Some(AlignContent::SpaceBetween));
    assert_eq!(taffy.layout(c0).unwrap().location.y, 0.0);
    assert_eq!(taffy.layout(c1).unwrap().location.y, 50.0); // 20 (own height) + 30 (gap)
    assert_eq!(taffy.layout(c2).unwrap().location.y, 100.0); // 40 + 60 (two gaps)
}

// V5: SpaceAround → first = free_space / (2n) = 10, gap = free_space / n = 20.
#[test]
fn block_align_content_space_around_distributes_half_gaps() {
    let (taffy, [c0, c1, c2]) = three_children_in_block_container(Some(AlignContent::SpaceAround));
    assert_eq!(taffy.layout(c0).unwrap().location.y, 10.0);
    assert_eq!(taffy.layout(c1).unwrap().location.y, 50.0); // 10 + 20 + 20
    assert_eq!(taffy.layout(c2).unwrap().location.y, 90.0); // 50 + 20 + 20
}

// V6: SpaceEvenly → first = free_space / (n + 1) = 15, gap = free_space / (n + 1) = 15.
#[test]
fn block_align_content_space_evenly_distributes_equal_gaps() {
    let (taffy, [c0, c1, c2]) = three_children_in_block_container(Some(AlignContent::SpaceEvenly));
    assert_eq!(taffy.layout(c0).unwrap().location.y, 15.0);
    assert_eq!(taffy.layout(c1).unwrap().location.y, 50.0); // 15 + 20 + 15
    assert_eq!(taffy.layout(c2).unwrap().location.y, 85.0); // 50 + 20 + 15
}

// V7: single child + Stretch → falls back to FlexStart (top).
#[test]
fn block_align_content_stretch_single_child_falls_back_to_start() {
    let mut taffy = new_test_tree();
    let child = taffy
        .new_leaf(Style { size: Size { width: length(50.0), height: length(20.0) }, ..Default::default() })
        .unwrap();
    let root = taffy
        .new_with_children(
            Style {
                display: Display::Block,
                size: Size { width: length(100.0), height: length(120.0) },
                align_content: Some(AlignContent::Stretch),
                ..Default::default()
            },
            &[child],
        )
        .unwrap();
    taffy.compute_layout(root, Size::MAX_CONTENT).unwrap();
    assert_eq!(taffy.layout(child).unwrap().location.y, 0.0);
}

// V8: SafeEnd + overflow (inflow > container) → falls back to Start (y == 0).
#[test]
fn block_align_content_safe_end_overflow_falls_back_to_start() {
    let mut taffy = new_test_tree();
    let c0 = taffy
        .new_leaf(Style { size: Size { width: length(50.0), height: length(80.0) }, ..Default::default() })
        .unwrap();
    let c1 = taffy
        .new_leaf(Style { size: Size { width: length(50.0), height: length(80.0) }, ..Default::default() })
        .unwrap();
    let root = taffy
        .new_with_children(
            Style {
                display: Display::Block,
                size: Size { width: length(100.0), height: length(100.0) },
                align_content: Some(AlignContent::SafeEnd),
                ..Default::default()
            },
            &[c0, c1],
        )
        .unwrap();
    taffy.compute_layout(root, Size::MAX_CONTENT).unwrap();
    // Inflow (160) overflows container (100). Safe fallback → Start: first child at 0.
    assert_eq!(taffy.layout(c0).unwrap().location.y, 0.0);
    assert_eq!(taffy.layout(c1).unwrap().location.y, 80.0);
}

// V9: absolutely positioned sibling is not shifted by align-content.
#[test]
fn block_align_content_does_not_shift_abs_positioned_sibling() {
    let mut taffy = new_test_tree();
    let in_flow = taffy
        .new_leaf(Style { size: Size { width: length(50.0), height: length(20.0) }, ..Default::default() })
        .unwrap();
    let abs = taffy
        .new_leaf(Style {
            size: Size { width: length(30.0), height: length(30.0) },
            position: Position::Absolute,
            inset: Rect { left: length(10.0), right: auto(), top: length(5.0), bottom: auto() },
            ..Default::default()
        })
        .unwrap();
    let root = taffy
        .new_with_children(
            Style {
                display: Display::Block,
                size: Size { width: length(100.0), height: length(120.0) },
                align_content: Some(AlignContent::End),
                ..Default::default()
            },
            &[in_flow, abs],
        )
        .unwrap();
    taffy.compute_layout(root, Size::MAX_CONTENT).unwrap();
    // In-flow shifted by free_space (100).
    assert_eq!(taffy.layout(in_flow).unwrap().location.y, 100.0);
    // Abs anchored by inset, unaffected by align-content.
    assert_eq!(taffy.layout(abs).unwrap().location.y, 5.0);
    assert_eq!(taffy.layout(abs).unwrap().location.x, 10.0);
}
