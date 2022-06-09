#[cfg(test)]
mod node {
    use sprawl::geometry::*;
    use sprawl::node::{MeasureFunc, Sprawl};
    use sprawl::style::*;

    #[test]
    fn children() {
        let mut sprawl = Sprawl::new();
        let child1 = sprawl.new_node(Style::default(), &[]).unwrap();
        let child2 = sprawl.new_node(Style::default(), &[]).unwrap();
        let node = sprawl.new_node(Style::default(), &[child1, child2]).unwrap();

        assert_eq!(sprawl.child_count(node).unwrap(), 2);
        assert_eq!(sprawl.children(node).unwrap()[0], child1);
        assert_eq!(sprawl.children(node).unwrap()[1], child2);
    }

    #[test]
    fn set_measure() {
        let mut sprawl = Sprawl::new();
        let node =
            sprawl.new_leaf(Style::default(), MeasureFunc::Raw(|_| Size { width: 200.0, height: 200.0 })).unwrap();
        sprawl.compute_layout(node, Size::undefined()).unwrap();
        assert_eq!(sprawl.layout(node).unwrap().size.width, 200.0);

        sprawl.set_measure(node, Some(MeasureFunc::Raw(|_| Size { width: 100.0, height: 100.0 }))).unwrap();
        sprawl.compute_layout(node, Size::undefined()).unwrap();
        assert_eq!(sprawl.layout(node).unwrap().size.width, 100.0);
    }

    #[test]
    fn add_child() {
        let mut sprawl = Sprawl::new();
        let node = sprawl.new_node(Style::default(), &[]).unwrap();
        assert_eq!(sprawl.child_count(node).unwrap(), 0);

        let child1 = sprawl.new_node(Style::default(), &[]).unwrap();
        sprawl.add_child(node, child1).unwrap();
        assert_eq!(sprawl.child_count(node).unwrap(), 1);

        let child2 = sprawl.new_node(Style::default(), &[]).unwrap();
        sprawl.add_child(node, child2).unwrap();
        assert_eq!(sprawl.child_count(node).unwrap(), 2);
    }

    #[test]
    fn remove_child() {
        let mut sprawl = Sprawl::new();

        let child1 = sprawl.new_node(Style::default(), &[]).unwrap();
        let child2 = sprawl.new_node(Style::default(), &[]).unwrap();

        let node = sprawl.new_node(Style::default(), &[child1, child2]).unwrap();
        assert_eq!(sprawl.child_count(node).unwrap(), 2);

        sprawl.remove_child(node, child1).unwrap();
        assert_eq!(sprawl.child_count(node).unwrap(), 1);
        assert_eq!(sprawl.children(node).unwrap()[0], child2);

        sprawl.remove_child(node, child2).unwrap();
        assert_eq!(sprawl.child_count(node).unwrap(), 0);
    }

    #[test]
    fn remove_child_at_index() {
        let mut sprawl = Sprawl::new();

        let child1 = sprawl.new_node(Style::default(), &[]).unwrap();
        let child2 = sprawl.new_node(Style::default(), &[]).unwrap();

        let node = sprawl.new_node(Style::default(), &[child1, child2]).unwrap();
        assert_eq!(sprawl.child_count(node).unwrap(), 2);

        sprawl.remove_child_at_index(node, 0).unwrap();
        assert_eq!(sprawl.child_count(node).unwrap(), 1);
        assert_eq!(sprawl.children(node).unwrap()[0], child2);

        sprawl.remove_child_at_index(node, 0).unwrap();
        assert_eq!(sprawl.child_count(node).unwrap(), 0);
    }

    #[test]
    fn replace_child_at_index() {
        let mut sprawl = Sprawl::new();

        let child1 = sprawl.new_node(Style::default(), &[]).unwrap();
        let child2 = sprawl.new_node(Style::default(), &[]).unwrap();

        let node = sprawl.new_node(Style::default(), &[child1]).unwrap();
        assert_eq!(sprawl.child_count(node).unwrap(), 1);
        assert_eq!(sprawl.children(node).unwrap()[0], child1);

        sprawl.replace_child_at_index(node, 0, child2).unwrap();
        assert_eq!(sprawl.child_count(node).unwrap(), 1);
        assert_eq!(sprawl.children(node).unwrap()[0], child2);
    }

