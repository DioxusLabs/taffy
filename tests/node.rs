#[cfg(test)]
mod node {
    use taffy::geometry::*;
    use taffy::node::{MeasureFunc, Taffy};
    use taffy::style::*;

    #[test]
    fn children() {
        let mut taffy = Taffy::new();
        let child1 = taffy.new_with_children(FlexboxLayout::default(), &[]).unwrap();
        let child2 = taffy.new_with_children(FlexboxLayout::default(), &[]).unwrap();
        let node = taffy.new_with_children(FlexboxLayout::default(), &[child1, child2]).unwrap();

        assert_eq!(taffy.child_count(node).unwrap(), 2);
        assert_eq!(taffy.children(node).unwrap()[0], child1);
        assert_eq!(taffy.children(node).unwrap()[1], child2);
    }

    #[test]
    fn set_measure() {
        let mut taffy = Taffy::new();
        let node = taffy
            .new_leaf(FlexboxLayout::default(), MeasureFunc::Raw(|_| Size { width: 200.0, height: 200.0 }))
            .unwrap();
        taffy.compute_layout(node, Size::undefined()).unwrap();
        assert_eq!(taffy.layout(node).unwrap().size.width, 200.0);

        taffy.set_measure(node, Some(MeasureFunc::Raw(|_| Size { width: 100.0, height: 100.0 }))).unwrap();
        taffy.compute_layout(node, Size::undefined()).unwrap();
        assert_eq!(taffy.layout(node).unwrap().size.width, 100.0);
    }

    #[test]
    fn add_child() {
        let mut taffy = Taffy::new();
        let node = taffy.new_with_children(FlexboxLayout::default(), &[]).unwrap();
        assert_eq!(taffy.child_count(node).unwrap(), 0);

        let child1 = taffy.new_with_children(FlexboxLayout::default(), &[]).unwrap();
        taffy.add_child(node, child1).unwrap();
        assert_eq!(taffy.child_count(node).unwrap(), 1);

        let child2 = taffy.new_with_children(FlexboxLayout::default(), &[]).unwrap();
        taffy.add_child(node, child2).unwrap();
        assert_eq!(taffy.child_count(node).unwrap(), 2);
    }

    #[test]
    fn remove_child() {
        let mut taffy = Taffy::new();

        let child1 = taffy.new_with_children(FlexboxLayout::default(), &[]).unwrap();
        let child2 = taffy.new_with_children(FlexboxLayout::default(), &[]).unwrap();

        let node = taffy.new_with_children(FlexboxLayout::default(), &[child1, child2]).unwrap();
        assert_eq!(taffy.child_count(node).unwrap(), 2);

        taffy.remove_child(node, child1).unwrap();
        assert_eq!(taffy.child_count(node).unwrap(), 1);
        assert_eq!(taffy.children(node).unwrap()[0], child2);

        taffy.remove_child(node, child2).unwrap();
        assert_eq!(taffy.child_count(node).unwrap(), 0);
    }

    #[test]
    fn remove_child_at_index() {
        let mut taffy = Taffy::new();

        let child1 = taffy.new_with_children(FlexboxLayout::default(), &[]).unwrap();
        let child2 = taffy.new_with_children(FlexboxLayout::default(), &[]).unwrap();

        let node = taffy.new_with_children(FlexboxLayout::default(), &[child1, child2]).unwrap();
        assert_eq!(taffy.child_count(node).unwrap(), 2);

        taffy.remove_child_at_index(node, 0).unwrap();
        assert_eq!(taffy.child_count(node).unwrap(), 1);
        assert_eq!(taffy.children(node).unwrap()[0], child2);

        taffy.remove_child_at_index(node, 0).unwrap();
        assert_eq!(taffy.child_count(node).unwrap(), 0);
    }

    #[test]
    fn replace_child_at_index() {
        let mut taffy = Taffy::new();

        let child1 = taffy.new_with_children(FlexboxLayout::default(), &[]).unwrap();
        let child2 = taffy.new_with_children(FlexboxLayout::default(), &[]).unwrap();

        let node = taffy.new_with_children(FlexboxLayout::default(), &[child1]).unwrap();
        assert_eq!(taffy.child_count(node).unwrap(), 1);
        assert_eq!(taffy.children(node).unwrap()[0], child1);

        taffy.replace_child_at_index(node, 0, child2).unwrap();
        assert_eq!(taffy.child_count(node).unwrap(), 1);
        assert_eq!(taffy.children(node).unwrap()[0], child2);
    }

