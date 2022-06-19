use crate::prelude::{Dimension, Rect, Size};

pub(crate) trait ResolveOrDefault<TContext, TOutput> {
    /// Resolves a value from something that can be either absolute or relative
    /// into one that is absolute.
    ///
    /// `context` is size of the parent
    fn resolve_or_default(self, context: TContext) -> TOutput;
}

pub(crate) trait MaybeResolve<T> {
    fn maybe_resolve(self, context: T) -> T;
}

impl MaybeResolve<Option<f32>> for Dimension {
    /// Converts the given [`Dimension`] into a concrete value of points
    ///
    /// Can return `None`
    fn maybe_resolve(self, context: Option<f32>) -> Option<f32> {
        match self {
            Dimension::Points(points) => Some(points),
            // parent_dim * percent
            Dimension::Percent(percent) => context.map(|dim| dim * percent),
            _ => None,
        }
    }
}

impl MaybeResolve<Size<Option<f32>>> for Size<Dimension> {
    /// Converts any `parent`-relative values for size into an absolute size
    fn maybe_resolve(self, context: Size<Option<f32>>) -> Size<Option<f32>> {
        Size { width: self.width.maybe_resolve(context.width), height: self.height.maybe_resolve(context.height) }
    }
}

impl ResolveOrDefault<Option<f32>, f32> for Dimension {
    /// Will return a default value of result is evaluated to `None`
    fn resolve_or_default(self, context: Option<f32>) -> f32 {
        self.maybe_resolve(context).unwrap_or(0.0)
    }
}

impl ResolveOrDefault<Size<Option<f32>>, Rect<f32>> for Rect<Dimension> {
    fn resolve_or_default(self, context: Size<Option<f32>>) -> Rect<f32> {
        Rect {
            start: self.start.resolve_or_default(context.width),
            end: self.end.resolve_or_default(context.width),
            top: self.top.resolve_or_default(context.height),
            bottom: self.bottom.resolve_or_default(context.height),
        }
    }
}
