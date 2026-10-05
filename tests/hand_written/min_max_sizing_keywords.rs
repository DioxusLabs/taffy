//! Tests for sizing keywords in `min_size` and `max_size` that cannot be generated from Chrome:
//!
//! - The `fit-content(<length-percentage>)` function, which Chrome does not accept in sizing properties
//! - The number of times that leaf nodes are measured
#[cfg(test)]
mod min_max_sizing_keywords {
    use taffy::prelude::*;
    use taffy::{NodeId, TaffyTree};
    use taffy_test_helpers::{new_test_tree, test_measure_function, TestNodeContext, WritingMode};

    /// Text with a min-content width of 20px, a max-content width of 80px and a line height of 10px
    fn text() -> TestNodeContext {
        TestNodeContext::ahem_text("HH\u{200b}HH\u{200b}HH\u{200b}HH".into(), WritingMode::Horizontal)
    }

    /// The containers that a child is tested in. Each of them is 200px wide and 100px tall, and
    /// stretches the child's automatic width to 200px.
    fn containers() -> [Style; 3] {
        let size = Size { width: length(200.0), height: length(100.0) };
        [
            Style { display: Display::Block, size, ..Default::default() },
            Style { display: Display::Flex, flex_direction: FlexDirection::Column, size, ..Default::default() },
            Style { display: Display::Grid, size, ..Default::default() },
        ]
    }

    /// Lay out a text child with the given style in a container. Returns the tree and the child.
    fn layout(container: Style, child: Style) -> (TaffyTree<TestNodeContext>, NodeId) {
        let mut taffy = new_test_tree();
        let child = taffy.new_leaf_with_context(child, text()).unwrap();
        let root = taffy.new_with_children(container, &[child]).unwrap();
        taffy.compute_layout_with_measure(root, Size::MAX_CONTENT, test_measure_function).unwrap();
        (taffy, child)
    }

    fn child_size(container: Style, child: Style) -> Size<f32> {
        let (taffy, child) = layout(container, child);
        taffy.layout(child).unwrap().size
    }

    fn measure_count(container: Style, child: Style) -> usize {
        let (taffy, child) = layout(container, child);
        taffy.get_node_context(child).unwrap().count
    }

    #[test]
    fn max_width_fit_content_length() {
        for container in containers() {
            // Clamped between the min-content and max-content widths
            for (limit, expected_width, expected_height) in
                [(10.0, 20.0, 40.0), (50.0, 50.0, 20.0), (500.0, 80.0, 10.0)]
            {
                let style = Style {
                    max_size: Size { width: Dimension::fit_content_px(limit), height: auto() },
                    align_self: Some(AlignSelf::START),
                    ..Default::default()
                };
                let size = child_size(container.clone(), style);
                assert_eq!(size, Size { width: expected_width, height: expected_height }, "{:?}", container.display);
            }
        }
    }

    #[test]
    fn max_width_fit_content_length_clamps_definite_width() {
        for container in containers() {
            let style = Style {
                size: Size { width: length(150.0), height: auto() },
                max_size: Size { width: Dimension::fit_content_px(50.0), height: auto() },
                align_self: Some(AlignSelf::START),
                ..Default::default()
            };
            let size = child_size(container.clone(), style);
            assert_eq!(size, Size { width: 50.0, height: 20.0 }, "{:?}", container.display);
        }
    }

    #[test]
    fn max_width_fit_content_percent() {
        for container in containers() {
            // 25% of the container's width of 200px
            let style = Style {
                max_size: Size { width: Dimension::fit_content_percent(0.25), height: auto() },
                align_self: Some(AlignSelf::START),
                ..Default::default()
            };
            let size = child_size(container.clone(), style);
            assert_eq!(size, Size { width: 50.0, height: 20.0 }, "{:?}", container.display);
        }
    }

    #[test]
    fn min_width_fit_content_length() {
        for container in containers() {
            for (limit, expected_width) in [(10.0, 20.0), (50.0, 50.0), (500.0, 80.0)] {
                let style = Style {
                    size: Size { width: length(5.0), height: auto() },
                    min_size: Size { width: Dimension::fit_content_px(limit), height: auto() },
                    align_self: Some(AlignSelf::START),
                    ..Default::default()
                };
                let size = child_size(container.clone(), style);
                assert_eq!(size.width, expected_width, "{:?}", container.display);
            }
        }
    }

    #[test]
    fn min_width_fit_content_percent() {
        for container in containers() {
            let style = Style {
                size: Size { width: length(5.0), height: auto() },
                min_size: Size { width: Dimension::fit_content_percent(0.25), height: auto() },
                align_self: Some(AlignSelf::START),
                ..Default::default()
            };
            let size = child_size(container.clone(), style);
            assert_eq!(size.width, 50.0, "{:?}", container.display);
        }
    }

