//! Contains the print_tree function for printing a debug representation of the tree
use crate::tree::{NodeId, PrintTree};

/// Prints a debug representation of the computed layout for a tree of nodes, starting with the passed root node.
pub fn print_tree(tree: &impl PrintTree, root: NodeId) {
    println!("TREE");
    print_node(tree, root, false, String::new());

    /// Recursive function that prints each node in the tree
    fn print_node(tree: &impl PrintTree, node_id: NodeId, has_sibling: bool, lines_string: String) {
        let layout = &tree.get_final_layout(node_id);
        let display = tree.get_debug_label(node_id);
        let num_children = tree.child_count(node_id);

        let fork_string = if has_sibling { "├── " } else { "└── " };
        #[cfg(feature = "content_size")]
        println!(
            "{lines}{fork} {display} [x: {x:<4} y: {y:<4} w: {width:<4} h: {height:<4} scroll_w: {scroll_width:<4} scroll_h: {scroll_height:<4} border: l:{bl} r:{br} t:{bt} b:{bb}, padding: l:{pl} r:{pr} t:{pt} b:{pb} scrollbar: w:{sbw} h:{sbh} scrollable_overflow: x:{sol}..{sor} y: {sot}..{sob}] ({key:?})",
            lines = lines_string,
            fork = fork_string,
            display = display,
            x = layout.location.x,
            y = layout.location.y,
            width = layout.size.width,
            height = layout.size.height,
            scroll_width = layout.scroll_width(),
            scroll_height = layout.scroll_height(),
            bl = layout.border.left,
            br = layout.border.right,
            bt = layout.border.top,
            bb = layout.border.bottom,
            pl = layout.padding.left,
            pr = layout.padding.right,
            pt = layout.padding.top,
            pb = layout.padding.bottom,
            sbw = layout.scrollbar_size.width,
            sbh = layout.scrollbar_size.height,
            sol = layout.scrollable_overflow.left,
            sor = layout.scrollable_overflow.right,
            sot = layout.scrollable_overflow.top,
            sob = layout.scrollable_overflow.bottom,
            key = node_id,
        );
        #[cfg(not(feature = "content_size"))]
        println!(
            "{lines}{fork} {display} [x: {x:<4} y: {y:<4} width: {width:<4} height: {height:<4}] ({key:?})",
            lines = lines_string,
            fork = fork_string,
            display = display,
            x = layout.location.x,
            y = layout.location.y,
            width = layout.size.width,
            height = layout.size.height,
            key = node_id,
        );
        let bar = if has_sibling { "│   " } else { "    " };
        let new_string = lines_string + bar;

        // Recurse into children
        for (index, child) in tree.child_ids(node_id).enumerate() {
            let has_sibling = index < num_children - 1;
            print_node(tree, child, has_sibling, new_string.clone());
        }
    }
}
