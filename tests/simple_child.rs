use sprawl::{geometry::Point, style::Dimension};

#[test]
fn simple_child() {
    let mut sprawl = sprawl::Sprawl::new();
    let node0_0 = sprawl
        .new_node(
            sprawl::style::Style {
                align_self: sprawl::prelude::AlignSelf::Center,
                size: sprawl::geometry::Size { width: Dimension::Points(10f32), height: Dimension::Points(10f32) },
                ..Default::default()
            },
            &[],
        )
        .unwrap();
    let node0 = sprawl
        .new_node(
            sprawl::style::Style {
                size: sprawl::geometry::Size { width: Dimension::Points(10f32), height: Dimension::Points(10f32) },
                ..Default::default()
            },
            &[node0_0],
        )
        .unwrap();
    let node1_0 = sprawl
        .new_node(
            sprawl::style::Style {
                align_self: sprawl::prelude::AlignSelf::Center,
                size: sprawl::geometry::Size { width: Dimension::Points(10f32), height: Dimension::Points(10f32) },
                ..Default::default()
            },
            &[],
        )
        .unwrap();
    let node1_1 = sprawl
        .new_node(
            sprawl::style::Style {
                align_self: sprawl::prelude::AlignSelf::Center,
                size: sprawl::geometry::Size { width: Dimension::Points(10f32), height: Dimension::Points(10f32) },
                ..Default::default()
            },
            &[],
        )
        .unwrap();
    let node1 = sprawl
        .new_node(
            sprawl::style::Style {
                size: sprawl::geometry::Size { width: Dimension::Undefined, height: Dimension::Undefined },
                ..Default::default()
            },
            &[node1_0, node1_1],
        )
        .unwrap();
    let node = sprawl
        .new_node(
            sprawl::style::Style {
                size: sprawl::geometry::Size { width: Dimension::Percent(100.0), height: Dimension::Percent(100.0) },
                ..Default::default()
            },
            &[node0, node1],
        )
        .unwrap();
    sprawl
        .compute_layout(
            node,
            sprawl::geometry::Size {
                width: sprawl::prelude::Number::Defined(100f32),
                height: sprawl::prelude::Number::Defined(100f32),
            },
        )
        .unwrap();
    assert_eq!(sprawl.layout(node).unwrap().location, Point { x: 0.0, y: 0.0 });
    assert_eq!(sprawl.layout(node0).unwrap().location, Point { x: 0.0, y: 0.0 });
    assert_eq!(sprawl.layout(node1).unwrap().location, Point { x: 10.0, y: 0.0 });
    assert_eq!(sprawl.layout(node0_0).unwrap().location, Point { x: 0.0, y: 0.0 });
    // Layout is relative so node1_0 location starts at (0,0) and is not ofset by it's parent location
    assert_eq!(sprawl.layout(node1_0).unwrap().location, Point { x: 00.0, y: 0.0 });
    assert_eq!(sprawl.layout(node1_1).unwrap().location, Point { x: 10.0, y: 0.0 });
}
