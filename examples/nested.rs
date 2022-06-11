use taffy::prelude::*;

fn main() -> Result<(), NodeNotFoundError> {
    let mut taffy = Taffy::new();

    // left
    let child_t1 = taffy.new_with_children(
        FlexboxLayout {
            size: Size { width: Dimension::Points(5.0), height: Dimension::Points(5.0) },
            ..Default::default()
        },
        &[],
    )?;

    let div1 = taffy.new_with_children(
        FlexboxLayout {
            size: Size { width: Dimension::Percent(0.5), height: Dimension::Percent(1.0) },
            // justify_content: JustifyContent::Center,
            ..Default::default()
        },
        &[child_t1],
    )?;

    // right
    let child_t2 = taffy.new_with_children(
        FlexboxLayout {
            size: Size { width: Dimension::Points(5.0), height: Dimension::Points(5.0) },
            ..Default::default()
        },
        &[],
    )?;

    let div2 = taffy.new_with_children(
        FlexboxLayout {
            size: Size { width: Dimension::Percent(0.5), height: Dimension::Percent(1.0) },
            // justify_content: JustifyContent::Center,
            ..Default::default()
        },
        &[child_t2],
    )?;

    let container = taffy.new_with_children(
        FlexboxLayout {
            size: Size { width: Dimension::Percent(1.0), height: Dimension::Percent(1.0) },
            ..Default::default()
        },
        &[div1, div2],
    )?;

    taffy.compute_layout(container, Size { height: Number::Defined(100.0), width: Number::Defined(100.0) })?;

    println!("node: {:#?}", taffy.layout(container)?);

    println!("div1: {:#?}", taffy.layout(div1)?);
    println!("div2: {:#?}", taffy.layout(div2)?);

    println!("child1: {:#?}", taffy.layout(child_t1)?);
    println!("child2: {:#?}", taffy.layout(child_t2)?);

    Ok(())
}
