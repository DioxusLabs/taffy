//! Code for resolving name grid lines and areas

use crate::{
    CheapCloneStr, GenericGridTemplateComponent, GenericRepetition as _, GridAreaAxis, GridAreaEnd, GridContainerStyle,
    GridPlacement, GridTemplateArea, Line, NonNamedGridPlacement, RepetitionCount,
};
use core::{borrow::Borrow, cmp::Ordering, fmt::Debug};

use super::{GridLine, MAX_GRID_TRACKS};
#[cfg(feature = "detailed_layout_info")]
use crate::compute::grid::GridLineNames;
#[cfg(feature = "detailed_layout_info")]
use crate::geometry::AbsoluteAxis;
// use alloc::fmt::format;
use crate::sys::{format, single_value_vec, Map, Vec};

/// Wrap an `AsRef<str>` type with a type which implements Hash by first
/// deferring to the underlying `&str`'s implementation of Hash.
#[derive(Debug, Clone)]
pub(crate) struct StrHasher<T: CheapCloneStr>(pub T);
impl<T: CheapCloneStr> PartialOrd for StrHasher<T> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
impl<T: CheapCloneStr> Ord for StrHasher<T> {
    fn cmp(&self, other: &Self) -> Ordering {
        self.0.as_ref().cmp(other.0.as_ref())
    }
}
impl<T: CheapCloneStr> PartialEq for StrHasher<T> {
    fn eq(&self, other: &Self) -> bool {
        other.0.as_ref() == self.0.as_ref()
    }
}
impl<T: CheapCloneStr> Eq for StrHasher<T> {}
#[cfg(feature = "std")]
impl<T: CheapCloneStr> std::hash::Hash for StrHasher<T> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.0.as_ref().hash(state)
    }
}
impl<T: CheapCloneStr> Borrow<str> for StrHasher<T> {
    fn borrow(&self) -> &str {
        self.0.as_ref()
    }
}

/// Resolver that takes grid lines names and area names as input and can then be used to
/// resolve line names of grid placement properties into line numbers.
pub(crate) struct NamedLineResolver<S: CheapCloneStr> {
    /// Map of row line names to line numbers. Each line name may correspond to multiple lines
    /// so we store a `Vec`
    row_lines: Map<StrHasher<S>, Vec<u32>>,
    /// Map of column line names to line numbers. Each line name may correspond to multiple lines
    /// so we store a `Vec`
    column_lines: Map<StrHasher<S>, Vec<u32>>,
    /// Map of area names to area definitions (start and end lines numbers in each axis)
    areas: Map<StrHasher<S>, GridTemplateArea<S>>,
    /// Number of columns implied by grid area definitions
    area_column_count: u16,
    /// Number of rows implied by grid area definitions
    area_row_count: u16,
    /// The number of explicit columns in the grid. This is an *input* to the `NamedLineResolver` and is
    /// used when computing the fallback line when a non-existent named line is specified.
    explicit_column_count: u16,
    /// The number of explicit rows in the grid. This is an *input* to the `NamedLineResolver` and is
    /// used when computing the fallback line when a non-existent named line is specified.
    explicit_row_count: u16,
    /// The (1-indexed line number, name) pairs of every named column line, in source order
    /// (template names before `grid-template-areas`-generated names)
    #[cfg(feature = "detailed_layout_info")]
    column_line_name_pairs: Vec<(u32, S)>,
    /// The (1-indexed line number, name) pairs of every named row line, in source order
    /// (template names before `grid-template-areas`-generated names)
    #[cfg(feature = "detailed_layout_info")]
    row_line_name_pairs: Vec<(u32, S)>,
}

/// Utility function to create or update an entry in a line name map
fn upsert_line_name_map<S: CheapCloneStr>(map: &mut Map<StrHasher<S>, Vec<u32>>, key: S, value: u32) {
    map.entry(StrHasher(key)).and_modify(|lines| lines.push(value)).or_insert_with(|| single_value_vec(value));
}

