use taffy::prelude::*;
use taffy::Point;
use taffy_test_helpers::new_test_tree;

#[test]
fn rounding_doesnt_leave_gaps() {
    // First create an instance of TaffyTree
    let mut taffy = new_test_tree();

    let w_square = Size { width: length(100.3), height: length(100.3) };
    let child_a = taffy.new_leaf(Style { size: w_square, ..Default::default() }).unwrap();
    let child_b = taffy.new_leaf(Style { size: w_square, ..Default::default() }).unwrap();

    let root_node = taffy
        .new_with_children(
            Style {
                size: Size { width: length(963.3333), height: length(1000.) },
                justify_content: JustifyContent::CENTER,
                ..Default::default()
            },
            &[child_a, child_b],
        )
        .unwrap();

    taffy.compute_layout(root_node, Size::MAX_CONTENT).unwrap();
    taffy.print_tree(root_node);

    let layout_a = taffy.layout(child_a).unwrap();
    let layout_b = taffy.layout(child_b).unwrap();
    assert_eq!(layout_a.location.x + layout_a.size.width, layout_b.location.x);
}

// Keep final layouts separate from hidden layout, as custom low-level trees do.
#[derive(Default)]
struct RoundingNode {
    style: Style,
    children: Vec<NodeId>,
    hoisted_children: Vec<NodeId>,
    unrounded_layout: Layout,
    final_layout: Layout,
    round_count: usize,
}

#[derive(Default)]
struct RoundingTree(Vec<RoundingNode>);

impl RoundingTree {
    fn add_node(&mut self, position: Position, children: &[NodeId]) -> NodeId {
        let node_id = NodeId::from(self.0.len());
        self.0.push(RoundingNode {
            style: Style { position, ..Default::default() },
            children: children.to_vec(),
            unrounded_layout: Layout {
                location: Point { x: 0.4, y: 0.4 },
                size: Size { width: 10.4, height: 10.4 },
                ..Layout::with_order(0)
            },
            ..Default::default()
        });
        node_id
    }
}

impl taffy::TraversePartialTree for RoundingTree {
    type ChildIter<'a> = std::iter::Copied<std::slice::Iter<'a, NodeId>>;

    fn child_ids(&self, node_id: NodeId) -> Self::ChildIter<'_> {
        self.0[usize::from(node_id)].children.iter().copied()
    }

    fn child_count(&self, node_id: NodeId) -> usize {
        self.0[usize::from(node_id)].children.len()
    }

    fn get_child_id(&self, node_id: NodeId, index: usize) -> NodeId {
        self.0[usize::from(node_id)].children[index]
    }
}

impl taffy::TraverseTree for RoundingTree {}

impl taffy::RoundTree for RoundingTree {
    fn get_unrounded_layout(&self, node_id: NodeId) -> Layout {
        self.0[usize::from(node_id)].unrounded_layout
    }

    fn set_final_layout(&mut self, node_id: NodeId, layout: &Layout) {
        let node = &mut self.0[usize::from(node_id)];
        node.final_layout = *layout;
        node.round_count += 1;
    }

    fn is_out_of_flow(&self, node_id: NodeId) -> bool {
        let style = &self.0[usize::from(node_id)].style;
        style.position.is_out_of_flow() && style.display != Display::None
    }

    fn is_hidden(&self, node_id: NodeId) -> bool {
        self.0[usize::from(node_id)].style.display == Display::None
    }

    fn hoisted_child_count(&self, node_id: NodeId) -> usize {
        self.0[usize::from(node_id)].hoisted_children.len()
    }

    fn get_hoisted_child_id(&self, node_id: NodeId, index: usize) -> NodeId {
        self.0[usize::from(node_id)].hoisted_children[index]
    }
}

impl taffy::LayoutPartialTree for RoundingTree {
    type CoreContainerStyle<'a> = &'a Style;
    type CustomIdent = String;

    fn get_core_container_style(&self, node_id: NodeId) -> Self::CoreContainerStyle<'_> {
        &self.0[usize::from(node_id)].style
    }

    fn set_unrounded_layout(&mut self, node_id: NodeId, layout: &Layout) {
        self.0[usize::from(node_id)].unrounded_layout = *layout;
    }

    fn compute_child_layout(&mut self, node_id: NodeId, inputs: taffy::LayoutInput) -> taffy::LayoutOutput {
        assert_eq!(inputs.run_mode, taffy::RunMode::PerformHiddenLayout);
        taffy::compute_hidden_layout(self, node_id)
    }
}

