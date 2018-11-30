#[cfg(test)]
mod tests {
    #[test]
    fn test_1() {
        let layout = stretch::compute(&stretch::style::Node {
            width: stretch::style::Dimension::Points(100.0),
            height: stretch::style::Dimension::Points(100.0),
            ..Default::default()
        });

        assert_eq!(layout.width, 100.0);
        assert_eq!(layout.height, 100.0);
    }

    #[test]
    fn test_2() {
        let layout = stretch::compute(&stretch::style::Node {
            width: stretch::style::Dimension::Points(100.0),
            height: stretch::style::Dimension::Points(100.0),

            children: vec![stretch::style::Node {
                width: stretch::style::Dimension::Percent(0.5),

                ..Default::default()
            }],

            ..Default::default()
        });

        assert_eq!(layout.children[0].width, 50.0);
        assert_eq!(layout.children[0].height, 100.0);
    }

    #[test]
    fn test_3() {
        let layout = stretch::compute(&stretch::style::Node {
            width: stretch::style::Dimension::Points(100.0),
            height: stretch::style::Dimension::Points(100.0),

            children: vec![
                stretch::style::Node {
                    width: stretch::style::Dimension::Percent(0.5),

                    ..Default::default()
                },
                stretch::style::Node {
                    width: stretch::style::Dimension::Percent(0.5),

                    ..Default::default()
                },
            ],

            ..Default::default()
        });

        assert_eq!(layout.children[0].width, 50.0);
        assert_eq!(layout.children[0].height, 100.0);
        assert_eq!(layout.children[0].x, 0.0);
        assert_eq!(layout.children[0].y, 0.0);

        assert_eq!(layout.children[1].width, 50.0);
        assert_eq!(layout.children[1].height, 100.0);
        assert_eq!(layout.children[1].x, 50.0);
        assert_eq!(layout.children[1].y, 0.0);
    }

    #[test]
    fn test_4() {
        let layout = stretch::compute(&stretch::style::Node {
            width: stretch::style::Dimension::Points(100.0),
            height: stretch::style::Dimension::Points(100.0),

            children: vec![
                stretch::style::Node {
                    width: stretch::style::Dimension::Points(20.0),
                    flex_grow: 1.0,

                    ..Default::default()
                },
                stretch::style::Node {
                    width: stretch::style::Dimension::Points(20.0),

                    ..Default::default()
                },
            ],

            ..Default::default()
        });

        assert_eq!(layout.children[0].width, 80.0);
        assert_eq!(layout.children[0].height, 100.0);
        assert_eq!(layout.children[0].x, 0.0);
        assert_eq!(layout.children[0].y, 0.0);

        assert_eq!(layout.children[1].width, 20.0);
        assert_eq!(layout.children[1].height, 100.0);
        assert_eq!(layout.children[1].x, 80.0);
        assert_eq!(layout.children[1].y, 0.0);
    }

    #[test]
    fn test_5() {
        let layout = stretch::compute(&stretch::style::Node {
            width: stretch::style::Dimension::Points(100.0),
            height: stretch::style::Dimension::Points(100.0),

            children: vec![
                stretch::style::Node {
                    width: stretch::style::Dimension::Points(100.0),
                    flex_shrink: 1.0,

                    ..Default::default()
                },
                stretch::style::Node {
                    width: stretch::style::Dimension::Points(100.0),
                    flex_shrink: 1.0,

                    ..Default::default()
                },
            ],

            ..Default::default()
        });

        assert_eq!(layout.children[0].width, 50.0);
        assert_eq!(layout.children[0].height, 100.0);
        assert_eq!(layout.children[0].x, 0.0);
        assert_eq!(layout.children[0].y, 0.0);

        assert_eq!(layout.children[1].width, 50.0);
        assert_eq!(layout.children[1].height, 100.0);
        assert_eq!(layout.children[1].x, 50.0);
        assert_eq!(layout.children[1].y, 0.0);
    }

