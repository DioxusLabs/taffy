#[test]
#[allow(non_snake_case)]
fn grid_auto_fit_definite_percentage__border_box_ltr() {
    #[allow(unused_imports)]
    use taffy::{prelude::*, Layout};
    let mut taffy = crate::new_test_tree();
    let node00 =
        taffy.new_leaf(taffy::style::Style { display: taffy::style::Display::Block, ..Default::default() }).unwrap();
    let node01 =
        taffy.new_leaf(taffy::style::Style { display: taffy::style::Display::Block, ..Default::default() }).unwrap();
    let node02 =
        taffy.new_leaf(taffy::style::Style { display: taffy::style::Display::Block, ..Default::default() }).unwrap();
    let node03 =
        taffy.new_leaf(taffy::style::Style { display: taffy::style::Display::Block, ..Default::default() }).unwrap();
    let node04 =
        taffy.new_leaf(taffy::style::Style { display: taffy::style::Display::Block, ..Default::default() }).unwrap();
    let node05 =
        taffy.new_leaf(taffy::style::Style { display: taffy::style::Display::Block, ..Default::default() }).unwrap();
    let node06 =
        taffy.new_leaf(taffy::style::Style { display: taffy::style::Display::Block, ..Default::default() }).unwrap();
    let node07 =
        taffy.new_leaf(taffy::style::Style { display: taffy::style::Display::Block, ..Default::default() }).unwrap();
    let node0 = taffy
        .new_with_children(
            taffy::style::Style {
                display: taffy::style::Display::Grid,
                gap: taffy::geometry::Size { width: length(10f32), height: length(10f32) },
                grid_template_columns: vec![repeat(RepetitionCount::AutoFill, vec![minmax(length(150f32), fr(1f32))])],
                size: taffy::geometry::Size {
                    width: taffy::style::Dimension::from_percent(1f32),
                    height: taffy::style::Dimension::from_percent(1f32),
                },
                padding: taffy::geometry::Rect {
                    left: length(10f32),
                    right: length(10f32),
                    top: length(10f32),
                    bottom: length(10f32),
                },
                ..Default::default()
            },
            &[node00, node01, node02, node03, node04, node05, node06, node07],
        )
        .unwrap();
    let node = taffy
        .new_with_children(
            taffy::style::Style {
                size: taffy::geometry::Size {
                    width: taffy::style::Dimension::from_length(730f32),
                    height: taffy::style::Dimension::from_length(300f32),
                },
                ..Default::default()
            },
            &[node0],
        )
        .unwrap();
    taffy.compute_layout_with_measure(node, taffy::geometry::Size::MAX_CONTENT, crate::test_measure_function).unwrap();
    println!("\nComputed tree:");
    taffy.print_tree(node);
    println!();
    let layout = taffy.layout(node).unwrap();
    let Layout { size, location, .. } = layout;
    assert_eq!(size.width, 730f32, "width of node {:?}. Expected {}. Actual {}", node, 730f32, size.width);
    assert_eq!(size.height, 300f32, "height of node {:?}. Expected {}. Actual {}", node, 300f32, size.height);
    assert_eq!(location.x, 0f32, "x of node {:?}. Expected {}. Actual {}", node, 0f32, location.x);
    assert_eq!(location.y, 0f32, "y of node {:?}. Expected {}. Actual {}", node, 0f32, location.y);
    let layout = taffy.layout(node0).unwrap();
    let Layout { size, location, .. } = layout;
    assert_eq!(size.width, 730f32, "width of node {:?}. Expected {}. Actual {}", node0, 730f32, size.width);
    assert_eq!(size.height, 300f32, "height of node {:?}. Expected {}. Actual {}", node0, 300f32, size.height);
    assert_eq!(location.x, 0f32, "x of node {:?}. Expected {}. Actual {}", node0, 0f32, location.x);
    assert_eq!(location.y, 0f32, "y of node {:?}. Expected {}. Actual {}", node0, 0f32, location.y);
    let layout = taffy.layout(node00).unwrap();
    let Layout { size, location, .. } = layout;
    assert_eq!(size.width, 170f32, "width of node {:?}. Expected {}. Actual {}", node00, 170f32, size.width);
    assert_eq!(size.height, 135f32, "height of node {:?}. Expected {}. Actual {}", node00, 135f32, size.height);
    assert_eq!(location.x, 10f32, "x of node {:?}. Expected {}. Actual {}", node00, 10f32, location.x);
    assert_eq!(location.y, 10f32, "y of node {:?}. Expected {}. Actual {}", node00, 10f32, location.y);
    let layout = taffy.layout(node01).unwrap();
    let Layout { size, location, .. } = layout;
    assert_eq!(size.width, 170f32, "width of node {:?}. Expected {}. Actual {}", node01, 170f32, size.width);
    assert_eq!(size.height, 135f32, "height of node {:?}. Expected {}. Actual {}", node01, 135f32, size.height);
    assert_eq!(location.x, 190f32, "x of node {:?}. Expected {}. Actual {}", node01, 190f32, location.x);
    assert_eq!(location.y, 10f32, "y of node {:?}. Expected {}. Actual {}", node01, 10f32, location.y);
    let layout = taffy.layout(node02).unwrap();
    let Layout { size, location, .. } = layout;
    assert_eq!(size.width, 170f32, "width of node {:?}. Expected {}. Actual {}", node02, 170f32, size.width);
    assert_eq!(size.height, 135f32, "height of node {:?}. Expected {}. Actual {}", node02, 135f32, size.height);
    assert_eq!(location.x, 370f32, "x of node {:?}. Expected {}. Actual {}", node02, 370f32, location.x);
    assert_eq!(location.y, 10f32, "y of node {:?}. Expected {}. Actual {}", node02, 10f32, location.y);
    let layout = taffy.layout(node03).unwrap();
    let Layout { size, location, .. } = layout;
    assert_eq!(size.width, 170f32, "width of node {:?}. Expected {}. Actual {}", node03, 170f32, size.width);
    assert_eq!(size.height, 135f32, "height of node {:?}. Expected {}. Actual {}", node03, 135f32, size.height);
    assert_eq!(location.x, 550f32, "x of node {:?}. Expected {}. Actual {}", node03, 550f32, location.x);
    assert_eq!(location.y, 10f32, "y of node {:?}. Expected {}. Actual {}", node03, 10f32, location.y);
    let layout = taffy.layout(node04).unwrap();
    let Layout { size, location, .. } = layout;
    assert_eq!(size.width, 170f32, "width of node {:?}. Expected {}. Actual {}", node04, 170f32, size.width);
    assert_eq!(size.height, 135f32, "height of node {:?}. Expected {}. Actual {}", node04, 135f32, size.height);
    assert_eq!(location.x, 10f32, "x of node {:?}. Expected {}. Actual {}", node04, 10f32, location.x);
    assert_eq!(location.y, 155f32, "y of node {:?}. Expected {}. Actual {}", node04, 155f32, location.y);
    let layout = taffy.layout(node05).unwrap();
    let Layout { size, location, .. } = layout;
    assert_eq!(size.width, 170f32, "width of node {:?}. Expected {}. Actual {}", node05, 170f32, size.width);
    assert_eq!(size.height, 135f32, "height of node {:?}. Expected {}. Actual {}", node05, 135f32, size.height);
    assert_eq!(location.x, 190f32, "x of node {:?}. Expected {}. Actual {}", node05, 190f32, location.x);
    assert_eq!(location.y, 155f32, "y of node {:?}. Expected {}. Actual {}", node05, 155f32, location.y);
    let layout = taffy.layout(node06).unwrap();
    let Layout { size, location, .. } = layout;
    assert_eq!(size.width, 170f32, "width of node {:?}. Expected {}. Actual {}", node06, 170f32, size.width);
    assert_eq!(size.height, 135f32, "height of node {:?}. Expected {}. Actual {}", node06, 135f32, size.height);
    assert_eq!(location.x, 370f32, "x of node {:?}. Expected {}. Actual {}", node06, 370f32, location.x);
    assert_eq!(location.y, 155f32, "y of node {:?}. Expected {}. Actual {}", node06, 155f32, location.y);
    let layout = taffy.layout(node07).unwrap();
    let Layout { size, location, .. } = layout;
    assert_eq!(size.width, 170f32, "width of node {:?}. Expected {}. Actual {}", node07, 170f32, size.width);
    assert_eq!(size.height, 135f32, "height of node {:?}. Expected {}. Actual {}", node07, 135f32, size.height);
    assert_eq!(location.x, 550f32, "x of node {:?}. Expected {}. Actual {}", node07, 550f32, location.x);
    assert_eq!(location.y, 155f32, "y of node {:?}. Expected {}. Actual {}", node07, 155f32, location.y);
}

