#[cfg(test)]
mod node {
    use stretch::geometry::*;
    use stretch::node::{MeasureFunc, Stretch};
    use stretch::style::*;

    #[test]
    fn children() {
        let mut stretch = Stretch::new();
        let child1 = stretch.new_node(Style::default(), &[]).unwrap();
        let child2 = stretch.new_node(Style::default(), &[]).unwrap();
        let node = stretch.new_node(Style::default(), &[child1, child2]).unwrap();

        assert_eq!(stretch.child_count(node).unwrap(), 2);
        assert_eq!(stretch.children(node).unwrap()[0], child1);
        assert_eq!(stretch.children(node).unwrap()[1], child2);
    }

    #[test]
    fn set_measure() {
        let mut stretch = Stretch::new();
        let node =
            stretch.new_leaf(Style::default(), MeasureFunc::Raw(|_| Size { width: 200.0, height: 200.0 })).unwrap();
        stretch.compute_layout(node, Size::undefined()).unwrap();
        assert_eq!(stretch.layout(node).unwrap().size.width, 200.0);

        stretch.set_measure(node, Some(MeasureFunc::Raw(|_| Size { width: 100.0, height: 100.0 }))).unwrap();
        stretch.compute_layout(node, Size::undefined()).unwrap();
        assert_eq!(stretch.layout(node).unwrap().size.width, 100.0);
    }

    #[test]
    fn add_child() {
        let mut stretch = Stretch::new();
        let node = stretch.new_node(Style::default(), &[]).unwrap();
        assert_eq!(stretch.child_count(node).unwrap(), 0);

        let child1 = stretch.new_node(Style::default(), &[]).unwrap();
        stretch.add_child(node, child1).unwrap();
        assert_eq!(stretch.child_count(node).unwrap(), 1);

        let child2 = stretch.new_node(Style::default(), &[]).unwrap();
        stretch.add_child(node, child2).unwrap();
        assert_eq!(stretch.child_count(node).unwrap(), 2);
    }

    #[test]
    fn remove_child() {
        let mut stretch = Stretch::new();

        let child1 = stretch.new_node(Style::default(), &[]).unwrap();
        let child2 = stretch.new_node(Style::default(), &[]).unwrap();

        let node = stretch.new_node(Style::default(), &[child1, child2]).unwrap();
        assert_eq!(stretch.child_count(node).unwrap(), 2);

        stretch.remove_child(node, child1).unwrap();
        assert_eq!(stretch.child_count(node).unwrap(), 1);
        assert_eq!(stretch.children(node).unwrap()[0], child2);

        stretch.remove_child(node, child2).unwrap();
        assert_eq!(stretch.child_count(node).unwrap(), 0);
    }

    #[test]
    fn remove_child_at_index() {
        let mut stretch = Stretch::new();

        let child1 = stretch.new_node(Style::default(), &[]).unwrap();
        let child2 = stretch.new_node(Style::default(), &[]).unwrap();

        let node = stretch.new_node(Style::default(), &[child1, child2]).unwrap();
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

        let child1 = stretch.new_node(Style::default(), &[]).unwrap();
        let child2 = stretch.new_node(Style::default(), &[]).unwrap();

        let node = stretch.new_node(Style::default(), &[child1]).unwrap();
        assert_eq!(stretch.child_count(node).unwrap(), 1);
        assert_eq!(stretch.children(node).unwrap()[0], child1);

        stretch.replace_child_at_index(node, 0, child2).unwrap();
        assert_eq!(stretch.child_count(node).unwrap(), 1);
        assert_eq!(stretch.children(node).unwrap()[0], child2);
    }

    #[test]
    fn remove() {
        let mut stretch = Stretch::new();

        let style2 = Style { flex_direction: FlexDirection::Column, ..Style::default() };

        // Build a linear tree layout: <0> <- <1> <- <2>
        let node2 = stretch.new_node(style2, &[]).unwrap();
        let node1 = stretch.new_node(Style::default(), &[node2]).unwrap();
        let node0 = stretch.new_node(Style::default(), &[node1]).unwrap();

        assert_eq!(stretch.children(node0).unwrap().as_slice(), &[node1]);

        // Disconnect the tree: <0> <2>
        stretch.remove(node1);

        assert!(stretch.style(node1).is_err());

        assert!(stretch.children(node0).unwrap().is_empty());
        assert!(stretch.children(node2).unwrap().is_empty());
        assert_eq!(stretch.style(node2).unwrap().flex_direction, style2.flex_direction);
    }

    #[test]
    fn set_children() {
        let mut stretch = Stretch::new();

        let child1 = stretch.new_node(Style::default(), &[]).unwrap();
        let child2 = stretch.new_node(Style::default(), &[]).unwrap();
        let node = stretch.new_node(Style::default(), &[child1, child2]).unwrap();

        assert_eq!(stretch.child_count(node).unwrap(), 2);
        assert_eq!(stretch.children(node).unwrap()[0], child1);
        assert_eq!(stretch.children(node).unwrap()[1], child2);

        let child3 = stretch.new_node(Style::default(), &[]).unwrap();
        let child4 = stretch.new_node(Style::default(), &[]).unwrap();
        stretch.set_children(node, &[child3, child4]).unwrap();

        assert_eq!(stretch.child_count(node).unwrap(), 2);
        assert_eq!(stretch.children(node).unwrap()[0], child3);
        assert_eq!(stretch.children(node).unwrap()[1], child4);
    }

    #[test]
    fn set_style() {
        let mut stretch = Stretch::new();

        let node = stretch.new_node(Style::default(), &[]).unwrap();
        assert_eq!(stretch.style(node).unwrap().display, Display::Flex);

        stretch.set_style(node, Style { display: Display::None, ..Style::default() }).unwrap();
        assert_eq!(stretch.style(node).unwrap().display, Display::None);
    }

    #[test]
    fn mark_dirty() {
        let mut stretch = Stretch::new();

        let child1 = stretch.new_node(Style::default(), &[]).unwrap();
        let child2 = stretch.new_node(Style::default(), &[]).unwrap();
        let node = stretch.new_node(Style::default(), &[child1, child2]).unwrap();

        stretch.compute_layout(node, stretch::geometry::Size::undefined()).unwrap();

        assert_eq!(stretch.dirty(child1).unwrap(), false);
        assert_eq!(stretch.dirty(child2).unwrap(), false);
        assert_eq!(stretch.dirty(node).unwrap(), false);

        stretch.mark_dirty(node).unwrap();
        assert_eq!(stretch.dirty(child1).unwrap(), false);
        assert_eq!(stretch.dirty(child2).unwrap(), false);
        assert_eq!(stretch.dirty(node).unwrap(), true);

        stretch.compute_layout(node, stretch::geometry::Size::undefined()).unwrap();
        stretch.mark_dirty(child1).unwrap();
        assert_eq!(stretch.dirty(child1).unwrap(), true);
        assert_eq!(stretch.dirty(child2).unwrap(), false);
        assert_eq!(stretch.dirty(node).unwrap(), true);
    }

    #[test]
    fn remove_last_node() {
        let mut stretch = Stretch::new();

        let parent = stretch.new_node(Style::default(), &[]).unwrap();
        let child = stretch.new_node(Style::default(), &[]).unwrap();
        stretch.add_child(parent, child).unwrap();

        stretch.remove(child);
        stretch.remove(parent);
    }
}
