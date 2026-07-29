//! Tests for the definiteness of flex item main sizes when performing layout on their contents
//! See <https://www.w3.org/TR/css-flexbox-1/#definite-sizes>
#[cfg(test)]
mod flex_definiteness {
    use taffy::prelude::*;
    use taffy::Point;
    use taffy_test_helpers::{new_test_tree, test_measure_function, TestNodeContext, WritingMode};

    // Two words of 10 characters each. With the Ahem font each character is 10x10px, so this
    // text has a min-content width of 100, a max-content width of 200, and a line height of 10.
    const TEXT: &str = "HHHHHHHHHH\u{200B}HHHHHHHHHH";

    fn text_leaf() -> TestNodeContext {
        TestNodeContext::ahem_text(TEXT.to_string(), WritingMode::Horizontal)
    }

    /// A `column` + `wrap` flex container with an auto (indefinite) height must not wrap its items
    /// into multiple columns when nested in another indefinite-height column flex container, even
    /// if the first item has a percentage flex basis (which does not resolve against the
    /// content-derived height of the container).
    ///
    /// Regression test for <https://github.com/DioxusLabs/taffy/issues/999>
    #[test]
    fn indefinite_height_column_wrap_nested_does_not_wrap() {
        let mut taffy = new_test_tree();

        let a = taffy
            .new_leaf_with_context(
                Style { flex_basis: percent(1.0), flex_shrink: 0.0, ..Default::default() },
                text_leaf(),
            )
            .unwrap();
        let b = taffy.new_leaf_with_context(Style::default(), text_leaf()).unwrap();
        let container = taffy
            .new_with_children(
                Style {
                    display: Display::Flex,
                    flex_direction: FlexDirection::Column,
                    flex_wrap: FlexWrap::Wrap,
                    size: Size { width: length(300.0), height: auto() },
                    ..Default::default()
                },
                &[a, b],
            )
            .unwrap();
        let parent = taffy
            .new_with_children(
                Style {
                    display: Display::Flex,
                    flex_direction: FlexDirection::Column,
                    size: Size { width: length(300.0), height: auto() },
                    ..Default::default()
                },
                &[container],
            )
            .unwrap();

        taffy
            .compute_layout_with_measure(
                parent,
                Size { width: AvailableSpace::Definite(300.0), height: AvailableSpace::MaxContent },
                test_measure_function,
            )
            .unwrap();

        // The container's height is indefinite (derived from its own content), so the items are
        // laid out in a single column: each item is 300px wide and one line (10px) tall.
        let container_layout = taffy.layout(container).unwrap();
        assert_eq!(container_layout.size, Size { width: 300.0, height: 20.0 });

        let a_layout = taffy.layout(a).unwrap();
        assert_eq!(a_layout.location, Point { x: 0.0, y: 0.0 });
        assert_eq!(a_layout.size, Size { width: 300.0, height: 10.0 });

        let b_layout = taffy.layout(b).unwrap();
        assert_eq!(b_layout.location, Point { x: 0.0, y: 10.0 });
        assert_eq!(b_layout.size, Size { width: 300.0, height: 10.0 });
    }

    /// In contrast, when the outer flex container has a *definite* height, the post-flexing height
    /// of the nested container is definite, so the percentage flex basis resolves and the container
    /// wraps its items into two columns.
    #[test]
    fn definite_height_column_wrap_nested_wraps() {
        let mut taffy = new_test_tree();

        let a = taffy
            .new_leaf_with_context(
                Style { flex_basis: percent(1.0), flex_shrink: 0.0, ..Default::default() },
                text_leaf(),
            )
            .unwrap();
        let b = taffy.new_leaf_with_context(Style::default(), text_leaf()).unwrap();
        let container = taffy
            .new_with_children(
                Style {
                    display: Display::Flex,
                    flex_direction: FlexDirection::Column,
                    flex_wrap: FlexWrap::Wrap,
                    size: Size { width: length(300.0), height: auto() },
                    ..Default::default()
                },
                &[a, b],
            )
            .unwrap();
        let parent = taffy
            .new_with_children(
                Style {
                    display: Display::Flex,
                    flex_direction: FlexDirection::Column,
                    size: Size { width: length(300.0), height: length(500.0) },
                    ..Default::default()
                },
                &[container],
            )
            .unwrap();

        taffy
            .compute_layout_with_measure(
                parent,
                Size { width: AvailableSpace::Definite(300.0), height: AvailableSpace::Definite(500.0) },
                test_measure_function,
            )
            .unwrap();

        // `a` has `flex-basis: 100%` which resolves against the container's definite height, so it
        // fills the first column on its own and `b` wraps into a second column.
        let a_layout = taffy.layout(a).unwrap();
        assert_eq!(a_layout.location, Point { x: 0.0, y: 0.0 });

        let b_layout = taffy.layout(b).unwrap();
        assert_eq!(b_layout.location.x, 200.0);
        assert_eq!(b_layout.location.y, 0.0);
    }
}