    /// The root node's `stretch` min and max sizes resolve against the available space (the viewport)
    #[test]
    fn root_stretch() {
        for display in [Display::Block, Display::Flex, Display::Grid] {
            let mut taffy = new_test_tree();
            let child = taffy
                .new_leaf(Style { size: Size { width: length(500.0), height: length(50.0) }, ..Default::default() })
                .unwrap();
            let style = Style {
                display,
                min_size: Size { width: auto(), height: Dimension::stretch() },
                max_size: Size { width: Dimension::stretch(), height: auto() },
                ..Default::default()
            };
            let root = taffy.new_with_children(style.clone(), &[child]).unwrap();

            let viewport = Size { width: AvailableSpace::Definite(300.0), height: AvailableSpace::Definite(200.0) };
            taffy.compute_layout_with_measure(root, viewport, test_measure_function).unwrap();
            assert_eq!(taffy.layout(root).unwrap().size, Size { width: 300.0, height: 200.0 }, "{display:?}");

            // The keywords behave as the initial value if the available space is indefinite
            taffy.compute_layout_with_measure(root, Size::MAX_CONTENT, test_measure_function).unwrap();
            assert_eq!(taffy.layout(root).unwrap().size, Size { width: 500.0, height: 50.0 }, "{display:?}");
        }
    }

    /// A `stretch` min or max size is resolved without measuring the child, so the child is measured
    /// exactly as many times as it is with the equivalent length
    #[test]
    fn stretch_does_not_add_measurements() {
        for container in containers() {
            for (keyword, length_px) in [
                (Size { width: Dimension::stretch(), height: auto() }, Size { width: length(200.0), height: auto() }),
                (Size { width: auto(), height: Dimension::stretch() }, Size { width: auto(), height: length(100.0) }),
            ] {
                let base = Style {
                    align_self: Some(AlignSelf::START),
                    justify_self: Some(AlignSelf::START),
                    ..Default::default()
                };
                let min_keyword = measure_count(container.clone(), Style { min_size: keyword, ..base.clone() });
                let min_length = measure_count(container.clone(), Style { min_size: length_px, ..base.clone() });
                assert_eq!(min_keyword, min_length, "min {:?}", container.display);
                let max_keyword = measure_count(container.clone(), Style { max_size: keyword, ..base.clone() });
                let max_length = measure_count(container.clone(), Style { max_size: length_px, ..base.clone() });
                assert_eq!(max_keyword, max_length, "max {:?}", container.display);
            }
        }
    }

    /// A content-based min or max width on a child whose width is determined by its content is resolved
    /// by the measurement that determines the child's width, so the child is not measured any more times
    /// than it is without the keyword
    #[test]
    fn content_keywords_do_not_add_measurements_to_content_sized_children() {
        let keywords = [
            Dimension::min_content(),
            Dimension::max_content(),
            Dimension::fit_content(),
            Dimension::fit_content_px(50.0),
        ];
        for container in [containers()[0].clone(), containers()[2].clone()] {
            let base = Style {
                // Shrink-to-fit in both block and grid containers
                float: taffy::Float::Left,
                justify_self: Some(AlignSelf::START),
                ..Default::default()
            };
            let baseline = measure_count(container.clone(), base.clone());
            for keyword in keywords {
                let size = Size { width: keyword, height: auto() };
                let min = measure_count(container.clone(), Style { min_size: size, ..base.clone() });
                assert!(min <= baseline, "min {:?} {:?}: {min} > {baseline}", container.display, keyword);
                let max = measure_count(container.clone(), Style { max_size: size, ..base.clone() });
                assert!(max <= baseline, "max {:?} {:?}: {max} > {baseline}", container.display, keyword);
            }
        }
    }

    /// A content-based min or max width on a child whose width is not determined by its content requires
    /// the child's content to be measured once per bound and containing block size
    #[test]
    fn content_keywords_add_one_measurement_per_bound_to_definite_children() {
        // Grid items are measured both before and after the size of their grid area is known
        for (container, measurements_per_bound) in containers().into_iter().zip([1, 1, 2]) {
            let base = Style {
                size: Size { width: length(150.0), height: auto() },
                align_self: Some(AlignSelf::START),
                ..Default::default()
            };
            let baseline = measure_count(container.clone(), base.clone());
            let min_content = Size { width: Dimension::min_content(), height: auto() };
            let max_content = Size { width: Dimension::max_content(), height: auto() };

            let max = measure_count(container.clone(), Style { max_size: min_content, ..base.clone() });
            assert_eq!(max - baseline, measurements_per_bound, "max {:?}", container.display);

            let min = measure_count(container.clone(), Style { min_size: max_content, ..base.clone() });
            assert_eq!(min - baseline, measurements_per_bound, "min {:?}", container.display);

            let both = measure_count(
                container.clone(),
                Style { min_size: min_content, max_size: max_content, ..base.clone() },
            );
            assert_eq!(both - baseline, 2 * measurements_per_bound, "both {:?}", container.display);
        }
    }
}
