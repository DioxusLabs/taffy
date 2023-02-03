// This creates a so-called "holy grail" layout using the CSS Grid layout algorithm
// See: https://en.wikipedia.org/wiki/Holy_grail_(web_design)

// NOTE: This example requires the `grid` feature flag to be enabled.

#[cfg(not(feature = "grid"))]
fn main() {
    println!("Error: this example requires the 'grid' feature to be enabled");
    println!("Try:");
    println!("    cargo run --example grid_holy_grail --features grid")
}

#[cfg(feature = "grid")]
fn default<T: Default>() -> T {
    Default::default()
}

#[cfg(feature = "grid")]
fn main() -> Result<(), taffy::error::TaffyError> {
    use taffy::prelude::*;

    let mut taffy = Taffy::new();

    // Setup the grid
    let root_style = Style {
        display: Display::Grid,
        size: Size { width: points(800.0), height: points(600.0) },
        grid_template_columns: vec![points(250.0), fr(1.0), points(250.0)],
        grid_template_rows: vec![points(150.0), fr(1.0), points(150.0)],
        ..default()
    };

    // Define the child nodes
    let header = taffy.new_leaf(Style { grid_row: line(1), grid_column: span(3), ..default() })?;
    let left_sidebar = taffy.new_leaf(Style { grid_row: line(2), grid_column: line(1), ..default() })?;
    let content_area = taffy.new_leaf(Style { grid_row: line(2), grid_column: line(2), ..default() })?;
    let right_sidebar = taffy.new_leaf(Style { grid_row: line(2), grid_column: line(3), ..default() })?;
    let footer = taffy.new_leaf(Style { grid_row: line(3), grid_column: span(3), ..default() })?;

    // Create the container with the children
    let root = taffy.new_with_children(root_style, &[header, left_sidebar, content_area, right_sidebar, footer])?;

    // Compute layout and print result
    taffy.compute_layout(root, Size { width: points(800.0), height: points(600.0) })?;
    taffy::debug::print_tree(&taffy, root);

    Ok(())
}