#[test]
#[allow(non_snake_case)]
fn grid_auto_fit_definite_percentage__content_box_ltr() {
    #[allow(unused_imports)]
    use taffy::{prelude::*, Layout};
    let mut taffy = crate::new_test_tree();
    let node00 = taffy
        .new_leaf(taffy::style::Style {
            display: taffy::style::Display::Block,
            box_sizing: taffy::style::BoxSizing::ContentBox,
            ..Default::default()
        })
        .unwrap();
    let node01 = taffy
        .new_leaf(taffy::style::Style {
            display: taffy::style::Display::Block,
            box_sizing: taffy::style::BoxSizing::ContentBox,
            ..Default::default()
        })
        .unwrap();
    let node02 = taffy
        .new_leaf(taffy::style::Style {
            display: taffy::style::Display::Block,
            box_sizing: taffy::style::BoxSizing::ContentBox,
            ..Default::default()
        })
        .unwrap();
    let node03 = taffy
        .new_leaf(taffy::style::Style {
            display: taffy::style::Display::Block,
            box_sizing: taffy::style::BoxSizing::ContentBox,
            ..Default::default()
        })
        .unwrap();
    let node04 = taffy
        .new_leaf(taffy::style::Style {
            display: taffy::style::Display::Block,
            box_sizing: taffy::style::BoxSizing::ContentBox,
            ..Default::default()
        })
        .unwrap();
    let node05 = taffy
        .new_leaf(taffy::style::Style {
            display: taffy::style::Display::Block,
            box_sizing: taffy::style::BoxSizing::ContentBox,
            ..Default::default()
        })
        .unwrap();
    let node06 = taffy
        .new_leaf(taffy::style::Style {
            display: taffy::style::Display::Block,
            box_sizing: taffy::style::BoxSizing::ContentBox,
            ..Default::default()
        })
        .unwrap();
    let node07 = taffy
        .new_leaf(taffy::style::Style {
            display: taffy::style::Display::Block,
            box_sizing: taffy::style::BoxSizing::ContentBox,
            ..Default::default()
        })
        .unwrap();
    let node0 = taffy
        .new_with_children(
            taffy::style::Style {
                display: taffy::style::Display::Grid,
                box_sizing: taffy::style::BoxSizing::ContentBox,
                gap: taffy::geometry::Size { width: length(10f32), height: length(10f32) },
                grid_template_columns: vec![repeat(RepetitionCount::AutoFill, vec![minmax(length(150f32), fr(1f32))])],
                size: taffy::geometry::Size {
                    width: taffy::style::Dimension::from_percent(1f32),
                    height: taffy::style::Dimension::from_percent(1f32),
                },
                padding: taffy::geometry::Rect {
                    left: length(10f32),
                    right: length(10f32),
                    top: length(10f32),
                    bottom: length(10f32),
                },
                ..Default::default()
            },
            &[node00, node01, node02, node03, node04, node05, node06, node07],
        )
        .unwrap();
    let node = taffy
        .new_with_children(
            taffy::style::Style {
                box_sizing: taffy::style::BoxSizing::ContentBox,
                size: taffy::geometry::Size {
                    width: taffy::style::Dimension::from_length(730f32),
                    height: taffy::style::Dimension::from_length(300f32),
                },
                ..Default::default()
            },
            &[node0],
        )
        .unwrap();
    taffy.compute_layout_with_measure(node, taffy::geometry::Size::MAX_CONTENT, crate::test_measure_function).unwrap();
    println!("\nComputed tree:");
    taffy.print_tree(node);
    println!();
    let layout = taffy.layout(node).unwrap();
    let Layout { size, location, .. } = layout;
    assert_eq!(size.width, 730f32, "width of node {:?}. Expected {}. Actual {}", node, 730f32, size.width);
    assert_eq!(size.height, 300f32, "height of node {:?}. Expected {}. Actual {}", node, 300f32, size.height);
    assert_eq!(location.x, 0f32, "x of node {:?}. Expected {}. Actual {}", node, 0f32, location.x);
    assert_eq!(location.y, 0f32, "y of node {:?}. Expected {}. Actual {}", node, 0f32, location.y);
    let layout = taffy.layout(node0).unwrap();
    let Layout { size, location, .. } = layout;
    assert_eq!(size.width, 730f32, "width of node {:?}. Expected {}. Actual {}", node0, 730f32, size.width);
    assert_eq!(size.height, 320f32, "height of node {:?}. Expected {}. Actual {}", node0, 320f32, size.height);
    assert_eq!(location.x, 0f32, "x of node {:?}. Expected {}. Actual {}", node0, 0f32, location.x);
    assert_eq!(location.y, 0f32, "y of node {:?}. Expected {}. Actual {}", node0, 0f32, location.y);
    let layout = taffy.layout(node00).unwrap();
    let Layout { size, location, .. } = layout;
    assert_eq!(size.width, 170f32, "width of node {:?}. Expected {}. Actual {}", node00, 170f32, size.width);
    assert_eq!(size.height, 145f32, "height of node {:?}. Expected {}. Actual {}", node00, 145f32, size.height);
    assert_eq!(location.x, 10f32, "x of node {:?}. Expected {}. Actual {}", node00, 10f32, location.x);
    assert_eq!(location.y, 10f32, "y of node {:?}. Expected {}. Actual {}", node00, 10f32, location.y);
    let layout = taffy.layout(node01).unwrap();
    let Layout { size, location, .. } = layout;
    assert_eq!(size.width, 170f32, "width of node {:?}. Expected {}. Actual {}", node01, 170f32, size.width);
    assert_eq!(size.height, 145f32, "height of node {:?}. Expected {}. Actual {}", node01, 145f32, size.height);
    assert_eq!(location.x, 190f32, "x of node {:?}. Expected {}. Actual {}", node01, 190f32, location.x);
    assert_eq!(location.y, 10f32, "y of node {:?}. Expected {}. Actual {}", node01, 10f32, location.y);
    let layout = taffy.layout(node02).unwrap();
    let Layout { size, location, .. } = layout;
    assert_eq!(size.width, 170f32, "width of node {:?}. Expected {}. Actual {}", node02, 170f32, size.width);
    assert_eq!(size.height, 145f32, "height of node {:?}. Expected {}. Actual {}", node02, 145f32, size.height);
    assert_eq!(location.x, 370f32, "x of node {:?}. Expected {}. Actual {}", node02, 370f32, location.x);
    assert_eq!(location.y, 10f32, "y of node {:?}. Expected {}. Actual {}", node02, 10f32, location.y);
    let layout = taffy.layout(node03).unwrap();
    let Layout { size, location, .. } = layout;
    assert_eq!(size.width, 170f32, "width of node {:?}. Expected {}. Actual {}", node03, 170f32, size.width);
    assert_eq!(size.height, 145f32, "height of node {:?}. Expected {}. Actual {}", node03, 145f32, size.height);
    assert_eq!(location.x, 550f32, "x of node {:?}. Expected {}. Actual {}", node03, 550f32, location.x);
    assert_eq!(location.y, 10f32, "y of node {:?}. Expected {}. Actual {}", node03, 10f32, location.y);
    let layout = taffy.layout(node04).unwrap();
    let Layout { size, location, .. } = layout;
    assert_eq!(size.width, 170f32, "width of node {:?}. Expected {}. Actual {}", node04, 170f32, size.width);
    assert_eq!(size.height, 145f32, "height of node {:?}. Expected {}. Actual {}", node04, 145f32, size.height);
    assert_eq!(location.x, 10f32, "x of node {:?}. Expected {}. Actual {}", node04, 10f32, location.x);
    assert_eq!(location.y, 165f32, "y of node {:?}. Expected {}. Actual {}", node04, 165f32, location.y);
    let layout = taffy.layout(node05).unwrap();
    let Layout { size, location, .. } = layout;
    assert_eq!(size.width, 170f32, "width of node {:?}. Expected {}. Actual {}", node05, 170f32, size.width);
    assert_eq!(size.height, 145f32, "height of node {:?}. Expected {}. Actual {}", node05, 145f32, size.height);
    assert_eq!(location.x, 190f32, "x of node {:?}. Expected {}. Actual {}", node05, 190f32, location.x);
    assert_eq!(location.y, 165f32, "y of node {:?}. Expected {}. Actual {}", node05, 165f32, location.y);
    let layout = taffy.layout(node06).unwrap();
    let Layout { size, location, .. } = layout;
    assert_eq!(size.width, 170f32, "width of node {:?}. Expected {}. Actual {}", node06, 170f32, size.width);
    assert_eq!(size.height, 145f32, "height of node {:?}. Expected {}. Actual {}", node06, 145f32, size.height);
    assert_eq!(location.x, 370f32, "x of node {:?}. Expected {}. Actual {}", node06, 370f32, location.x);
    assert_eq!(location.y, 165f32, "y of node {:?}. Expected {}. Actual {}", node06, 165f32, location.y);
    let layout = taffy.layout(node07).unwrap();
    let Layout { size, location, .. } = layout;
    assert_eq!(size.width, 170f32, "width of node {:?}. Expected {}. Actual {}", node07, 170f32, size.width);
    assert_eq!(size.height, 145f32, "height of node {:?}. Expected {}. Actual {}", node07, 145f32, size.height);
    assert_eq!(location.x, 550f32, "x of node {:?}. Expected {}. Actual {}", node07, 550f32, location.x);
    assert_eq!(location.y, 165f32, "y of node {:?}. Expected {}. Actual {}", node07, 165f32, location.y);
}

