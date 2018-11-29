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

            children: vec![
                stretch::style::Node {
                    width: stretch::style::Dimension::Percent(0.5),

                    ..Default::default()
                },
            ],
          
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
}
