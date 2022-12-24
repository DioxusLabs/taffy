#[test]
fn wrapped_row_within_align_items_flex_start() {
    use slotmap::Key;
    #[allow(unused_imports)]
    use taffy::{layout::Layout, prelude::*};
    let mut taffy = taffy::Taffy::new();
    let node00 = taffy
        .new_leaf(taffy::style::Style {
            size: taffy::geometry::Size {
                width: taffy::style::Dimension::Points(150f32),
                height: taffy::style::Dimension::Points(80f32),
            },
            ..Default::default()
        })
        .unwrap();
    let node01 = taffy
        .new_leaf(taffy::style::Style {
            size: taffy::geometry::Size {
                width: taffy::style::Dimension::Points(80f32),
                height: taffy::style::Dimension::Points(80f32),
            },
            ..Default::default()
        })
        .unwrap();
    let node0 = taffy
        .new_with_children(
            taffy::style::Style { flex_wrap: taffy::style::FlexWrap::Wrap, ..Default::default() },
            &[node00, node01],
        )
        .unwrap();
    let node = taffy
        .new_with_children(
            taffy::style::Style {
                flex_direction: taffy::style::FlexDirection::Column,
                align_items: Some(taffy::style::AlignItems::Start),
                size: taffy::geometry::Size {
                    width: taffy::style::Dimension::Points(200f32),
                    height: taffy::style::Dimension::Points(200f32),
                },
                ..Default::default()
            },
            &[node0],
        )
        .unwrap();
    taffy.compute_layout(node, taffy::geometry::Size::MAX_CONTENT).unwrap();
    println!("\nComputed tree:");
    taffy::debug::print_tree(&taffy, node);
    println!();
    let Layout { size, location, .. } = taffy.layout(node).unwrap();
    assert_eq!(size.width, 200f32, "width of node {:?}. Expected {}. Actual {}", node.data(), 200f32, size.width);
    assert_eq!(size.height, 200f32, "height of node {:?}. Expected {}. Actual {}", node.data(), 200f32, size.height);
    assert_eq!(location.x, 0f32, "x of node {:?}. Expected {}. Actual {}", node.data(), 0f32, location.x);
    assert_eq!(location.y, 0f32, "y of node {:?}. Expected {}. Actual {}", node.data(), 0f32, location.y);
    let Layout { size, location, .. } = taffy.layout(node0).unwrap();
    assert_eq!(size.width, 200f32, "width of node {:?}. Expected {}. Actual {}", node0.data(), 200f32, size.width);
    assert_eq!(size.height, 160f32, "height of node {:?}. Expected {}. Actual {}", node0.data(), 160f32, size.height);
    assert_eq!(location.x, 0f32, "x of node {:?}. Expected {}. Actual {}", node0.data(), 0f32, location.x);
    assert_eq!(location.y, 0f32, "y of node {:?}. Expected {}. Actual {}", node0.data(), 0f32, location.y);
    let Layout { size, location, .. } = taffy.layout(node00).unwrap();
    assert_eq!(size.width, 150f32, "width of node {:?}. Expected {}. Actual {}", node00.data(), 150f32, size.width);
    assert_eq!(size.height, 80f32, "height of node {:?}. Expected {}. Actual {}", node00.data(), 80f32, size.height);
    assert_eq!(location.x, 0f32, "x of node {:?}. Expected {}. Actual {}", node00.data(), 0f32, location.x);
    assert_eq!(location.y, 0f32, "y of node {:?}. Expected {}. Actual {}", node00.data(), 0f32, location.y);
    let Layout { size, location, .. } = taffy.layout(node01).unwrap();
    assert_eq!(size.width, 80f32, "width of node {:?}. Expected {}. Actual {}", node01.data(), 80f32, size.width);
    assert_eq!(size.height, 80f32, "height of node {:?}. Expected {}. Actual {}", node01.data(), 80f32, size.height);
    assert_eq!(location.x, 0f32, "x of node {:?}. Expected {}. Actual {}", node01.data(), 0f32, location.x);
    assert_eq!(location.y, 80f32, "y of node {:?}. Expected {}. Actual {}", node01.data(), 80f32, location.y);
}