    #[test]
    fn test_6() {
        let layout = stretch::compute(&stretch::style::Node {
            width: stretch::style::Dimension::Points(100.0),
            height: stretch::style::Dimension::Points(100.0),

            children: vec![
                stretch::style::Node {
                    width: stretch::style::Dimension::Points(50.0),

                    ..Default::default()
                },
                stretch::style::Node {
                    width: stretch::style::Dimension::Points(20.0),
                    margin: stretch::style::Edges {
                        start: stretch::style::Dimension::Points(30.0),
                        ..Default::default()
                    },

                    ..Default::default()
                },
            ],

            ..Default::default()
        });

        assert_eq!(layout.children[0].width, 50.0);
        assert_eq!(layout.children[0].height, 100.0);
        assert_eq!(layout.children[0].x, 0.0);
        assert_eq!(layout.children[0].y, 0.0);

        assert_eq!(layout.children[1].width, 20.0);
        assert_eq!(layout.children[1].height, 100.0);
        assert_eq!(layout.children[1].x, 80.0);
        assert_eq!(layout.children[1].y, 0.0);
    }

    #[test]
    fn test_7() {
        let layout = stretch::compute(&stretch::style::Node {
            width: stretch::style::Dimension::Points(100.0),
            height: stretch::style::Dimension::Points(100.0),
            padding: stretch::style::Edges {
                start: stretch::style::Dimension::Points(10.0),
                end: stretch::style::Dimension::Points(10.0),
                top: stretch::style::Dimension::Points(10.0),
                bottom: stretch::style::Dimension::Points(10.0),
            },

            children: vec![
                stretch::style::Node {
                    width: stretch::style::Dimension::Points(40.0),

                    ..Default::default()
                },
                stretch::style::Node {
                    width: stretch::style::Dimension::Points(40.0),

                    ..Default::default()
                },
            ],

            ..Default::default()
        });

        assert_eq!(layout.children[0].width, 40.0);
        assert_eq!(layout.children[0].height, 80.0);
        assert_eq!(layout.children[0].x, 10.0);
        assert_eq!(layout.children[0].y, 10.0);

        assert_eq!(layout.children[1].width, 40.0);
        assert_eq!(layout.children[1].height, 80.0);
        assert_eq!(layout.children[1].x, 50.0);
        assert_eq!(layout.children[1].y, 10.0);
    }

    #[test]
    fn test_8() {
        let layout = stretch::compute(&stretch::style::Node {
            width: stretch::style::Dimension::Points(100.0),
            height: stretch::style::Dimension::Points(100.0),
            border: stretch::style::Edges {
                start: stretch::style::Dimension::Points(10.0),
                end: stretch::style::Dimension::Points(10.0),
                top: stretch::style::Dimension::Points(10.0),
                bottom: stretch::style::Dimension::Points(10.0),
            },

            children: vec![
                stretch::style::Node {
                    width: stretch::style::Dimension::Points(40.0),

                    ..Default::default()
                },
                stretch::style::Node {
                    width: stretch::style::Dimension::Points(40.0),

                    ..Default::default()
                },
            ],

            ..Default::default()
        });

        assert_eq!(layout.children[0].width, 40.0);
        assert_eq!(layout.children[0].height, 80.0);
        assert_eq!(layout.children[0].x, 10.0);
        assert_eq!(layout.children[0].y, 10.0);

        assert_eq!(layout.children[1].width, 40.0);
        assert_eq!(layout.children[1].height, 80.0);
        assert_eq!(layout.children[1].x, 50.0);
        assert_eq!(layout.children[1].y, 10.0);
    }

