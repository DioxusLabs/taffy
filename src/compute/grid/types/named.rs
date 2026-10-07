//! Code for resolving name grid lines and areas

use crate::{
    CheapCloneStr, GenericGridTemplateComponent, GenericRepetition as _, GridAreaAxis, GridAreaEnd, GridContainerStyle,
    GridPlacement, Line, NonNamedGridPlacement, RepetitionCount,
};
use core::{borrow::Borrow, cmp::Ordering, fmt::Debug};

use super::{GridLine, OriginZeroLine, MAX_GRID_TRACKS, MAX_OZ_LINE, MIN_OZ_LINE};
use crate::geometry::AbsoluteAxis;
use crate::sys::DefaultCheapStr;
use crate::sys::{Map, Vec};
use smallvec::{smallvec, SmallVec};

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

/// The one-indexed line positions of a single grid line name. Inline capacity of 4 keeps the
/// type the same size as `Vec<u32>` (24 bytes) while avoiding a heap allocation for names
/// mapping to at most 4 lines (the common case)
pub(crate) type LinePositions = SmallVec<[u32; 4]>;

/// Map from a grid line name to its one-indexed line positions
type NamedGridLinesMap<S> = Map<StrHasher<S>, LinePositions>;

/// The one-indexed positions of the first line that starts and the first line that ends a named grid area in
/// each axis, i.e. the first line named `<area-name>-start`/`<area-name>-end` (explicitly or by
/// `grid-template-areas`). A value of 0 means that there is no such line.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub(crate) struct AreaLines {
    /// The first line that starts the area in the row axis
    row_start: u32,
    /// The first line that ends the area in the row axis
    row_end: u32,
    /// The first line that starts the area in the column axis
    column_start: u32,
    /// The first line that ends the area in the column axis
    column_end: u32,
}

impl AreaLines {
    /// The line for the passed edge of the area (if there is one)
    fn line(&self, axis: GridAreaAxis, end: GridAreaEnd) -> Option<&u32> {
        let line = match (axis, end) {
            (GridAreaAxis::Row, GridAreaEnd::Start) => &self.row_start,
            (GridAreaAxis::Row, GridAreaEnd::End) => &self.row_end,
            (GridAreaAxis::Column, GridAreaEnd::Start) => &self.column_start,
            (GridAreaAxis::Column, GridAreaEnd::End) => &self.column_end,
        };
        Some(line).filter(|line| **line != 0)
    }

    /// The line for the passed edge of the area (0 if there is none)
    fn line_mut(&mut self, axis: GridAreaAxis, end: GridAreaEnd) -> &mut u32 {
        match (axis, end) {
            (GridAreaAxis::Row, GridAreaEnd::Start) => &mut self.row_start,
            (GridAreaAxis::Row, GridAreaEnd::End) => &mut self.row_end,
            (GridAreaAxis::Column, GridAreaEnd::Start) => &mut self.column_start,
            (GridAreaAxis::Column, GridAreaEnd::End) => &mut self.column_end,
        }
    }
}

/// Map from the name of a grid area to the lines of its edges.
///
/// This is how the `<area-name>-start` and `<area-name>-end` line names that `grid-template-areas` implicitly
/// generates are represented: storing them as line names would require allocating a string for each of them.
pub(crate) type GridAreasMap<S> = Map<StrHasher<S>, AreaLines>;

/// Split a line name of the form `<area-name>-start` or `<area-name>-end` into the area name and the edge
fn split_area_edge_name(name: &str) -> Option<(&str, GridAreaEnd)> {
    if let Some(area_name) = name.strip_suffix("-start") {
        Some((area_name, GridAreaEnd::Start))
    } else if let Some(area_name) = name.strip_suffix("-end") {
        Some((area_name, GridAreaEnd::End))
    } else {
        None
    }
}

