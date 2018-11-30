#[cfg(test)]
mod generated {
    #[test]
    fn sample() {
        let layout = stretch::compute(&stretch::style::Node {
            width: stretch::style::Dimension::Points(100.0),
            height: stretch::style::Dimension::Points(100.0),
            children: vec![stretch::style::Node {
                width: stretch::style::Dimension::Points(50.0),
                children: vec![],
                ..Default::default()
            }],
            ..Default::default()
        });

        assert_eq!(layout.width, 100.0);
        assert_eq!(layout.height, 100.0);
        assert_eq!(layout.x, 0.0);
        assert_eq!(layout.y, 0.0);

        assert_eq!(layout.children[0].width, 50.0);
        assert_eq!(layout.children[0].height, 100.0);
        assert_eq!(layout.children[0].x, 0.0);
        assert_eq!(layout.children[0].y, 0.0);
    }

}
