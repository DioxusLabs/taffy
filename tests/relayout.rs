use taffy::prelude::*;

#[test]
fn relayout() {
    let mut taffy = taffy::Taffy::new();
    let node1 = taffy.new_leaf(taffy::style::Style {
        size: taffy::geometry::Size { width: length(8.0), height: length(80.0) },
        ..Default::default()
    });
    let node0 = taffy.new_with_children(
        taffy::style::Style {
            align_self: Some(taffy::prelude::AlignSelf::Center),
            size: taffy::geometry::Size { width: Dimension::Auto, height: Dimension::Auto },
            // size: taffy::geometry::Size { width: Dimension::Percent(1.0), height: Dimension::Percent(1.0) },
            ..Default::default()
        },
        &[node1],
    );
    let node = taffy.new_with_children(
        taffy::style::Style {
            size: taffy::geometry::Size { width: Dimension::Percent(1f32), height: Dimension::Percent(1f32) },
            ..Default::default()
        },
        &[node0],
    );
    println!("0:");
    taffy.compute_layout(
        node,
        taffy::geometry::Size { width: AvailableSpace::Definite(100f32), height: AvailableSpace::Definite(100f32) },
    );
    let initial = taffy.layout(node).location;
    let initial0 = taffy.layout(node0).location;
    let initial1 = taffy.layout(node1).location;
    for i in 1..10 {
        println!("\n\n{i}:");
        taffy.compute_layout(
            node,
            taffy::geometry::Size { width: AvailableSpace::Definite(100f32), height: AvailableSpace::Definite(100f32) },
        );
        assert_eq!(taffy.layout(node).location, initial);
        assert_eq!(taffy.layout(node0).location, initial0);
        assert_eq!(taffy.layout(node1).location, initial1);
    }
}

#[test]
fn toggle_root_display_none() {
    let hidden_style = Style {
        display: Display::None,
        size: Size { width: length(100.0), height: length(100.0) },
        ..Default::default()
    };

    let flex_style = Style {
        display: Display::Flex,
        size: Size { width: length(100.0), height: length(100.0) },
        ..Default::default()
    };

    // Setup
    let mut taffy = taffy::Taffy::new();
    let node = taffy.new_leaf(hidden_style.clone());

    // Layout 1 (None)
    taffy.compute_layout(node, Size::MAX_CONTENT);
    let layout = taffy.layout(node);
    assert_eq!(layout.location.x, 0.0);
    assert_eq!(layout.location.y, 0.0);
    assert_eq!(layout.size.width, 0.0);
    assert_eq!(layout.size.height, 0.0);

    // Layout 2 (Flex)
    taffy.set_style(node, flex_style);
    taffy.compute_layout(node, Size::MAX_CONTENT);
    let layout = taffy.layout(node);
    assert_eq!(layout.location.x, 0.0);
    assert_eq!(layout.location.y, 0.0);
    assert_eq!(layout.size.width, 100.0);
    assert_eq!(layout.size.height, 100.0);

    // Layout 3 (None)
    taffy.set_style(node, hidden_style);
    taffy.compute_layout(node, Size::MAX_CONTENT);
    let layout = taffy.layout(node);
    assert_eq!(layout.location.x, 0.0);
    assert_eq!(layout.location.y, 0.0);
    assert_eq!(layout.size.width, 0.0);
    assert_eq!(layout.size.height, 0.0);
}

#[test]
fn toggle_root_display_none_with_children() {
    use taffy::prelude::*;

    let mut taffy = taffy::Taffy::new();

    let child =
        taffy.new_leaf(Style { size: Size { width: length(800.0), height: length(100.0) }, ..Default::default() });

    let parent = taffy.new_with_children(
        Style { size: Size { width: length(800.0), height: length(100.0) }, ..Default::default() },
        &[child],
    );

    let root = taffy.new_with_children(Style::default(), &[parent]);
    taffy.compute_layout(root, Size::MAX_CONTENT);
    assert_eq!(taffy.layout(child).size.width, 800.0);
    assert_eq!(taffy.layout(child).size.height, 100.0);

    taffy.set_style(root, Style { display: Display::None, ..Default::default() });
    taffy.compute_layout(root, Size::MAX_CONTENT);
    assert_eq!(taffy.layout(child).size.width, 0.0);
    assert_eq!(taffy.layout(child).size.height, 0.0);

    taffy.set_style(root, Style::default());
    taffy.compute_layout(root, Size::MAX_CONTENT);
    assert_eq!(taffy.layout(parent).size.width, 800.0);
    assert_eq!(taffy.layout(parent).size.height, 100.0);
    assert_eq!(taffy.layout(child).size.width, 800.0);
    assert_eq!(taffy.layout(child).size.height, 100.0);
}

#[test]
fn toggle_flex_child_display_none() {
    let hidden_style = Style {
        display: Display::None,
        size: Size { width: length(100.0), height: length(100.0) },
        ..Default::default()
    };

    let flex_style = Style {
        display: Display::Flex,
        size: Size { width: length(100.0), height: length(100.0) },
        ..Default::default()
    };

    // Setup
    let mut taffy = taffy::Taffy::new();
    let node = taffy.new_leaf(hidden_style.clone());
    let root = taffy.new_with_children(flex_style.clone(), &[node]);

    // Layout 1 (None)
    taffy.compute_layout(root, Size::MAX_CONTENT);
    let layout = taffy.layout(node);
    assert_eq!(layout.location.x, 0.0);
    assert_eq!(layout.location.y, 0.0);
    assert_eq!(layout.size.width, 0.0);
    assert_eq!(layout.size.height, 0.0);

    // Layout 2 (Flex)
    taffy.set_style(node, flex_style);
    taffy.compute_layout(root, Size::MAX_CONTENT);
    let layout = taffy.layout(node);
    assert_eq!(layout.location.x, 0.0);
    assert_eq!(layout.location.y, 0.0);
    assert_eq!(layout.size.width, 100.0);
    assert_eq!(layout.size.height, 100.0);

    // Layout 3 (None)
    taffy.set_style(node, hidden_style);
    taffy.compute_layout(root, Size::MAX_CONTENT);
    let layout = taffy.layout(node);
    assert_eq!(layout.location.x, 0.0);
    assert_eq!(layout.location.y, 0.0);
    assert_eq!(layout.size.width, 0.0);
    assert_eq!(layout.size.height, 0.0);
}

