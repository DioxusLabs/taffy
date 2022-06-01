use sprawl::style::Dimension;

#[test]
fn relayout() {
    let mut sprawl = sprawl::Sprawl::new();
    let node1 = sprawl
        .new_node(
            sprawl::style::Style {
                size: sprawl::geometry::Size { width: Dimension::Points(8f32), height: Dimension::Points(80f32) },
                ..Default::default()
            },
            &[],
        )
        .unwrap();
    let node0 = sprawl
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
    let node = sprawl
        .new_node(
            sprawl::style::Style {
                size: sprawl::geometry::Size { width: Dimension::Percent(1f32), height: Dimension::Percent(1f32) },
                ..Default::default()
            },
            &[node0],
        )
        .unwrap();
    println!("0:");
    sprawl
        .compute_layout(
            node,
            sprawl::geometry::Size {
                width: sprawl::prelude::Number::Defined(100f32),
                height: sprawl::prelude::Number::Defined(100f32),
            },
        )
        .unwrap();
    let initial = sprawl.layout(node).unwrap().location;
    let initial0 = sprawl.layout(node0).unwrap().location;
    let initial1 = sprawl.layout(node1).unwrap().location;
    for i in 1..10 {
        println!("\n\n{i}:");
        sprawl
            .compute_layout(
                node,
                sprawl::geometry::Size {
                    width: sprawl::prelude::Number::Defined(100f32),
                    height: sprawl::prelude::Number::Defined(100f32),
                },
            )
            .unwrap();
        assert_eq!(sprawl.layout(node).unwrap().location, initial);
        assert_eq!(sprawl.layout(node0).unwrap().location, initial0);
        assert_eq!(sprawl.layout(node1).unwrap().location, initial1);
    }
}
