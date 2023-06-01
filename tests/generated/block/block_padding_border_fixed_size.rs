#[test]
fn block_padding_border_fixed_size() {
    #[allow(unused_imports)]
    use taffy::{
        prelude::*,
        tree::{Layout, MeasureFunc},
        Taffy,
    };
    let mut taffy: Taffy<MeasureFunc<()>> = Taffy::new();
    let node0 = taffy
        .new_leaf(taffy::style::Style {
            size: taffy::geometry::Size { width: auto(), height: taffy::style::Dimension::Length(10f32) },
            ..Default::default()
        })
        .unwrap();
    let node1 = taffy
        .new_leaf(taffy::style::Style {
            size: taffy::geometry::Size { width: auto(), height: taffy::style::Dimension::Length(10f32) },
            ..Default::default()
        })
        .unwrap();
    let node = taffy
        .new_with_children(
            taffy::style::Style {
                display: taffy::style::Display::Block,
                size: taffy::geometry::Size {
                    width: taffy::style::Dimension::Length(50f32),
                    height: taffy::style::Dimension::Length(50f32),
                },
                padding: taffy::geometry::Rect {
                    left: taffy::style::LengthPercentage::Length(7f32),
                    right: taffy::style::LengthPercentage::Length(3f32),
                    top: taffy::style::LengthPercentage::Length(1f32),
                    bottom: taffy::style::LengthPercentage::Length(5f32),
                },
                border: taffy::geometry::Rect {
                    left: taffy::style::LengthPercentage::Length(8f32),
                    right: taffy::style::LengthPercentage::Length(4f32),
                    top: taffy::style::LengthPercentage::Length(2f32),
                    bottom: taffy::style::LengthPercentage::Length(6f32),
                },
                ..Default::default()
            },
            &[node0, node1],
        )
        .unwrap();
    taffy.compute_layout(node, taffy::geometry::Size::MAX_CONTENT).unwrap();
    println!("\nComputed tree:");
    taffy::util::print_tree(&taffy, node);
    println!();
    let Layout { size, location, .. } = taffy.layout(node).unwrap();
    assert_eq!(size.width, 50f32, "width of node {:?}. Expected {}. Actual {}", node, 50f32, size.width);
    assert_eq!(size.height, 50f32, "height of node {:?}. Expected {}. Actual {}", node, 50f32, size.height);
    assert_eq!(location.x, 0f32, "x of node {:?}. Expected {}. Actual {}", node, 0f32, location.x);
    assert_eq!(location.y, 0f32, "y of node {:?}. Expected {}. Actual {}", node, 0f32, location.y);
    let Layout { size, location, .. } = taffy.layout(node0).unwrap();
    assert_eq!(size.width, 28f32, "width of node {:?}. Expected {}. Actual {}", node0, 28f32, size.width);
    assert_eq!(size.height, 10f32, "height of node {:?}. Expected {}. Actual {}", node0, 10f32, size.height);
    assert_eq!(location.x, 15f32, "x of node {:?}. Expected {}. Actual {}", node0, 15f32, location.x);
    assert_eq!(location.y, 3f32, "y of node {:?}. Expected {}. Actual {}", node0, 3f32, location.y);
    let Layout { size, location, .. } = taffy.layout(node1).unwrap();
    assert_eq!(size.width, 28f32, "width of node {:?}. Expected {}. Actual {}", node1, 28f32, size.width);
    assert_eq!(size.height, 10f32, "height of node {:?}. Expected {}. Actual {}", node1, 10f32, size.height);
    assert_eq!(location.x, 15f32, "x of node {:?}. Expected {}. Actual {}", node1, 15f32, location.x);
    assert_eq!(location.y, 13f32, "y of node {:?}. Expected {}. Actual {}", node1, 13f32, location.y);
}
