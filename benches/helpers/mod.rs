#[cfg(feature = "yoga_benchmark")]
pub mod yoga_helpers;

/// A helper function to recursively construct a deep tree
#[allow(dead_code)]
pub fn build_deep_tree<T, N>(
    tree: &mut T,
    max_nodes: u32,
    branching_factor: u32,
    create_leaf_node: &mut impl FnMut(&mut T) -> N,
    create_container_node: &mut impl FnMut(&mut T, Vec<N>) -> N,
) -> Vec<N> {
    if max_nodes <= branching_factor {
        // Build leaf nodes
        return (0..max_nodes).map(|_| create_leaf_node(tree)).collect();
    }

    // Add another layer to the tree
    // Each child gets an equal amount of the remaining nodes
    (0..branching_factor)
        .map(|_| {
            let max_nodes = (max_nodes - branching_factor) / branching_factor;
            let sub_children =
                build_deep_tree(tree, max_nodes, branching_factor, create_leaf_node, create_container_node);
            create_container_node(tree, sub_children)
        })
        .collect()
}

/// A helper function to construct a tree with a fixed number of children per level
/* The tree would look something like (for per_level_child_count = 3):
N, N, N
      |
      N, N, N
            |
            N, N, N
                   ...
*/
#[allow(dead_code)]
pub fn build_linear_tree_with_n_children_per_level<T, N: Copy>(
    tree: &mut T,
    max_nodes: u32,
    per_level_child_count: u32,
    create_node: &mut impl FnMut(&mut T) -> N,
    append_child: &mut impl FnMut(&mut T, N, N),
) -> N {
    let mut node_count = max_nodes as i32;
    let root = create_node(tree);
    let mut parent = root;

    while node_count > 0 {
        let mut next_parent = parent;
        for _ in 0..per_level_child_count {
            let child = create_node(tree);
            append_child(tree, parent, child);
            node_count -= 1;
            next_parent = child;
        }
        parent = next_parent;
    }

    root
}