#[test]
#[allow(non_snake_case)]
fn grid_auto_fit_definite_percentage__border_box_rtl() {
    #[allow(unused_imports)]
    use taffy::{prelude::*, Layout};
    let mut taffy = crate::new_test_tree();
    let node00 = taffy
        .new_leaf(taffy::style::Style {
            display: taffy::style::Display::Block,
            direction: taffy::style::Direction::Rtl,
            ..Default::default()
        })
        .unwrap();
    let node01 = taffy
        .new_leaf(taffy::style::Style {
            display: taffy::style::Display::Block,
            direction: taffy::style::Direction::Rtl,
            ..Default::default()
        })
        .unwrap();
    let node02 = taffy
        .new_leaf(taffy::style::Style {
            display: taffy::style::Display::Block,
            direction: taffy::style::Direction::Rtl,
            ..Default::default()
        })
        .unwrap();
    let node03 = taffy
        .new_leaf(taffy::style::Style {
            display: taffy::style::Display::Block,
            direction: taffy::style::Direction::Rtl,
            ..Default::default()
        })
        .unwrap();
    let node04 = taffy
        .new_leaf(taffy::style::Style {
            display: taffy::style::Display::Block,
            direction: taffy::style::Direction::Rtl,
            ..Default::default()
        })
        .unwrap();
    let node05 = taffy
        .new_leaf(taffy::style::Style {
            display: taffy::style::Display::Block,
            direction: taffy::style::Direction::Rtl,
            ..Default::default()
        })
        .unwrap();
    let node06 = taffy
        .new_leaf(taffy::style::Style {
            display: taffy::style::Display::Block,
            direction: taffy::style::Direction::Rtl,
            ..Default::default()
        })
        .unwrap();
    let node07 = taffy
        .new_leaf(taffy::style::Style {
            display: taffy::style::Display::Block,
            direction: taffy::style::Direction::Rtl,
            ..Default::default()
        })
        .unwrap();
    let node0 = taffy
        .new_with_children(
            taffy::style::Style {
                display: taffy::style::Display::Grid,
                direction: taffy::style::Direction::Rtl,
                gap: taffy::geometry::Size { width: length(10f32), height: length(10f32) },
                grid_template_columns: vec![repeat(RepetitionCount::AutoFill, vec![minmax(length(150f32), fr(1f32))])],
                size: taffy::geometry::Size {
                    width: taffy::style::Dimension::from_percent(1f32),
                    height: taffy::style::Dimension::from_percent(1f32),
                },
                padding: taffy::geometry::Rect {
                    left: length(10f32),
                    right: length(10f32),
                    top: length(10f32),
                    bottom: length(10f32),
                },
                ..Default::default()
            },
            &[node00, node01, node02, node03, node04, node05, node06, node07],
        )
        .unwrap();
    let node = taffy
        .new_with_children(
            taffy::style::Style {
                direction: taffy::style::Direction::Rtl,
                size: taffy::geometry::Size {
                    width: taffy::style::Dimension::from_length(730f32),
                    height: taffy::style::Dimension::from_length(300f32),
                },
                ..Default::default()
            },
            &[node0],
        )
        .unwrap();
    taffy.compute_layout_with_measure(node, taffy::geometry::Size::MAX_CONTENT, crate::test_measure_function).unwrap();
    println!("\nComputed tree:");
    taffy.print_tree(node);
    println!();
    let layout = taffy.layout(node).unwrap();
    let Layout { size, location, .. } = layout;
    assert_eq!(size.width, 730f32, "width of node {:?}. Expected {}. Actual {}", node, 730f32, size.width);
    assert_eq!(size.height, 300f32, "height of node {:?}. Expected {}. Actual {}", node, 300f32, size.height);
    assert_eq!(location.x, 0f32, "x of node {:?}. Expected {}. Actual {}", node, 0f32, location.x);
    assert_eq!(location.y, 0f32, "y of node {:?}. Expected {}. Actual {}", node, 0f32, location.y);
    let layout = taffy.layout(node0).unwrap();
    let Layout { size, location, .. } = layout;
    assert_eq!(size.width, 730f32, "width of node {:?}. Expected {}. Actual {}", node0, 730f32, size.width);
    assert_eq!(size.height, 300f32, "height of node {:?}. Expected {}. Actual {}", node0, 300f32, size.height);
    assert_eq!(location.x, 0f32, "x of node {:?}. Expected {}. Actual {}", node0, 0f32, location.x);
    assert_eq!(location.y, 0f32, "y of node {:?}. Expected {}. Actual {}", node0, 0f32, location.y);
    let layout = taffy.layout(node00).unwrap();
    let Layout { size, location, .. } = layout;
    assert_eq!(size.width, 170f32, "width of node {:?}. Expected {}. Actual {}", node00, 170f32, size.width);
    assert_eq!(size.height, 135f32, "height of node {:?}. Expected {}. Actual {}", node00, 135f32, size.height);
    assert_eq!(location.x, 550f32, "x of node {:?}. Expected {}. Actual {}", node00, 550f32, location.x);
    assert_eq!(location.y, 10f32, "y of node {:?}. Expected {}. Actual {}", node00, 10f32, location.y);
    let layout = taffy.layout(node01).unwrap();
    let Layout { size, location, .. } = layout;
    assert_eq!(size.width, 170f32, "width of node {:?}. Expected {}. Actual {}", node01, 170f32, size.width);
    assert_eq!(size.height, 135f32, "height of node {:?}. Expected {}. Actual {}", node01, 135f32, size.height);
    assert_eq!(location.x, 370f32, "x of node {:?}. Expected {}. Actual {}", node01, 370f32, location.x);
    assert_eq!(location.y, 10f32, "y of node {:?}. Expected {}. Actual {}", node01, 10f32, location.y);
    let layout = taffy.layout(node02).unwrap();
    let Layout { size, location, .. } = layout;
    assert_eq!(size.width, 170f32, "width of node {:?}. Expected {}. Actual {}", node02, 170f32, size.width);
    assert_eq!(size.height, 135f32, "height of node {:?}. Expected {}. Actual {}", node02, 135f32, size.height);
    assert_eq!(location.x, 190f32, "x of node {:?}. Expected {}. Actual {}", node02, 190f32, location.x);
    assert_eq!(location.y, 10f32, "y of node {:?}. Expected {}. Actual {}", node02, 10f32, location.y);
    let layout = taffy.layout(node03).unwrap();
    let Layout { size, location, .. } = layout;
    assert_eq!(size.width, 170f32, "width of node {:?}. Expected {}. Actual {}", node03, 170f32, size.width);
    assert_eq!(size.height, 135f32, "height of node {:?}. Expected {}. Actual {}", node03, 135f32, size.height);
    assert_eq!(location.x, 10f32, "x of node {:?}. Expected {}. Actual {}", node03, 10f32, location.x);
    assert_eq!(location.y, 10f32, "y of node {:?}. Expected {}. Actual {}", node03, 10f32, location.y);
    let layout = taffy.layout(node04).unwrap();
    let Layout { size, location, .. } = layout;
    assert_eq!(size.width, 170f32, "width of node {:?}. Expected {}. Actual {}", node04, 170f32, size.width);
    assert_eq!(size.height, 135f32, "height of node {:?}. Expected {}. Actual {}", node04, 135f32, size.height);
    assert_eq!(location.x, 550f32, "x of node {:?}. Expected {}. Actual {}", node04, 550f32, location.x);
    assert_eq!(location.y, 155f32, "y of node {:?}. Expected {}. Actual {}", node04, 155f32, location.y);
    let layout = taffy.layout(node05).unwrap();
    let Layout { size, location, .. } = layout;
    assert_eq!(size.width, 170f32, "width of node {:?}. Expected {}. Actual {}", node05, 170f32, size.width);
    assert_eq!(size.height, 135f32, "height of node {:?}. Expected {}. Actual {}", node05, 135f32, size.height);
    assert_eq!(location.x, 370f32, "x of node {:?}. Expected {}. Actual {}", node05, 370f32, location.x);
    assert_eq!(location.y, 155f32, "y of node {:?}. Expected {}. Actual {}", node05, 155f32, location.y);
    let layout = taffy.layout(node06).unwrap();
    let Layout { size, location, .. } = layout;
    assert_eq!(size.width, 170f32, "width of node {:?}. Expected {}. Actual {}", node06, 170f32, size.width);
    assert_eq!(size.height, 135f32, "height of node {:?}. Expected {}. Actual {}", node06, 135f32, size.height);
    assert_eq!(location.x, 190f32, "x of node {:?}. Expected {}. Actual {}", node06, 190f32, location.x);
    assert_eq!(location.y, 155f32, "y of node {:?}. Expected {}. Actual {}", node06, 155f32, location.y);
    let layout = taffy.layout(node07).unwrap();
    let Layout { size, location, .. } = layout;
    assert_eq!(size.width, 170f32, "width of node {:?}. Expected {}. Actual {}", node07, 170f32, size.width);
    assert_eq!(size.height, 135f32, "height of node {:?}. Expected {}. Actual {}", node07, 135f32, size.height);
    assert_eq!(location.x, 10f32, "x of node {:?}. Expected {}. Actual {}", node07, 10f32, location.x);
    assert_eq!(location.y, 155f32, "y of node {:?}. Expected {}. Actual {}", node07, 155f32, location.y);
}

