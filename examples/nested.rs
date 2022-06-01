use sprawl::prelude::*;

fn main() -> Result<(), Error> {
    let mut sprawl = Sprawl::new();

    // left
    let child_t1 = sprawl.new_node(
        Style { size: Size { width: Dimension::Points(5.0), height: Dimension::Points(5.0) }, ..Default::default() },
        &[],
    )?;

    let div1 = sprawl.new_node(
        Style {
            size: Size { width: Dimension::Percent(0.5), height: Dimension::Percent(1.0) },
            // justify_content: JustifyContent::Center,
            ..Default::default()
        },
        &[child_t1],
    )?;

    // right
    let child_t2 = sprawl.new_node(
        Style { size: Size { width: Dimension::Points(5.0), height: Dimension::Points(5.0) }, ..Default::default() },
        &[],
    )?;

    let div2 = sprawl.new_node(
        Style {
            size: Size { width: Dimension::Percent(0.5), height: Dimension::Percent(1.0) },
            // justify_content: JustifyContent::Center,
            ..Default::default()
        },
        &[child_t2],
    )?;

    let container = sprawl.new_node(
        Style { size: Size { width: Dimension::Percent(1.0), height: Dimension::Percent(1.0) }, ..Default::default() },
        &[div1, div2],
    )?;

    sprawl.compute_layout(container, Size { height: Number::Defined(100.0), width: Number::Defined(100.0) })?;

    println!("node: {:#?}", sprawl.layout(container)?);

    println!("div1: {:#?}", sprawl.layout(div1)?);
    println!("div2: {:#?}", sprawl.layout(div2)?);

    println!("child1: {:#?}", sprawl.layout(child_t1)?);
    println!("child2: {:#?}", sprawl.layout(child_t2)?);

    Ok(())
}
