#[cfg(test)]
mod flex_line_count {
    use taffy::prelude::*;
    use taffy_test_helpers::{new_test_tree, test_measure_function, TestNodeContext, WritingMode};

    const TEXT: &str = "HHHHHHHHHH\u{200B}HHHHHHHHHH\u{200B}HHHHHHHHHH";

    fn container_style(flex_wrap: FlexWrap) -> Style {
        Style {
            display: Display::Flex,
            flex_direction: FlexDirection::Column,
            flex_wrap,
            flex_line_count: 2,
            size: Size { width: Dimension::from_length(210.0), height: Dimension::from_length(100.0) },
            gap: Size { width: LengthPercentage::from_length(10.0), height: LengthPercentage::ZERO },
            align_items: Some(AlignItems::START),
            align_content: Some(AlignContent::START),
            ..Default::default()
        }
    }

    fn measure_text_child(flex_wrap: FlexWrap) -> Size<f32> {
        let mut taffy = new_test_tree();
        let text = TestNodeContext::ahem_text(TEXT.to_string(), WritingMode::Horizontal);
        let child = taffy.new_leaf_with_context(Style::default(), text).unwrap();
        let container = taffy.new_with_children(container_style(flex_wrap), &[child]).unwrap();
        taffy.compute_layout_with_measure(container, Size::MAX_CONTENT, test_measure_function).unwrap();
        taffy.layout(child).unwrap().size
    }

    // A multi-line container with `flex-line-count: 2` measures content against the cross-axis
    // available space divided between the requested number of lines (after subtracting the
    // cross-axis gaps): (210 - 10) / 2 = 100.
    #[test]
    fn wrap_line_count_divides_cross_available_space() {
        assert_eq!(measure_text_child(FlexWrap::Wrap), Size { width: 100.0, height: 30.0 });
    }

    #[test]
    fn wrap_reverse_line_count_divides_cross_available_space() {
        assert_eq!(measure_text_child(FlexWrap::WrapReverse), Size { width: 100.0, height: 30.0 });
    }

    // `flex-line-count` has no effect on `nowrap` containers.
    #[test]
    fn nowrap_line_count_has_no_effect() {
        assert_eq!(measure_text_child(FlexWrap::NoWrap), Size { width: 210.0, height: 20.0 });
    }
}