impl<S: CheapCloneStr> NamedLineResolver<S> {
    /// Create and initialise a new `NamedLineResolver`
    pub(crate) fn new(
        style: &impl GridContainerStyle<CustomIdent = S>,
        column_auto_repetitions: u16,
        row_auto_repetitions: u16,
    ) -> Self {
        let mut areas: Map<StrHasher<S>, GridTemplateArea<_>> = Map::new();
        let mut column_lines: Map<StrHasher<S>, Vec<u32>> = Map::new();
        let mut row_lines: Map<StrHasher<S>, Vec<u32>> = Map::new();

        #[cfg(feature = "detailed_layout_info")]
        let mut column_line_name_pairs: Vec<(u32, S)> = Vec::new();
        #[cfg(feature = "detailed_layout_info")]
        let mut row_line_name_pairs: Vec<(u32, S)> = Vec::new();

        let mut current_line = 0;
        if let Some(mut column_tracks) = style.grid_template_columns() {
            if let Some(column_line_names_iter) = style.grid_template_column_names() {
                for line_names in column_line_names_iter {
                    current_line += 1;
                    for line_name in line_names.into_iter() {
                        #[cfg(feature = "detailed_layout_info")]
                        column_line_name_pairs.push((current_line, line_name.clone()));
                        column_lines
                            .entry(StrHasher(line_name.clone()))
                            .and_modify(|lines: &mut Vec<u32>| lines.push(current_line))
                            .or_insert_with(|| single_value_vec(current_line));
                    }

                    if let Some(GenericGridTemplateComponent::Repeat(repeat)) = column_tracks.next() {
                        let repeat_count = match repeat.count() {
                            RepetitionCount::Count(count) => count,
                            RepetitionCount::AutoFill | RepetitionCount::AutoFit => column_auto_repetitions,
                        };

                        // Line name sets are positional: set `i` names the `i`th line of each
                        // repetition, and the final line name set of each repetition collapses
                        // with the first line name set of the following one. An empty list means
                        // the repetition's lines are unnamed; any other length must be exactly
                        // `track_count + 1` (one set per line, including both edge lines).
                        let line_name_set_count = repeat.lines_names().len() as u32;
                        let lines_per_repetition = repeat.track_count() as u32;
                        assert!(
                            line_name_set_count == 0 || line_name_set_count == lines_per_repetition + 1,
                            "grid template repetition must have no line name sets or exactly track count + 1 of them ({} tracks but {} line name sets)",
                            lines_per_repetition,
                            line_name_set_count,
                        );

                        for _ in 0..repeat_count {
                            for (line, line_name_set) in (current_line..).zip(repeat.lines_names()) {
                                for line_name in line_name_set {
                                    #[cfg(feature = "detailed_layout_info")]
                                    column_line_name_pairs.push((line, line_name.clone()));
                                    upsert_line_name_map(&mut column_lines, line_name.clone(), line);
                                }
                            }
                            current_line += lines_per_repetition;

                            // Names for lines beyond the maximum track limit are never resolvable:
                            // stop generating them (the explicit grid is clamped to MAX_GRID_TRACKS)
                            if current_line > MAX_GRID_TRACKS as u32 {
                                break;
                            }
                        }
                        // Last line name set collapses with following line name set
                        if repeat_count > 0 {
                            current_line = current_line.saturating_sub(1);
                        }
                    }
                }
            }
        }

        let mut current_line = 0;
        if let Some(mut row_tracks) = style.grid_template_rows() {
            if let Some(row_line_names_iter) = style.grid_template_row_names() {
                for line_names in row_line_names_iter {
                    current_line += 1;
                    for line_name in line_names.into_iter() {
                        #[cfg(feature = "detailed_layout_info")]
                        row_line_name_pairs.push((current_line, line_name.clone()));
                        row_lines
                            .entry(StrHasher(line_name.clone()))
                            .and_modify(|lines: &mut Vec<u32>| lines.push(current_line))
                            .or_insert_with(|| single_value_vec(current_line));
                    }

                    if let Some(GenericGridTemplateComponent::Repeat(repeat)) = row_tracks.next() {
                        let repeat_count = match repeat.count() {
                            RepetitionCount::Count(count) => count,
                            RepetitionCount::AutoFill | RepetitionCount::AutoFit => row_auto_repetitions,
                        };

                        // Line name sets are positional: set `i` names the `i`th line of each
                        // repetition, and the final line name set of each repetition collapses
                        // with the first line name set of the following one. An empty list means
                        // the repetition's lines are unnamed; any other length must be exactly
                        // `track_count + 1` (one set per line, including both edge lines).
                        let line_name_set_count = repeat.lines_names().len() as u32;
                        let lines_per_repetition = repeat.track_count() as u32;
                        assert!(
                            line_name_set_count == 0 || line_name_set_count == lines_per_repetition + 1,
                            "grid template repetition must have no line name sets or exactly track count + 1 of them ({} tracks but {} line name sets)",
                            lines_per_repetition,
                            line_name_set_count,
                        );

                        for _ in 0..repeat_count {
                            for (line, line_name_set) in (current_line..).zip(repeat.lines_names()) {
                                for line_name in line_name_set {
                                    #[cfg(feature = "detailed_layout_info")]
                                    row_line_name_pairs.push((line, line_name.clone()));
                                    upsert_line_name_map(&mut row_lines, line_name.clone(), line);
                                }
                            }
                            current_line += lines_per_repetition;

                            // Names for lines beyond the maximum track limit are never resolvable:
                            // stop generating them (the explicit grid is clamped to MAX_GRID_TRACKS)
                            if current_line > MAX_GRID_TRACKS as u32 {
                                break;
                            }
                        }
                        // Last line name set collapses with following line name set
                        if repeat_count > 0 {
                            current_line = current_line.saturating_sub(1);
                        }
                    }
                }
            }
        }
        // The size of the area template may be larger than the extents of the named areas
        // due to unnamed (`.`) cells, so it is taken from the style rather than being derived
        // from the areas themselves.
        let area_column_count = style.grid_template_area_column_count();
        let area_row_count = style.grid_template_area_row_count();
        if let Some(area_iter) = style.grid_template_areas() {
            for area in area_iter.into_iter() {
                // TODO: Investigate eliminating clones
                areas.insert(StrHasher(area.name.clone()), area.clone());

                let col_start_name = S::from(format!("{}-start", area.name.as_ref()));
                #[cfg(feature = "detailed_layout_info")]
                column_line_name_pairs.push((area.column_start as u32, col_start_name.clone()));
                upsert_line_name_map(&mut column_lines, col_start_name, area.column_start as u32);
                let col_end_name = S::from(format!("{}-end", area.name.as_ref()));
                #[cfg(feature = "detailed_layout_info")]
                column_line_name_pairs.push((area.column_end as u32, col_end_name.clone()));
                upsert_line_name_map(&mut column_lines, col_end_name, area.column_end as u32);
                let row_start_name = S::from(format!("{}-start", area.name.as_ref()));
                #[cfg(feature = "detailed_layout_info")]
                row_line_name_pairs.push((area.row_start as u32, row_start_name.clone()));
                upsert_line_name_map(&mut row_lines, row_start_name, area.row_start as u32);
                let row_end_name = S::from(format!("{}-end", area.name.as_ref()));
                #[cfg(feature = "detailed_layout_info")]
                row_line_name_pairs.push((area.row_end as u32, row_end_name.clone()));
                upsert_line_name_map(&mut row_lines, row_end_name, area.row_end as u32);
            }
        }

        // Sort and dedup lines for each column name
        for lines in column_lines.values_mut() {
            lines.sort_unstable();
            lines.dedup();
        }
        // Sort and dedup lines for each row name
        for lines in row_lines.values_mut() {
            lines.sort_unstable();
            lines.dedup();
        }

        Self {
            area_column_count,
            area_row_count,
            explicit_column_count: 0, // Overwritten later
            explicit_row_count: 0,    // Overwritten later
            areas,
            row_lines,
            column_lines,
            #[cfg(feature = "detailed_layout_info")]
            column_line_name_pairs,
            #[cfg(feature = "detailed_layout_info")]
            row_line_name_pairs,
        }
    }

