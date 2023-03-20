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
