//! Pseudo-random generation of data (e.g. UI nodes) to use in benchmarks.

use rand::Rng;

use crate::geometry::Size;
use crate::style::Dimension;
use crate::style::Style;

/// A trait for generating pseudo-random instances.
pub trait Randomizeable {
    /// Generate a pseudo-random instance of this type.
    ///
    /// This can be useful for benchmarking.
    fn random<R>(rng: &mut R) -> Self
    where
        R: Rng;
}

impl Randomizeable for Dimension {
    fn random<R>(rng: &mut R) -> Self
    where
        R: Rng,
    {
        let switch: f32 = rng.gen_range(0.0..=1.0);

        if switch < 0.2 {
            Dimension::Auto
        } else if switch < 0.8 {
            Dimension::Points(rng.gen_range(0.0..500.0))
        } else {
            Dimension::Percent(rng.gen_range(0.0..1.0))
        }
    }
}

impl Randomizeable for Size<Dimension> {
    fn random<R>(rng: &mut R) -> Self
    where
        R: Rng,
    {
        Size { width: Dimension::random(rng), height: Dimension::random(rng) }
    }
}

impl Randomizeable for Style {
    fn random<R>(rng: &mut R) -> Self
    where
        R: Rng,
    {
        // TODO: Add more attributes
        Style { size: Size::<Dimension>::random(rng), ..Default::default() }
    }
}
