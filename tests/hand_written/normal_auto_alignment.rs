//! Tests for the `normal` and `auto` alignment keywords.
//!
//! `auto` (the default for `align-self` / `justify-self`) defers to the parent's
//! `align-items` / `justify-items`. `normal` (the default for `align-items` / `justify-items`)
//! is the default alignment of the layout mode. `auto` behaves as `normal` when it is set on
//! `align-items` / `justify-items`.

use taffy::geometry::Point;
use taffy::prelude::*;
use taffy_test_helpers::new_test_tree;

/// Lay out a 100x100 container with the given display mode and `*-items` alignment containing
/// an in-flow child and an absolutely positioned child (each with the given `*-self` alignment
/// and a height but no width), and return the layouts of the two children.
fn layout_children(display: Display, items: AlignItems, self_: AlignSelf) -> [(Point<f32>, Size<f32>); 2] {
    let mut taffy = new_test_tree();
    let child_style = Style {
        size: Size { width: auto(), height: length(20.0) },
        min_size: Size { width: length(10.0), height: auto() },
        align_self: self_,
        justify_self: self_,
        ..Default::default()
    };
    let in_flow = taffy.new_leaf(child_style.clone()).unwrap();
    let abspos = taffy
        .new_leaf(Style {
            position: Position::Absolute,
            inset: Rect { left: length(10.0), right: length(10.0), top: length(10.0), bottom: length(10.0) },
            ..child_style
        })
        .unwrap();
    let root = taffy
        .new_with_children(
            Style {
                display,
                size: Size { width: length(100.0), height: length(100.0) },
                align_items: items,
                justify_items: items,
                ..Default::default()
            },
            &[in_flow, abspos],
        )
        .unwrap();
    taffy.compute_layout(root, Size::MAX_CONTENT).unwrap();
    [in_flow, abspos].map(|node| {
        let layout = taffy.layout(node).unwrap();
        (layout.location, layout.size)
    })
}

fn check_normal_auto(display: Display) {
    // The default values are `normal` for `*-items` and `auto` for `*-self`
    let default_style: Style = Style::default();
    assert_eq!(default_style.align_items, AlignItems::NORMAL);
    assert_eq!(default_style.justify_items, JustifyItems::NORMAL);
    assert_eq!(default_style.align_self, AlignSelf::AUTO);
    assert_eq!(default_style.justify_self, JustifySelf::AUTO);
    let default = layout_children(display, AlignItems::NORMAL, AlignSelf::AUTO);

    // `auto` on `*-items` behaves as `normal`
    assert_eq!(layout_children(display, AlignItems::AUTO, AlignSelf::AUTO), default);
    // An explicit `normal` on `*-self` is the same as deferring to `normal` on `*-items`...
    assert_eq!(layout_children(display, AlignItems::NORMAL, AlignSelf::NORMAL), default);
    assert_eq!(layout_children(display, AlignItems::AUTO, AlignSelf::NORMAL), default);
    // ...and overrides the parent's `*-items`
    assert_eq!(layout_children(display, AlignItems::CENTER, AlignSelf::NORMAL), default);
    assert_eq!(layout_children(display, AlignItems::END, AlignSelf::NORMAL), default);

    // `auto` on `*-self` defers to the parent's `*-items` for in-flow children
    let [in_flow_end, _] = layout_children(display, AlignItems::NORMAL, AlignSelf::END);
    let [in_flow_deferred, abspos_deferred] = layout_children(display, AlignItems::END, AlignSelf::AUTO);
    assert_eq!(in_flow_deferred, in_flow_end);
    assert_ne!(in_flow_deferred, default[0]);
    // ...but behaves as `normal` for absolutely positioned children with non-auto insets
    assert_eq!(abspos_deferred, default[1]);
    let [_, abspos_end] = layout_children(display, AlignItems::NORMAL, AlignSelf::END);
    assert_ne!(abspos_end, default[1]);
}

#[cfg(feature = "flexbox")]
#[test]
fn flex_normal_auto_alignment() {
    check_normal_auto(Display::Flex);
    // `normal` behaves as `stretch` for flex items
    assert_eq!(
        layout_children(Display::Flex, AlignItems::NORMAL, AlignSelf::AUTO)[0],
        layout_children(Display::Flex, AlignItems::STRETCH, AlignSelf::AUTO)[0],
    );
}

#[cfg(feature = "grid")]
#[test]
fn grid_normal_auto_alignment() {
    check_normal_auto(Display::Grid);
    // `normal` behaves as `stretch` for grid items without a preferred size
    let [(location, size), _] = layout_children(Display::Grid, AlignItems::NORMAL, AlignSelf::AUTO);
    assert_eq!(location, Point { x: 0.0, y: 0.0 });
    assert_eq!(size, Size { width: 100.0, height: 20.0 });
}

#[cfg(feature = "block_layout")]
#[test]
fn block_normal_auto_alignment() {
    check_normal_auto(Display::Block);
    // `normal` behaves as `stretch` for block-level boxes
    let [(location, size), _] = layout_children(Display::Block, AlignItems::NORMAL, AlignSelf::AUTO);
    assert_eq!(location, Point { x: 0.0, y: 0.0 });
    assert_eq!(size, Size { width: 100.0, height: 20.0 });
}
