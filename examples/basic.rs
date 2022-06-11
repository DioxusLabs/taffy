use taffy::prelude::*;

fn main() -> Result<(), taffy::error::InvalidNode> {
    let mut taffy = Taffy::new();

    let child = taffy.new_with_children(
        FlexboxLayout { size: Size { width: Dimension::Percent(0.5), height: Dimension::Auto }, ..Default::default() },
        &[],
    )?;

    let node = taffy.new_with_children(
        FlexboxLayout {
            size: Size { width: Dimension::Points(100.0), height: Dimension::Points(100.0) },
            justify_content: JustifyContent::Center,
            ..Default::default()
        },
        &[child],
    )?;

    taffy.compute_layout(node, Size { height: Number::Defined(100.0), width: Number::Defined(100.0) })?;

    // or just use undefined for 100 x 100
    // taffy.compute_layout(node, Size::undefined())?;

    println!("node: {:#?}", taffy.layout(node)?);
    println!("child: {:#?}", taffy.layout(child)?);

    Ok(())
}
