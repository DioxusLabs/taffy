//! A port of [Yoga](https://github.com/facebook/yoga)'s flexbox algorithm, for hosts (such as React Native)
//! that need Yoga's exact layout results rather than CSS flexbox.
//!
//! Each file is a port of the same-named Yoga source file at commit `08250431e09bc4b864917e51f9e9d52998ee2dce`
//! and follows it closely, so that upstream changes can be ported diff by diff. Yoga is MIT licensed,
//! Copyright (c) Meta Platforms, Inc. and affiliates; see `LICENSE-YOGA`.

mod config;
mod enums;
pub(crate) mod numeric;

pub use config::{YogaConfig, YogaExperimentalFeatures};
pub use enums::{YogaErrata, YogaExperimentalFeature, YogaMeasureMode, YogaSizingMode};