    #[test]
    fn remove() {
        let mut sprawl = Sprawl::new();

        let style2 = Style { flex_direction: FlexDirection::Column, ..Style::default() };

        // Build a linear tree layout: <0> <- <1> <- <2>
        let node2 = sprawl.new_node(style2, &[]).unwrap();
        let node1 = sprawl.new_node(Style::default(), &[node2]).unwrap();
        let node0 = sprawl.new_node(Style::default(), &[node1]).unwrap();

        assert_eq!(sprawl.children(node0).unwrap().as_slice(), &[node1]);

        // Disconnect the tree: <0> <2>
        let _ = sprawl.remove(node1).unwrap();

        assert!(sprawl.style(node1).is_err());

        assert!(sprawl.children(node0).unwrap().is_empty());
        assert!(sprawl.children(node2).unwrap().is_empty());
        assert_eq!(sprawl.style(node2).unwrap().flex_direction, style2.flex_direction);
    }

    #[test]
    fn set_children() {
        let mut sprawl = Sprawl::new();

        let child1 = sprawl.new_node(Style::default(), &[]).unwrap();
        let child2 = sprawl.new_node(Style::default(), &[]).unwrap();
        let node = sprawl.new_node(Style::default(), &[child1, child2]).unwrap();

        assert_eq!(sprawl.child_count(node).unwrap(), 2);
        assert_eq!(sprawl.children(node).unwrap()[0], child1);
        assert_eq!(sprawl.children(node).unwrap()[1], child2);

        let child3 = sprawl.new_node(Style::default(), &[]).unwrap();
        let child4 = sprawl.new_node(Style::default(), &[]).unwrap();
        sprawl.set_children(node, &[child3, child4]).unwrap();

        assert_eq!(sprawl.child_count(node).unwrap(), 2);
        assert_eq!(sprawl.children(node).unwrap()[0], child3);
        assert_eq!(sprawl.children(node).unwrap()[1], child4);
    }

    #[test]
    fn set_style() {
        let mut sprawl = Sprawl::new();

        let node = sprawl.new_node(Style::default(), &[]).unwrap();
        assert_eq!(sprawl.style(node).unwrap().display, Display::Flex);

        sprawl.set_style(node, Style { display: Display::None, ..Style::default() }).unwrap();
        assert_eq!(sprawl.style(node).unwrap().display, Display::None);
    }

    #[test]
    fn mark_dirty() {
        let mut sprawl = Sprawl::new();

        let child1 = sprawl.new_node(Style::default(), &[]).unwrap();
        let child2 = sprawl.new_node(Style::default(), &[]).unwrap();
        let node = sprawl.new_node(Style::default(), &[child1, child2]).unwrap();

        sprawl.compute_layout(node, sprawl::geometry::Size::undefined()).unwrap();

        assert_eq!(sprawl.dirty(child1).unwrap(), false);
        assert_eq!(sprawl.dirty(child2).unwrap(), false);
        assert_eq!(sprawl.dirty(node).unwrap(), false);

        sprawl.mark_dirty(node).unwrap();
        assert_eq!(sprawl.dirty(child1).unwrap(), false);
        assert_eq!(sprawl.dirty(child2).unwrap(), false);
        assert_eq!(sprawl.dirty(node).unwrap(), true);

        sprawl.compute_layout(node, sprawl::geometry::Size::undefined()).unwrap();
        sprawl.mark_dirty(child1).unwrap();
        assert_eq!(sprawl.dirty(child1).unwrap(), true);
        assert_eq!(sprawl.dirty(child2).unwrap(), false);
        assert_eq!(sprawl.dirty(node).unwrap(), true);
    }

    #[test]
    fn remove_last_node() {
        let mut sprawl = Sprawl::new();

        let parent = sprawl.new_node(Style::default(), &[]).unwrap();
        let child = sprawl.new_node(Style::default(), &[]).unwrap();
        sprawl.add_child(parent, child).unwrap();

        let _ = sprawl.remove(child).unwrap();
        let _ = sprawl.remove(parent).unwrap();
    }
}
