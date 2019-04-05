use stretch::geometry::Size;
use stretch::node::Node;
use stretch::style::*;

fn main() {
    let node = Node::new(
        Style {
            size: Size { width: Dimension::Points(100.0), height: Dimension::Points(100.0) },
            justify_content: JustifyContent::Center,
            ..Default::default()
        },
        vec![&Node::new(
            Style { size: Size { width: Dimension::Percent(0.5), height: Dimension::Auto }, ..Default::default() },
            vec![],
        )],
    );

    let layout = node.compute_layout(Size::undefined()).unwrap();

    println!("{:#?}", layout);
}
