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
                align_self: taffy::prelude::AlignSelf::CENTER,
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
                position: Position::Relative,
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

/// Lays out the tree that `build` makes with `before`, restyles the node that `build` returns first
/// with `after` and lays the tree out again. Returns the unrounded layout of every node that `build`
/// returns, alongside those of a fresh tree built with `after` directly. `build` returns the root last.
#[cfg(all(feature = "block_layout", feature = "flexbox"))]
fn relayout_and_fresh(
    before: Style,
    after: Style,
    build: fn(&mut TaffyTree<taffy_test_helpers::TestNodeContext>, Style) -> Vec<NodeId>,
) -> (Vec<taffy::Layout>, Vec<taffy::Layout>) {
    let layouts = |tree: &TaffyTree<taffy_test_helpers::TestNodeContext>, ids: &[NodeId]| {
        ids.iter().map(|&id| *tree.unrounded_layout(id)).collect::<Vec<_>>()
    };

    let mut tree = new_test_tree();
    let ids = build(&mut tree, before);
    let root = *ids.last().unwrap();
    tree.compute_layout(root, Size::MAX_CONTENT).unwrap();
    tree.set_style(ids[0], after.clone()).unwrap();
    tree.compute_layout(root, Size::MAX_CONTENT).unwrap();

    let mut fresh = new_test_tree();
    let fresh_ids = build(&mut fresh, after);
    fresh.compute_layout(*fresh_ids.last().unwrap(), Size::MAX_CONTENT).unwrap();

    (layouts(&tree, &ids), layouts(&fresh, &fresh_ids))
}

/// ```html
/// <div style="display: block; width: 100px; height: 200px">
///   <div style="display: block; height: 50px">
///     <div style="display: block; height: 10px; margin-top: 20px"></div>
///   </div>
/// </div>
/// ```
///
/// The outer box then becomes `container`. The inner box's margin collapses through the middle box
/// only while the middle box is in a block formatting context with its parent: as a flex or grid item
/// the middle box establishes an independent formatting context, which contains the margin.
#[cfg(all(feature = "block_layout", feature = "flexbox"))]
fn relayout_block_parent_as(container: Style) {
    let size = Size { width: length(100.0), height: length(200.0) };
    let block = Style { display: Display::Block, size, ..Default::default() };
    let container = Style { size, ..container };

    let (relaid, fresh) = relayout_and_fresh(block, container, |tree, root_style| {
        let inner = tree
            .new_leaf(Style {
                display: Display::Block,
                size: Size { width: auto(), height: length(10.0) },
                margin: Rect { top: length(20.0), ..Rect::zero() },
                ..Default::default()
            })
            .unwrap();
        let middle = tree
            .new_with_children(
                Style {
                    display: Display::Block,
                    size: Size { width: auto(), height: length(50.0) },
                    ..Default::default()
                },
                &[inner],
            )
            .unwrap();
        let root = tree.new_with_children(root_style, &[middle]).unwrap();
        vec![root, middle, inner, root]
    });

    assert_eq!(fresh[1].location.y, 0.0, "the item is at the top of its container");
    assert_eq!(fresh[2].location.y, 20.0, "the item contains its child's margin");
    assert_eq!(relaid, fresh);
}

/// A flex container lays its items out with `SizingMode::ContentSize` and without collapsible
/// margins. The block container it was before used `SizingMode::InherentSize` and collapsible margins.
#[test]
#[cfg(all(feature = "block_layout", feature = "flexbox"))]
fn block_child_does_not_keep_collapsed_margin_as_flex_item() {
    relayout_block_parent_as(Style {
        display: Display::Flex,
        flex_direction: FlexDirection::Column,
        ..Default::default()
    });
}