    #[test]
    fn test_9() {
        let layout = stretch::compute(&stretch::style::Node {
            width: stretch::style::Dimension::Points(100.0),
            height: stretch::style::Dimension::Points(100.0),
            justify_content: stretch::style::JustifyContent::Center,

            children: vec![
                stretch::style::Node {
                    width: stretch::style::Dimension::Points(20.0),
                    height: stretch::style::Dimension::Points(20.0),

                    ..Default::default()
                },
                stretch::style::Node {
                    width: stretch::style::Dimension::Points(20.0),
                    height: stretch::style::Dimension::Points(20.0),

                    ..Default::default()
                },
            ],

            ..Default::default()
        });

        assert_eq!(layout.children[0].width, 20.0);
        assert_eq!(layout.children[0].height, 20.0);
        assert_eq!(layout.children[0].x, 30.0);
        assert_eq!(layout.children[0].y, 0.0);

        assert_eq!(layout.children[1].width, 20.0);
        assert_eq!(layout.children[1].height, 20.0);
        assert_eq!(layout.children[1].x, 50.0);
        assert_eq!(layout.children[1].y, 0.0);
    }

    #[test]
    fn test_10() {
        let layout = stretch::compute(&stretch::style::Node {
            width: stretch::style::Dimension::Points(100.0),
            height: stretch::style::Dimension::Points(100.0),
            justify_content: stretch::style::JustifyContent::FlexEnd,

            children: vec![
                stretch::style::Node {
                    width: stretch::style::Dimension::Points(20.0),
                    height: stretch::style::Dimension::Points(20.0),

                    ..Default::default()
                },
                stretch::style::Node {
                    width: stretch::style::Dimension::Points(20.0),
                    height: stretch::style::Dimension::Points(20.0),

                    ..Default::default()
                },
            ],

            ..Default::default()
        });

        assert_eq!(layout.children[0].width, 20.0);
        assert_eq!(layout.children[0].height, 20.0);
        assert_eq!(layout.children[0].x, 60.0);
        assert_eq!(layout.children[0].y, 0.0);

        assert_eq!(layout.children[1].width, 20.0);
        assert_eq!(layout.children[1].height, 20.0);
        assert_eq!(layout.children[1].x, 80.0);
        assert_eq!(layout.children[1].y, 0.0);
    }

    #[test]
    fn test_11() {
        let layout = stretch::compute(&stretch::style::Node {
            width: stretch::style::Dimension::Points(100.0),
            height: stretch::style::Dimension::Points(100.0),
            justify_content: stretch::style::JustifyContent::SpaceBetween,

            children: vec![
                stretch::style::Node {
                    width: stretch::style::Dimension::Points(20.0),
                    height: stretch::style::Dimension::Points(20.0),

                    ..Default::default()
                },
                stretch::style::Node {
                    width: stretch::style::Dimension::Points(20.0),
                    height: stretch::style::Dimension::Points(20.0),

                    ..Default::default()
                },
            ],

            ..Default::default()
        });

        assert_eq!(layout.children[0].width, 20.0);
        assert_eq!(layout.children[0].height, 20.0);
        assert_eq!(layout.children[0].x, 0.0);
        assert_eq!(layout.children[0].y, 0.0);

        assert_eq!(layout.children[1].width, 20.0);
        assert_eq!(layout.children[1].height, 20.0);
        assert_eq!(layout.children[1].x, 80.0);
        assert_eq!(layout.children[1].y, 0.0);
    }

    #[test]
    fn test_12() {
        let layout = stretch::compute(&stretch::style::Node {
            width: stretch::style::Dimension::Points(100.0),
            height: stretch::style::Dimension::Points(100.0),
            align_items: stretch::style::AlignItems::FlexEnd,

            children: vec![
                stretch::style::Node {
                    width: stretch::style::Dimension::Points(20.0),
                    height: stretch::style::Dimension::Points(20.0),

                    ..Default::default()
                },
                stretch::style::Node {
                    width: stretch::style::Dimension::Points(20.0),
                    height: stretch::style::Dimension::Points(20.0),

                    ..Default::default()
                },
            ],

            ..Default::default()
        });

        assert_eq!(layout.children[0].width, 20.0);
        assert_eq!(layout.children[0].height, 20.0);
        assert_eq!(layout.children[0].x, 0.0);
        assert_eq!(layout.children[0].y, 80.0);

        assert_eq!(layout.children[1].width, 20.0);
        assert_eq!(layout.children[1].height, 20.0);
        assert_eq!(layout.children[1].x, 20.0);
        assert_eq!(layout.children[1].y, 80.0);
    }

