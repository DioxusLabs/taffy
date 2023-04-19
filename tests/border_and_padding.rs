use taffy::prelude::*;
use taffy::style_helpers::TaffyZero;

fn arr_to_rect<T: Copy>(items: [T; 4]) -> Rect<T> {
    Rect { left: items[0], right: items[1], top: items[2], bottom: items[3] }
}

#[test]
#[ignore]
fn border_on_a_single_axis_doesnt_increase_size() {
    for i in 0..4 {
        let mut taffy = Taffy::new();
        let node = taffy
            .new_leaf(Style {
                border: {
                    let mut lengths = [LengthPercentage::ZERO; 4];
                    lengths[i] = LengthPercentage::Points(10.);
                    arr_to_rect(lengths)
                },
                ..Default::default()
            })
            .unwrap();

        taffy
            .compute_layout(
                node,
                Size { height: AvailableSpace::Definite(100.0), width: AvailableSpace::Definite(100.0) },
            )
            .ok();

        let layout = taffy.layout(node).unwrap();
        assert_eq!(layout.size.width * layout.size.height, 0.);
    }
}

#[test]
#[ignore]
fn padding_on_a_single_axis_doesnt_increase_size() {
    for i in 0..4 {
        let mut taffy = Taffy::new();
        let node = taffy
            .new_leaf(Style {
                padding: {
                    let mut lengths = [LengthPercentage::ZERO; 4];
                    lengths[i] = LengthPercentage::Points(10.);
                    arr_to_rect(lengths)
                },
                ..Default::default()
            })
            .unwrap();

        taffy
            .compute_layout(
                node,
                Size { height: AvailableSpace::Definite(100.0), width: AvailableSpace::Definite(100.0) },
            )
            .ok();

        let layout = taffy.layout(node).unwrap();
        assert_eq!(layout.size.width * layout.size.height, 0.);
    }
}

#[test]
#[ignore]
fn border_and_padding_on_a_single_axis_doesnt_increase_size() {
    for i in 0..4 {
        let mut taffy = Taffy::new();
        let rect = {
            let mut lengths = [LengthPercentage::ZERO; 4];
            lengths[i] = LengthPercentage::Points(10.);
            arr_to_rect(lengths)
        };
        let node = taffy.new_leaf(Style { border: rect, padding: rect, ..Default::default() }).unwrap();

        taffy
            .compute_layout(
                node,
                Size { height: AvailableSpace::Definite(100.0), width: AvailableSpace::Definite(100.0) },
            )
            .ok();
        let layout = taffy.layout(node).unwrap();
        assert_eq!(layout.size.width * layout.size.height, 0.);
    }
}

#[test]
#[ignore]
fn vertical_border_and_padding_percentage_values_use_available_space_correctly() {
    let mut taffy = Taffy::new();

    let node = taffy
        .new_leaf(Style {
            padding: Rect { left: LengthPercentage::Percent(1.0), top: LengthPercentage::Percent(1.0), ..Rect::zero() },
            ..Default::default()
        })
        .unwrap();

    taffy
        .compute_layout(node, Size { width: AvailableSpace::Definite(200.0), height: AvailableSpace::Definite(100.0) })
        .ok();

    let layout = taffy.layout(node).unwrap();
    assert_eq!(layout.size.width, 200.0);
    assert_eq!(layout.size.height, 200.0);
}
