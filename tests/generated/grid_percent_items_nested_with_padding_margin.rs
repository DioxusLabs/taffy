#[test]
fn grid_percent_items_nested_with_padding_margin() {
    #[allow(unused_imports)]
    use taffy::{prelude::*, tree::Layout};
    let mut taffy = taffy::Taffy::new();
    let node000 = taffy.new_leaf(taffy::style::Style {
        size: taffy::geometry::Size { width: taffy::style::Dimension::Percent(0.45f32), height: auto() },
        margin: taffy::geometry::Rect {
            left: taffy::style::LengthPercentageAuto::Percent(0.05f32),
            right: taffy::style::LengthPercentageAuto::Percent(0.05f32),
            top: taffy::style::LengthPercentageAuto::Percent(0.05f32),
            bottom: taffy::style::LengthPercentageAuto::Percent(0.05f32),
        },
        padding: taffy::geometry::Rect {
            left: taffy::style::LengthPercentage::Length(3f32),
            right: taffy::style::LengthPercentage::Length(3f32),
            top: taffy::style::LengthPercentage::Length(3f32),
            bottom: taffy::style::LengthPercentage::Length(3f32),
        },
        ..Default::default()
    });
    let node00 = taffy.new_with_children(
        taffy::style::Style {
            display: taffy::style::Display::Grid,
            size: taffy::geometry::Size { width: taffy::style::Dimension::Percent(0.5f32), height: auto() },
            margin: taffy::geometry::Rect {
                left: taffy::style::LengthPercentageAuto::Length(5f32),
                right: taffy::style::LengthPercentageAuto::Length(5f32),
                top: taffy::style::LengthPercentageAuto::Length(5f32),
                bottom: taffy::style::LengthPercentageAuto::Length(5f32),
            },
            padding: taffy::geometry::Rect {
                left: taffy::style::LengthPercentage::Percent(0.03f32),
                right: taffy::style::LengthPercentage::Percent(0.03f32),
                top: taffy::style::LengthPercentage::Percent(0.03f32),
                bottom: taffy::style::LengthPercentage::Percent(0.03f32),
            },
            ..Default::default()
        },
        &[node000],
    );
    let node0 = taffy.new_with_children(
        taffy::style::Style {
            display: taffy::style::Display::Grid,
            min_size: taffy::geometry::Size { width: taffy::style::Dimension::Percent(0.6f32), height: auto() },
            margin: taffy::geometry::Rect {
                left: taffy::style::LengthPercentageAuto::Length(5f32),
                right: taffy::style::LengthPercentageAuto::Length(5f32),
                top: taffy::style::LengthPercentageAuto::Length(5f32),
                bottom: taffy::style::LengthPercentageAuto::Length(5f32),
            },
            padding: taffy::geometry::Rect {
                left: taffy::style::LengthPercentage::Length(3f32),
                right: taffy::style::LengthPercentage::Length(3f32),
                top: taffy::style::LengthPercentage::Length(3f32),
                bottom: taffy::style::LengthPercentage::Length(3f32),
            },
            ..Default::default()
        },
        &[node00],
    );
    let node1 = taffy.new_leaf(taffy::style::Style { ..Default::default() });
    let node = taffy.new_with_children(
        taffy::style::Style {
            display: taffy::style::Display::Grid,
            grid_template_rows: vec![fr(1f32), fr(4f32)],
            size: taffy::geometry::Size {
                width: taffy::style::Dimension::Length(200f32),
                height: taffy::style::Dimension::Length(200f32),
            },
            ..Default::default()
        },
        &[node0, node1],
    );
    taffy.compute_layout(node, taffy::geometry::Size::MAX_CONTENT);
    println!("\nComputed tree:");
    taffy::util::print_tree(&taffy, node);
    println!();
    let Layout { size, location, .. } = taffy.layout(node);
    assert_eq!(size.width, 200f32, "width of node {:?}. Expected {}. Actual {}", node, 200f32, size.width);
    assert_eq!(size.height, 200f32, "height of node {:?}. Expected {}. Actual {}", node, 200f32, size.height);
    assert_eq!(location.x, 0f32, "x of node {:?}. Expected {}. Actual {}", node, 0f32, location.x);
    assert_eq!(location.y, 0f32, "y of node {:?}. Expected {}. Actual {}", node, 0f32, location.y);
    let Layout { size, location, .. } = taffy.layout(node0);
    assert_eq!(size.width, 190f32, "width of node {:?}. Expected {}. Actual {}", node0, 190f32, size.width);
    assert_eq!(size.height, 41f32, "height of node {:?}. Expected {}. Actual {}", node0, 41f32, size.height);
    assert_eq!(location.x, 5f32, "x of node {:?}. Expected {}. Actual {}", node0, 5f32, location.x);
    assert_eq!(location.y, 5f32, "y of node {:?}. Expected {}. Actual {}", node0, 5f32, location.y);
    let Layout { size, location, .. } = taffy.layout(node00);
    assert_eq!(size.width, 92f32, "width of node {:?}. Expected {}. Actual {}", node00, 92f32, size.width);
    assert_eq!(size.height, 25f32, "height of node {:?}. Expected {}. Actual {}", node00, 25f32, size.height);
    assert_eq!(location.x, 8f32, "x of node {:?}. Expected {}. Actual {}", node00, 8f32, location.x);
    assert_eq!(location.y, 8f32, "y of node {:?}. Expected {}. Actual {}", node00, 8f32, location.y);
    let Layout { size, location, .. } = taffy.layout(node000);
    assert_eq!(size.width, 36f32, "width of node {:?}. Expected {}. Actual {}", node000, 36f32, size.width);
    assert_eq!(size.height, 6f32, "height of node {:?}. Expected {}. Actual {}", node000, 6f32, size.height);
    assert_eq!(location.x, 10f32, "x of node {:?}. Expected {}. Actual {}", node000, 10f32, location.x);
    assert_eq!(location.y, 10f32, "y of node {:?}. Expected {}. Actual {}", node000, 10f32, location.y);
    let Layout { size, location, .. } = taffy.layout(node1);
    assert_eq!(size.width, 200f32, "width of node {:?}. Expected {}. Actual {}", node1, 200f32, size.width);
    assert_eq!(size.height, 149f32, "height of node {:?}. Expected {}. Actual {}", node1, 149f32, size.height);
    assert_eq!(location.x, 0f32, "x of node {:?}. Expected {}. Actual {}", node1, 0f32, location.x);
    assert_eq!(location.y, 51f32, "y of node {:?}. Expected {}. Actual {}", node1, 51f32, location.y);
}
