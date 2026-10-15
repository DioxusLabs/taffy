//! A port of [Yoga](https://github.com/facebook/yoga)'s flexbox algorithm, for hosts (such as React Native)
//! that need Yoga's exact layout results rather than CSS flexbox.
//!
//! Each file is a port of the same-named Yoga source file at commit `08250431e09bc4b864917e51f9e9d52998ee2dce`
//! and follows it closely, so that upstream changes can be ported diff by diff. Yoga is MIT licensed,
//! Copyright (c) Meta Platforms, Inc. and affiliates; see `LICENSE-YOGA`.

pub(crate) mod algorithm;
pub(crate) mod axis;
pub(crate) mod cache;
mod config;
mod enums;
mod leaf;
mod node_state;
pub(crate) mod numeric;
pub(crate) mod pixel_grid;
mod style;

pub use config::{YogaConfig, YogaExperimentalFeatures};
pub use enums::{YogaErrata, YogaExperimentalFeature, YogaMeasureMode, YogaSizingMode};
pub use leaf::compute_yoga_leaf_layout;
pub use node_state::YogaFlexboxNodeState;
pub use style::{
    default_style, YogaEdges, YogaFlex, YogaGutters, DEFAULT_FLEX_GROW, DEFAULT_FLEX_SHRINK, WEB_DEFAULT_FLEX_SHRINK,
};

/// Compute the layout of a [`Display::YogaFlexbox`](crate::style::Display::YogaFlexbox) container.
///
/// Placeholder until the Yoga algorithm port lands: it currently runs Taffy's native CSS flexbox
/// algorithm so the display mode, per-node state and cache plumbing can be exercised end to end.
pub fn compute_yoga_flexbox_layout(
    tree: &mut impl crate::tree::LayoutYogaFlexboxContainer,
    node: crate::NodeId,
    inputs: crate::tree::LayoutInput,
) -> crate::tree::LayoutOutput {
    crate::compute::compute_flexbox_layout(tree, node, inputs)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::prelude::*;
    use crate::tree::LayoutYogaFlexboxContainer;
    use crate::TaffyTree;

    fn yoga_style() -> Style {
        let mut style = default_style(&YogaConfig::default());
        style.display = Display::YogaFlexbox;
        style
    }

    #[test]
    fn yoga_leaf_is_measured_with_yoga_semantics() {
        let mut tree: TaffyTree<Size<f32>> = TaffyTree::new();
        let leaf = tree
            .new_leaf_with_context(
                Style {
                    padding: Rect::length(5.0),
                    min_size: Size { width: length(30.0), height: auto() },
                    ..yoga_style()
                },
                Size { width: 20.0, height: 10.0 },
            )
            .unwrap();
        let mut root_style = yoga_style();
        root_style.size = Size { width: length(200.0), height: length(100.0) };
        root_style.align_items = AlignItems::FLEX_START;
        let root = tree.new_with_children(root_style, &[leaf]).unwrap();

        tree.compute_layout_with_measure(root, Size::MAX_CONTENT, |inputs, _, context, style| {
            let intrinsic = *context.unwrap();
            crate::compute::compute_leaf_layout(inputs, style, |_, _| 0.0, |_, _| intrinsic)
        })
        .unwrap();

        // Measured 20x10, plus 5px padding on each side, then bound by min-width 30.
        assert_eq!(tree.layout(leaf).unwrap().size, Size { width: 30.0, height: 20.0 });
        assert_eq!(tree.layout(root).unwrap().size, Size { width: 200.0, height: 100.0 });
    }

    #[test]
    fn node_state_is_reset_by_mark_dirty_and_config_change() {
        let mut tree: TaffyTree<()> = TaffyTree::new();
        let leaf = tree.new_leaf(yoga_style()).unwrap();
        let root = tree.new_with_children(yoga_style(), &[leaf]).unwrap();
        tree.compute_layout(root, Size::MAX_CONTENT).unwrap();

        let state = YogaFlexboxNodeState { computed_flex_basis: Some(12.0), had_overflow: true };
        {
            let mut view = tree.as_yoga_layout_tree();
            view.set_yoga_node_state(leaf, state);
            assert_eq!(view.yoga_node_state(leaf), state);
            assert!(!view.has_measure_func(leaf));
        }

        tree.mark_dirty(leaf).unwrap();
        let after_dirty = tree.as_yoga_layout_tree().yoga_node_state(leaf);
        assert_eq!(after_dirty.computed_flex_basis, None);
        assert!(after_dirty.had_overflow);

        tree.compute_layout(root, Size::MAX_CONTENT).unwrap();
        assert!(!tree.dirty(root).unwrap());
        let config = YogaConfig { point_scale_factor: 2.0, ..YogaConfig::default() };
        tree.set_yoga_config(config);
        assert!(tree.dirty(root).unwrap());
        assert_eq!(tree.yoga_config().point_scale_factor, 2.0);
    }
}
