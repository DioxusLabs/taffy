#[test]
fn max_height_overrides_height_on_root() {
    #[allow(unused_imports)]
    use taffy::{
        prelude::*,
        tree::{Layout, MeasureFunc},
        Taffy,
    };
    let mut taffy: Taffy<crate::TextMeasure> = Taffy::new();
    let node = taffy
        .new_leaf(taffy::style::Style {
            size: taffy::geometry::Size { width: auto(), height: taffy::style::Dimension::Length(200f32) },
            max_size: taffy::geometry::Size { width: auto(), height: taffy::style::Dimension::Length(100f32) },
            ..Default::default()
        })
        .unwrap();
    taffy.compute_layout(node, taffy::geometry::Size::MAX_CONTENT).unwrap();
    println!("\nComputed tree:");
    taffy::util::print_tree(&taffy, node);
    println!();
    let Layout { size, location, .. } = taffy.layout(node).unwrap();
    assert_eq!(size.width, 0f32, "width of node {:?}. Expected {}. Actual {}", node, 0f32, size.width);
    assert_eq!(size.height, 100f32, "height of node {:?}. Expected {}. Actual {}", node, 100f32, size.height);
    assert_eq!(location.x, 0f32, "x of node {:?}. Expected {}. Actual {}", node, 0f32, location.x);
    assert_eq!(location.y, 0f32, "y of node {:?}. Expected {}. Actual {}", node, 0f32, location.y);
}