/// Resolver for named placements in one grid axis
struct NamedLineResolverAxis<'a, S: CheapCloneStr> {
    /// Named lines and their one-indexed positions
    lines: &'a NamedGridLinesMap<S>,
    /// The edges of named grid areas (in both axes)
    areas: &'a GridAreasMap<S>,
    /// The axis being resolved
    axis: GridAreaAxis,
    /// Number of explicit tracks in this axis
    explicit_track_count: u16,
}

/// Resolver that takes grid lines names and area names as input and can then be used to
/// resolve line names of grid placement properties into line numbers.
pub(crate) struct NamedLineResolver<S: CheapCloneStr> {
    /// Map of row line names to line numbers. Each line name may correspond to multiple lines
    /// so we store a `SmallVec`
    row_lines: NamedGridLinesMap<S>,
    /// Map of column line names to line numbers. Each line name may correspond to multiple lines
    /// so we store a `SmallVec`
    column_lines: NamedGridLinesMap<S>,
    /// Map of area names to the lines of the area's edges
    areas: GridAreasMap<S>,
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
    /// The (1-indexed line number, name) pairs of every column line named in the track template,
    /// in source order (excludes the implicit names generated by `grid-template-areas`)
    column_line_name_pairs: Vec<(u32, S)>,
    /// The (1-indexed line number, name) pairs of every row line named in the track template,
    /// in source order (excludes the implicit names generated by `grid-template-areas`)
    row_line_name_pairs: Vec<(u32, S)>,
}

/// Utility function to create or update an entry in a line name map
///
/// `areas` must already contain the edges of the areas defined by `grid-template-areas`
fn upsert_line_name_map<S: CheapCloneStr>(
    map: &mut NamedGridLinesMap<S>,
    areas: &mut GridAreasMap<S>,
    axis: GridAreaAxis,
    key: S,
    value: u32,
) {
    // A line named `foo-start` or `foo-end` is also an edge of the (implicitly) named area `foo`. Record it as such
    // if it is the first such line. If `foo` is also an area defined by `grid-template-areas`, then the line which
    // that area generated also has this name.
    let mut area_line = 0;
    if let Some((area_name, end)) = split_area_edge_name(key.as_ref()) {
        // The key is only created if the area is not already in the map
        if !areas.contains_key(area_name) {
            areas.insert(StrHasher(S::from(area_name)), AreaLines::default());
        }
        let edge = areas.get_mut(area_name).unwrap().line_mut(axis, end);
        area_line = *edge;
        *edge = if *edge == 0 { value } else { (*edge).min(value) };
    }

    map.entry(StrHasher(key)).and_modify(|lines| lines.push(value)).or_insert_with(|| {
        // Until this entry is created, the only line that can have been recorded for this edge is the area's own
        if area_line == 0 {
            smallvec![value]
        } else {
            smallvec![area_line, value]
        }
    });
}

