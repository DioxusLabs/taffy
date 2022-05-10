use stretch::{geometry::Point, style::Dimension};
use stretch2 as stretch;

#[test]
fn simple_child() {
    let mut stretch = stretch::Stretch::new();
    let node0_0 = stretch
        .new_node(
            stretch::style::Style {
                align_self: stretch::prelude::AlignSelf::Center,
                size: stretch::geometry::Size { width: Dimension::Points(10f32), height: Dimension::Points(10f32) },
                ..Default::default()
            },
            &[],
        )
        .unwrap();
    let node0 = stretch
        .new_node(
            stretch::style::Style {
                size: stretch::geometry::Size { width: Dimension::Points(10f32), height: Dimension::Points(10f32) },
                ..Default::default()
            },
            &[node0_0],
        )
        .unwrap();
    let node1_0 = stretch
        .new_node(
            stretch::style::Style {
                align_self: stretch::prelude::AlignSelf::Center,
                size: stretch::geometry::Size { width: Dimension::Points(10f32), height: Dimension::Points(10f32) },
                ..Default::default()
            },
            &[],
        )
        .unwrap();
    let node1_1 = stretch
        .new_node(
            stretch::style::Style {
                align_self: stretch::prelude::AlignSelf::Center,
                size: stretch::geometry::Size { width: Dimension::Points(10f32), height: Dimension::Points(10f32) },
                ..Default::default()
            },
            &[],
        )
        .unwrap();
    let node1 = stretch
        .new_node(
            stretch::style::Style {
                size: stretch::geometry::Size { width: Dimension::Undefined, height: Dimension::Undefined },
                ..Default::default()
            },
            &[node1_0, node1_1],
        )
        .unwrap();
    let node = stretch
        .new_node(
            stretch::style::Style {
                size: stretch::geometry::Size { width: Dimension::Percent(100.0), height: Dimension::Percent(100.0) },
                ..Default::default()
            },
            &[node0, node1],
        )
        .unwrap();
    stretch
        .compute_layout(
            node,
            stretch::geometry::Size {
                width: stretch::prelude::Number::Defined(100f32),
                height: stretch::prelude::Number::Defined(100f32),
            },
        )
        .unwrap();
    assert_eq!(stretch.layout(node).unwrap().location, Point { x: 0.0, y: 0.0 });
    assert_eq!(stretch.layout(node0).unwrap().location, Point { x: 0.0, y: 0.0 });
    assert_eq!(stretch.layout(node1).unwrap().location, Point { x: 10.0, y: 0.0 });
    assert_eq!(stretch.layout(node0_0).unwrap().location, Point { x: 0.0, y: 0.0 });
    // Layout is relative so node1_0 location starts at (0,0) and is not ofset by it's parent location
    assert_eq!(stretch.layout(node1_0).unwrap().location, Point { x: 00.0, y: 0.0 });
    assert_eq!(stretch.layout(node1_1).unwrap().location, Point { x: 10.0, y: 0.0 });
}
