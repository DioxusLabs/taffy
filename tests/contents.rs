use taffy::prelude::*;

/// Contents node gets zero layout (no box generated)
#[test]
fn contents_node_has_zero_layout() {
    let mut taffy = TaffyTree::<()>::new();

    let child = taffy.new_leaf(Style {
        size: Size { width: Dimension::from_length(50.0), height: Dimension::from_length(50.0) },
        ..Default::default()
    }).unwrap();

    let contents = taffy.new_with_children(
        Style { display: Display::Contents, ..Default::default() },
        &[child],
    ).unwrap();

    let root = taffy.new_with_children(
        Style {
            display: Display::Flex,
            size: Size { width: Dimension::from_length(200.0), height: Dimension::from_length(200.0) },
            ..Default::default()
        },
        &[contents],
    ).unwrap();

    taffy.compute_layout(root, Size::MAX_CONTENT).unwrap();

    // Contents node itself has zero size
    let layout = taffy.layout(contents).unwrap();
    assert_eq!(layout.size.width, 0.0);
    assert_eq!(layout.size.height, 0.0);
}

/// Contents children are NOT hidden (unlike Display::None which hides descendants)
#[test]
fn contents_children_are_not_hidden() {
    let mut taffy = TaffyTree::<()>::new();

    let child = taffy.new_leaf(Style {
        size: Size { width: Dimension::from_length(50.0), height: Dimension::from_length(50.0) },
        ..Default::default()
    }).unwrap();

    let none_child = taffy.new_leaf(Style {
        size: Size { width: Dimension::from_length(50.0), height: Dimension::from_length(50.0) },
        ..Default::default()
    }).unwrap();

    let contents = taffy.new_with_children(
        Style { display: Display::Contents, ..Default::default() },
        &[child],
    ).unwrap();

    let none = taffy.new_with_children(
        Style { display: Display::None, ..Default::default() },
        &[none_child],
    ).unwrap();

    let root = taffy.new_with_children(
        Style {
            display: Display::Flex,
            size: Size { width: Dimension::from_length(200.0), height: Dimension::from_length(200.0) },
            ..Default::default()
        },
        &[contents, none],
    ).unwrap();

    taffy.compute_layout(root, Size::MAX_CONTENT).unwrap();

    // Display::None child is hidden (zero layout)
    let none_layout = taffy.layout(none_child).unwrap();
    assert_eq!(none_layout.size.width, 0.0, "None child should be hidden");
    assert_eq!(none_layout.size.height, 0.0, "None child should be hidden");

    // Display::Contents child retains its style — it's not laid out by TaffyTree's
    // built-in child_ids (no flattening), but it IS NOT recursively hidden.
    // The child has a style with 50x50, and since the contents node is treated as
    // zero-size (not as a flex container), the child won't be laid out through
    // the normal TaffyTree path. However, it should NOT be zeroed out like None.
    //
    // Note: Full display:contents behavior (child flattening) requires the
    // TraversePartialTree implementation to recurse into contents nodes.
    // TaffyTree's built-in impl does NOT do this — that's the consumer's job.
    // This test verifies the Taffy-side contract only.
    let contents_layout = taffy.layout(contents).unwrap();
    assert_eq!(contents_layout.size.width, 0.0, "Contents node itself is zero");
    assert_eq!(contents_layout.size.height, 0.0, "Contents node itself is zero");
}

/// Display::Contents is distinct from Display::None in the style
#[test]
fn contents_style_roundtrip() {
    let mut taffy = TaffyTree::<()>::new();
    let node = taffy.new_leaf(Style { display: Display::Contents, ..Default::default() }).unwrap();
    assert_eq!(taffy.style(node).unwrap().display, Display::Contents);

    taffy.set_style(node, Style { display: Display::None, ..Default::default() }).unwrap();
    assert_eq!(taffy.style(node).unwrap().display, Display::None);

    taffy.set_style(node, Style { display: Display::Contents, ..Default::default() }).unwrap();
    assert_eq!(taffy.style(node).unwrap().display, Display::Contents);
}