    #[test]
    fn test_13() {
        let layout = stretch::compute(&stretch::style::Node {
            width: stretch::style::Dimension::Points(100.0),
            height: stretch::style::Dimension::Points(100.0),
            align_items: stretch::style::AlignItems::Center,

            children: vec![
                stretch::style::Node {
                    width: stretch::style::Dimension::Points(20.0),
                    height: stretch::style::Dimension::Points(20.0),

                    ..Default::default()
                },
                stretch::style::Node {
                    width: stretch::style::Dimension::Points(20.0),
                    height: stretch::style::Dimension::Points(20.0),

                    ..Default::default()
                },
            ],

            ..Default::default()
        });

        assert_eq!(layout.children[0].width, 20.0);
        assert_eq!(layout.children[0].height, 20.0);
        assert_eq!(layout.children[0].x, 0.0);
        assert_eq!(layout.children[0].y, 40.0);

        assert_eq!(layout.children[1].width, 20.0);
        assert_eq!(layout.children[1].height, 20.0);
        assert_eq!(layout.children[1].x, 20.0);
        assert_eq!(layout.children[1].y, 40.0);
    }

    #[test]
    fn test_14() {
        let layout = stretch::compute(&stretch::style::Node {
            width: stretch::style::Dimension::Points(100.0),
            height: stretch::style::Dimension::Points(100.0),
            align_items: stretch::style::AlignItems::Center,

            children: vec![
                stretch::style::Node {
                    width: stretch::style::Dimension::Points(20.0),
                    height: stretch::style::Dimension::Points(20.0),

                    ..Default::default()
                },
                stretch::style::Node {
                    width: stretch::style::Dimension::Points(20.0),
                    height: stretch::style::Dimension::Points(20.0),
                    align_self: stretch::style::AlignSelf::FlexStart,

                    ..Default::default()
                },
            ],

            ..Default::default()
        });

        assert_eq!(layout.children[0].width, 20.0);
        assert_eq!(layout.children[0].height, 20.0);
        assert_eq!(layout.children[0].x, 0.0);
        assert_eq!(layout.children[0].y, 40.0);

        assert_eq!(layout.children[1].width, 20.0);
        assert_eq!(layout.children[1].height, 20.0);
        assert_eq!(layout.children[1].x, 20.0);
        assert_eq!(layout.children[1].y, 0.0);
    }

    #[test]
    fn test_15() {
        let layout = stretch::compute(&stretch::style::Node {
            width: stretch::style::Dimension::Points(100.0),

            children: vec![
                stretch::style::Node {
                    width: stretch::style::Dimension::Points(40.0),
                    height: stretch::style::Dimension::Points(40.0),

                    ..Default::default()
                },
                stretch::style::Node {
                    width: stretch::style::Dimension::Points(40.0),
                    height: stretch::style::Dimension::Points(40.0),

                    ..Default::default()
                },
            ],

            ..Default::default()
        });

        assert_eq!(layout.height, 40.0);

        assert_eq!(layout.children[0].width, 40.0);
        assert_eq!(layout.children[0].height, 40.0);
        assert_eq!(layout.children[0].x, 0.0);
        assert_eq!(layout.children[0].y, 0.0);

        assert_eq!(layout.children[1].width, 40.0);
        assert_eq!(layout.children[1].height, 40.0);
        assert_eq!(layout.children[1].x, 40.0);
        assert_eq!(layout.children[1].y, 0.0);
    }

