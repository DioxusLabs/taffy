use sprawl::prelude::*;

fn main() -> Result<(), Error> {
    let mut stretch = Stretch::new();

    // left
    let child_t1 = stretch.new_node(
        Style { size: Size { width: Dimension::Points(5.0), height: Dimension::Points(5.0) }, ..Default::default() },
        &[],
    )?;

    let div1 = stretch.new_node(
        Style {
            size: Size { width: Dimension::Percent(0.5), height: Dimension::Percent(1.0) },
            // justify_content: JustifyContent::Center,
            ..Default::default()
        },
        &[child_t1],
    )?;

    // right
    let child_t2 = stretch.new_node(
        Style { size: Size { width: Dimension::Points(5.0), height: Dimension::Points(5.0) }, ..Default::default() },
        &[],
    )?;

    let div2 = stretch.new_node(
        Style {
            size: Size { width: Dimension::Percent(0.5), height: Dimension::Percent(1.0) },
            // justify_content: JustifyContent::Center,
            ..Default::default()
        },
        &[child_t2],
    )?;

    let container = stretch.new_node(
        Style { size: Size { width: Dimension::Percent(1.0), height: Dimension::Percent(1.0) }, ..Default::default() },
        &[div1, div2],
    )?;

    stretch.compute_layout(container, Size { height: Number::Defined(100.0), width: Number::Defined(100.0) })?;

    println!("node: {:#?}", stretch.layout(container)?);

    println!("div1: {:#?}", stretch.layout(div1)?);
    println!("div2: {:#?}", stretch.layout(div2)?);

    println!("child1: {:#?}", stretch.layout(child_t1)?);
    println!("child2: {:#?}", stretch.layout(child_t2)?);

    Ok(())
}
