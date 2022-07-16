use crate::{
    layout::{Cache, Layout},
    prelude::*,
};

pub trait LayoutTree {
    fn children(&self, node: Node) -> &[Node];

    fn child(&self, node: Node, id: usize) -> Node;

    fn parent(&self, node: Node) -> Option<Node>;

    fn style(&self, node: Node) -> &FlexboxLayout;

    fn layout_mut(&mut self, node: Node) -> &mut Layout;

    fn mark_dirty(&mut self, node: Node);

    fn measure_node(&self, node: Node) -> Option<Size<f32>>;

    /// Node needs to be measured
    fn needs_measure(&self, node: Node) -> bool;

    fn primary_cache(&mut self, node: Node) -> &mut Option<Cache>;

    fn secondary_cache(&mut self, node: Node) -> &mut Option<Cache>;
}

impl LayoutTree for Taffy {
    fn children(&self, node: Node) -> &[Node] {
        todo!()
    }

    fn parent(&self, node: Node) -> Option<Node> {
        todo!()
    }

    fn style(&self, node: Node) -> &FlexboxLayout {
        todo!()
    }

    fn layout_mut(&mut self, node: Node) -> &mut Layout {
        todo!()
    }

    fn mark_dirty(&mut self, node: Node) {
        todo!()
    }

    fn measure_node(&self, node: Node) -> Option<Size<f32>> {
        todo!()
    }

    fn needs_measure(&self, node: Node) -> bool {
        todo!()
    }

    fn primary_cache(&mut self, node: Node) -> &mut Option<Cache> {
        todo!()
    }

    fn secondary_cache(&mut self, node: Node) -> &mut Option<Cache> {
        todo!()
    }

    fn child(&self, node: Node, id: usize) -> Node {
        todo!()
    }
}

/*


// re-layout the tree
// The "mark dirty" function is called internally by the layout algorithm.
// To intelligently re-render only dirty nodes, add functionaly to mark_dirty
let adjusted_nodes = tree.layout();




impl<'a> LayoutTree for DioxusStyleView<'a> {
    fn get_children(&self, node: Node) -> &[Node] {
        self.get_children(node)
    }


    fn mark_dirty() {

    }
}

*/
