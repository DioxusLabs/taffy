//! A fixed-capacity `Vec` builder that can be filled from both ends
use crate::util::sys::Vec;

/// Builds a `Vec` of a fixed length that is known up front by pushing items to either its front or its back.
///
/// Items pushed to the front end up at the start of the `Vec` in the order that they were pushed. Items pushed to the
/// back end up at the end of the `Vec` in the reverse of the order that they were pushed. This allows a list of items
/// to be partitioned into two groups in a single pass and with a single allocation.
///
/// Exactly `capacity` items must be pushed before calling [`into_vec`](Self::into_vec). If the `FrontBackVec` is
/// dropped before then, the items that were pushed to it are leaked rather than dropped.
pub(crate) struct FrontBackVec<T> {
    /// The underlying storage. Its length is kept at zero until `into_vec` is called, and its allocated capacity
    /// is at least `capacity`.
    vec: Vec<T>,
    /// The total number of items that the `Vec` will contain
    capacity: usize,
    /// The number of items pushed to the front. The slots `0..front_len` are initialized.
    front_len: usize,
    /// The number of items pushed to the back. The slots `(capacity - back_len)..capacity` are initialized.
    back_len: usize,
}

impl<T> FrontBackVec<T> {
    /// Create a new `FrontBackVec` that will contain exactly `capacity` items
    #[inline(always)]
    pub(crate) fn with_capacity(capacity: usize) -> Self {
        Self { vec: Vec::with_capacity(capacity), capacity, front_len: 0, back_len: 0 }
    }

    /// Add an item after the items that have already been pushed to the front.
    ///
    /// Panics if `capacity` items have already been pushed.
    #[inline(always)]
    pub(crate) fn push_front(&mut self, value: T) {
        assert!(
            self.front_len + self.back_len < self.capacity,
            "pushed more items than the capacity of a FrontBackVec"
        );
        self.vec.spare_capacity_mut()[self.front_len].write(value);
        self.front_len += 1;
    }

    /// Add an item before the items that have already been pushed to the back.
    ///
    /// Panics if `capacity` items have already been pushed.
    #[inline(always)]
    pub(crate) fn push_back(&mut self, value: T) {
        assert!(
            self.front_len + self.back_len < self.capacity,
            "pushed more items than the capacity of a FrontBackVec"
        );
        self.vec.spare_capacity_mut()[self.capacity - self.back_len - 1].write(value);
        self.back_len += 1;
    }

    /// Returns the completed `Vec`, along with the number of items that were pushed to the front (which is also
    /// the index of the first item that was pushed to the back).
    ///
    /// Panics if fewer than `capacity` items have been pushed.
    #[inline(always)]
    pub(crate) fn into_vec(self) -> (Vec<T>, usize) {
        let Self { mut vec, capacity, front_len, back_len } = self;
        assert_eq!(front_len + back_len, capacity, "pushed fewer items than the capacity of a FrontBackVec");
        // SAFETY:
        //   - `vec` was allocated with a capacity of at least `capacity` and has not been reallocated since.
        //   - `push_front` and `push_back` each initialize one slot that was not previously initialized (they refuse to
        //     write once `front_len + back_len == capacity`, so the two initialized ranges cannot overlap), so the
        //     slots `0..front_len` and `(capacity - back_len)..capacity` are initialized.
        //   - `front_len + back_len == capacity` (asserted above), so those ranges cover all of `0..capacity`.
        #[allow(unsafe_code)]
        unsafe {
            vec.set_len(capacity);
        }
        (vec, front_len)
    }
}

#[cfg(test)]
mod tests {
    use super::FrontBackVec;
    use crate::util::sys::Vec;

    #[test]
    fn front_items_keep_their_order_and_back_items_are_reversed() {
        let mut builder = FrontBackVec::with_capacity(6);
        for value in 0..6 {
            if value % 2 == 0 {
                builder.push_front(value);
            } else {
                builder.push_back(value);
            }
        }
        let (vec, front_len) = builder.into_vec();
        assert_eq!(vec, [0, 2, 4, 5, 3, 1]);
        assert_eq!(front_len, 3);
    }

    #[test]
    fn all_items_pushed_to_one_end() {
        let mut builder = FrontBackVec::with_capacity(3);
        (0..3).for_each(|value| builder.push_front(value));
        assert_eq!(builder.into_vec(), (Vec::from([0, 1, 2]), 3));

        let mut builder = FrontBackVec::with_capacity(3);
        (0..3).for_each(|value| builder.push_back(value));
        assert_eq!(builder.into_vec(), (Vec::from([2, 1, 0]), 0));
    }

    #[test]
    fn zero_capacity() {
        let (vec, front_len) = FrontBackVec::<u8>::with_capacity(0).into_vec();
        assert!(vec.is_empty());
        assert_eq!(front_len, 0);
    }

    #[test]
    fn owned_items_are_moved_into_the_vec() {
        let mut builder = FrontBackVec::with_capacity(2);
        builder.push_back(Vec::from([1, 2]));
        builder.push_front(Vec::from([3]));
        let (vec, front_len) = builder.into_vec();
        assert_eq!(vec, [Vec::from([3]), Vec::from([1, 2])]);
        assert_eq!(front_len, 1);
    }

    #[test]
    #[should_panic(expected = "pushed more items than the capacity")]
    fn push_front_beyond_capacity_panics() {
        let mut builder = FrontBackVec::with_capacity(1);
        builder.push_back(1);
        builder.push_front(2);
    }

    #[test]
    #[should_panic(expected = "pushed more items than the capacity")]
    fn push_back_beyond_capacity_panics() {
        let mut builder = FrontBackVec::with_capacity(1);
        builder.push_front(1);
        builder.push_back(2);
    }

    #[test]
    #[should_panic(expected = "pushed fewer items than the capacity")]
    fn into_vec_before_full_panics() {
        let mut builder = FrontBackVec::with_capacity(2);
        builder.push_front(1);
        let _ = builder.into_vec();
    }
}
