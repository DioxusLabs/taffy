//! The layout algorithms themselves

pub(crate) mod common;
pub(crate) mod leaf;

#[cfg(feature = "block_layout")]
pub(crate) mod block;

#[cfg(feature = "flexbox")]
pub(crate) mod flexbox;

#[cfg(feature = "grid")]
pub(crate) mod grid;

pub use leaf::compute_leaf_layout;

#[cfg(feature = "block_layout")]
pub use self::block::compute_block_layout;

#[cfg(feature = "flexbox")]
pub use self::flexbox::compute_flexbox_layout;

#[cfg(feature = "grid")]
pub use self::grid::compute_grid_layout;

#[cfg(feature = "taffy_tree")]
pub(crate) mod taffy_tree;

use crate::tree::{Layout, LayoutOutput, LayoutTree, NodeId};

/// Creates a layout for this node and its children, recursively.
/// Each hidden node has zero size and is placed at the origin
pub fn compute_hidden_layout(tree: &mut impl LayoutTree, node: NodeId) -> LayoutOutput {
    /// Recursive function to apply hidden layout to all descendents
    fn perform_hidden_layout_inner(tree: &mut impl LayoutTree, node: NodeId, order: u32) {
        *tree.layout_mut(node) = Layout::with_order(order);
        for order in 0..tree.child_count(node) {
            perform_hidden_layout_inner(tree, tree.child(node, order), order as _);
        }
    }

    for order in 0..tree.child_count(node) {
        perform_hidden_layout_inner(tree, tree.child(node, order), order as _);
    }

    LayoutOutput::HIDDEN
}

#[cfg(test)]
mod tests {
    use super::compute_hidden_layout;
    use crate::geometry::{Point, Size};
    use crate::style::{Display, Style};
    use crate::Taffy;

    #[test]
    fn hidden_layout_should_hide_recursively() {
        let mut taffy: Taffy<()> = Taffy::new();

        let style: Style = Style { display: Display::Flex, size: Size::from_lengths(50.0, 50.0), ..Default::default() };

        let grandchild_00 = taffy.new_leaf(style.clone()).unwrap();
        let grandchild_01 = taffy.new_leaf(style.clone()).unwrap();
        let child_00 = taffy.new_with_children(style.clone(), &[grandchild_00, grandchild_01]).unwrap();

        let grandchild_02 = taffy.new_leaf(style.clone()).unwrap();
        let child_01 = taffy.new_with_children(style.clone(), &[grandchild_02]).unwrap();

        let root = taffy
            .new_with_children(
                Style { display: Display::None, size: Size::from_lengths(50.0, 50.0), ..Default::default() },
                &[child_00, child_01],
            )
            .unwrap();

        compute_hidden_layout(&mut taffy.as_layout_tree(), root.into());

        // Whatever size and display-mode the nodes had previously,
        // all layouts should resolve to ZERO due to the root's DISPLAY::NONE
        for (node, _) in taffy.nodes.iter().filter(|(node, _)| *node != root.into()) {
            if let Ok(layout) = taffy.layout(node.into()) {
                assert_eq!(layout.size, Size::zero());
                assert_eq!(layout.location, Point::zero());
            }
        }
    }
}
