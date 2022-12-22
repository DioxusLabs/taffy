use taffy::prelude::*;

// Creates three 20px x 20px children, evenly spaced 10px apart from each other
// Thus the container is 80px x 20px.

fn main() -> Result<(), taffy::error::TaffyError> {
    let mut taffy = Taffy::new();

    let child_style = Style { size: Size { width: points(20.0), height: points(20.0) }, ..Default::default() };
    let child0 = taffy.new_leaf(child_style.clone())?;
    let child1 = taffy.new_leaf(child_style.clone())?;
    let child2 = taffy.new_leaf(child_style.clone())?;

    let root = taffy.new_with_children(
        Style { gap: Size { width: points(10.0), height: zero() }, ..Default::default() },
        &[child0, child1, child2],
    )?;

    // Compute layout and print result
    taffy.compute_layout(root, Size::MAX_CONTENT)?;
    taffy::debug::print_tree(&taffy, root);

    Ok(())
}
