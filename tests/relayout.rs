use sprawl::style::Dimension;

#[test]
fn relayout() {
    let mut stretch = sprawl::Sprawl::new();
    let node1 = stretch
        .new_node(
            sprawl::style::Style {
                size: sprawl::geometry::Size { width: Dimension::Points(8f32), height: Dimension::Points(80f32) },
                ..Default::default()
            },
            &[],
        )
        .unwrap();
    let node0 = stretch
        .new_node(
            sprawl::style::Style {
                align_self: sprawl::prelude::AlignSelf::Center,
                size: sprawl::geometry::Size { width: Dimension::Auto, height: Dimension::Auto },
                // size: sprawl::geometry::Size { width: Dimension::Percent(1.0), height: Dimension::Percent(1.0) },
                ..Default::default()
            },
            &[node1],
        )
        .unwrap();
    let node = stretch
        .new_node(
            sprawl::style::Style {
                size: sprawl::geometry::Size { width: Dimension::Percent(1f32), height: Dimension::Percent(1f32) },
                ..Default::default()
            },
            &[node0],
        )
        .unwrap();
    println!("0:");
    stretch
        .compute_layout(
            node,
            sprawl::geometry::Size {
                width: sprawl::prelude::Number::Defined(100f32),
                height: sprawl::prelude::Number::Defined(100f32),
            },
        )
        .unwrap();
    let initial = stretch.layout(node).unwrap().location;
    let initial0 = stretch.layout(node0).unwrap().location;
    let initial1 = stretch.layout(node1).unwrap().location;
    for i in 1..10 {
        println!("\n\n{i}:");
        stretch
            .compute_layout(
                node,
                sprawl::geometry::Size {
                    width: sprawl::prelude::Number::Defined(100f32),
                    height: sprawl::prelude::Number::Defined(100f32),
                },
            )
            .unwrap();
        assert_eq!(stretch.layout(node).unwrap().location, initial);
        assert_eq!(stretch.layout(node0).unwrap().location, initial0);
        assert_eq!(stretch.layout(node1).unwrap().location, initial1);
    }
}
