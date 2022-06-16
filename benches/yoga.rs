//! These benchmarks try to replicate the benchmarks from Yoga (Facebook's flexbox library) to serve as an initial comparison.
//!
//! See <https://github.com/facebook/yoga/blob/578d197dd6652225b46af090c0b46471dc887361/javascript/tests/Benchmarks/YGBenchmark.js> for the Yoga implementation.
use criterion::{criterion_group, criterion_main, Criterion};
use taffy::prelude::*;

const ITERATIONS: u32 = 2000;

fn taffy_benchmarks(c: &mut Criterion) {
    c.bench_function("huge nested layout", |b| {
        b.iter(|| {
            let mut taffy = taffy::node::Taffy::new();

            // FIXME: Surely this can be done prettier than this
            let iterations = (ITERATIONS as f32).powf(0.25) as i32;

            // FIXME: We use FlexDirection::Row as default, so it would be the same as `row_style`. Is this the same in Yoga?
            let style = FlexboxLayout {
                flex_grow: 1.0,
                size: Size { width: Dimension::Points(10.0), height: Dimension::Points(10.0) },
                ..Default::default()
            };

            let row_style = FlexboxLayout {
                flex_direction: FlexDirection::Row,
                flex_grow: 1.0,
                size: Size { width: Dimension::Points(10.0), height: Dimension::Points(10.0) },
                ..Default::default()
            };

            let mut children = Vec::new();

            for _ in 0..iterations {
                let mut grand_children = Vec::new();

                for _ in 0..iterations {
                    let mut grand_grand_children = Vec::new();

                    for _ in 0..iterations {
                        let mut grand_grand_grand_children = Vec::new();

                        for _ in 0..iterations {
                            let grand_grand_grand_child = taffy.new_with_children(row_style, &[]).unwrap();
                            grand_grand_grand_children.push(grand_grand_grand_child);
                        }

                        let grand_grand_child =
                            taffy.new_with_children(style, grand_grand_grand_children.as_slice()).unwrap();
                        grand_grand_children.push(grand_grand_child);
                    }

                    let grand_child = taffy.new_with_children(row_style, grand_grand_children.as_slice()).unwrap();
                    grand_children.push(grand_child);
                }

                let child = taffy.new_with_children(style, grand_children.as_slice()).unwrap();
                children.push(child);
            }

            let root = taffy.new_with_children(FlexboxLayout::default(), children.as_slice()).unwrap();
            // FIXME: What does the `size` parameter do here, how can we make this similar to Yoga?
            let _ = taffy.compute_layout(root, Size::undefined());
        })
    });
}

criterion_group!(benches, taffy_benchmarks);
criterion_main!(benches);
