#[test]
fn flex_grow_less_than_factor_one() {
    #[allow(unused_imports)]
    use taffy::{prelude::*, tree::Layout};
    let mut taffy = taffy::Taffy::new();
    let node0 = taffy.new_leaf(taffy::style::Style {
        flex_grow: 0.2f32,
        flex_shrink: 0f32,
        flex_basis: taffy::style::Dimension::Length(40f32),
        ..Default::default()
    });
    let node1 = taffy.new_leaf(taffy::style::Style { flex_grow: 0.2f32, flex_shrink: 0f32, ..Default::default() });
    let node2 = taffy.new_leaf(taffy::style::Style { flex_grow: 0.4f32, flex_shrink: 0f32, ..Default::default() });
    let node = taffy.new_with_children(
        taffy::style::Style {
            size: taffy::geometry::Size {
                width: taffy::style::Dimension::Length(500f32),
                height: taffy::style::Dimension::Length(200f32),
            },
            ..Default::default()
        },
        &[node0, node1, node2],
    );
    taffy.compute_layout(node, taffy::geometry::Size::MAX_CONTENT);
    println!("\nComputed tree:");
    taffy::util::print_tree(&taffy, node);
    println!();
    let Layout { size, location, .. } = taffy.layout(node);
    assert_eq!(size.width, 500f32, "width of node {:?}. Expected {}. Actual {}", node, 500f32, size.width);
    assert_eq!(size.height, 200f32, "height of node {:?}. Expected {}. Actual {}", node, 200f32, size.height);
    assert_eq!(location.x, 0f32, "x of node {:?}. Expected {}. Actual {}", node, 0f32, location.x);
    assert_eq!(location.y, 0f32, "y of node {:?}. Expected {}. Actual {}", node, 0f32, location.y);
    let Layout { size, location, .. } = taffy.layout(node0);
    assert_eq!(size.width, 132f32, "width of node {:?}. Expected {}. Actual {}", node0, 132f32, size.width);
    assert_eq!(size.height, 200f32, "height of node {:?}. Expected {}. Actual {}", node0, 200f32, size.height);
    assert_eq!(location.x, 0f32, "x of node {:?}. Expected {}. Actual {}", node0, 0f32, location.x);
    assert_eq!(location.y, 0f32, "y of node {:?}. Expected {}. Actual {}", node0, 0f32, location.y);
    let Layout { size, location, .. } = taffy.layout(node1);
    assert_eq!(size.width, 92f32, "width of node {:?}. Expected {}. Actual {}", node1, 92f32, size.width);
    assert_eq!(size.height, 200f32, "height of node {:?}. Expected {}. Actual {}", node1, 200f32, size.height);
    assert_eq!(location.x, 132f32, "x of node {:?}. Expected {}. Actual {}", node1, 132f32, location.x);
    assert_eq!(location.y, 0f32, "y of node {:?}. Expected {}. Actual {}", node1, 0f32, location.y);
    let Layout { size, location, .. } = taffy.layout(node2);
    assert_eq!(size.width, 184f32, "width of node {:?}. Expected {}. Actual {}", node2, 184f32, size.width);
    assert_eq!(size.height, 200f32, "height of node {:?}. Expected {}. Actual {}", node2, 200f32, size.height);
    assert_eq!(location.x, 224f32, "x of node {:?}. Expected {}. Actual {}", node2, 224f32, location.x);
    assert_eq!(location.y, 0f32, "y of node {:?}. Expected {}. Actual {}", node2, 0f32, location.y);
}