#[test]
#[allow(non_snake_case)]
fn grid_auto_fit_definite_percentage__content_box_rtl() {
    #[allow(unused_imports)]
    use taffy::{prelude::*, Layout};
    let mut taffy = crate::new_test_tree();
    let node00 = taffy
        .new_leaf(taffy::style::Style {
            display: taffy::style::Display::Block,
            box_sizing: taffy::style::BoxSizing::ContentBox,
            direction: taffy::style::Direction::Rtl,
            ..Default::default()
        })
        .unwrap();
    let node01 = taffy
        .new_leaf(taffy::style::Style {
            display: taffy::style::Display::Block,
            box_sizing: taffy::style::BoxSizing::ContentBox,
            direction: taffy::style::Direction::Rtl,
            ..Default::default()
        })
        .unwrap();
    let node02 = taffy
        .new_leaf(taffy::style::Style {
            display: taffy::style::Display::Block,
            box_sizing: taffy::style::BoxSizing::ContentBox,
            direction: taffy::style::Direction::Rtl,
            ..Default::default()
        })
        .unwrap();
    let node03 = taffy
        .new_leaf(taffy::style::Style {
            display: taffy::style::Display::Block,
            box_sizing: taffy::style::BoxSizing::ContentBox,
            direction: taffy::style::Direction::Rtl,
            ..Default::default()
        })
        .unwrap();
    let node04 = taffy
        .new_leaf(taffy::style::Style {
            display: taffy::style::Display::Block,
            box_sizing: taffy::style::BoxSizing::ContentBox,
            direction: taffy::style::Direction::Rtl,
            ..Default::default()
        })
        .unwrap();
    let node05 = taffy
        .new_leaf(taffy::style::Style {
            display: taffy::style::Display::Block,
            box_sizing: taffy::style::BoxSizing::ContentBox,
            direction: taffy::style::Direction::Rtl,
            ..Default::default()
        })
        .unwrap();
    let node06 = taffy
        .new_leaf(taffy::style::Style {
            display: taffy::style::Display::Block,
            box_sizing: taffy::style::BoxSizing::ContentBox,
            direction: taffy::style::Direction::Rtl,
            ..Default::default()
        })
        .unwrap();
    let node07 = taffy
        .new_leaf(taffy::style::Style {
            display: taffy::style::Display::Block,
            box_sizing: taffy::style::BoxSizing::ContentBox,
            direction: taffy::style::Direction::Rtl,
            ..Default::default()
        })
        .unwrap();
    let node0 = taffy
        .new_with_children(
            taffy::style::Style {
                display: taffy::style::Display::Grid,
                box_sizing: taffy::style::BoxSizing::ContentBox,
                direction: taffy::style::Direction::Rtl,
                gap: taffy::geometry::Size { width: length(10f32), height: length(10f32) },
                grid_template_columns: vec![repeat(RepetitionCount::AutoFill, vec![minmax(length(150f32), fr(1f32))])],
                size: taffy::geometry::Size {
                    width: taffy::style::Dimension::from_percent(1f32),
                    height: taffy::style::Dimension::from_percent(1f32),
                },
                padding: taffy::geometry::Rect {
                    left: length(10f32),
                    right: length(10f32),
                    top: length(10f32),
                    bottom: length(10f32),
                },
                ..Default::default()
            },
            &[node00, node01, node02, node03, node04, node05, node06, node07],
        )
        .unwrap();
    let node = taffy
        .new_with_children(
            taffy::style::Style {
                box_sizing: taffy::style::BoxSizing::ContentBox,
                direction: taffy::style::Direction::Rtl,
                size: taffy::geometry::Size {
                    width: taffy::style::Dimension::from_length(730f32),
                    height: taffy::style::Dimension::from_length(300f32),
                },
                ..Default::default()
            },
            &[node0],
        )
        .unwrap();
    taffy.compute_layout_with_measure(node, taffy::geometry::Size::MAX_CONTENT, crate::test_measure_function).unwrap();
    println!("\nComputed tree:");
    taffy.print_tree(node);
    println!();
    let layout = taffy.layout(node).unwrap();
    let Layout { size, location, .. } = layout;
    assert_eq!(size.width, 730f32, "width of node {:?}. Expected {}. Actual {}", node, 730f32, size.width);
    assert_eq!(size.height, 300f32, "height of node {:?}. Expected {}. Actual {}", node, 300f32, size.height);
    assert_eq!(location.x, 0f32, "x of node {:?}. Expected {}. Actual {}", node, 0f32, location.x);
    assert_eq!(location.y, 0f32, "y of node {:?}. Expected {}. Actual {}", node, 0f32, location.y);
    let layout = taffy.layout(node0).unwrap();
    let Layout { size, location, .. } = layout;
    assert_eq!(size.width, 730f32, "width of node {:?}. Expected {}. Actual {}", node0, 730f32, size.width);
    assert_eq!(size.height, 320f32, "height of node {:?}. Expected {}. Actual {}", node0, 320f32, size.height);
    assert_eq!(location.x, 0f32, "x of node {:?}. Expected {}. Actual {}", node0, 0f32, location.x);
    assert_eq!(location.y, 0f32, "y of node {:?}. Expected {}. Actual {}", node0, 0f32, location.y);
    let layout = taffy.layout(node00).unwrap();
    let Layout { size, location, .. } = layout;
    assert_eq!(size.width, 170f32, "width of node {:?}. Expected {}. Actual {}", node00, 170f32, size.width);
    assert_eq!(size.height, 145f32, "height of node {:?}. Expected {}. Actual {}", node00, 145f32, size.height);
    assert_eq!(location.x, 550f32, "x of node {:?}. Expected {}. Actual {}", node00, 550f32, location.x);
    assert_eq!(location.y, 10f32, "y of node {:?}. Expected {}. Actual {}", node00, 10f32, location.y);
    let layout = taffy.layout(node01).unwrap();
    let Layout { size, location, .. } = layout;
    assert_eq!(size.width, 170f32, "width of node {:?}. Expected {}. Actual {}", node01, 170f32, size.width);
    assert_eq!(size.height, 145f32, "height of node {:?}. Expected {}. Actual {}", node01, 145f32, size.height);
    assert_eq!(location.x, 370f32, "x of node {:?}. Expected {}. Actual {}", node01, 370f32, location.x);
    assert_eq!(location.y, 10f32, "y of node {:?}. Expected {}. Actual {}", node01, 10f32, location.y);
    let layout = taffy.layout(node02).unwrap();
    let Layout { size, location, .. } = layout;
    assert_eq!(size.width, 170f32, "width of node {:?}. Expected {}. Actual {}", node02, 170f32, size.width);
    assert_eq!(size.height, 145f32, "height of node {:?}. Expected {}. Actual {}", node02, 145f32, size.height);
    assert_eq!(location.x, 190f32, "x of node {:?}. Expected {}. Actual {}", node02, 190f32, location.x);
    assert_eq!(location.y, 10f32, "y of node {:?}. Expected {}. Actual {}", node02, 10f32, location.y);
    let layout = taffy.layout(node03).unwrap();
    let Layout { size, location, .. } = layout;
    assert_eq!(size.width, 170f32, "width of node {:?}. Expected {}. Actual {}", node03, 170f32, size.width);
    assert_eq!(size.height, 145f32, "height of node {:?}. Expected {}. Actual {}", node03, 145f32, size.height);
    assert_eq!(location.x, 10f32, "x of node {:?}. Expected {}. Actual {}", node03, 10f32, location.x);
    assert_eq!(location.y, 10f32, "y of node {:?}. Expected {}. Actual {}", node03, 10f32, location.y);
    let layout = taffy.layout(node04).unwrap();
    let Layout { size, location, .. } = layout;
    assert_eq!(size.width, 170f32, "width of node {:?}. Expected {}. Actual {}", node04, 170f32, size.width);
    assert_eq!(size.height, 145f32, "height of node {:?}. Expected {}. Actual {}", node04, 145f32, size.height);
    assert_eq!(location.x, 550f32, "x of node {:?}. Expected {}. Actual {}", node04, 550f32, location.x);
    assert_eq!(location.y, 165f32, "y of node {:?}. Expected {}. Actual {}", node04, 165f32, location.y);
    let layout = taffy.layout(node05).unwrap();
    let Layout { size, location, .. } = layout;
    assert_eq!(size.width, 170f32, "width of node {:?}. Expected {}. Actual {}", node05, 170f32, size.width);
    assert_eq!(size.height, 145f32, "height of node {:?}. Expected {}. Actual {}", node05, 145f32, size.height);
    assert_eq!(location.x, 370f32, "x of node {:?}. Expected {}. Actual {}", node05, 370f32, location.x);
    assert_eq!(location.y, 165f32, "y of node {:?}. Expected {}. Actual {}", node05, 165f32, location.y);
    let layout = taffy.layout(node06).unwrap();
    let Layout { size, location, .. } = layout;
    assert_eq!(size.width, 170f32, "width of node {:?}. Expected {}. Actual {}", node06, 170f32, size.width);
    assert_eq!(size.height, 145f32, "height of node {:?}. Expected {}. Actual {}", node06, 145f32, size.height);
    assert_eq!(location.x, 190f32, "x of node {:?}. Expected {}. Actual {}", node06, 190f32, location.x);
    assert_eq!(location.y, 165f32, "y of node {:?}. Expected {}. Actual {}", node06, 165f32, location.y);
    let layout = taffy.layout(node07).unwrap();
    let Layout { size, location, .. } = layout;
    assert_eq!(size.width, 170f32, "width of node {:?}. Expected {}. Actual {}", node07, 170f32, size.width);
    assert_eq!(size.height, 145f32, "height of node {:?}. Expected {}. Actual {}", node07, 145f32, size.height);
    assert_eq!(location.x, 10f32, "x of node {:?}. Expected {}. Actual {}", node07, 10f32, location.x);
    assert_eq!(location.y, 165f32, "y of node {:?}. Expected {}. Actual {}", node07, 165f32, location.y);
}
