#[test]
fn wrapped_column_max_height() {
    let mut sprawl = sprawl::Sprawl::new();
    let node0 = sprawl
        .new_with_children(
            sprawl::style::Style {
                size: sprawl::geometry::Size {
                    width: sprawl::style::Dimension::Points(100f32),
                    height: sprawl::style::Dimension::Points(500f32),
                    ..Default::default()
                },
                max_size: sprawl::geometry::Size {
                    height: sprawl::style::Dimension::Points(200f32),
                    ..Default::default()
                },
                ..Default::default()
            },
            &[],
        )
        .unwrap();
    let node1 = sprawl
        .new_with_children(
            sprawl::style::Style {
                size: sprawl::geometry::Size {
                    width: sprawl::style::Dimension::Points(200f32),
                    height: sprawl::style::Dimension::Points(200f32),
                    ..Default::default()
                },
                margin: sprawl::geometry::Rect {
                    start: sprawl::style::Dimension::Points(20f32),
                    end: sprawl::style::Dimension::Points(20f32),
                    top: sprawl::style::Dimension::Points(20f32),
                    bottom: sprawl::style::Dimension::Points(20f32),
                    ..Default::default()
                },
                ..Default::default()
            },
            &[],
        )
        .unwrap();
    let node2 = sprawl
        .new_with_children(
            sprawl::style::Style {
                size: sprawl::geometry::Size {
                    width: sprawl::style::Dimension::Points(100f32),
                    height: sprawl::style::Dimension::Points(100f32),
                    ..Default::default()
                },
                ..Default::default()
            },
            &[],
        )
        .unwrap();
    let node = sprawl
        .new_with_children(
            sprawl::style::Style {
                flex_direction: sprawl::style::FlexDirection::Column,
                flex_wrap: sprawl::style::FlexWrap::Wrap,
                align_items: sprawl::style::AlignItems::Center,
                align_content: sprawl::style::AlignContent::Center,
                justify_content: sprawl::style::JustifyContent::Center,
                size: sprawl::geometry::Size {
                    width: sprawl::style::Dimension::Points(700f32),
                    height: sprawl::style::Dimension::Points(500f32),
                    ..Default::default()
                },
                ..Default::default()
            },
            &[node0, node1, node2],
        )
        .unwrap();
    sprawl.compute_layout(node, sprawl::geometry::Size::undefined()).unwrap();
    assert_eq!(sprawl.layout(node).unwrap().size.width, 700f32);
    assert_eq!(sprawl.layout(node).unwrap().size.height, 500f32);
    assert_eq!(sprawl.layout(node).unwrap().location.x, 0f32);
    assert_eq!(sprawl.layout(node).unwrap().location.y, 0f32);
    assert_eq!(sprawl.layout(node0).unwrap().size.width, 100f32);
    assert_eq!(sprawl.layout(node0).unwrap().size.height, 200f32);
    assert_eq!(sprawl.layout(node0).unwrap().location.x, 250f32);
    assert_eq!(sprawl.layout(node0).unwrap().location.y, 30f32);
    assert_eq!(sprawl.layout(node1).unwrap().size.width, 200f32);
    assert_eq!(sprawl.layout(node1).unwrap().size.height, 200f32);
    assert_eq!(sprawl.layout(node1).unwrap().location.x, 200f32);
    assert_eq!(sprawl.layout(node1).unwrap().location.y, 250f32);
    assert_eq!(sprawl.layout(node2).unwrap().size.width, 100f32);
    assert_eq!(sprawl.layout(node2).unwrap().size.height, 100f32);
    assert_eq!(sprawl.layout(node2).unwrap().location.x, 420f32);
    assert_eq!(sprawl.layout(node2).unwrap().location.y, 200f32);
}
