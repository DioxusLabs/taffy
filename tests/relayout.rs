use taffy::prelude::*;

#[test]
fn relayout() {
    let mut taffy: Taffy<()> = taffy::Taffy::new();
    let node1 = taffy
        .new_leaf(taffy::style::Style {
            size: taffy::geometry::Size { width: length(8.0), height: length(80.0) },
            ..Default::default()
        })
        .unwrap();
    let node0 = taffy
        .new_with_children(
            taffy::style::Style {
                align_self: Some(taffy::prelude::AlignSelf::Center),
                size: taffy::geometry::Size { width: Dimension::Auto, height: Dimension::Auto },
                // size: taffy::geometry::Size { width: Dimension::Percent(1.0), height: Dimension::Percent(1.0) },
                ..Default::default()
            },
            &[node1],
        )
        .unwrap();
    let node = taffy
        .new_with_children(
            taffy::style::Style {
                size: taffy::geometry::Size { width: Dimension::Percent(1f32), height: Dimension::Percent(1f32) },
                ..Default::default()
            },
            &[node0],
        )
        .unwrap();
    taffy
        .compute_layout(
            node,
            taffy::geometry::Size { width: AvailableSpace::Definite(100f32), height: AvailableSpace::Definite(100f32) },
        )
        .unwrap();
    let initial = taffy.layout(node).unwrap().location;
    let initial0 = taffy.layout(node0).unwrap().location;
    let initial1 = taffy.layout(node1).unwrap().location;
    for _ in 1..10 {
        taffy
            .compute_layout(
                node,
                taffy::geometry::Size {
                    width: AvailableSpace::Definite(100f32),
                    height: AvailableSpace::Definite(100f32),
                },
            )
            .unwrap();
        assert_eq!(taffy.layout(node).unwrap().location, initial);
        assert_eq!(taffy.layout(node0).unwrap().location, initial0);
        assert_eq!(taffy.layout(node1).unwrap().location, initial1);
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
    let mut taffy: Taffy<()> = taffy::Taffy::new();
    let node = taffy.new_leaf(hidden_style.clone()).unwrap();

    // Layout 1 (None)
    taffy.compute_layout(node, Size::MAX_CONTENT).unwrap();
    let layout = taffy.layout(node).unwrap();
    assert_eq!(layout.location.x, 0.0);
    assert_eq!(layout.location.y, 0.0);
    assert_eq!(layout.size.width, 0.0);
    assert_eq!(layout.size.height, 0.0);

    // Layout 2 (Flex)
    taffy.set_style(node, flex_style).unwrap();
    taffy.compute_layout(node, Size::MAX_CONTENT).unwrap();
    let layout = taffy.layout(node).unwrap();
    assert_eq!(layout.location.x, 0.0);
    assert_eq!(layout.location.y, 0.0);
    assert_eq!(layout.size.width, 100.0);
    assert_eq!(layout.size.height, 100.0);

    // Layout 3 (None)
    taffy.set_style(node, hidden_style).unwrap();
    taffy.compute_layout(node, Size::MAX_CONTENT).unwrap();
    let layout = taffy.layout(node).unwrap();
    assert_eq!(layout.location.x, 0.0);
    assert_eq!(layout.location.y, 0.0);
    assert_eq!(layout.size.width, 0.0);
    assert_eq!(layout.size.height, 0.0);
}

#[test]
fn toggle_root_display_none_with_children() {
    use taffy::prelude::*;

    let mut taffy: Taffy<()> = taffy::Taffy::new();

    let child = taffy
        .new_leaf(Style { size: Size { width: length(800.0), height: length(100.0) }, ..Default::default() })
        .unwrap();

    let parent = taffy
        .new_with_children(
            Style { size: Size { width: length(800.0), height: length(100.0) }, ..Default::default() },
            &[child],
        )
        .unwrap();

    let root = taffy.new_with_children(Style::default(), &[parent]).unwrap();
    taffy.compute_layout(root, Size::MAX_CONTENT).unwrap();
    assert_eq!(taffy.layout(child).unwrap().size.width, 800.0);
    assert_eq!(taffy.layout(child).unwrap().size.height, 100.0);

    taffy.set_style(root, Style { display: Display::None, ..Default::default() }).unwrap();
    taffy.compute_layout(root, Size::MAX_CONTENT).unwrap();
    assert_eq!(taffy.layout(child).unwrap().size.width, 0.0);
    assert_eq!(taffy.layout(child).unwrap().size.height, 0.0);

    taffy.set_style(root, Style::default()).unwrap();
    taffy.compute_layout(root, Size::MAX_CONTENT).unwrap();
    assert_eq!(taffy.layout(parent).unwrap().size.width, 800.0);
    assert_eq!(taffy.layout(parent).unwrap().size.height, 100.0);
    assert_eq!(taffy.layout(child).unwrap().size.width, 800.0);
    assert_eq!(taffy.layout(child).unwrap().size.height, 100.0);
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
    let mut taffy: Taffy<()> = taffy::Taffy::new();
    let node = taffy.new_leaf(hidden_style.clone()).unwrap();
    let root = taffy.new_with_children(flex_style.clone(), &[node]).unwrap();

    // Layout 1 (None)
    taffy.compute_layout(root, Size::MAX_CONTENT).unwrap();
    let layout = taffy.layout(node).unwrap();
    assert_eq!(layout.location.x, 0.0);
    assert_eq!(layout.location.y, 0.0);
    assert_eq!(layout.size.width, 0.0);
    assert_eq!(layout.size.height, 0.0);

    // Layout 2 (Flex)
    taffy.set_style(node, flex_style).unwrap();
    taffy.compute_layout(root, Size::MAX_CONTENT).unwrap();
    let layout = taffy.layout(node).unwrap();
    assert_eq!(layout.location.x, 0.0);
    assert_eq!(layout.location.y, 0.0);
    assert_eq!(layout.size.width, 100.0);
    assert_eq!(layout.size.height, 100.0);

    // Layout 3 (None)
    taffy.set_style(node, hidden_style).unwrap();
    taffy.compute_layout(root, Size::MAX_CONTENT).unwrap();
    let layout = taffy.layout(node).unwrap();
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
    let mut taffy: Taffy<()> = taffy::Taffy::new();
    let node = taffy.new_leaf(hidden_style.clone()).unwrap();
    let root = taffy.new_with_children(hidden_style.clone(), &[node]).unwrap();

    // Layout 1 (None)
    taffy.compute_layout(root, Size::MAX_CONTENT).unwrap();
    let layout = taffy.layout(root).unwrap();
    assert_eq!(layout.location.x, 0.0);
    assert_eq!(layout.location.y, 0.0);
    assert_eq!(layout.size.width, 0.0);
    assert_eq!(layout.size.height, 0.0);

    // Layout 2 (Flex)
    taffy.set_style(root, flex_style).unwrap();
    taffy.compute_layout(root, Size::MAX_CONTENT).unwrap();
    let layout = taffy.layout(root).unwrap();
    assert_eq!(layout.location.x, 0.0);
    assert_eq!(layout.location.y, 0.0);
    assert_eq!(layout.size.width, 100.0);
    assert_eq!(layout.size.height, 100.0);

    // Layout 3 (None)
    taffy.set_style(root, hidden_style).unwrap();
    taffy.compute_layout(root, Size::MAX_CONTENT).unwrap();
    let layout = taffy.layout(root).unwrap();
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
    let mut taffy: Taffy<()> = taffy::Taffy::new();
    let node = taffy.new_leaf(hidden_style.clone()).unwrap();
    let root = taffy.new_with_children(grid_style.clone(), &[node]).unwrap();

    // Layout 1 (None)
    taffy.compute_layout(root, Size::MAX_CONTENT).unwrap();
    let layout = taffy.layout(node).unwrap();
    assert_eq!(layout.location.x, 0.0);
    assert_eq!(layout.location.y, 0.0);
    assert_eq!(layout.size.width, 0.0);
    assert_eq!(layout.size.height, 0.0);

    // Layout 2 (Flex)
    taffy.set_style(node, grid_style).unwrap();
    taffy.compute_layout(root, Size::MAX_CONTENT).unwrap();
    let layout = taffy.layout(node).unwrap();
    assert_eq!(layout.location.x, 0.0);
    assert_eq!(layout.location.y, 0.0);
    assert_eq!(layout.size.width, 100.0);
    assert_eq!(layout.size.height, 100.0);

    // Layout 3 (None)
    taffy.set_style(node, hidden_style).unwrap();
    taffy.compute_layout(root, Size::MAX_CONTENT).unwrap();
    let layout = taffy.layout(node).unwrap();
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
    let mut taffy: Taffy<()> = taffy::Taffy::new();
    let node = taffy.new_leaf(hidden_style.clone()).unwrap();
    let root = taffy.new_with_children(hidden_style.clone(), &[node]).unwrap();

    // Layout 1 (None)
    taffy.compute_layout(root, Size::MAX_CONTENT).unwrap();
    let layout = taffy.layout(root).unwrap();
    assert_eq!(layout.location.x, 0.0);
    assert_eq!(layout.location.y, 0.0);
    assert_eq!(layout.size.width, 0.0);
    assert_eq!(layout.size.height, 0.0);

    // Layout 2 (Flex)
    taffy.set_style(root, grid_style).unwrap();
    taffy.compute_layout(root, Size::MAX_CONTENT).unwrap();
    let layout = taffy.layout(root).unwrap();
    assert_eq!(layout.location.x, 0.0);
    assert_eq!(layout.location.y, 0.0);
    assert_eq!(layout.size.width, 100.0);
    assert_eq!(layout.size.height, 100.0);

    // Layout 3 (None)
    taffy.set_style(root, hidden_style).unwrap();
    taffy.compute_layout(root, Size::MAX_CONTENT).unwrap();
    let layout = taffy.layout(root).unwrap();
    assert_eq!(layout.location.x, 0.0);
    assert_eq!(layout.location.y, 0.0);
    assert_eq!(layout.size.width, 0.0);
    assert_eq!(layout.size.height, 0.0);
}

#[test]
fn relayout_is_stable_with_rounding() {
    let mut taffy: Taffy<()> = taffy::Taffy::new();
    taffy.enable_rounding();

    // <div style="width: 1920px; height: 1080px">
    //     <div style="width: 100%; left: 1.5px">
    //         <div style="width: 150px; justify-content: end">
    //             <div style="min-width: 300px" />
    //         </div>
    //     </div>
    // </div>

    let inner =
        taffy.new_leaf(Style { min_size: Size { width: length(300.), height: auto() }, ..Default::default() }).unwrap();
    let wrapper = taffy
        .new_with_children(
            Style {
                size: Size { width: length(150.), height: auto() },
                justify_content: Some(JustifyContent::End),
                ..Default::default()
            },
            &[inner],
        )
        .unwrap();
    let outer = taffy
        .new_with_children(
            Style {
                size: Size { width: percent(1.), height: auto() },
                inset: Rect { left: length(1.5), right: auto(), top: auto(), bottom: auto() },
                ..Default::default()
            },
            &[wrapper],
        )
        .unwrap();
    let root = taffy
        .new_with_children(
            Style { size: Size { width: length(1920.), height: length(1080.) }, ..Default::default() },
            &[outer],
        )
        .unwrap();
    for _ in 0..5 {
        taffy.mark_dirty(root).ok();
        taffy.compute_layout(root, Size::MAX_CONTENT).ok();
        taffy.print_tree(root);

        let root_layout = taffy.layout(root).unwrap();
        assert_eq!(root_layout.location.x, 0.0);
        assert_eq!(root_layout.location.y, 0.0);
        assert_eq!(root_layout.size.width, 1920.0);
        assert_eq!(root_layout.size.height, 1080.0);

        let outer_layout = taffy.layout(outer).unwrap();
        assert_eq!(outer_layout.location.x, 2.0);
        assert_eq!(outer_layout.location.y, 0.0);
        assert_eq!(outer_layout.size.width, 1920.0);
        assert_eq!(outer_layout.size.height, 1080.0);

        let wrapper_layout = taffy.layout(wrapper).unwrap();
        assert_eq!(wrapper_layout.location.x, 0.0);
        assert_eq!(wrapper_layout.location.y, 0.0);
        assert_eq!(wrapper_layout.size.width, 150.0);
        assert_eq!(wrapper_layout.size.height, 1080.0);

        let inner_layout = taffy.layout(inner).unwrap();
        assert_eq!(inner_layout.location.x, -150.0);
        assert_eq!(inner_layout.location.y, 0.0);
        assert_eq!(inner_layout.size.width, 301.0);
        assert_eq!(inner_layout.size.height, 1080.0);
    }
}
