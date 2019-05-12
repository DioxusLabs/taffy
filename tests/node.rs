#[cfg(test)]
mod node {
    use stretch::geometry::*;
    use stretch::node::Stretch;
    use stretch::style::*;

    #[test]
    fn children() {
        let mut stretch = Stretch::new();
        let child1 = stretch.new_node(Style::default(), vec![]).unwrap();
        let child2 = stretch.new_node(Style::default(), vec![]).unwrap();
        let node = stretch.new_node(Style::default(), vec![child1, child2]).unwrap();

        assert_eq!(stretch.child_count(node).unwrap(), 2);
        assert_eq!(stretch.children(node).unwrap()[0], child1);
        assert_eq!(stretch.children(node).unwrap()[1], child2);
    }

    #[test]
    fn set_measure() {
        let mut stretch = Stretch::new();
        let node = stretch.new_leaf(Style::default(), Box::new(|_| Ok(Size { width: 200.0, height: 200.0 })));
        stretch.compute_layout(node, Size::undefined()).unwrap();
        assert_eq!(stretch.layout(node).unwrap().size.width, 200.0);

        stretch.set_measure(node, Some(Box::new(|_| Ok(Size { width: 100.0, height: 100.0 })))).unwrap();
        stretch.compute_layout(node, Size::undefined()).unwrap();
        assert_eq!(stretch.layout(node).unwrap().size.width, 100.0);
    }

    #[test]
    fn add_child() {
        let mut stretch = Stretch::new();
        let node = stretch.new_node(Style::default(), vec![]).unwrap();
        assert_eq!(stretch.child_count(node).unwrap(), 0);

        let child1 = stretch.new_node(Style::default(), vec![]).unwrap();
        stretch.add_child(node, child1).unwrap();
        assert_eq!(stretch.child_count(node).unwrap(), 1);

        let child2 = stretch.new_node(Style::default(), vec![]).unwrap();
        stretch.add_child(node, child2);
        assert_eq!(stretch.child_count(node).unwrap(), 2);
    }

    #[test]
    fn remove_child() {
        let mut stretch = Stretch::new();

        let child1 = stretch.new_node(Style::default(), vec![]).unwrap();
        let child2 = stretch.new_node(Style::default(), vec![]).unwrap();

        let node = stretch.new_node(Style::default(), vec![child1, child2]).unwrap();
        assert_eq!(stretch.child_count(node).unwrap(), 2);

        stretch.remove_child(node, child1);
        assert_eq!(stretch.child_count(node).unwrap(), 1);
        assert_eq!(stretch.children(node).unwrap()[0], child2);

        stretch.remove_child(node, child2);
        assert_eq!(stretch.child_count(node).unwrap(), 0);
    }

    #[test]
    fn remove_child_at_index() {
        let mut stretch = Stretch::new();

        let child1 = stretch.new_node(Style::default(), vec![]).unwrap();
        let child2 = stretch.new_node(Style::default(), vec![]).unwrap();

        let node = stretch.new_node(Style::default(), vec![child1, child2]).unwrap();
        assert_eq!(stretch.child_count(node).unwrap(), 2);

        stretch.remove_child_at_index(node, 0).unwrap();
        assert_eq!(stretch.child_count(node).unwrap(), 1);
        assert_eq!(stretch.children(node).unwrap()[0], child2);

        stretch.remove_child_at_index(node, 0).unwrap();
        assert_eq!(stretch.child_count(node).unwrap(), 0);
    }

    #[test]
    fn replace_child_at_index() {
        let mut stretch = Stretch::new();

        let child1 = stretch.new_node(Style::default(), vec![]).unwrap();
        let child2 = stretch.new_node(Style::default(), vec![]).unwrap();

        let node = stretch.new_node(Style::default(), vec![child1]).unwrap();
        assert_eq!(stretch.child_count(node).unwrap(), 1);
        assert_eq!(stretch.children(node).unwrap()[0], child1);

        stretch.replace_child_at_index(node, 0, child2).unwrap();
        assert_eq!(stretch.child_count(node).unwrap(), 1);
        assert_eq!(stretch.children(node).unwrap()[0], child2);
    }

    #[test]
    fn set_children() {
        let mut stretch = Stretch::new();

        let child1 = stretch.new_node(Style::default(), vec![]).unwrap();
        let child2 = stretch.new_node(Style::default(), vec![]).unwrap();
        let node = stretch.new_node(Style::default(), vec![child1, child2]).unwrap();

        assert_eq!(stretch.child_count(node).unwrap(), 2);
        assert_eq!(stretch.children(node).unwrap()[0], child1);
        assert_eq!(stretch.children(node).unwrap()[1], child2);

        let child3 = stretch.new_node(Style::default(), vec![]).unwrap();
        let child4 = stretch.new_node(Style::default(), vec![]).unwrap();
        stretch.set_children(node, vec![child3, child4]);

        assert_eq!(stretch.child_count(node).unwrap(), 2);
        assert_eq!(stretch.children(node).unwrap()[0], child3);
        assert_eq!(stretch.children(node).unwrap()[1], child4);
    }

    #[test]
    fn set_style() {
        let mut stretch = Stretch::new();

        let node = stretch.new_node(Style::default(), vec![]).unwrap();
        assert_eq!(stretch.style(node).unwrap().display, Display::Flex);

        stretch.set_style(node, Style { display: Display::None, ..Style::default() }).unwrap();
        assert_eq!(stretch.style(node).unwrap().display, Display::None);
    }

    #[test]
    fn mark_dirty() {
        let mut stretch = Stretch::new();

        let child1 = stretch.new_node(Style::default(), vec![]).unwrap();
        let child2 = stretch.new_node(Style::default(), vec![]).unwrap();
        let node = stretch.new_node(Style::default(), vec![child1, child2]).unwrap();

        stretch.compute_layout(node, stretch::geometry::Size::undefined()).unwrap();

        assert_eq!(stretch.dirty(child1).unwrap(), false);
        assert_eq!(stretch.dirty(child2).unwrap(), false);
        assert_eq!(stretch.dirty(node).unwrap(), false);

        stretch.mark_dirty(node);
        assert_eq!(stretch.dirty(child1).unwrap(), false);
        assert_eq!(stretch.dirty(child2).unwrap(), false);
        assert_eq!(stretch.dirty(node).unwrap(), true);

        stretch.compute_layout(node, stretch::geometry::Size::undefined()).unwrap();
        stretch.mark_dirty(child1);
        assert_eq!(stretch.dirty(child1).unwrap(), true);
        assert_eq!(stretch.dirty(child2).unwrap(), false);
        assert_eq!(stretch.dirty(node).unwrap(), true);
    }
}