impl taffy::CacheTree for RoundingTree {
    fn cache_get(&mut self, _node_id: NodeId, _inputs: &taffy::LayoutInput) -> Option<taffy::LayoutOutput> {
        None
    }

    fn cache_store(&mut self, _node_id: NodeId, _inputs: &taffy::LayoutInput, _output: taffy::LayoutOutput) {}

    fn cache_clear(&mut self, _node_id: NodeId) {}
}

#[test]
fn rounding_visits_out_of_flow_descendants_of_hidden_nodes() {
    for position in [Position::Absolute, Position::Fixed] {
        for hidden_position in [Position::Static, Position::Relative, Position::Absolute] {
            for hidden_is_root in [false, true] {
                let mut tree = RoundingTree::default();
                let leaf = tree.add_node(Position::Static, &[]);
                let nested = tree.add_node(Position::Absolute, &[leaf]);
                let out_of_flow = tree.add_node(position, &[nested]);
                let in_flow = tree.add_node(Position::Static, &[out_of_flow]);
                let hidden = tree.add_node(hidden_position, &[in_flow]);
                let root = if hidden_is_root { hidden } else { tree.add_node(Position::Relative, &[hidden]) };
                // Seed a visible pass with containing-block-relative layouts and hoisted lists.
                tree.0[usize::from(out_of_flow)].hoisted_children = vec![nested];
                if position == Position::Absolute && hidden_position != Position::Static {
                    tree.0[usize::from(hidden)].hoisted_children.push(out_of_flow);
                } else {
                    tree.0[usize::from(root)].hoisted_children.push(out_of_flow);
                }
                if hidden_position == Position::Absolute && !hidden_is_root {
                    tree.0[usize::from(root)].hoisted_children.push(hidden);
                }
                taffy::round_layout(&mut tree, root);
                for node in &mut tree.0 {
                    assert!(node.final_layout.size.width > 0.0);
                    node.round_count = 0;
                }

                tree.0[usize::from(hidden)].style.display = Display::None;
                // The visible containing block no longer hoists boxes from the hidden subtree.
                if !hidden_is_root {
                    tree.0[usize::from(root)].hoisted_children.clear();
                }
                taffy::compute_hidden_layout(&mut tree, hidden);
                for node_id in [hidden, in_flow, out_of_flow, nested, leaf] {
                    let node = &tree.0[usize::from(node_id)];
                    assert_eq!(node.unrounded_layout, Layout::with_order(0));
                    assert!(node.final_layout.size.width > 0.0);
                }

                taffy::round_layout(&mut tree, root);
                for node_id in [hidden, in_flow, out_of_flow, nested, leaf] {
                    let node = &tree.0[usize::from(node_id)];
                    assert_eq!(node.final_layout, Layout::with_order(0), "hidden node {node_id:?}");
                    assert_eq!(node.round_count, 1, "hidden node {node_id:?}");
                }
            }
        }
    }
}

#[test]
fn rounding_visible_out_of_flow_boxes_uses_containing_block_once() {
    for position in [Position::Absolute, Position::Fixed] {
        let mut tree = RoundingTree::default();
        let leaf = tree.add_node(Position::Static, &[]);
        let nested = tree.add_node(Position::Absolute, &[leaf]);
        let out_of_flow = tree.add_node(position, &[nested]);
        let parent = tree.add_node(Position::Static, &[out_of_flow]);
        let root = tree.add_node(Position::Relative, &[parent]);
        tree.0[usize::from(root)].hoisted_children = vec![out_of_flow];
        tree.0[usize::from(out_of_flow)].hoisted_children = vec![nested];

        taffy::round_layout(&mut tree, root);

        // The static parent's 0.4px offset must not contribute to out-of-flow rounding.
        for (node_id, offset, size) in
            [(root, 0.0, 11.0), (parent, 1.0, 10.0), (out_of_flow, 1.0, 10.0), (nested, 0.0, 11.0), (leaf, 1.0, 10.0)]
        {
            let node = &tree.0[usize::from(node_id)];
            let expected = Layout {
                location: Point { x: offset, y: offset },
                size: Size { width: size, height: size },
                ..Layout::with_order(0)
            };
            assert_eq!(node.final_layout, expected, "visible node {node_id:?}");
            assert_eq!(node.round_count, 1, "visible node {node_id:?}");
        }
    }
}
