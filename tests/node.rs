#[cfg(test)]
mod node {
    use stretch::geometry::*;
    use stretch::node::Node;
    use stretch::style::*;

    #[test]
    fn children() {
        let child1 = Node::new(Style::default(), vec![]);
        let child2 = Node::new(Style::default(), vec![]);
        let node = Node::new(Style::default(), vec![&child1, &child2]);

        assert_eq!(node.child_count(), 2);
        assert_eq!(node.children()[0], child1);
        assert_eq!(node.children()[1], child2);
    }

    #[test]
    fn set_measure() {
        let mut node = Node::new_leaf(Style::default(), Box::new(|_| Ok(Size { width: 200.0, height: 200.0 })));
        assert_eq!(node.compute_layout(Size::undefined()).unwrap().size.width, 200.0);

        node.set_measure(Some(Box::new(|_| Ok(Size { width: 100.0, height: 100.0 }))));
        assert_eq!(node.compute_layout(Size::undefined()).unwrap().size.width, 100.0);
    }

    #[test]
    fn add_child() {
        let mut node = Node::new(Style::default(), vec![]);
        assert_eq!(node.child_count(), 0);

        node.add_child(&Node::new(Style::default(), vec![]));
        assert_eq!(node.child_count(), 1);

        node.add_child(&Node::new(Style::default(), vec![]));
        assert_eq!(node.child_count(), 2);
    }

    #[test]
    fn remove_child() {
        let child1 = Node::new(Style::default(), vec![]);
        let child2 = Node::new(Style::default(), vec![]);

        let mut node = Node::new(Style::default(), vec![&child1, &child2]);
        assert_eq!(node.child_count(), 2);

        node.remove_child(&child1);
        assert_eq!(node.child_count(), 1);
        assert_eq!(node.children()[0], child2);

        node.remove_child(&child2);
        assert_eq!(node.child_count(), 0);
    }

    #[test]
    fn remove_child_at_index() {
        let child1 = Node::new(Style::default(), vec![]);
        let child2 = Node::new(Style::default(), vec![]);

        let mut node = Node::new(Style::default(), vec![&child1, &child2]);
        assert_eq!(node.child_count(), 2);

        node.remove_child_at_index(0);
        assert_eq!(node.child_count(), 1);
        assert_eq!(node.children()[0], child2);

        node.remove_child_at_index(0);
        assert_eq!(node.child_count(), 0);
    }

    #[test]
    fn replace_child_at_index() {
        let child1 = Node::new(Style::default(), vec![]);
        let child2 = Node::new(Style::default(), vec![]);

        let mut node = Node::new(Style::default(), vec![&child1]);
        assert_eq!(node.child_count(), 1);
        assert_eq!(node.children()[0], child1);

        node.replace_child_at_index(0, &child2);
        assert_eq!(node.child_count(), 1);
        assert_eq!(node.children()[0], child2);
    }

    #[test]
    fn set_children() {
        let child1 = Node::new(Style::default(), vec![]);
        let child2 = Node::new(Style::default(), vec![]);
        let mut node = Node::new(Style::default(), vec![&child1, &child2]);

        assert_eq!(node.child_count(), 2);
        assert_eq!(node.children()[0], child1);
        assert_eq!(node.children()[1], child2);

        let child3 = Node::new(Style::default(), vec![]);
        let child4 = Node::new(Style::default(), vec![]);
        node.set_children(vec![&child3, &child4]);

        assert_eq!(node.child_count(), 2);
        assert_eq!(node.children()[0], child3);
        assert_eq!(node.children()[1], child4);
    }

    #[test]
    fn set_style() {
        let mut node = Node::new(Style::default(), vec![]);
        assert_eq!(node.style().display, Display::Flex);

        node.set_style(Style { display: Display::None, ..Style::default() });
        assert_eq!(node.style().display, Display::None);
    }

    #[test]
    fn mark_dirty() {
        let mut child1 = Node::new(Style::default(), vec![]);
        let child2 = Node::new(Style::default(), vec![]);
        let mut node = Node::new(Style::default(), vec![&child1, &child2]);

        node.compute_layout(stretch::geometry::Size::undefined()).unwrap();

        assert_eq!(child1.dirty(), false);
        assert_eq!(child2.dirty(), false);
        assert_eq!(node.dirty(), false);

        node.mark_dirty();
        assert_eq!(child1.dirty(), false);
        assert_eq!(child2.dirty(), false);
        assert_eq!(node.dirty(), true);

        node.compute_layout(stretch::geometry::Size::undefined()).unwrap();
        child1.mark_dirty();
        assert_eq!(child1.dirty(), true);
        assert_eq!(child2.dirty(), false);
        assert_eq!(node.dirty(), true);
    }
}
