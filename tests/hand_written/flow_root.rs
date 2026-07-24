#![cfg(all(feature = "block_layout", feature = "float_layout"))]

use taffy::geometry::{Point, Rect, Size};
use taffy::prelude::*;
use taffy::style::Float;
use taffy_test_helpers::new_test_tree;

#[test]
fn flow_root_does_not_collapse_margins_through_parent() {
    let child_style = Style {
        display: Display::Block,
        size: Size { width: length(40.0), height: length(10.0) },
        margin: Rect { top: length(20.0), bottom: length(30.0), ..Rect::zero() },
        ..Default::default()
    };

    let mut taffy = new_test_tree();
    let plain_child = taffy.new_leaf(child_style.clone()).unwrap();
    let plain_parent = taffy
        .new_with_children(
            Style {
                display: Display::Block,
                size: Size { width: length(100.0), height: auto() },
                ..Default::default()
            },
            &[plain_child],
        )
        .unwrap();

    let flow_child = taffy.new_leaf(child_style).unwrap();
    let flow_parent = taffy
        .new_with_children(
            Style {
                display: Display::FlowRoot,
                size: Size { width: length(100.0), height: auto() },
                ..Default::default()
            },
            &[flow_child],
        )
        .unwrap();
    let root = taffy
        .new_with_children(
            Style {
                display: Display::Block,
                size: Size { width: length(200.0), height: auto() },
                ..Default::default()
            },
            &[plain_parent, flow_parent],
        )
        .unwrap();
    taffy.compute_layout(root, Size::MAX_CONTENT).unwrap();

    assert_eq!(taffy.layout(plain_parent).unwrap().size.height, 10.0);
    assert_eq!(taffy.layout(flow_parent).unwrap().size.height, 60.0);
    assert_eq!(taffy.layout(flow_child).unwrap().location.y, 20.0);
}

#[test]
fn flow_root_contains_floated_children() {
    let mut taffy = new_test_tree();
    let float = taffy
        .new_leaf(Style {
            display: Display::Block,
            float: Float::Left,
            size: Size { width: length(20.0), height: length(30.0) },
            ..Default::default()
        })
        .unwrap();
    let flow_root = taffy
        .new_with_children(
            Style {
                display: Display::FlowRoot,
                size: Size { width: length(100.0), height: auto() },
                ..Default::default()
            },
            &[float],
        )
        .unwrap();
    taffy.compute_layout(flow_root, Size::MAX_CONTENT).unwrap();

    assert_eq!(taffy.layout(flow_root).unwrap().size.height, 30.0);
}

#[test]
fn flow_root_sibling_avoids_float() {
    let mut taffy = new_test_tree();
    let float = taffy
        .new_leaf(Style {
            display: Display::Block,
            float: Float::Left,
            size: Size { width: length(30.0), height: length(20.0) },
            ..Default::default()
        })
        .unwrap();
    let flow_child =
        taffy.new_leaf(Style { size: Size { width: auto(), height: length(10.0) }, ..Default::default() }).unwrap();
    let flow_root = taffy
        .new_with_children(
            Style { display: Display::FlowRoot, size: Size { width: auto(), height: auto() }, ..Default::default() },
            &[flow_child],
        )
        .unwrap();
    let parent = taffy
        .new_with_children(
            Style {
                display: Display::Block,
                size: Size { width: length(100.0), height: auto() },
                ..Default::default()
            },
            &[float, flow_root],
        )
        .unwrap();
    taffy.compute_layout(parent, Size::MAX_CONTENT).unwrap();

    assert_eq!(taffy.layout(flow_root).unwrap().location, Point { x: 30.0, y: 0.0 });
    assert_eq!(taffy.layout(flow_root).unwrap().size.width, 70.0);
}
