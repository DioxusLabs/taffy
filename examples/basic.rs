use taffy::prelude::*;

fn main() -> Result<(), taffy::TaffyError> {
    let mut taffy: TaffyTree<()> = TaffyTree::new();

    let child = taffy.new_leaf(Style {
        size: Size { width: Dimension::Percent(0.5), height: Dimension::Auto },
        ..Default::default()
    })?;

    let node = taffy.new_with_children(
        Style {
            size: Size { width: Dimension::Length(100.0), height: Dimension::Length(100.0) },
            justify_content: Some(JustifyContent::Center),
            ..Default::default()
        },
        &[child],
    )?;

    println!("Compute layout with 100x100 viewport:");
    taffy.compute_layout(
        node,
        Size { height: AvailableSpace::Definite(100.0), width: AvailableSpace::Definite(100.0) },
    )?;
    println!("node: {:#?}", taffy.layout(node)?);
    println!("child: {:#?}", taffy.layout(child)?);

    println!("Compute layout with undefined (infinite) viewport:");
    taffy.compute_layout(node, Size::MAX_CONTENT)?;
    println!("node: {:#?}", taffy.layout(node)?);
    println!("child: {:#?}", taffy.layout(child)?);

    Ok(())
}
