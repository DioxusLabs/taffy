use taffy::prelude::*;

fn main() -> Result<(), taffy::error::TaffyError> {
    let mut taffy = Taffy::new();

    let child = taffy.new_leaf(Style {
        size: Size { width: Dimension::Percent(0.5), height: Dimension::Auto },
        ..Default::default()
    })?;

    let node = taffy.new_with_children(
        Style {
            size: Size { width: Dimension::Points(100.0), height: Dimension::Points(100.0) },
            justify_content: Some(JustifyContent::Center),
            ..Default::default()
        },
        &[child],
    )?;

    taffy.compute_layout(
        node,
        Size { height: AvailableSpace::Definite(100.0), width: AvailableSpace::Definite(100.0) },
    )?;

    // or just use undefined for 100 x 100
    // taffy.compute_layout(node, Size::NONE)?;

    println!("node: {:#?}", taffy.layout(node)?);
    println!("child: {:#?}", taffy.layout(child)?);

    Ok(())
}
