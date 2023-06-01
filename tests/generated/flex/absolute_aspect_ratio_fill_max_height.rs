#[test]
fn absolute_aspect_ratio_fill_max_height() {
    #[allow(unused_imports)]
    use taffy::{
        prelude::*,
        tree::{Layout, MeasureFunc},
        Taffy,
    };
    let mut taffy: Taffy<MeasureFunc<()>> = Taffy::new();
    let node0 = taffy . new_leaf_with_measure (taffy :: style :: Style { position : taffy :: style :: Position :: Absolute , max_size : taffy :: geometry :: Size { width : taffy :: style :: Dimension :: Length (50f32) , height : auto () , } , aspect_ratio : Some (3f32) , .. Default :: default () } , taffy :: tree :: MeasureFunc :: Raw (| known_dimensions , available_space , _context | { const TEXT : & str = "HHHH\u{200b}HHHH\u{200b}HHHH\u{200b}HHHH\u{200b}HHHH\u{200b}HHHH\u{200b}HHHH\u{200b}HHHH\u{200b}HHHH" ; crate :: measure_standard_text (known_dimensions , available_space , TEXT , crate :: WritingMode :: Horizontal , Some (3f32)) }) ,) . unwrap () ;
    let node = taffy
        .new_with_children(
            taffy::style::Style {
                display: taffy::style::Display::Flex,
                size: taffy::geometry::Size {
                    width: taffy::style::Dimension::Length(400f32),
                    height: taffy::style::Dimension::Length(300f32),
                },
                ..Default::default()
            },
            &[node0],
        )
        .unwrap();
    taffy.compute_layout(node, taffy::geometry::Size::MAX_CONTENT).unwrap();
    println!("\nComputed tree:");
    taffy::util::print_tree(&taffy, node);
    println!();
    let Layout { size, location, .. } = taffy.layout(node).unwrap();
    assert_eq!(size.width, 400f32, "width of node {:?}. Expected {}. Actual {}", node, 400f32, size.width);
    assert_eq!(size.height, 300f32, "height of node {:?}. Expected {}. Actual {}", node, 300f32, size.height);
    assert_eq!(location.x, 0f32, "x of node {:?}. Expected {}. Actual {}", node, 0f32, location.x);
    assert_eq!(location.y, 0f32, "y of node {:?}. Expected {}. Actual {}", node, 0f32, location.y);
    let Layout { size, location, .. } = taffy.layout(node0).unwrap();
    assert_eq!(size.width, 50f32, "width of node {:?}. Expected {}. Actual {}", node0, 50f32, size.width);
    assert_eq!(size.height, 17f32, "height of node {:?}. Expected {}. Actual {}", node0, 17f32, size.height);
    assert_eq!(location.x, 0f32, "x of node {:?}. Expected {}. Actual {}", node0, 0f32, location.x);
    assert_eq!(location.y, 0f32, "y of node {:?}. Expected {}. Actual {}", node0, 0f32, location.y);
}