    #[test]
    fn remove() {
        let mut taffy = Taffy::new();

        let style2 = FlexboxLayout { flex_direction: FlexDirection::Column, ..FlexboxLayout::default() };

        // Build a linear tree layout: <0> <- <1> <- <2>
        let node2 = taffy.new_with_children(style2, &[]).unwrap();
        let node1 = taffy.new_with_children(FlexboxLayout::default(), &[node2]).unwrap();
        let node0 = taffy.new_with_children(FlexboxLayout::default(), &[node1]).unwrap();

        assert_eq!(taffy.children(node0).unwrap().as_slice(), &[node1]);

        // Disconnect the tree: <0> <2>
        taffy.remove(node1);

        assert!(taffy.style(node1).is_err());

        assert!(taffy.children(node0).unwrap().is_empty());
        assert!(taffy.children(node2).unwrap().is_empty());
        assert_eq!(taffy.style(node2).unwrap().flex_direction, style2.flex_direction);
    }

    #[test]
    fn set_children() {
        let mut taffy = Taffy::new();

        let child1 = taffy.new_with_children(FlexboxLayout::default(), &[]).unwrap();
        let child2 = taffy.new_with_children(FlexboxLayout::default(), &[]).unwrap();
        let node = taffy.new_with_children(FlexboxLayout::default(), &[child1, child2]).unwrap();

        assert_eq!(taffy.child_count(node).unwrap(), 2);
        assert_eq!(taffy.children(node).unwrap()[0], child1);
        assert_eq!(taffy.children(node).unwrap()[1], child2);

        let child3 = taffy.new_with_children(FlexboxLayout::default(), &[]).unwrap();
        let child4 = taffy.new_with_children(FlexboxLayout::default(), &[]).unwrap();
        taffy.set_children(node, &[child3, child4]).unwrap();

        assert_eq!(taffy.child_count(node).unwrap(), 2);
        assert_eq!(taffy.children(node).unwrap()[0], child3);
        assert_eq!(taffy.children(node).unwrap()[1], child4);
    }

    #[test]
    fn set_style() {
        let mut taffy = Taffy::new();

        let node = taffy.new_with_children(FlexboxLayout::default(), &[]).unwrap();
        assert_eq!(taffy.style(node).unwrap().display, Display::Flex);

        taffy.set_style(node, FlexboxLayout { display: Display::None, ..FlexboxLayout::default() }).unwrap();
        assert_eq!(taffy.style(node).unwrap().display, Display::None);
    }

    #[test]
    fn mark_dirty() {
        let mut taffy = Taffy::new();

        let child1 = taffy.new_with_children(FlexboxLayout::default(), &[]).unwrap();
        let child2 = taffy.new_with_children(FlexboxLayout::default(), &[]).unwrap();
        let node = taffy.new_with_children(FlexboxLayout::default(), &[child1, child2]).unwrap();

        taffy.compute_layout(node, taffy::geometry::Size::undefined()).unwrap();

        assert_eq!(taffy.dirty(child1).unwrap(), false);
        assert_eq!(taffy.dirty(child2).unwrap(), false);
        assert_eq!(taffy.dirty(node).unwrap(), false);

        taffy.mark_dirty(node).unwrap();
        assert_eq!(taffy.dirty(child1).unwrap(), false);
        assert_eq!(taffy.dirty(child2).unwrap(), false);
        assert_eq!(taffy.dirty(node).unwrap(), true);

        taffy.compute_layout(node, taffy::geometry::Size::undefined()).unwrap();
        taffy.mark_dirty(child1).unwrap();
        assert_eq!(taffy.dirty(child1).unwrap(), true);
        assert_eq!(taffy.dirty(child2).unwrap(), false);
        assert_eq!(taffy.dirty(node).unwrap(), true);
    }

    #[test]
    fn remove_last_node() {
        let mut taffy = Taffy::new();

        let parent = taffy.new_with_children(FlexboxLayout::default(), &[]).unwrap();
        let child = taffy.new_with_children(FlexboxLayout::default(), &[]).unwrap();
        taffy.add_child(parent, child).unwrap();

        taffy.remove(child);
        taffy.remove(parent);
    }
}