/// A grid container lays its items out with `SizingMode::InherentSize`, as the block container it
/// was before did: only `vertical_margins_are_collapsible` differs between the two layout inputs.
#[test]
#[cfg(all(feature = "block_layout", feature = "flexbox", feature = "grid"))]
fn block_child_does_not_keep_collapsed_margin_as_grid_item() {
    relayout_block_parent_as(Style { display: Display::Grid, ..Default::default() });
}

/// ```html
/// <div style="display: grid; width: 100px">
///   <div style="display: grid; flex-grow: 1; min-width: 40px; height: 20px"></div>
///   <div style="display: block; flex-grow: 1; height: 20px"></div>
/// </div>
/// ```
///
/// The outer box then becomes `display: flex`. The grid container measured the first item with
/// `SizingMode::InherentSize`, which applies the item's `min-width`. The flex container measures
/// the same item's flex base size with `SizingMode::ContentSize`, which does not: both items
/// have a flex base size of zero and grow equally.
#[test]
#[cfg(all(feature = "block_layout", feature = "flexbox", feature = "grid"))]
fn flex_base_size_is_not_answered_by_an_inherent_size_measurement() {
    let container =
        |display| Style { display, size: Size { width: length(100.0), height: auto() }, ..Default::default() };

    let (relaid, fresh) = relayout_and_fresh(container(Display::Grid), container(Display::Flex), |tree, root_style| {
        let first = tree
            .new_leaf(Style {
                display: Display::Grid,
                flex_grow: 1.0,
                size: Size { width: auto(), height: length(20.0) },
                min_size: Size { width: length(40.0), height: auto() },
                ..Default::default()
            })
            .unwrap();
        let second = tree
            .new_leaf(Style {
                display: Display::Block,
                flex_grow: 1.0,
                size: Size { width: auto(), height: length(20.0) },
                ..Default::default()
            })
            .unwrap();
        let root = tree.new_with_children(root_style, &[first, second]).unwrap();
        vec![root, first, second, root]
    });

    assert_eq!(fresh[1].size.width, 50.0);
    assert_eq!(fresh[2].size.width, 50.0);
    assert_eq!(relaid, fresh);
}

#[test]
#[cfg(all(feature = "block_layout", feature = "flexbox"))]
fn a_percentage_height_is_not_reused_after_its_containing_block_loses_its_height() {
    use taffy::style::{Dimension, Display, FlexDirection};
    // <div style="display:flex;flex-direction:column;width:78px">
    //   <div style="width:78px;height:62px">
    //     <div style="display:flex;width:31px;height:86%"> <div style="height:19px"></div>
    // </div></div></div>, then the middle block's height becomes auto: the item's
    // percentage no longer resolves (CSS 2.1 §10.5) and it is 19px tall, not 53.32px.
    let block = |height| Style {
        display: Display::Block,
        size: Size { width: Dimension::length(78.0), height },
        ..Style::default()
    };
    let (incremental, fresh) =
        relayout_and_fresh(block(Dimension::length(62.0)), block(Dimension::auto()), |tree, middle_style| {
            let leaf = tree
                .new_leaf(Style {
                    display: Display::Block,
                    size: Size { width: Dimension::auto(), height: Dimension::length(19.0) },
                    ..Style::default()
                })
                .unwrap();
            let item = tree
                .new_with_children(
                    Style {
                        display: Display::Flex,
                        size: Size { width: Dimension::length(31.0), height: Dimension::percent(0.86) },
                        ..Style::default()
                    },
                    &[leaf],
                )
                .unwrap();
            let middle = tree.new_with_children(middle_style, &[item]).unwrap();
            let root = tree
                .new_with_children(
                    Style {
                        display: Display::Flex,
                        flex_direction: FlexDirection::Column,
                        size: Size { width: Dimension::length(78.0), height: Dimension::auto() },
                        ..Style::default()
                    },
                    &[middle],
                )
                .unwrap();
            vec![middle, item, leaf, root]
        });
    assert_eq!(fresh[1].size.height, 19.0);
    assert_eq!(incremental, fresh);
}
