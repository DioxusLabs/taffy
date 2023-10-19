mod common {
    pub mod image;
    pub mod text;
}
use common::image::{image_measure_function, ImageContext};
use common::text::{text_measure_function, FontMetrics, TextContext, WritingMode, LOREM_IPSUM};
use taffy::tree::{Cache, PartialLayoutTree};
use taffy::util::print_tree;
use taffy::{
    compute_flexbox_layout, compute_grid_layout, compute_layout, compute_leaf_layout, prelude::*, round_layout,
};

#[derive(Debug, Copy, Clone)]
#[allow(dead_code)]
enum NodeKind {
    Flexbox,
    Grid,
    Text,
    Image,
}

struct Node {
    kind: NodeKind,
    style: Style,
    text_data: Option<TextContext>,
    image_data: Option<ImageContext>,
    cache: Cache,
    unrounded_layout: Layout,
    final_layout: Layout,
    children: Vec<usize>,
}

impl Default for Node {
    fn default() -> Self {
        Node {
            kind: NodeKind::Flexbox,
            style: Style::default(),
            text_data: None,
            image_data: None,
            cache: Cache::new(),
            unrounded_layout: Layout::with_order(0),
            final_layout: Layout::with_order(0),
            children: Vec::new(),
        }
    }
}

#[allow(dead_code)]
impl Node {
    pub fn new_row(style: Style) -> Node {
        Node {
            kind: NodeKind::Flexbox,
            style: Style { display: Display::Flex, flex_direction: FlexDirection::Row, ..style },
            ..Node::default()
        }
    }
    pub fn new_column(style: Style) -> Node {
        Node {
            kind: NodeKind::Flexbox,
            style: Style { display: Display::Flex, flex_direction: FlexDirection::Column, ..style },
            ..Node::default()
        }
    }
    pub fn new_grid(style: Style) -> Node {
        Node { kind: NodeKind::Grid, style: Style { display: Display::Grid, ..style }, ..Node::default() }
    }
    pub fn new_text(style: Style, text_data: TextContext) -> Node {
        Node { kind: NodeKind::Text, style, text_data: Some(text_data), ..Node::default() }
    }
    pub fn new_image(style: Style, image_data: ImageContext) -> Node {
        Node { kind: NodeKind::Image, style, image_data: Some(image_data), ..Node::default() }
    }
}

struct Tree {
    nodes: Vec<Node>,
}

impl Tree {
    pub fn new() -> Tree {
        Tree { nodes: Vec::new() }
    }

    pub fn add_node(&mut self, node: Node) -> usize {
        self.nodes.push(node);
        self.nodes.len() - 1
    }

    pub fn append_child(&mut self, parent: usize, child: usize) {
        self.nodes[parent].children.push(child);
    }

    #[inline(always)]
    fn node_from_id(&self, node_id: NodeId) -> &Node {
        &self.nodes[usize::from(node_id)]
    }

    #[inline(always)]
    fn node_from_id_mut(&mut self, node_id: NodeId) -> &mut Node {
        &mut self.nodes[usize::from(node_id)]
    }

    pub fn compute_layout(&mut self, root: usize, available_space: Size<AvailableSpace>, use_rounding: bool) {
        compute_layout(self, NodeId::from(root), available_space);
        if use_rounding {
            round_layout(self, NodeId::from(root))
        }
    }

    pub fn print_tree(&mut self, root: usize) {
        print_tree(self, NodeId::from(root));
    }
}

struct ChildIter<'a>(std::slice::Iter<'a, usize>);
impl<'a> Iterator for ChildIter<'a> {
    type Item = NodeId;
    fn next(&mut self) -> Option<Self::Item> {
        self.0.next().copied().map(NodeId::from)
    }
}

impl PartialLayoutTree for Tree {
    type ChildIter<'a> = ChildIter<'a>;

    fn child_ids(&self, node_id: NodeId) -> Self::ChildIter<'_> {
        ChildIter(self.node_from_id(node_id).children.iter())
    }

    fn child_count(&self, node_id: NodeId) -> usize {
        self.node_from_id(node_id).children.len()
    }

    fn get_child_id(&self, node_id: NodeId, index: usize) -> NodeId {
        NodeId::from(self.node_from_id(node_id).children[index])
    }

    fn get_style(&self, node_id: NodeId) -> &Style {
        &self.node_from_id(node_id).style
    }

    fn unrounded_layout_mut(&mut self, node_id: NodeId) -> &mut Layout {
        &mut self.node_from_id_mut(node_id).unrounded_layout
    }

    fn final_layout_mut(&mut self, node_id: NodeId) -> &mut Layout {
        &mut self.node_from_id_mut(node_id).final_layout
    }

    fn cache_mut(&mut self, node_id: NodeId) -> &mut Cache {
        &mut self.node_from_id_mut(node_id).cache
    }

    fn compute_child_layout(&mut self, node_id: NodeId, inputs: taffy::tree::LayoutInput) -> taffy::tree::LayoutOutput {
        let node = self.node_from_id_mut(node_id);
        let font_metrics = FontMetrics { char_width: 10.0, char_height: 10.0 };

        match node.kind {
            NodeKind::Flexbox => compute_flexbox_layout(self, node_id, inputs),
            NodeKind::Grid => compute_grid_layout(self, node_id, inputs),
            NodeKind::Text => compute_leaf_layout(
                inputs,
                &node.style,
                Some(|known_dimensions, available_space| {
                    text_measure_function(
                        known_dimensions,
                        available_space,
                        node.text_data.as_ref().unwrap(),
                        &font_metrics,
                    )
                }),
            ),
            NodeKind::Image => compute_leaf_layout(
                inputs,
                &node.style,
                Some(|known_dimensions, _available_space| {
                    image_measure_function(known_dimensions, node.image_data.as_ref().unwrap())
                }),
            ),
        }
    }
}

impl LayoutTree for Tree {
    fn final_layout(&self, node_id: NodeId) -> &Layout {
        &self.node_from_id(node_id).final_layout
    }
}

fn main() -> Result<(), taffy::TaffyError> {
    let mut tree = Tree::new();

    let root = Node::new_column(Style::DEFAULT);
    let root_id = tree.add_node(root);

    let text_node = Node::new_text(
        Style::default(),
        TextContext { text_content: LOREM_IPSUM.into(), writing_mode: WritingMode::Horizontal },
    );
    let text_node_id = tree.add_node(text_node);
    tree.append_child(root_id, text_node_id);

    let image_node = Node::new_image(Style::default(), ImageContext { width: 400.0, height: 300.0 });
    let image_node_id = tree.add_node(image_node);
    tree.append_child(root_id, image_node_id);

    // Compute layout and print result
    tree.compute_layout(root_id, Size::MAX_CONTENT, true);
    tree.print_tree(root_id);

    Ok(())
}
