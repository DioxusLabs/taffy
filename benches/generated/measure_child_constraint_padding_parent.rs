pub fn compute() {
    #[allow(unused_imports)]
    use taffy::prelude::*;
    let mut taffy = taffy::Taffy::new();
    let node0 = taffy . new_leaf_with_measure (taffy :: style :: Style { .. Default :: default () } , taffy :: node :: MeasureFunc :: Raw (| known_dimensions , available_space | { const TEXT : & str = "HHHHHHHHHH\u{200b}HHHHHHHHHH\u{200b}HHHHHHHHHH\u{200b}HHHHHHHHHH\u{200b}HHHHHHHHHH\u{200b}HHHHHHHHHH\u{200b}HHHHHHHHHH\u{200b}HHHHHHHHHH\u{200b}HHHHHHHHHH\u{200b}HHHHHHHHHH" ; super :: measure_standard_text (known_dimensions , available_space , TEXT , super :: WritingMode :: Horizontal , None) }) ,) . unwrap () ;
    let node = taffy
        .new_with_children(
            taffy::style::Style {
                size: taffy::geometry::Size {
                    width: taffy::style::Dimension::Points(50f32),
                    height: taffy::style::Dimension::Auto,
                },
                padding: taffy::geometry::Rect {
                    left: taffy::style::LengthPercentage::Points(10f32),
                    right: taffy::style::LengthPercentage::Points(10f32),
                    top: taffy::style::LengthPercentage::Points(10f32),
                    bottom: taffy::style::LengthPercentage::Points(10f32),
                },
                ..Default::default()
            },
            &[node0],
        )
        .unwrap();
    taffy.compute_layout(node, taffy::geometry::Size::MAX_CONTENT).unwrap();
}
