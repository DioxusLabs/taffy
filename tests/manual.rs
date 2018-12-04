#[cfg(test)]
mod manual {
    #[test]
    fn wrap() {
        let layout = stretch::compute(&stretch::style::Node {
            width: stretch::style::Dimension::Points(100.0),
            height: stretch::style::Dimension::Points(100.0),

            flexWrap: stretch::style::FlexWrap::WrapReverse,
            align_content: stretch::style::AlignContent::Stretch,

            children: vec![
                stretch::style::Node {
                    width: stretch::style::Dimension::Points(30.0),
                    height: stretch::style::Dimension::Points(30.0),
                    ..Default::default()
                },
                stretch::style::Node {
                    width: stretch::style::Dimension::Points(30.0),
                    height: stretch::style::Dimension::Points(30.0),
                    ..Default::default()
                },
                stretch::style::Node {
                    width: stretch::style::Dimension::Points(30.0),
                    height: stretch::style::Dimension::Points(30.0),
                    ..Default::default()
                },
                stretch::style::Node {
                    width: stretch::style::Dimension::Points(30.0),
                    height: stretch::style::Dimension::Points(30.0),
                    ..Default::default()
                },
            ],
            ..Default::default()
        });

        assert_eq!(layout.width, 100.0);
        assert_eq!(layout.height, 100.0);
        assert_eq!(layout.x, 0.0);
        assert_eq!(layout.y, 0.0);

        assert_eq!(layout.children[0].width, 30.0);
        assert_eq!(layout.children[0].height, 30.0);
        assert_eq!(layout.children[0].x, 0.0);
        assert_eq!(layout.children[0].y, 70.0);

        assert_eq!(layout.children[1].width, 30.0);
        assert_eq!(layout.children[1].height, 30.0);
        assert_eq!(layout.children[1].x, 30.0);
        assert_eq!(layout.children[1].y, 70.0);

        assert_eq!(layout.children[2].width, 30.0);
        assert_eq!(layout.children[2].height, 30.0);
        assert_eq!(layout.children[2].x, 60.0);
        assert_eq!(layout.children[2].y, 70.0);

        assert_eq!(layout.children[3].width, 30.0);
        assert_eq!(layout.children[3].height, 30.0);
        assert_eq!(layout.children[3].x, 0.0);
        assert_eq!(layout.children[3].y, 20.0);
    }
}