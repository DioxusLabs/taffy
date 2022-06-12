//! Contains numberical helper traits and functions

/// A trait to conveniently calculate minimums and maximums when some data may not be defined
///
/// If the left-hand value is [`None`], these operations return [`None`].
/// If the right-hand value is [`None`], it is treated as zero.
pub(crate) trait MaybeMath<In, Out> {
    /// Returns the minimum of `self` and `rhs`
    fn maybe_min(self, rhs: In) -> Out;

    /// Returns the maximum of `self` and `rhs`
    fn maybe_max(self, rhs: In) -> Out;

    /// Adds `self` and `rhs`.
    fn maybe_add(self, rhs: In) -> Out;

    /// Subracts rhs from `self`, treating [`None`] values as default
    fn maybe_sub(self, rhs: In) -> Out;
}

impl MaybeMath<Option<f32>, Option<f32>> for Option<f32> {
    fn maybe_min(self, rhs: Option<f32>) -> Option<f32> {
        match (self, rhs) {
            (Some(l), Some(r)) => Some(l.min(r)),
            (Some(_l), None) => self,
            (None, Some(_r)) => None,
            (None, None) => None,
        }
    }

    fn maybe_max(self, rhs: Option<f32>) -> Option<f32> {
        match (self, rhs) {
            (Some(l), Some(r)) => Some(l.max(r)),
            (Some(_l), None) => self,
            (None, Some(_r)) => None,
            (None, None) => None,
        }
    }

    fn maybe_add(self, rhs: Option<f32>) -> Option<f32> {
        match (self, rhs) {
            (Some(l), Some(r)) => Some(l + r),
            (Some(_l), None) => self,
            (None, Some(_r)) => None,
            (None, None) => None,
        }
    }

    fn maybe_sub(self, rhs: Option<f32>) -> Option<f32> {
        match (self, rhs) {
            (Some(l), Some(r)) => Some(l - r),
            (Some(_l), None) => self,
            (None, Some(_r)) => None,
            (None, None) => None,
        }
    }
}

impl MaybeMath<f32, Option<f32>> for Option<f32> {
    fn maybe_min(self, rhs: f32) -> Option<f32> {
        self.map(|val| val.min(rhs))
    }

    fn maybe_max(self, rhs: f32) -> Option<f32> {
        self.map(|val| val.max(rhs))
    }

    fn maybe_add(self, rhs: f32) -> Option<f32> {
        self.map(|val| val + rhs)
    }

    fn maybe_sub(self, rhs: f32) -> Option<f32> {
        self.map(|val| val - rhs)
    }
}

impl MaybeMath<Option<f32>, f32> for f32 {
    fn maybe_min(self, rhs: Option<f32>) -> f32 {
        match rhs {
            Some(val) => self.min(val),
            None => self,
        }
    }

    fn maybe_max(self, rhs: Option<f32>) -> f32 {
        match rhs {
            Some(val) => self.max(val),
            None => self,
        }
    }

    fn maybe_add(self, rhs: Option<f32>) -> f32 {
        match rhs {
            Some(val) => self + val,
            None => self,
        }
    }

    fn maybe_sub(self, rhs: Option<f32>) -> f32 {
        match rhs {
            Some(val) => self - val,
            None => self,
        }
    }
}
