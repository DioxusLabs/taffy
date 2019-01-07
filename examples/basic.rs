use stretch::geometry::Size;
use stretch::style::*;

fn main() {
    let node = Node {
        size: Size { width: Dimension::Points(100.0), height: Dimension::Points(100.0) },
        justify_content: JustifyContent::Center,

        children: vec![Node {
            size: Size { width: Dimension::Percent(0.5), height: Dimension::Auto },
            ..Default::default()
        }],

        ..Default::default()
    };

    let layout = stretch::compute(&node, Size::undefined());

    println!("{:#?}", layout);
}
