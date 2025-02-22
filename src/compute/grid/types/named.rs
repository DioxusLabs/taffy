use crate::{
    CheapCloneStr, GenericGridPlacement, GridAreaAxis, GridAreaEnd, GridPlacement, GridTemplateArea, Line,
    NonNamedGridPlacement,
};
use core::borrow::Borrow;
use std::{
    collections::HashMap,
    hash::{Hash, Hasher},
};

/// Wrap an `AsRef<str>` type with a type which implements Hash by first
/// deferring to the underlying `&str`'s implementation of Hash.
#[derive(Debug, Clone)]
pub(crate) struct StrHasher<T: CheapCloneStr>(pub T);
impl<T: CheapCloneStr> Hash for StrHasher<T> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        // Dele
        self.0.as_ref().hash(state)
    }
}
impl<T: CheapCloneStr> PartialEq for StrHasher<T> {
    fn eq(&self, other: &Self) -> bool {
        other.0.as_ref() == self.0.as_ref()
    }
}
impl<T: CheapCloneStr> Borrow<str> for StrHasher<T> {
    fn borrow(&self) -> &str {
        self.0.as_ref()
    }
}
impl<T: CheapCloneStr> Eq for StrHasher<T> {}

/// Resolve lines for
pub(crate) struct NamedLineResolver<S: CheapCloneStr> {
    lines: HashMap<StrHasher<S>, i16>,
    areas: HashMap<StrHasher<S>, GridTemplateArea<S>>,
    area_column_count: u16,
    area_row_count: u16,
}

impl<S: CheapCloneStr> NamedLineResolver<S> {
    pub(crate) fn new(area_styles: Option<impl Borrow<[GridTemplateArea<S>]>>) -> Self {
        let mut area_column_count = 0;
        let mut area_row_count = 0;
        let mut areas: HashMap<StrHasher<_>, GridTemplateArea<_>> = HashMap::new();
        if let Some(area_iter) = area_styles {
            for area in area_iter.borrow() {
                // TODO: Investigate eliminating clones
                areas.insert(StrHasher(area.name.clone()), area.clone());

                area_column_count = area_column_count.max(area.column_end);
                area_row_count = area_row_count.max(area.row_end);
            }
        }

        Self { area_column_count, area_row_count, areas, lines: HashMap::new() }
    }

    #[inline(always)]
    pub(crate) fn resolve_row_names(&self, line: &Line<GridPlacement<S>>) -> Line<NonNamedGridPlacement> {
        self.resolve_line_names(line, GridAreaAxis::Row)
    }

    #[inline(always)]
    pub(crate) fn resolve_column_names(&self, line: &Line<GridPlacement<S>>) -> Line<NonNamedGridPlacement> {
        self.resolve_line_names(line, GridAreaAxis::Column)
    }

    #[inline(always)]
    pub(crate) fn resolve_line_names(
        &self,
        line: &Line<GridPlacement<S>>,
        axis: GridAreaAxis,
    ) -> Line<NonNamedGridPlacement> {
        Line {
            start: self.resolve_line_name(&line.start, axis, GridAreaEnd::Start),
            end: self.resolve_line_name(&line.end, axis, GridAreaEnd::End),
        }
    }

    pub(crate) fn resolve_line_name(
        &self,
        placement: &GridPlacement<S>,
        axis: GridAreaAxis,
        end: GridAreaEnd,
    ) -> NonNamedGridPlacement {
        match placement {
            GridPlacement::Auto => NonNamedGridPlacement::Auto,
            GridPlacement::Line(grid_line) => NonNamedGridPlacement::Line(*grid_line),
            GridPlacement::Span(span) => NonNamedGridPlacement::Span(*span),
            GridPlacement::Named(name) => {
                let name = name.as_ref();
                if name.ends_with("-start") {
                    let area_name = &name[0..(name.len() - 6)];
                    if let Some(area) = self.areas.get(area_name) {
                        return GenericGridPlacement::Line(area.get_side(axis, GridAreaEnd::Start));
                    }
                } else if name.ends_with("-end") {
                    let area_name = &name[0..(name.len() - 6)];
                    if let Some(area) = self.areas.get(area_name) {
                        return GenericGridPlacement::Line(area.get_side(axis, GridAreaEnd::End));
                    }
                } else {
                    if let Some(area) = self.areas.get(name) {
                        return GenericGridPlacement::Line(area.get_side(axis, end));
                    }
                }

                // TODO: lookup names lines
                NonNamedGridPlacement::Auto
            }
        }
    }

    pub(crate) fn area_column_count(&self) -> u16 {
        self.area_column_count
    }

    pub(crate) fn area_row_count(&self) -> u16 {
        self.area_row_count
    }
}