    /// Build the per-line name groups of the explicit grid in the passed axis as a
    /// [`GridLineNames`], with `repeat()`s expanded and including the implicit
    /// `<name>-start`/`<name>-end` names generated by `grid-template-areas`.
    ///
    /// Line indices are relative to the explicit grid (line 0 = start of the first explicit track).
    #[cfg(feature = "detailed_layout_info")]
    pub(crate) fn detailed_line_names(&self, axis: AbsoluteAxis) -> GridLineNames<S> {
        let (pairs, explicit_track_count) = match axis {
            AbsoluteAxis::Horizontal => (&self.column_line_name_pairs, self.explicit_column_count),
            AbsoluteAxis::Vertical => (&self.row_line_name_pairs, self.explicit_row_count),
        };

        if pairs.is_empty() {
            return GridLineNames::default();
        }

        // Stable sort by line number preserves the source order of names within each line
        // (template names before area-generated names)
        let mut sorted_pairs: Vec<&(u32, S)> = pairs.iter().collect();
        sorted_pairs.sort_by_key(|(line, _)| *line);

        let line_count = explicit_track_count as usize + 1;
        let mut line_names = GridLineNames::with_capacity(sorted_pairs.len(), line_count + 1);
        let mut pair_iter = sorted_pairs.into_iter().peekable();
        for line in 1..=(line_count as u32) {
            line_names.start_line();
            while let Some(&&(pair_line, ref name)) = pair_iter.peek() {
                if pair_line != line {
                    break;
                }
                pair_iter.next();
                if !line_names.current_line_contains(name.as_ref()) {
                    line_names.push_name(name.clone());
                }
            }
        }
        line_names
    }

