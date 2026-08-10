//! Tests for [`crate::LayoutOutput::depends_on_block_size`]
//!
//! These live inside the crate because reading a `LayoutOutput` (rather than the committed
//! `Layout`) requires running a `ComputeSize` pass over the tree directly.

use crate::geometry::{Line, Size};
use crate::style::{Dimension, Display, Style};
use crate::style_helpers::TaffyMaxContent;
use crate::tree::{LayoutInput, LayoutPartialTree, RequestedAxis, RunMode, SizingMode};
use crate::{NodeId, TaffyTree};

/// Run a width-only `ComputeSize` pass over `node` and return its `depends_on_block_size`
fn measure_depends_on_block_size(taffy: &mut TaffyTree<()>, node: NodeId) -> bool {
    let mut tree = taffy.as_layout_tree();
    tree.compute_child_layout(
        node,
        LayoutInput {
            known_dimensions: Size::NONE,
            parent_size: Size::NONE,
            available_space: Size::MAX_CONTENT,
            sizing_mode: SizingMode::InherentSize,
            axis: RequestedAxis::Horizontal,
            run_mode: RunMode::ComputeSize,
            vertical_margins_are_collapsible: Line::FALSE,
        },
    )
    .depends_on_block_size
}

fn style(display: Display) -> Style {
    Style { display, ..Default::default() }
}

#[test]
fn plain_leaf_does_not_depend_on_block_size() {
    let mut taffy: TaffyTree<()> = TaffyTree::new();
    let leaf = taffy.new_leaf(Style::DEFAULT).unwrap();
    assert!(!measure_depends_on_block_size(&mut taffy, leaf));
}

#[test]
fn leaf_with_aspect_ratio_depends_on_block_size() {
    let mut taffy: TaffyTree<()> = TaffyTree::new();
    let leaf = taffy.new_leaf(Style { aspect_ratio: Some(2.0), ..Default::default() }).unwrap();
    assert!(measure_depends_on_block_size(&mut taffy, leaf));
}

#[test]
#[cfg(feature = "flexbox")]
fn flexbox_container_reports_descendant_aspect_ratio() {
    let mut taffy: TaffyTree<()> = TaffyTree::new();

    let plain = taffy.new_leaf(Style::DEFAULT).unwrap();
    let plain_parent = taffy.new_with_children(style(Display::Flex), &[plain]).unwrap();
    assert!(!measure_depends_on_block_size(&mut taffy, plain_parent));

    let ratio = taffy.new_leaf(Style { aspect_ratio: Some(2.0), ..Default::default() }).unwrap();
    let inner = taffy.new_with_children(style(Display::Flex), &[ratio]).unwrap();
    let outer = taffy.new_with_children(style(Display::Flex), &[inner]).unwrap();
    assert!(measure_depends_on_block_size(&mut taffy, outer));
}

#[test]
#[cfg(feature = "flexbox")]
fn wrapping_column_flexbox_depends_on_block_size() {
    use crate::style::{FlexDirection, FlexWrap};

    let mut taffy: TaffyTree<()> = TaffyTree::new();
    let leaf = taffy
        .new_leaf(Style {
            size: Size { width: Dimension::length(10.0), height: Dimension::length(10.0) },
            ..Default::default()
        })
        .unwrap();
    let container = taffy
        .new_with_children(
            Style {
                display: Display::Flex,
                flex_direction: FlexDirection::Column,
                flex_wrap: FlexWrap::Wrap,
                ..Default::default()
            },
            &[leaf],
        )
        .unwrap();

    assert!(measure_depends_on_block_size(&mut taffy, container));
}

#[test]
#[cfg(feature = "block_layout")]
fn block_container_reports_descendant_aspect_ratio() {
    let mut taffy: TaffyTree<()> = TaffyTree::new();

    let plain = taffy.new_leaf(Style::DEFAULT).unwrap();
    let plain_parent = taffy.new_with_children(style(Display::Block), &[plain]).unwrap();
    assert!(!measure_depends_on_block_size(&mut taffy, plain_parent));

    let ratio = taffy.new_leaf(Style { aspect_ratio: Some(2.0), ..Default::default() }).unwrap();
    let inner = taffy.new_with_children(style(Display::Block), &[ratio]).unwrap();
    let outer = taffy.new_with_children(style(Display::Block), &[inner]).unwrap();
    assert!(measure_depends_on_block_size(&mut taffy, outer));
}

#[test]
#[cfg(feature = "grid")]
fn grid_container_reports_descendant_aspect_ratio() {
    let mut taffy: TaffyTree<()> = TaffyTree::new();

    let plain = taffy.new_leaf(Style::DEFAULT).unwrap();
    let plain_parent = taffy.new_with_children(style(Display::Grid), &[plain]).unwrap();
    assert!(!measure_depends_on_block_size(&mut taffy, plain_parent));

    let ratio = taffy.new_leaf(Style { aspect_ratio: Some(2.0), ..Default::default() }).unwrap();
    let parent = taffy.new_with_children(style(Display::Grid), &[ratio]).unwrap();
    assert!(measure_depends_on_block_size(&mut taffy, parent));
}

#[test]
#[cfg(feature = "flexbox")]
fn flag_survives_the_cache_and_is_invalidated_with_it() {
    let mut taffy: TaffyTree<()> = TaffyTree::new();
    let leaf = taffy.new_leaf(Style::DEFAULT).unwrap();
    let parent = taffy.new_with_children(style(Display::Flex), &[leaf]).unwrap();

    assert!(!measure_depends_on_block_size(&mut taffy, parent));
    // Second run is served from the cache
    assert!(!measure_depends_on_block_size(&mut taffy, parent));

    taffy.set_style(leaf, Style { aspect_ratio: Some(2.0), ..Default::default() }).unwrap();
    assert!(measure_depends_on_block_size(&mut taffy, parent));
}

#[test]
fn measure_function_can_declare_independence() {
    use crate::compute::compute_leaf_layout;
    use crate::tree::LayoutOutput;

    let mut taffy: TaffyTree<bool> = TaffyTree::new();
    let leaf = taffy.new_leaf_with_context(Style::DEFAULT, true).unwrap();

    let mut tree = taffy.as_layout_tree_with_measure(
        |inputs: LayoutInput, _node, context: Option<&mut bool>, style: &Style| -> LayoutOutput {
            let independent = context.map(|c| *c).unwrap_or(false);
            compute_leaf_layout(inputs, style, |_, _| 0.0, |_, _| Size { width: 40.0, height: 20.0 })
                .with_depends_on_block_size(!independent)
        },
    );

    let output = tree.compute_child_layout(
        leaf,
        LayoutInput {
            known_dimensions: Size::NONE,
            parent_size: Size::NONE,
            available_space: Size::MAX_CONTENT,
            sizing_mode: SizingMode::InherentSize,
            axis: RequestedAxis::Horizontal,
            run_mode: RunMode::ComputeSize,
            vertical_margins_are_collapsible: Line::FALSE,
        },
    );

    assert!(!output.depends_on_block_size);
    assert_eq!(output.size.width, 40.0);
}