impl<S: CheapCloneStr> NamedLineResolverAxis<'_, S> {
    /// The line for the passed edge of the named grid area in this axis (if there is one)
    fn area_line(&self, area_name: &str, end: GridAreaEnd) -> Option<&u32> {
        self.areas.get(area_name)?.line(self.axis, end)
    }

    /// Resolve named lines and spans into numeric placements
    fn resolve_line_names(&self, line: &Line<GridPlacement<S>>) -> Line<NonNamedGridPlacement> {
        let start_holder;
        let start_line_resolved = if let GridPlacement::NamedLine(name, idx) = &line.start {
            start_holder = GridPlacement::Line(self.find_named_line(name, *idx, GridAreaEnd::Start));
            &start_holder
        } else {
            &line.start
        };

        let end_holder;
        let end_line_resolved = if let GridPlacement::NamedLine(name, idx) = &line.end {
            end_holder = GridPlacement::Line(self.find_named_line(name, *idx, GridAreaEnd::End));
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
            (GridPlacement::Line(start_line), GridPlacement::NamedSpan(name, idx)) if start_line.as_i16() != 0 => {
                let after_start_line = start_line.into_origin_zero_line(self.explicit_track_count) + 1;
                let end_line =
                    self.nth_line_forwards(self.lines_named(name.as_ref()), after_start_line, (*idx).max(1) as i32);
                Line {
                    start: NonNamedGridPlacement::Line(*start_line),
                    end: NonNamedGridPlacement::Line(end_line.into_grid_line(self.explicit_track_count)),
                }
            }
            (GridPlacement::NamedSpan(name, idx), GridPlacement::Line(end_line)) if end_line.as_i16() != 0 => {
                let before_end_line = end_line.into_origin_zero_line(self.explicit_track_count) - 1;
                let start_line =
                    self.nth_line_backwards(self.lines_named(name.as_ref()), before_end_line, (*idx).max(1) as i32);
                Line {
                    start: NonNamedGridPlacement::Line(start_line.into_grid_line(self.explicit_track_count)),
                    end: NonNamedGridPlacement::Line(*end_line),
                }
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

    /// Resolve the grid line for a named grid line (`<custom-ident>` or `<integer> <custom-ident>`).
    /// An index of 0 is used to represent "no index specified".
    ///
    /// <https://drafts.csswg.org/css-grid-2/#line-placement>
    fn find_named_line(&self, name: &S, idx: i16, end: GridAreaEnd) -> GridLine {
        let name = name.as_ref();

        // A `<custom-ident>` on its own first attempts to match the edge of a named grid area:
        // the first line named `<custom-ident>-start` (or `<custom-ident>-end` for an end line).
        // Otherwise it is treated as if the integer 1 had been specified along with it.
        if idx == 0 {
            if let Some(first_line) = self.area_line(name, end) {
                return clamp_oz_line(oz_line(*first_line)).into_grid_line(self.explicit_track_count);
            }
        }

        let lines = self.lines_named(name);
        let line = if idx >= 0 {
            self.nth_line_forwards(lines, OriginZeroLine(0), (idx as i32).max(1))
        } else {
            self.nth_line_backwards(lines, self.last_explicit_line(), -(idx as i32))
        };
        line.into_grid_line(self.explicit_track_count)
    }

    /// The sorted line numbers of the lines with the passed name
    ///
    /// A line generated by a grid area is included in the lines of its name if any line has been explicitly
    /// given the same name. Otherwise it is found from the area's edges.
    fn lines_named(&self, name: &str) -> &[u32] {
        match self.lines.get(name) {
            Some(lines) => lines.as_slice(),
            None => split_area_edge_name(name)
                .and_then(|(area_name, area_end)| self.area_line(area_name, area_end))
                .map(core::slice::from_ref)
                .unwrap_or(&[]),
        }
    }

    /// The line at the end edge of the explicit grid
    fn last_explicit_line(&self) -> OriginZeroLine {
        OriginZeroLine(self.explicit_track_count as i16)
    }

    /// Find the `n`th line in `lines` (searching forwards) that is not before `first_line`.
    ///
    /// If there are not enough such lines, then all implicit lines after the explicit grid are
    /// assumed to be in `lines` for the purpose of counting
    ///
    /// <https://drafts.csswg.org/css-grid-2/#grid-placement-int>
    fn nth_line_forwards(&self, lines: &[u32], first_line: OriginZeroLine, n: i32) -> OriginZeroLine {
        let first_line = first_line.0.max(0) as i32;
        let lines = &lines[lines.partition_point(|line| oz_line(*line) < first_line)..];
        let line = match lines.get(n as usize - 1) {
            Some(line) => oz_line(*line),
            None => first_line.max(self.last_explicit_line().0 as i32 + 1) + (n - lines.len() as i32) - 1,
        };
        clamp_oz_line(line)
    }

    /// Find the `n`th line in `lines` (searching backwards) that is not after `last_line`.
    ///
    /// If there are not enough such lines, then all implicit lines before the explicit grid are
    /// assumed to be in `lines` for the purpose of counting
    ///
    /// <https://drafts.csswg.org/css-grid-2/#grid-placement-int>
    fn nth_line_backwards(&self, lines: &[u32], last_line: OriginZeroLine, n: i32) -> OriginZeroLine {
        let last_line = last_line.0.min(self.last_explicit_line().0) as i32;
        let lines = &lines[..lines.partition_point(|line| oz_line(*line) <= last_line)];
        let line = match lines.len().checked_sub(n as usize) {
            Some(index) => oz_line(lines[index]),
            None => last_line.min(-1) - (n - lines.len() as i32) + 1,
        };
        clamp_oz_line(line)
    }
}

/// The named line maps store lines as 1-based "CSS Grid Line" numbers (which are always within the
/// explicit grid). Convert one to OriginZero coordinates.
fn oz_line(line: u32) -> i32 {
    line as i32 - 1
}

/// Clamp a computed OriginZero line into the limited grid `[-MAX_GRID_TRACKS, MAX_GRID_TRACKS]`
fn clamp_oz_line(line: i32) -> OriginZeroLine {
    OriginZeroLine(line.clamp(MIN_OZ_LINE as i32, MAX_OZ_LINE as i32) as i16)
}

impl<S: CheapCloneStr> NamedLineResolver<S> {
    /// Create and initialise a new `NamedLineResolver`
    pub(crate) fn new(
        style: &impl GridContainerStyle<CustomIdent = S>,
        column_auto_repetitions: u16,
        row_auto_repetitions: u16,
    ) -> Self {
        let mut column_lines: NamedGridLinesMap<S> = Map::new();
        let mut row_lines: NamedGridLinesMap<S> = Map::new();
        let mut areas: GridAreasMap<S> = Map::new();

        let mut column_line_name_pairs: Vec<(u32, S)> = Vec::new();
        let mut row_line_name_pairs: Vec<(u32, S)> = Vec::new();

        // The size of the area template may be larger than the extents of the named areas
        // due to unnamed (`.`) cells, so it is taken from the style rather than being derived
        // from the areas themselves.
        let area_column_count = style.grid_template_area_column_count();
        let area_row_count = style.grid_template_area_row_count();
        if let Some(area_iter) = style.grid_template_areas() {
            for area in area_iter.into_iter() {
                let lines = AreaLines {
                    row_start: area.row_start as u32,
                    row_end: area.row_end as u32,
                    column_start: area.column_start as u32,
                    column_end: area.column_end as u32,
                };
                areas.insert(StrHasher(area.name.clone()), lines);
            }
        }

        let mut current_line = 0;
        if let Some(mut column_tracks) = style.grid_template_columns() {
            if let Some(column_line_names_iter) = style.grid_template_column_names() {
                for line_names in column_line_names_iter {
                    current_line += 1;
                    for line_name in line_names.into_iter() {
                        column_line_name_pairs.push((current_line, line_name.clone()));
                        upsert_line_name_map(
                            &mut column_lines,
                            &mut areas,
                            GridAreaAxis::Column,
                            line_name.clone(),
                            current_line,
                        );
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
                                    column_line_name_pairs.push((line, line_name.clone()));
                                    upsert_line_name_map(
                                        &mut column_lines,
                                        &mut areas,
                                        GridAreaAxis::Column,
                                        line_name.clone(),
                                        line,
                                    );
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
                        row_line_name_pairs.push((current_line, line_name.clone()));
                        upsert_line_name_map(
                            &mut row_lines,
                            &mut areas,
                            GridAreaAxis::Row,
                            line_name.clone(),
                            current_line,
                        );
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
                                    row_line_name_pairs.push((line, line_name.clone()));
                                    upsert_line_name_map(
                                        &mut row_lines,
                                        &mut areas,
                                        GridAreaAxis::Row,
                                        line_name.clone(),
                                        line,
                                    );
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
            row_lines,
            column_lines,
            areas,
            column_line_name_pairs,
            row_line_name_pairs,
        }
    }

    /// Build the per-line name groups of the explicit grid in the passed axis as a
    /// [`GridLineNames`], with `repeat()`s expanded. Only names given explicitly in the track
    /// template are included; the implicit `<name>-start`/`<name>-end` names generated by
    /// `grid-template-areas` are omitted (matching the resolved value of
    /// `grid-template-rows`/`grid-template-columns` in browsers) but remain resolvable for
    /// placement via [`GridLineNames::resolve_line_names`].
    ///
    /// Line indices are relative to the explicit grid (line 0 = start of the first explicit track).
    pub(crate) fn detailed_line_names(&self, axis: AbsoluteAxis) -> GridLineNames<S> {
        let (pairs, explicit_track_count) = match axis {
            AbsoluteAxis::Horizontal => (&self.column_line_name_pairs, self.explicit_column_count),
            AbsoluteAxis::Vertical => (&self.row_line_name_pairs, self.explicit_row_count),
        };

        if pairs.is_empty() {
            return GridLineNames::default();
        }

        // Stable sort by line number preserves the source order of names within each line
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
        let (lines, explicit_track_count) = match axis {
            GridAreaAxis::Row => (&self.row_lines, self.explicit_row_count),
            GridAreaAxis::Column => (&self.column_lines, self.explicit_column_count),
        };
        NamedLineResolverAxis { lines, areas: &self.areas, axis, explicit_track_count }.resolve_line_names(line)
    }

    /// Move the row and column line-name maps into the detailed grid information, and return the grid area map
    /// for the same purpose
    pub(crate) fn populate_detailed_line_resolvers(
        self,
        rows: &mut GridLineNames<S>,
        columns: &mut GridLineNames<S>,
    ) -> GridAreasMap<S> {
        rows.resolver = self.row_lines;
        columns.resolver = self.column_lines;
        self.areas
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
        writeln!(f, "Grid Areas (row-start / column-start / row-end / column-end):")?;
        for (name, area) in self.areas.iter() {
            let AreaLines { row_start, column_start, row_end, column_end } = area;
            writeln!(f, "{}: {row_start} / {column_start} / {row_end} / {column_end}", name.0.as_ref())?;
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

/// The names of each explicit grid line in a single axis, stored in CSR (compressed sparse
/// row) format: a single flat `Vec` of names plus a `Vec` of offsets into it (one more offset
/// than there are lines). The names of line `i` (0-indexed) are `names[offsets[i]..offsets[i + 1]]`.
///
/// Line indices here are relative to the explicit grid;
/// [`DetailedGridTracksInfo`](crate::DetailedGridTracksInfo) provides accessors with indices
/// relative to the full grid (including implicit tracks).
///
/// Iterate over per-line name groups with [`GridLineNames::iter`], or access a single line's
/// names with [`GridLineNames::line`]. A grid with no named lines is represented by two empty
/// `Vec`s (see [`GridLineNames::is_empty`]).
#[derive(Debug, Clone, PartialEq, Default)]
pub struct GridLineNames<S: CheapCloneStr = DefaultCheapStr> {
    /// The names of every grid line in the axis, concatenated in line order
    names: Vec<S>,
    /// Offsets into `names`: line `i`'s names are `names[offsets[i]..offsets[i + 1]]`.
    /// Either empty (no named lines) or of length `line count + 1`.
    offsets: Vec<u32>,
    /// Named line lookup used to resolve arbitrary grid placements
    resolver: NamedGridLinesMap<S>,
}

impl<S: CheapCloneStr> GridLineNames<S> {
    /// Create an empty `GridLineNames` with pre-allocated capacity
    pub(crate) fn with_capacity(name_capacity: usize, offset_capacity: usize) -> Self {
        let mut offsets = Vec::with_capacity(offset_capacity);
        offsets.push(0);
        Self { names: Vec::with_capacity(name_capacity), offsets, resolver: Map::new() }
    }

    /// Start a new (initially empty) line
    pub(crate) fn start_line(&mut self) {
        self.offsets.push(self.names.len() as u32);
    }

    /// Append a name to the current (last) line
    pub(crate) fn push_name(&mut self, name: S) {
        self.names.push(name);
        *self.offsets.last_mut().unwrap() = self.names.len() as u32;
    }

    /// Whether the current (last) line already contains the passed name
    pub(crate) fn current_line_contains(&self, name: &str) -> bool {
        self.line(self.line_count().wrapping_sub(1)).iter().any(|n| n.as_ref() == name)
    }

    /// Resolve named lines and spans using the retained line-name map and the passed grid area map
    pub(crate) fn resolve_line_names(
        &self,
        line: &Line<GridPlacement<S>>,
        areas: &GridAreasMap<S>,
        axis: GridAreaAxis,
        explicit_track_count: u16,
    ) -> Line<NonNamedGridPlacement> {
        NamedLineResolverAxis { lines: &self.resolver, areas, axis, explicit_track_count }.resolve_line_names(line)
    }

    /// Whether the axis has any named lines at all
    pub fn is_empty(&self) -> bool {
        self.names.is_empty()
    }

    /// The number of grid lines represented (zero if the grid has no named lines)
    pub fn line_count(&self) -> usize {
        self.offsets.len().saturating_sub(1)
    }

    /// The names of the line with the passed 0-indexed line index.
    /// Returns an empty slice if the line has no names or the index is out of range.
    pub fn line(&self, line_index: usize) -> &[S] {
        match (self.offsets.get(line_index), self.offsets.get(line_index + 1)) {
            (Some(&start), Some(&end)) => &self.names[start as usize..end as usize],
            _ => &[],
        }
    }

    /// Iterate over the name group (`&[S]`) of each grid line in line order
    pub fn iter(&self) -> GridLineNamesIter<'_, S> {
        self.iter_padded(0, 0)
    }

    /// Iterate over the name group (`&[S]`) of each grid line in line order, additionally
    /// yielding `leading_empty` empty groups before the stored lines and `trailing_empty`
    /// empty groups after them (representing unnamed implicit grid lines)
    pub(crate) fn iter_padded(&self, leading_empty: usize, trailing_empty: usize) -> GridLineNamesIter<'_, S> {
        GridLineNamesIter { names: &self.names, offsets: self.offsets.windows(2), leading_empty, trailing_empty }
    }
}

impl<'a, S: CheapCloneStr> IntoIterator for &'a GridLineNames<S> {
    type Item = &'a [S];
    type IntoIter = GridLineNamesIter<'a, S>;
    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

/// Iterator over the per-line name groups of a [`GridLineNames`]. Yields one `&[S]` per grid
/// line (which is empty for unnamed lines)
#[derive(Debug, Clone)]
pub struct GridLineNamesIter<'a, S: CheapCloneStr> {
    /// The flat name storage being iterated over
    names: &'a [S],
    /// Iterator over adjacent pairs of offsets into `names`
    offsets: core::slice::Windows<'a, u32>,
    /// Number of (unnamed, implicit) lines remaining before the stored lines
    leading_empty: usize,
    /// Number of (unnamed, implicit) lines remaining after the stored lines
    trailing_empty: usize,
}

impl<'a, S: CheapCloneStr> Iterator for GridLineNamesIter<'a, S> {
    type Item = &'a [S];

    fn next(&mut self) -> Option<Self::Item> {
        if self.leading_empty > 0 {
            self.leading_empty -= 1;
            return Some(&[]);
        }
        if let Some(window) = self.offsets.next() {
            return Some(&self.names[window[0] as usize..window[1] as usize]);
        }
        if self.trailing_empty > 0 {
            self.trailing_empty -= 1;
            return Some(&[]);
        }
        None
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let len = self.leading_empty + self.offsets.len() + self.trailing_empty;
        (len, Some(len))
    }
}

impl<S: CheapCloneStr> ExactSizeIterator for GridLineNamesIter<'_, S> {}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::style::GenericGridPlacement;
    use crate::sys::DefaultCheapStr;
    use crate::GridTemplateAreas;
    use crate::Style;
    use crate::{GridTemplateArea, NonNamedGridPlacement};

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
        // Lines are clamped to the limits of the grid, in the same way as `GridLine::into_origin_zero_line`
        let resolver = resolver(10_000);
        assert_eq!(
            resolved_start_line(&resolver, GridPlacement::NamedLine(DefaultCheapStr::from("missing"), i16::MAX)),
            OriginZeroLine(MAX_OZ_LINE).into_grid_line(10_000).as_i16()
        );
        assert_eq!(
            resolved_start_line(&resolver, GridPlacement::NamedLine(DefaultCheapStr::from("missing"), i16::MIN)),
            OriginZeroLine(MIN_OZ_LINE).into_grid_line(10_000).as_i16()
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
            GenericGridPlacement::Line(line) => assert_eq!(line, OriginZeroLine(MAX_OZ_LINE).into_grid_line(10_000)),
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
            OriginZeroLine(MAX_OZ_LINE).into_grid_line(0).as_i16()
        );
        assert_eq!(
            resolved_start_line(&resolver, GridPlacement::NamedLine(DefaultCheapStr::from("area"), 0)),
            OriginZeroLine(MAX_OZ_LINE).into_grid_line(0).as_i16()
        );
    }

    /// A resolver for a grid with 4 explicit columns, the passed names for each of the 5 column lines, and a
    /// single grid area named "area" which spans columns 2 and 3 (lines 2 to 4)
    fn area_resolver(column_line_names: [&[&str]; 5]) -> NamedLineResolver<DefaultCheapStr> {
        let style = Style {
            grid_template_column_names: column_line_names
                .iter()
                .map(|names| names.iter().map(|name| DefaultCheapStr::from(*name)).collect())
                .collect(),
            grid_template_areas: Some(GridTemplateAreas {
                areas: vec![GridTemplateArea {
                    name: DefaultCheapStr::from("area"),
                    row_start: 1,
                    row_end: 2,
                    column_start: 2,
                    column_end: 4,
                }],
                row_count: 1,
                column_count: 4,
            }),
            ..Style::DEFAULT
        };
        let mut resolver = NamedLineResolver::new(&style, 0, 0);
        resolver.set_explicit_column_count(4);
        resolver.set_explicit_row_count(1);
        resolver
    }

    fn named_line(name: &str, index: i16) -> GridPlacement<DefaultCheapStr> {
        GridPlacement::NamedLine(DefaultCheapStr::from(name), index)
    }

    fn resolved_end_line(
        resolver: &NamedLineResolver<DefaultCheapStr>,
        placement: GridPlacement<DefaultCheapStr>,
    ) -> i16 {
        let resolved = resolver.resolve_column_names(&Line { start: GridPlacement::Auto, end: placement });
        match resolved.end {
            GenericGridPlacement::Line(line) => line.as_i16(),
            _ => panic!("expected a resolved line"),
        }
    }

    #[test]
    fn bare_ident_resolves_to_area_edge() {
        let resolver = area_resolver([&[], &[], &[], &[], &[]]);
        assert_eq!(resolved_start_line(&resolver, named_line("area", 0)), 2);
        assert_eq!(resolved_end_line(&resolver, named_line("area", 0)), 4);

        let rows = resolver.resolve_row_names(&Line { start: named_line("area", 0), end: named_line("area", 0) });
        assert_eq!(rows.start, NonNamedGridPlacement::Line(GridLine::from(1)));
        assert_eq!(rows.end, NonNamedGridPlacement::Line(GridLine::from(2)));
    }

    #[test]
    fn bare_ident_resolves_to_edge_of_implicitly_named_area() {
        let resolver = area_resolver([&[], &["foo-start"], &["foo-start"], &["foo-end"], &["foo-end"]]);
        assert_eq!(resolved_start_line(&resolver, named_line("foo", 0)), 2);
        assert_eq!(resolved_end_line(&resolver, named_line("foo", 0)), 4);
    }

    #[test]
    fn area_edges_are_lines_named_with_start_and_end_suffixes() {
        let resolver = area_resolver([&[], &[], &[], &[], &[]]);
        assert_eq!(resolved_start_line(&resolver, named_line("area-start", 0)), 2);
        assert_eq!(resolved_start_line(&resolver, named_line("area-start", 1)), 2);
        assert_eq!(resolved_start_line(&resolver, named_line("area-end", 1)), 4);
        assert_eq!(resolved_start_line(&resolver, named_line("area-end", -1)), 4);
        // There is only one such line, so the second is the first implicit line after the grid
        assert_eq!(resolved_start_line(&resolver, named_line("area-start", 2)), 6);
    }

    #[test]
    fn area_edges_are_merged_with_explicitly_named_lines() {
        // The area's start edge is line 2 and its end edge is line 4
        let resolver = area_resolver([&["area-start"], &[], &["area-start"], &["area-end"], &["area-end"]]);
        assert_eq!(resolved_start_line(&resolver, named_line("area-start", 1)), 1);
        assert_eq!(resolved_start_line(&resolver, named_line("area-start", 2)), 2);
        assert_eq!(resolved_start_line(&resolver, named_line("area-start", 3)), 3);
        assert_eq!(resolved_start_line(&resolver, named_line("area-start", -1)), 3);
        assert_eq!(resolved_start_line(&resolver, named_line("area-start", 4)), 6);
        // The area's end edge has the same line number as an explicitly named line
        assert_eq!(resolved_start_line(&resolver, named_line("area-end", 1)), 4);
        assert_eq!(resolved_start_line(&resolver, named_line("area-end", 2)), 5);
        assert_eq!(resolved_start_line(&resolver, named_line("area-end", 3)), 6);
        // A bare ident matches the first line of the edge
        assert_eq!(resolved_start_line(&resolver, named_line("area", 0)), 1);
        assert_eq!(resolved_end_line(&resolver, named_line("area", 0)), 4);

        let span = resolver.resolve_column_names(&Line {
            start: GridPlacement::Line(GridLine::from(1)),
            end: GridPlacement::NamedSpan(DefaultCheapStr::from("area-start"), 2),
        });
        assert_eq!(span.end, NonNamedGridPlacement::Line(GridLine::from(3)));
    }

    #[test]
    fn area_edges_are_only_matched_by_bare_idents() {
        let resolver = area_resolver([&[], &[], &[], &[], &[]]);
        // There are no lines named "area", so these resolve to implicit lines after the grid
        assert_eq!(resolved_start_line(&resolver, named_line("area", 1)), 6);
        assert_eq!(resolved_start_line(&resolver, named_line("area", 2)), 7);

        let span = resolver.resolve_column_names(&Line {
            start: GridPlacement::Line(GridLine::from(1)),
            end: GridPlacement::NamedSpan(DefaultCheapStr::from("area"), 0),
        });
        assert_eq!(span.end, NonNamedGridPlacement::Line(GridLine::from(6)));
    }

    #[test]
    fn area_edge_is_the_first_line_with_the_edge_name() {
        // Explicit lines named after the area's edges which come after the area's generated lines (lines 2 and
        // 4) don't replace them
        let resolver = area_resolver([&[], &[], &["area-start"], &[], &["area-end"]]);
        assert_eq!(resolved_start_line(&resolver, named_line("area", 0)), 2);
        assert_eq!(resolved_end_line(&resolver, named_line("area", 0)), 4);
        assert_eq!(resolved_start_line(&resolver, named_line("area-start", 1)), 2);
        assert_eq!(resolved_start_line(&resolver, named_line("area-start", 2)), 3);
        assert_eq!(resolved_start_line(&resolver, named_line("area-end", 2)), 5);

        // An explicit line with the same number as the generated line is the same line
        let resolver = area_resolver([&[], &["area-start"], &[], &["area-end"], &[]]);
        assert_eq!(resolved_start_line(&resolver, named_line("area", 0)), 2);
        assert_eq!(resolved_end_line(&resolver, named_line("area", 0)), 4);
        assert_eq!(resolved_start_line(&resolver, named_line("area-start", 1)), 2);
        assert_eq!(resolved_start_line(&resolver, named_line("area-start", 2)), 6);
        assert_eq!(resolved_start_line(&resolver, named_line("area-end", 2)), 6);
    }
}
