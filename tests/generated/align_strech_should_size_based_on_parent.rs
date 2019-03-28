#[test]
fn align_strech_should_size_based_on_parent() {
    let layout = stretch::compute(
        &stretch::style::Node {
            size: stretch::geometry::Size {
                width: stretch::style::Dimension::Points(100.0000),
                height: stretch::style::Dimension::Points(100.0000),
                ..Default::default()
            },
            children: vec![stretch::style::Node {
                justify_content: stretch::style::JustifyContent::Center,
                flex_grow: 0.0000,
                flex_shrink: 1.0000,
                children: vec![stretch::style::Node {
                    flex_grow: 1.0000,
                    flex_shrink: 1.0000,
                    children: vec![stretch::style::Node {
                        size: stretch::geometry::Size {
                            width: stretch::style::Dimension::Points(20.0000),
                            height: stretch::style::Dimension::Points(20.0000),
                            ..Default::default()
                        },
                        ..Default::default()
                    }],
                    ..Default::default()
                }],
                ..Default::default()
            }],
            ..Default::default()
        },
        stretch::geometry::Size::undefined(),
    )
    .unwrap();

    assert_eq!(layout.size.width, 100.0000);
    assert_eq!(layout.size.height, 100.0000);
    assert_eq!(layout.location.x, 0.0000);
    assert_eq!(layout.location.y, 0.0000);

    assert_eq!(layout.children[0].size.width, 20.0000);
    assert_eq!(layout.children[0].size.height, 100.0000);
    assert_eq!(layout.children[0].location.x, 0.0000);
    assert_eq!(layout.children[0].location.y, 0.0000);

    assert_eq!(layout.children[0].children[0].size.width, 20.0000);
    assert_eq!(layout.children[0].children[0].size.height, 100.0000);
    assert_eq!(layout.children[0].children[0].location.x, 0.0000);
    assert_eq!(layout.children[0].children[0].location.y, 0.0000);

    assert_eq!(layout.children[0].children[0].children[0].size.width, 20.0000);
    assert_eq!(layout.children[0].children[0].children[0].size.height, 20.0000);
    assert_eq!(layout.children[0].children[0].children[0].location.x, 0.0000);
    assert_eq!(layout.children[0].children[0].children[0].location.y, 0.0000);
}