    #[test]
    fn test_16() {
        let layout = stretch::compute(&stretch::style::Node {
            width: stretch::style::Dimension::Points(100.0),
            wrap: stretch::style::Wrap::Wrap,

            children: vec![
                stretch::style::Node {
                    width: stretch::style::Dimension::Points(40.0),
                    height: stretch::style::Dimension::Points(40.0),

                    ..Default::default()
                },
                stretch::style::Node {
                    width: stretch::style::Dimension::Points(40.0),
                    height: stretch::style::Dimension::Points(40.0),

                    ..Default::default()
                },
                stretch::style::Node {
                    width: stretch::style::Dimension::Points(40.0),
                    height: stretch::style::Dimension::Points(40.0),

                    ..Default::default()
                },
                stretch::style::Node {
                    width: stretch::style::Dimension::Points(40.0),
                    height: stretch::style::Dimension::Points(40.0),

                    ..Default::default()
                },
                stretch::style::Node {
                    width: stretch::style::Dimension::Points(40.0),
                    height: stretch::style::Dimension::Points(40.0),

                    ..Default::default()
                },
            ],

            ..Default::default()
        });

        assert_eq!(layout.height, 120.0);

        assert_eq!(layout.children[0].width, 40.0);
        assert_eq!(layout.children[0].height, 40.0);
        assert_eq!(layout.children[0].x, 0.0);
        assert_eq!(layout.children[0].y, 0.0);

        assert_eq!(layout.children[1].width, 40.0);
        assert_eq!(layout.children[1].height, 40.0);
        assert_eq!(layout.children[1].x, 40.0);
        assert_eq!(layout.children[1].y, 0.0);

        assert_eq!(layout.children[2].width, 40.0);
        assert_eq!(layout.children[2].height, 40.0);
        assert_eq!(layout.children[2].x, 0.0);
        assert_eq!(layout.children[2].y, 40.0);

        assert_eq!(layout.children[3].width, 40.0);
        assert_eq!(layout.children[3].height, 40.0);
        assert_eq!(layout.children[3].x, 40.0);
        assert_eq!(layout.children[3].y, 40.0);

        assert_eq!(layout.children[4].width, 40.0);
        assert_eq!(layout.children[4].height, 40.0);
        assert_eq!(layout.children[4].x, 0.0);
        assert_eq!(layout.children[4].y, 80.0);
    }

    #[test]
    fn test_17() {
        let layout = stretch::compute(&stretch::style::Node {
            width: stretch::style::Dimension::Points(100.0),
            height: stretch::style::Dimension::Points(100.0),
            align_content: stretch::style::AlignContent::Center,
            wrap: stretch::style::Wrap::Wrap,

            children: vec![
                stretch::style::Node {
                    width: stretch::style::Dimension::Points(40.0),
                    height: stretch::style::Dimension::Points(40.0),

                    ..Default::default()
                },
                stretch::style::Node {
                    width: stretch::style::Dimension::Points(40.0),
                    height: stretch::style::Dimension::Points(40.0),

                    ..Default::default()
                },
                stretch::style::Node {
                    width: stretch::style::Dimension::Points(40.0),
                    height: stretch::style::Dimension::Points(40.0),

                    ..Default::default()
                },
                stretch::style::Node {
                    width: stretch::style::Dimension::Points(40.0),
                    height: stretch::style::Dimension::Points(40.0),

                    ..Default::default()
                },
            ],

            ..Default::default()
        });

        assert_eq!(layout.height, 100.0);

        assert_eq!(layout.children[0].width, 40.0);
        assert_eq!(layout.children[0].height, 40.0);
        assert_eq!(layout.children[0].x, 0.0);
        assert_eq!(layout.children[0].y, 10.0);

        assert_eq!(layout.children[1].width, 40.0);
        assert_eq!(layout.children[1].height, 40.0);
        assert_eq!(layout.children[1].x, 40.0);
        assert_eq!(layout.children[1].y, 10.0);

        assert_eq!(layout.children[2].width, 40.0);
        assert_eq!(layout.children[2].height, 40.0);
        assert_eq!(layout.children[2].x, 0.0);
        assert_eq!(layout.children[2].y, 50.0);

        assert_eq!(layout.children[3].width, 40.0);
        assert_eq!(layout.children[3].height, 40.0);
        assert_eq!(layout.children[3].x, 40.0);
        assert_eq!(layout.children[3].y, 50.0);
    }
}
