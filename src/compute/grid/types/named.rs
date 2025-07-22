use crate::{
    CheapCloneStr, GenericGridPlacement, GenericRepetition as _, GridAreaAxis, GridAreaEnd, GridContainerStyle,
    GridPlacement, GridTemplateArea, GridTemplateComponentRef, Line, NonNamedGridPlacement, RepetitionCount,
};
use core::borrow::Borrow;
use std::{
    collections::HashMap,
    hash::{Hash, Hasher},
};

use super::GridLine;

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
    row_lines: HashMap<StrHasher<S>, Vec<u16>>,
    column_lines: HashMap<StrHasher<S>, Vec<u16>>,
    areas: HashMap<StrHasher<S>, GridTemplateArea<S>>,
    area_column_count: u16,
    area_row_count: u16,
}

impl<S: CheapCloneStr> NamedLineResolver<S> {
    pub(crate) fn new<'a>(
        style: &impl GridContainerStyle<CustomIdent = S>,
        column_auto_repetitions: u16,
        row_auto_repetitions: u16,
    ) -> Self {
        let mut area_column_count = 0;
        let mut area_row_count = 0;
        let mut areas: HashMap<StrHasher<_>, GridTemplateArea<_>> = HashMap::new();
        if let Some(area_iter) = style.grid_template_areas() {
            for area in area_iter.into_iter() {
                // TODO: Investigate eliminating clones
                areas.insert(StrHasher(area.name.clone()), area.clone());

                area_column_count = area_column_count.max(area.column_end);
                area_row_count = area_row_count.max(area.row_end);
            }
        }

        // ---

        let mut current_line = 0;
        let mut column_lines: HashMap<StrHasher<S>, Vec<u16>> = HashMap::new();
        let mut column_tracks = style.grid_template_columns();
        if let Some(column_line_names_iter) = style.grid_template_column_names() {
            current_line += 1;
            for line_names in column_line_names_iter {
                for line_name in line_names.into_iter() {
                    column_lines
                        .entry(StrHasher(line_name.clone()))
                        .and_modify(|lines: &mut Vec<u16>| lines.push(current_line))
                        .or_insert_with(|| vec![current_line]);
                }

                if let Some(GridTemplateComponentRef::Repeat(repeat)) = column_tracks.next() {
                    let repeat_count = match repeat.count() {
                        RepetitionCount::Count(count) => count,
                        RepetitionCount::AutoFill | RepetitionCount::AutoFit => column_auto_repetitions,
                    };

                    for _ in 0..repeat_count {
                        for line_name_set in repeat.lines_names().into_iter() {
                            for line_name in line_name_set.into_iter() {
                                column_lines
                                    .entry(StrHasher(line_name.clone()))
                                    .and_modify(|lines: &mut Vec<u16>| lines.push(current_line))
                                    .or_insert_with(|| vec![current_line]);
                            }
                            current_line += 1;
                        }
                        // Last line name set collapses with following line name set
                        current_line -= 1;
                    }
                }
            }
        }

        let mut current_line = 0;
        let mut row_lines: HashMap<StrHasher<S>, Vec<u16>> = HashMap::new();
        let mut row_tracks = style.grid_template_rows();
        if let Some(row_line_names_iter) = style.grid_template_row_names() {
            current_line += 1;
            for line_names in row_line_names_iter {
                for line_name in line_names.into_iter() {
                    row_lines
                        .entry(StrHasher(line_name.clone()))
                        .and_modify(|lines: &mut Vec<u16>| lines.push(current_line))
                        .or_insert_with(|| vec![current_line]);
                }

                if let Some(GridTemplateComponentRef::Repeat(repeat)) = row_tracks.next() {
                    let repeat_count = match repeat.count() {
                        RepetitionCount::Count(count) => count,
                        RepetitionCount::AutoFill | RepetitionCount::AutoFit => row_auto_repetitions,
                    };

                    for _ in 0..repeat_count {
                        for line_name_set in repeat.lines_names().into_iter() {
                            for line_name in line_name_set.into_iter() {
                                row_lines
                                    .entry(StrHasher(line_name.clone()))
                                    .and_modify(|lines: &mut Vec<u16>| lines.push(current_line))
                                    .or_insert_with(|| vec![current_line]);
                            }
                            current_line += 1;
                        }
                        // Last line name set collapses with following line name set
                        current_line -= 1;
                    }
                }
            }
        }

        // dbg!(&areas);
        // dbg!(&column_lines);
        // dbg!(&row_lines);

        Self { area_column_count, area_row_count, areas, row_lines, column_lines }
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

                // dbg!(axis, &name);

                // Lookup areas
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

                // Lookup lines
                let line_lookup = match axis {
                    GridAreaAxis::Row => &self.row_lines,
                    GridAreaAxis::Column => &self.column_lines,
                };
                if let Some(lines) = line_lookup.get(name) {
                    // TODO: handle multiple names for same line properly
                    return GenericGridPlacement::Line(GridLine::from(lines[0] as i16));
                } else {
                    // TODO: eliminate string allocations
                    match end {
                        GridAreaEnd::Start => {
                            let implicit_name = format!("{name}-start");
                            if let Some(lines) = line_lookup.get(&*implicit_name) {
                                // println!("IMPLICIT COL {implicit_name}");
                                return GenericGridPlacement::Line(GridLine::from(lines[0] as i16));
                            }
                        }
                        GridAreaEnd::End => {
                            let implicit_name = format!("{name}-end");
                            if let Some(lines) = line_lookup.get(&*implicit_name) {
                                // println!("IMPLICIT ROW {implicit_name}");
                                return GenericGridPlacement::Line(GridLine::from(lines[0] as i16));
                            }
                        }
                    }
                }

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
