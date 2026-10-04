//! A fixed-capacity `Vec` builder that can be filled from both ends
use crate::util::sys::Vec;

/// Builds a `Vec` of a fixed length that is known up front by pushing items to either its front or its back.
///
/// Items pushed to the front end up at the start of the `Vec` in the order that they were pushed. Items pushed to the
/// back end up at the end of the `Vec` in the reverse of the order that they were pushed. This allows a list of items
/// to be partitioned into two groups in a single pass and with a single allocation.
///
/// Exactly `capacity` items must be pushed before calling [`into_vec`](Self::into_vec), which panics otherwise.
/// If the `FrontBackVecBuilder` is dropped before then (or if too many items are pushed), then items that were
/// pushed to it are leaked rather than dropped.
pub(crate) struct FrontBackVecBuilder<T> {
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

impl<T> FrontBackVecBuilder<T> {
    /// Create a new `FrontBackVecBuilder` that will contain exactly `capacity` items
    #[inline(always)]
    pub(crate) fn with_capacity(capacity: usize) -> Self {
        Self { vec: Vec::with_capacity(capacity), capacity, front_len: 0, back_len: 0 }
    }

    /// Add an item after the items that have already been pushed to the front.
    ///
    /// Pushing more than `capacity` items in total is not checked here: it either panics or overwrites (and leaks)
    /// a previously pushed item, and then causes [`into_vec`](Self::into_vec) to panic.
    #[inline(always)]
    pub(crate) fn push_front(&mut self, value: T) {
        self.vec.spare_capacity_mut()[self.front_len].write(value);
        self.front_len += 1;
    }

    /// Add an item before the items that have already been pushed to the back.
    ///
    /// Pushing more than `capacity` items in total is not checked here: it either panics or overwrites (and leaks)
    /// a previously pushed item, and then causes [`into_vec`](Self::into_vec) to panic.
    #[inline(always)]
    pub(crate) fn push_back(&mut self, value: T) {
        self.vec.spare_capacity_mut()[self.capacity - self.back_len - 1].write(value);
        self.back_len += 1;
    }

    /// Returns the completed `Vec`, along with the number of items that were pushed to the front (which is also
    /// the index of the first item that was pushed to the back).
    ///
    /// Panics if the total number of items that have been pushed is not exactly `capacity`.
    #[inline(always)]
    pub(crate) fn into_vec(self) -> (Vec<T>, usize) {
        let Self { mut vec, capacity, front_len, back_len } = self;
        assert_eq!(
            front_len + back_len,
            capacity,
            "the number of items pushed to a FrontBackVecBuilder must equal its capacity"
        );
        // SAFETY:
        //   - `vec` was allocated with a capacity of at least `capacity` and has not been reallocated since.
        //   - Each `push_front` initializes the slot at index `front_len` and then increments `front_len`, so the
        //     slots `0..front_len` are initialized. Likewise each `push_back` initializes the slot just before the
        //     ones it has already initialized, so the slots `(capacity - back_len)..capacity` are initialized.
        //     (These writes are bounds checked, and writing to a slot that is already initialized is safe).
        //   - `front_len + back_len == capacity` (asserted above), so `capacity - back_len == front_len` and those
        //     two ranges cover all of `0..capacity`.
        #[allow(unsafe_code)]
        unsafe {
            vec.set_len(capacity);
        }
        (vec, front_len)
    }
}

#[cfg(test)]
mod tests {
    use super::FrontBackVecBuilder;
    use crate::util::sys::Vec;

    #[test]
    fn front_items_keep_their_order_and_back_items_are_reversed() {
        let mut builder = FrontBackVecBuilder::with_capacity(6);
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
        let mut builder = FrontBackVecBuilder::with_capacity(3);
        (0..3).for_each(|value| builder.push_front(value));
        assert_eq!(builder.into_vec(), (Vec::from([0, 1, 2]), 3));

        let mut builder = FrontBackVecBuilder::with_capacity(3);
        (0..3).for_each(|value| builder.push_back(value));
        assert_eq!(builder.into_vec(), (Vec::from([2, 1, 0]), 0));
    }

    #[test]
    fn zero_capacity() {
        let (vec, front_len) = FrontBackVecBuilder::<u8>::with_capacity(0).into_vec();
        assert!(vec.is_empty());
        assert_eq!(front_len, 0);
    }

    #[test]
    fn owned_items_are_moved_into_the_vec() {
        let mut builder = FrontBackVecBuilder::with_capacity(2);
        builder.push_back(Vec::from([1, 2]));
        builder.push_front(Vec::from([3]));
        let (vec, front_len) = builder.into_vec();
        assert_eq!(vec, [Vec::from([3]), Vec::from([1, 2])]);
        assert_eq!(front_len, 1);
    }

    #[test]
    #[should_panic(expected = "must equal its capacity")]
    fn pushing_too_many_items_to_the_front_panics_in_into_vec() {
        let mut builder = FrontBackVecBuilder::with_capacity(1);
        builder.push_back(1);
        builder.push_front(2);
        let _ = builder.into_vec();
    }

    #[test]
    #[should_panic(expected = "must equal its capacity")]
    fn pushing_too_many_items_to_the_back_panics_in_into_vec() {
        let mut builder = FrontBackVecBuilder::with_capacity(1);
        builder.push_front(1);
        builder.push_back(2);
        let _ = builder.into_vec();
    }

    #[test]
    #[should_panic]
    fn push_front_out_of_bounds_panics() {
        let mut builder = FrontBackVecBuilder::with_capacity(0);
        builder.push_front(1);
    }

    #[test]
    #[should_panic]
    fn push_back_out_of_bounds_panics() {
        let mut builder = FrontBackVecBuilder::with_capacity(1);
        builder.push_back(1);
        builder.push_back(2);
    }

    #[test]
    #[should_panic(expected = "must equal its capacity")]
    fn into_vec_before_full_panics() {
        let mut builder = FrontBackVecBuilder::with_capacity(2);
        builder.push_front(1);
        let _ = builder.into_vec();
    }
}
