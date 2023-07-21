#[test]
fn bevy_issue_7976_4_level() {
    #[allow(unused_imports)]
    use taffy::{prelude::*, tree::Layout};
    let mut taffy = taffy::Taffy::new();
    let node000 = taffy.new_leaf(taffy::style::Style { ..Default::default() });
    let node00 = taffy.new_with_children(
        taffy::style::Style {
            size: taffy::geometry::Size {
                width: taffy::style::Dimension::Percent(1f32),
                height: taffy::style::Dimension::Percent(1f32),
            },
            padding: taffy::geometry::Rect {
                left: taffy::style::LengthPercentage::Length(5f32),
                right: taffy::style::LengthPercentage::Length(5f32),
                top: taffy::style::LengthPercentage::Length(5f32),
                bottom: taffy::style::LengthPercentage::Length(5f32),
            },
            ..Default::default()
        },
        &[node000],
    );
    let node0 = taffy.new_with_children(
        taffy::style::Style {
            min_size: taffy::geometry::Size {
                width: taffy::style::Dimension::Length(40f32),
                height: taffy::style::Dimension::Length(40f32),
            },
            margin: taffy::geometry::Rect {
                left: taffy::style::LengthPercentageAuto::Length(5f32),
                right: taffy::style::LengthPercentageAuto::Length(5f32),
                top: taffy::style::LengthPercentageAuto::Length(5f32),
                bottom: taffy::style::LengthPercentageAuto::Length(5f32),
            },
            padding: taffy::geometry::Rect {
                left: taffy::style::LengthPercentage::Length(5f32),
                right: taffy::style::LengthPercentage::Length(5f32),
                top: taffy::style::LengthPercentage::Length(5f32),
                bottom: taffy::style::LengthPercentage::Length(5f32),
            },
            ..Default::default()
        },
        &[node00],
    );
    let node = taffy.new_with_children(
        taffy::style::Style {
            align_content: Some(taffy::style::AlignContent::Start),
            size: taffy::geometry::Size {
                width: taffy::style::Dimension::Length(200f32),
                height: taffy::style::Dimension::Length(200f32),
            },
            ..Default::default()
        },
        &[node0],
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
    assert_eq!(size.width, 40f32, "width of node {:?}. Expected {}. Actual {}", node0, 40f32, size.width);
    assert_eq!(size.height, 190f32, "height of node {:?}. Expected {}. Actual {}", node0, 190f32, size.height);
    assert_eq!(location.x, 5f32, "x of node {:?}. Expected {}. Actual {}", node0, 5f32, location.x);
    assert_eq!(location.y, 5f32, "y of node {:?}. Expected {}. Actual {}", node0, 5f32, location.y);
    let Layout { size, location, .. } = taffy.layout(node00);
    assert_eq!(size.width, 30f32, "width of node {:?}. Expected {}. Actual {}", node00, 30f32, size.width);
    assert_eq!(size.height, 180f32, "height of node {:?}. Expected {}. Actual {}", node00, 180f32, size.height);
    assert_eq!(location.x, 5f32, "x of node {:?}. Expected {}. Actual {}", node00, 5f32, location.x);
    assert_eq!(location.y, 5f32, "y of node {:?}. Expected {}. Actual {}", node00, 5f32, location.y);
    let Layout { size, location, .. } = taffy.layout(node000);
    assert_eq!(size.width, 0f32, "width of node {:?}. Expected {}. Actual {}", node000, 0f32, size.width);
    assert_eq!(size.height, 170f32, "height of node {:?}. Expected {}. Actual {}", node000, 170f32, size.height);
    assert_eq!(location.x, 5f32, "x of node {:?}. Expected {}. Actual {}", node000, 5f32, location.x);
    assert_eq!(location.y, 5f32, "y of node {:?}. Expected {}. Actual {}", node000, 5f32, location.y);
}
