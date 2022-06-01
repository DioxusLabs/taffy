use sprawl::prelude::*;

fn main() -> Result<(), Error> {
    let mut stretch = Sprawl::new();

    let child = stretch.new_node(
        Style { size: Size { width: Dimension::Percent(0.5), height: Dimension::Auto }, ..Default::default() },
        &[],
    )?;

    let node = stretch.new_node(
        Style {
            size: Size { width: Dimension::Points(100.0), height: Dimension::Points(100.0) },
            justify_content: JustifyContent::Center,
            ..Default::default()
        },
        &[child],
    )?;

    stretch.compute_layout(node, Size { height: Number::Defined(100.0), width: Number::Defined(100.0) })?;

    // or just use undefined for 100 x 100
    // stretch.compute_layout(node, Size::undefined())?;

    println!("node: {:#?}", stretch.layout(node)?);
    println!("child: {:#?}", stretch.layout(child)?);

    Ok(())
}