    /// Resolve named lines for both the `start` and `end` of a row-axis grid placement
    #[inline(always)]
    pub(crate) fn resolve_row_names(&self, line: &Line<GridPlacement<S>>) -> Line<NonNamedGridPlacement> {
        self.resolve_line_names(line, GridAreaAxis::Row)
    }

    /// Resolve named lines for both the `start` and `end` of a column-axis grid placement
    #[inline(always)]
    pub(crate) fn resolve_column_names(&self, line: &Line<GridPlacement<S>>) -> Line<NonNamedGridPlacement> {
        self.resolve_line_names(line, GridAreaAxis::Column)
    }

    /// Resolve named lines for both the `start` and `end` of a grid placement
    #[inline(always)]
    pub(crate) fn resolve_line_names(
        &self,
        line: &Line<GridPlacement<S>>,
        axis: GridAreaAxis,
    ) -> Line<NonNamedGridPlacement> {
        let start_holder;
        let start_line_resolved = if let GridPlacement::NamedLine(name, idx) = &line.start {
            start_holder =
                GridPlacement::Line(self.find_line_index(name, *idx as i32, axis, GridAreaEnd::Start, &|lines| lines));
            &start_holder
        } else {
            &line.start
        };

        let end_holder;
        let end_line_resolved = if let GridPlacement::NamedLine(name, idx) = &line.end {
            end_holder =
                GridPlacement::Line(self.find_line_index(name, *idx as i32, axis, GridAreaEnd::End, &|lines| lines));
            &end_holder
        } else {
            &line.end
        };

        // If both the *-start and *-end values of its grid-placement properties specify a line, its grid span is implicit.
        // If it has an explicit span value, its grid span is explicit.
        // Otherwise, its grid span is automatic:
        //   - if it is subgridded in that axis, its grid span is determined from its <line-name-list>;
        //   - otherwise its grid span is 1.
        //
        // <https://drafts.csswg.org/css-grid-2/#grid-span>
        match (&start_line_resolved, &end_line_resolved) {
            (GridPlacement::Line(start_line), GridPlacement::NamedSpan(name, idx)) => {
                let explicit_track_count = match axis {
                    GridAreaAxis::Row => self.explicit_row_count as i32,
                    GridAreaAxis::Column => self.explicit_column_count as i32,
                };
                let normalized_start_line = if start_line.as_i16() > 0 {
                    start_line.as_i16() as u32
                } else {
                    (explicit_track_count + 1 + start_line.as_i16() as i32).max(0) as u32
                };
                let end_line = self.find_line_index(name, *idx as i32, axis, GridAreaEnd::End, &|lines| {
                    let point = lines.partition_point(|line| *line <= normalized_start_line);
                    &lines[point..]
                });
                Line { start: NonNamedGridPlacement::Line(*start_line), end: NonNamedGridPlacement::Line(end_line) }
            }
            (GridPlacement::NamedSpan(name, idx), GridPlacement::Line(end_line)) => {
                let explicit_track_count = match axis {
                    GridAreaAxis::Row => self.explicit_row_count as i32,
                    GridAreaAxis::Column => self.explicit_column_count as i32,
                };
                let normalized_end_line = if end_line.as_i16() > 0 {
                    end_line.as_i16() as u32
                } else {
                    (explicit_track_count + 1 + end_line.as_i16() as i32).max(0) as u32
                };
                let start_line = self.find_line_index(name, *idx as i32, axis, GridAreaEnd::Start, &|lines| {
                    let point = lines.partition_point(|line| *line < normalized_end_line);
                    &lines[..point]
                });
                Line { start: NonNamedGridPlacement::Line(start_line), end: NonNamedGridPlacement::Line(*end_line) }
            }
            (start, end) => Line {
                start: match start {
                    GridPlacement::Auto => NonNamedGridPlacement::Auto,
                    GridPlacement::Line(grid_line) => NonNamedGridPlacement::Line(*grid_line),
                    GridPlacement::Span(span) => NonNamedGridPlacement::Span(*span),
                    GridPlacement::NamedSpan(_, _) => NonNamedGridPlacement::Span(1),
                    _ => unreachable!(),
                },
                end: match end {
                    GridPlacement::Auto => NonNamedGridPlacement::Auto,
                    GridPlacement::Line(grid_line) => NonNamedGridPlacement::Line(*grid_line),
                    GridPlacement::Span(span) => NonNamedGridPlacement::Span(*span),
                    GridPlacement::NamedSpan(_, _) => NonNamedGridPlacement::Span(1),
                    _ => unreachable!(),
                },
            },
        }
    }

