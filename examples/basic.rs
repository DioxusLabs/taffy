use sprawl::prelude::*;

fn main() -> Result<(), Error> {
    let mut sprawl = Sprawl::new();

    let child = sprawl.new_with_children(
        Style { size: Size { width: Dimension::Percent(0.5), height: Dimension::Auto }, ..Default::default() },
        &[],
    )?;

    let node = sprawl.new_with_children(
        Style {
            size: Size { width: Dimension::Points(100.0), height: Dimension::Points(100.0) },
            justify_content: JustifyContent::Center,
            ..Default::default()
        },
        &[child],
    )?;

    sprawl.compute_layout(node, Size { height: Number::Defined(100.0), width: Number::Defined(100.0) })?;

    // or just use undefined for 100 x 100
    // sprawl.compute_layout(node, Size::undefined())?;

    println!("node: {:#?}", sprawl.layout(node)?);
    println!("child: {:#?}", sprawl.layout(child)?);

    Ok(())
}
