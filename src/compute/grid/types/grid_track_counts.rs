use crate::geometry::Line;
use core::ops::Range;

#[derive(Clone, Copy, Debug, PartialEq)]
pub(crate) struct TrackCounts {
    pub negative_implicit: u16,
    pub explicit: u16,
    pub positive_implicit: u16,
}

impl TrackCounts {
    pub fn from_raw(negative_implicit: u16, explicit: u16, positive_implicit: u16) -> Self {
        Self { negative_implicit, explicit, positive_implicit }
    }
}

impl Default for TrackCounts {
    fn default() -> Self {
        Self { explicit: 0, negative_implicit: 0, positive_implicit: 0 }
    }
}

//  0  1  2  3  4  5  6  7  8  9
// [I, I, I, E, E, E, E, I, I, I]

// negative_implicit_track_count=3;
// explicit_track_count=4;
// positive_implicit_track_count=3

// -2 =>

impl TrackCounts {
    pub fn from_explicit(count: u16) -> Self {
        Self { negative_implicit: 0, explicit: count, positive_implicit: 0 }
    }

    pub fn len(&self) -> usize {
        return (self.negative_implicit + self.explicit + self.positive_implicit) as usize;
    }

    pub fn oz_line_to_next_track(&self, index: i16) -> i16 {
        index + (self.negative_implicit as i16)
    }

    pub fn oz_line_to_grid_track_vec_index(&self, index: i16) -> u16 {
        assert!(
            index >= -(self.negative_implicit as i16),
            "origin-zero grid line cannot be less than the number of negative grid lines"
        );
        assert!(
            index <= (self.explicit + self.positive_implicit) as i16,
            "origin-zero grid line cannot be more than the number of positive grid lines"
        );
        2 * ((index + self.negative_implicit as i16) as u16)
    }

    pub fn track_to_prev_oz_line(&self, index: u16) -> i16 {
        (index as i16) - (self.negative_implicit as i16)
    }

    pub fn oz_line_range_to_track_range(&self, input: Line<i16>) -> Range<i16> {
        let start = self.oz_line_to_next_track(input.start);
        let end = self.oz_line_to_next_track(input.end); // Don't subtract 1 as output range is exclusive
        start..end
    }

    pub fn track_range_to_oz_line_range(&self, input: Range<i16>) -> Line<i16> {
        let start = self.track_to_prev_oz_line(input.start as u16);
        let end = self.track_to_prev_oz_line(input.end as u16); // Don't add 1 as input range is exclusive
        Line { start, end }
    }
}
