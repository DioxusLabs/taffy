pub fn compute() {
    #[allow(unused_imports)]
    use taffy::prelude::*;
    let mut taffy = taffy::Taffy::new();
    let node0 = taffy . new_leaf_with_measure (taffy :: style :: Style { position : taffy :: style :: Position :: Absolute , max_size : taffy :: geometry :: Size { width : auto () , height : taffy :: style :: Dimension :: Points (50f32) , } , aspect_ratio : Some (0.5f32) , .. Default :: default () } , taffy :: node :: MeasureFunc :: Raw (| known_dimensions , available_space | { const TEXT : & str = "HHHH\u{200b}HHHH\u{200b}HHHH\u{200b}HHHH\u{200b}HHHH\u{200b}HHHH\u{200b}HHHH\u{200b}HHHH\u{200b}HHHH" ; super :: measure_standard_text (known_dimensions , available_space , TEXT , super :: WritingMode :: Horizontal , Some (0.5f32)) }) ,) . unwrap () ;
    let node = taffy
        .new_with_children(
            taffy::style::Style {
                display: taffy::style::Display::Flex,
                size: taffy::geometry::Size {
                    width: taffy::style::Dimension::Points(400f32),
                    height: taffy::style::Dimension::Points(300f32),
                },
                ..Default::default()
            },
            &[node0],
        )
        .unwrap();
    taffy.compute_layout(node, taffy::geometry::Size::MAX_CONTENT).unwrap();
}
