use taffy::geometry::Point;
use taffy::prelude::*;
use taffy_test_helpers::new_test_tree;

fn leaf<T>(taffy: &mut TaffyTree<T>, width: f32) -> NodeId {
    taffy.new_leaf(Style { size: Size { width: length(width), height: length(10.0) }, ..Default::default() }).unwrap()
}

fn contents<T>(taffy: &mut TaffyTree<T>, children: &[NodeId]) -> NodeId {
    taffy.new_with_children(Style { display: Display::Contents, ..Default::default() }, children).unwrap()
}

fn flex_row<T>(taffy: &mut TaffyTree<T>, children: &[NodeId]) -> NodeId {
    taffy
        .new_with_children(
            Style {
                display: Display::Flex,
                size: Size { width: length(200.0), height: length(50.0) },
                ..Default::default()
            },
            children,
        )
        .unwrap()
}

fn x<T>(taffy: &TaffyTree<T>, node: NodeId) -> f32 {
    taffy.layout(node).unwrap().location.x
}

#[test]
fn contents_node_has_empty_layout_and_children_are_relative_to_box_parent() {
    let mut taffy = new_test_tree();
    let a = leaf(&mut taffy, 10.0);
    let b = leaf(&mut taffy, 20.0);
    let c = leaf(&mut taffy, 30.0);
    let wrapper = contents(&mut taffy, &[b, c]);
    let root = flex_row(&mut taffy, &[a, wrapper]);
    taffy.compute_layout(root, Size::MAX_CONTENT).unwrap();

    assert_eq!(*taffy.layout(wrapper).unwrap(), Layout::new());
    assert_eq!((x(&taffy, a), x(&taffy, b), x(&taffy, c)), (0.0, 10.0, 30.0));
    assert_eq!(taffy.children(root).unwrap(), vec![a, wrapper]);
    assert_eq!(taffy.parent(b), Some(wrapper));
}

#[test]
fn toggling_display_contents_relayouts_parent() {
    let mut taffy = new_test_tree();
    let a = leaf(&mut taffy, 10.0);
    let b = leaf(&mut taffy, 20.0);
    let wrapper = taffy
        .new_with_children(Style { size: Size { width: length(100.0), height: auto() }, ..Default::default() }, &[b])
        .unwrap();
    let c = leaf(&mut taffy, 30.0);
    let root = flex_row(&mut taffy, &[a, wrapper, c]);
    taffy.compute_layout(root, Size::MAX_CONTENT).unwrap();
    assert_eq!(x(&taffy, c), 110.0);

    taffy.set_style(wrapper, Style { display: Display::Contents, ..Default::default() }).unwrap();
    taffy.compute_layout(root, Size::MAX_CONTENT).unwrap();
    assert_eq!((x(&taffy, b), x(&taffy, c)), (10.0, 30.0));
    assert_eq!(*taffy.layout(wrapper).unwrap(), Layout::new());

    taffy
        .set_style(wrapper, Style { size: Size { width: length(100.0), height: auto() }, ..Default::default() })
        .unwrap();
    taffy.compute_layout(root, Size::MAX_CONTENT).unwrap();
    assert_eq!((x(&taffy, wrapper), x(&taffy, b), x(&taffy, c)), (10.0, 0.0, 110.0));
}

#[test]
fn style_change_inside_contents_marks_box_ancestor_dirty() {
    let mut taffy = new_test_tree();
    let a = leaf(&mut taffy, 10.0);
    let b = leaf(&mut taffy, 20.0);
    let inner = contents(&mut taffy, &[a]);
    let outer = contents(&mut taffy, &[inner]);
    let root = flex_row(&mut taffy, &[outer, b]);
    taffy.compute_layout(root, Size::MAX_CONTENT).unwrap();
    assert_eq!(x(&taffy, b), 10.0);

    taffy
        .set_style(a, Style { size: Size { width: length(50.0), height: length(10.0) }, ..Default::default() })
        .unwrap();
    assert!(taffy.dirty(root).unwrap());
    taffy.compute_layout(root, Size::MAX_CONTENT).unwrap();
    assert_eq!(x(&taffy, b), 50.0);
}

#[test]
fn child_list_changes_inside_contents_update_box_parent() {
    let mut taffy = new_test_tree();
    let a = leaf(&mut taffy, 10.0);
    let b = leaf(&mut taffy, 20.0);
    let c = leaf(&mut taffy, 30.0);
    let inner = contents(&mut taffy, &[]);
    let outer = contents(&mut taffy, &[inner]);
    let root = flex_row(&mut taffy, &[outer, c]);
    taffy.compute_layout(root, Size::MAX_CONTENT).unwrap();
    assert_eq!(x(&taffy, c), 0.0);

    taffy.add_child(inner, a).unwrap();
    taffy.insert_child_at_index(outer, 0, b).unwrap();
    taffy.compute_layout(root, Size::MAX_CONTENT).unwrap();
    assert_eq!((x(&taffy, b), x(&taffy, a), x(&taffy, c)), (0.0, 20.0, 30.0));

    taffy.remove_child(outer, b).unwrap();
    taffy.compute_layout(root, Size::MAX_CONTENT).unwrap();
    assert_eq!((x(&taffy, a), x(&taffy, c)), (0.0, 10.0));

    taffy.replace_child_at_index(inner, 0, b).unwrap();
    taffy.compute_layout(root, Size::MAX_CONTENT).unwrap();
    assert_eq!((x(&taffy, b), x(&taffy, c)), (0.0, 20.0));

    taffy.remove(inner).unwrap();
    taffy.compute_layout(root, Size::MAX_CONTENT).unwrap();
    assert_eq!(x(&taffy, c), 0.0);

    taffy.set_children(outer, &[a, b]).unwrap();
    taffy.compute_layout(root, Size::MAX_CONTENT).unwrap();
    assert_eq!((x(&taffy, a), x(&taffy, b), x(&taffy, c)), (0.0, 10.0, 30.0));

    taffy.remove_children_range(root, 0..1).unwrap();
    taffy.compute_layout(root, Size::MAX_CONTENT).unwrap();
    assert_eq!(x(&taffy, c), 0.0);
}

#[test]
fn contents_root_is_laid_out_as_block() {
    let mut taffy = new_test_tree();
    let a = leaf(&mut taffy, 10.0);
    let b = leaf(&mut taffy, 20.0);
    let inner = contents(&mut taffy, &[b]);
    let root = contents(&mut taffy, &[a, inner]);
    taffy
        .compute_layout(root, Size { width: AvailableSpace::Definite(100.0), height: AvailableSpace::MaxContent })
        .unwrap();

    assert_eq!(taffy.layout(root).unwrap().size, Size { width: 100.0, height: 20.0 });
    assert_eq!(taffy.layout(a).unwrap().location, Point { x: 0.0, y: 0.0 });
    assert_eq!(taffy.layout(b).unwrap().location, Point { x: 0.0, y: 10.0 });
}