    /// Resolve the grid line for a named grid line or span
    fn find_line_index(
        &self,
        name: &S,
        idx: i32,
        axis: GridAreaAxis,
        end: GridAreaEnd,
        filter_lines: &dyn Fn(&[u32]) -> &[u32],
    ) -> GridLine {
        let name = name.as_ref();
        let mut idx = idx;
        let explicit_track_count = match axis {
            GridAreaAxis::Row => self.explicit_row_count as i32,
            GridAreaAxis::Column => self.explicit_column_count as i32,
        };

        // An index of 0 is used to represent "no index specified".
        if idx == 0 {
            idx = 1;
        }

        fn get_line(lines: &[u32], explicit_track_count: i32, idx: i32) -> i16 {
            let abs_idx = idx.unsigned_abs() as usize;
            let line = if abs_idx <= lines.len() {
                if idx > 0 {
                    lines[abs_idx - 1] as i64
                } else {
                    lines[lines.len() - abs_idx] as i64
                }
            } else {
                let remaining_lines = (abs_idx - lines.len()) as i64 * idx.signum() as i64;
                if idx > 0 {
                    explicit_track_count as i64 + 1 + remaining_lines
                } else {
                    -(explicit_track_count as i64 + 1 + remaining_lines)
                }
            };
            line.clamp(i16::MIN as i64, i16::MAX as i64) as i16
        }

        // Lookup lines
        let line_lookup = match axis {
            GridAreaAxis::Row => &self.row_lines,
            GridAreaAxis::Column => &self.column_lines,
        };
        if let Some(lines) = line_lookup.get(name) {
            return GridLine::from(get_line(filter_lines(lines), explicit_track_count, idx));
        } else {
            // TODO: eliminate string allocations
            match end {
                GridAreaEnd::Start => {
                    let implicit_name = format!("{name}-start");
                    if let Some(lines) = line_lookup.get(&*implicit_name) {
                        // println!("IMPLICIT COL {implicit_name}");
                        return GridLine::from(get_line(filter_lines(lines), explicit_track_count, idx));
                    }
                }
                GridAreaEnd::End => {
                    let implicit_name = format!("{name}-end");
                    if let Some(lines) = line_lookup.get(&*implicit_name) {
                        // println!("IMPLICIT ROW {implicit_name}");
                        return GridLine::from(get_line(filter_lines(lines), explicit_track_count, idx));
                    }
                }
            }
        }

        // The CSS Grid specification has a weird quirk where it matches non-existent line names
        // to the first (positive) implicit line in the grid
        //
        // We add/subtract 2 to the explicit track count because (in each axis) a grid has one more explicit
        // grid line than it has tracks. And the fallback line is the line *after* that.
        //
        // See: <https://github.com/w3c/csswg-drafts/issues/966#issuecomment-277042153>
        let line = if idx > 0 {
            explicit_track_count as i64 + 1 + idx as i64
        } else {
            -(explicit_track_count as i64 + 1 + idx as i64)
        };

        GridLine::from(line.clamp(i16::MIN as i64, i16::MAX as i64) as i16)
    }