#[test]
fn toggle_flex_container_display_none() {
    let hidden_style = Style {
        display: Display::None,
        size: Size { width: length(100.0), height: length(100.0) },
        ..Default::default()
    };

    let flex_style = Style {
        display: Display::Flex,
        size: Size { width: length(100.0), height: length(100.0) },
        ..Default::default()
    };

    // Setup
    let mut taffy = taffy::Taffy::new();
    let node = taffy.new_leaf(hidden_style.clone());
    let root = taffy.new_with_children(hidden_style.clone(), &[node]);

    // Layout 1 (None)
    taffy.compute_layout(root, Size::MAX_CONTENT);
    let layout = taffy.layout(root);
    assert_eq!(layout.location.x, 0.0);
    assert_eq!(layout.location.y, 0.0);
    assert_eq!(layout.size.width, 0.0);
    assert_eq!(layout.size.height, 0.0);

    // Layout 2 (Flex)
    taffy.set_style(root, flex_style);
    taffy.compute_layout(root, Size::MAX_CONTENT);
    let layout = taffy.layout(root);
    assert_eq!(layout.location.x, 0.0);
    assert_eq!(layout.location.y, 0.0);
    assert_eq!(layout.size.width, 100.0);
    assert_eq!(layout.size.height, 100.0);

    // Layout 3 (None)
    taffy.set_style(root, hidden_style);
    taffy.compute_layout(root, Size::MAX_CONTENT);
    let layout = taffy.layout(root);
    assert_eq!(layout.location.x, 0.0);
    assert_eq!(layout.location.y, 0.0);
    assert_eq!(layout.size.width, 0.0);
    assert_eq!(layout.size.height, 0.0);
}

#[test]
fn toggle_grid_child_display_none() {
    let hidden_style = Style {
        display: Display::None,
        size: Size { width: length(100.0), height: length(100.0) },
        ..Default::default()
    };

    let grid_style = Style {
        display: Display::Grid,
        size: Size { width: length(100.0), height: length(100.0) },
        ..Default::default()
    };

    // Setup
    let mut taffy = taffy::Taffy::new();
    let node = taffy.new_leaf(hidden_style.clone());
    let root = taffy.new_with_children(grid_style.clone(), &[node]);

    // Layout 1 (None)
    taffy.compute_layout(root, Size::MAX_CONTENT);
    let layout = taffy.layout(node);
    assert_eq!(layout.location.x, 0.0);
    assert_eq!(layout.location.y, 0.0);
    assert_eq!(layout.size.width, 0.0);
    assert_eq!(layout.size.height, 0.0);

    // Layout 2 (Flex)
    taffy.set_style(node, grid_style);
    taffy.compute_layout(root, Size::MAX_CONTENT);
    let layout = taffy.layout(node);
    assert_eq!(layout.location.x, 0.0);
    assert_eq!(layout.location.y, 0.0);
    assert_eq!(layout.size.width, 100.0);
    assert_eq!(layout.size.height, 100.0);

    // Layout 3 (None)
    taffy.set_style(node, hidden_style);
    taffy.compute_layout(root, Size::MAX_CONTENT);
    let layout = taffy.layout(node);
    assert_eq!(layout.location.x, 0.0);
    assert_eq!(layout.location.y, 0.0);
    assert_eq!(layout.size.width, 0.0);
    assert_eq!(layout.size.height, 0.0);
}

#[test]
fn toggle_grid_container_display_none() {
    let hidden_style = Style {
        display: Display::None,
        size: Size { width: length(100.0), height: length(100.0) },
        ..Default::default()
    };

    let grid_style = Style {
        display: Display::Grid,
        size: Size { width: length(100.0), height: length(100.0) },
        ..Default::default()
    };

    // Setup
    let mut taffy = taffy::Taffy::new();
    let node = taffy.new_leaf(hidden_style.clone());
    let root = taffy.new_with_children(hidden_style.clone(), &[node]);

    // Layout 1 (None)
    taffy.compute_layout(root, Size::MAX_CONTENT);
    let layout = taffy.layout(root);
    assert_eq!(layout.location.x, 0.0);
    assert_eq!(layout.location.y, 0.0);
    assert_eq!(layout.size.width, 0.0);
    assert_eq!(layout.size.height, 0.0);

    // Layout 2 (Flex)
    taffy.set_style(root, grid_style);
    taffy.compute_layout(root, Size::MAX_CONTENT);
    let layout = taffy.layout(root);
    assert_eq!(layout.location.x, 0.0);
    assert_eq!(layout.location.y, 0.0);
    assert_eq!(layout.size.width, 100.0);
    assert_eq!(layout.size.height, 100.0);

    // Layout 3 (None)
    taffy.set_style(root, hidden_style);
    taffy.compute_layout(root, Size::MAX_CONTENT);
    let layout = taffy.layout(root);
    assert_eq!(layout.location.x, 0.0);
    assert_eq!(layout.location.y, 0.0);
    assert_eq!(layout.size.width, 0.0);
    assert_eq!(layout.size.height, 0.0);
}