    /// Get the number of columns defined by the grid areas
    pub(crate) fn area_column_count(&self) -> u16 {
        self.area_column_count
    }

    /// Get the number of rows defined by the grid areas
    pub(crate) fn area_row_count(&self) -> u16 {
        self.area_row_count
    }

    /// Set the number of columns in the explicit grid
    pub(crate) fn set_explicit_column_count(&mut self, count: u16) {
        self.explicit_column_count = count;
    }

    /// Set the number of rows in the explicit grid
    pub(crate) fn set_explicit_row_count(&mut self, count: u16) {
        self.explicit_row_count = count;
    }
}

impl<S: CheapCloneStr> Debug for NamedLineResolver<S> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        writeln!(f, "Grid Areas:")?;
        for area in self.areas.values() {
            writeln!(
                f,
                "{}: row:{}/{} col: {}/{}",
                area.name.as_ref(),
                area.row_start,
                area.row_end,
                area.column_start,
                area.column_end
            )?;
        }

        writeln!(f, "Grid Rows:")?;
        for (name, lines) in self.row_lines.iter() {
            write!(f, "{}: ", name.0.as_ref())?;
            for line in lines {
                write!(f, "{line}  ")?;
            }
            writeln!(f)?;
        }

        writeln!(f, "Grid Columns:")?;
        for (name, lines) in self.column_lines.iter() {
            write!(f, "{}: ", name.0.as_ref())?;
            for line in lines {
                write!(f, "{line}  ")?;
            }
            writeln!(f)?;
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::style::GenericGridPlacement;
    use crate::sys::DefaultCheapStr;
    use crate::GridTemplateAreas;
    use crate::Style;

    fn resolver(explicit_track_count: u16) -> NamedLineResolver<DefaultCheapStr> {
        let mut resolver = NamedLineResolver::new(&Style::DEFAULT, 0, 0);
        resolver.set_explicit_column_count(explicit_track_count);
        resolver
    }

    fn resolved_start_line(
        resolver: &NamedLineResolver<DefaultCheapStr>,
        placement: GridPlacement<DefaultCheapStr>,
    ) -> i16 {
        let resolved = resolver.resolve_column_names(&Line { start: placement, end: GridPlacement::Auto });
        match resolved.start {
            GenericGridPlacement::Line(line) => line.as_i16(),
            _ => panic!("expected a resolved line"),
        }
    }

    #[test]
    fn extreme_missing_named_line_indices_do_not_overflow() {
        let resolver = resolver(10_000);
        assert_eq!(
            resolved_start_line(&resolver, GridPlacement::NamedLine(DefaultCheapStr::from("missing"), i16::MAX)),
            i16::MAX
        );
        assert_eq!(
            resolved_start_line(&resolver, GridPlacement::NamedLine(DefaultCheapStr::from("missing"), i16::MIN)),
            22_767
        );
    }

    #[test]
    fn large_named_span_does_not_wrap_negative() {
        let resolver = resolver(10_000);
        let resolved = resolver.resolve_column_names(&Line {
            start: GridPlacement::Line(GridLine::from(1)),
            end: GridPlacement::NamedSpan(DefaultCheapStr::from("missing"), u16::MAX),
        });
        match resolved.end {
            GenericGridPlacement::Line(line) => assert_eq!(line.as_i16(), i16::MAX),
            _ => panic!("expected a resolved line"),
        }
    }

    #[test]
    fn area_lines_saturate_when_converted_to_grid_lines() {
        let style = Style {
            grid_template_areas: Some(GridTemplateAreas {
                areas: vec![GridTemplateArea {
                    name: DefaultCheapStr::from("area"),
                    row_start: 1,
                    row_end: 2,
                    column_start: u16::MAX,
                    column_end: u16::MAX,
                }],
                row_count: 1,
                column_count: u16::MAX,
            }),
            ..Style::DEFAULT
        };
        let resolver = NamedLineResolver::new(&style, 0, 0);
        assert_eq!(
            resolved_start_line(&resolver, GridPlacement::NamedLine(DefaultCheapStr::from("area-start"), 1)),
            i16::MAX
        );
    }
}
