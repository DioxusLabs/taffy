#![allow(unsafe_code)]

// Copyright (c) 2017 Jorge Aparicio

// Permission is hereby granted, free of charge, to any
// person obtaining a copy of this software and associated
// documentation files (the "Software"), to deal in the
// Software without restriction, including without
// limitation the rights to use, copy, modify, merge,
// publish, distribute, sublicense, and/or sell copies of
// the Software, and to permit persons to whom the Software
// is furnished to do so, subject to the following
// conditions:

// The above copyright notice and this permission notice
// shall be included in all copies or substantial portions
// of the Software.

// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF
// ANY KIND, EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED
// TO THE WARRANTIES OF MERCHANTABILITY, FITNESS FOR A
// PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT
// SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY
// CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION
// OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR
// IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER
// DEALINGS IN THE SOFTWARE.

mod sealed {
    // copied from https://github.com/japaric/heapless/blob/39273893deda34fa741f79438134fd4d4d7ac3ae/src/sealed.rs
    #[allow(clippy::no_effect)]
    #[allow(dead_code)]
    #[allow(path_statements)]
    pub(super) const fn smaller_than<const N: usize, const MAX: usize>() {
        Assert::<N, MAX>::LESS;
    }

    #[allow(clippy::no_effect)]
    #[allow(dead_code)]
    #[allow(path_statements)]
    pub(super) const fn greater_than_eq_0<const N: usize>() {
        Assert::<N, 0>::GREATER_EQ;
    }

    #[allow(clippy::no_effect)]
    #[allow(dead_code)]
    #[allow(path_statements)]
    pub(super) const fn greater_than_0<const N: usize>() {
        Assert::<N, 0>::GREATER;
    }

    #[allow(clippy::no_effect)]
    #[allow(dead_code)]
    #[allow(path_statements)]
    pub(super) const fn greater_than_1<const N: usize>() {
        Assert::<N, 1>::GREATER;
    }

    #[allow(clippy::no_effect)]
    #[allow(dead_code)]
    #[allow(path_statements)]
    pub(super) const fn power_of_two<const N: usize>() {
        Assert::<N, 0>::GREATER;
        Assert::<N, 0>::POWER_OF_TWO;
    }

    #[allow(clippy::no_effect)]
    #[allow(dead_code)]
    /// Const assert hack
    pub(super) struct Assert<const L: usize, const R: usize>;

    #[allow(dead_code)]
    impl<const L: usize, const R: usize> Assert<L, R> {
        /// Const assert hack
        pub(super) const GREATER_EQ: usize = L - R;

        /// Const assert hack
        pub(super) const LESS_EQ: usize = R - L;

        #[allow(clippy::erasing_op)]
        /// Const assert hack
        pub(super) const NOT_EQ: isize = 0 / (R as isize - L as isize);

        /// Const assert hack
        pub(super) const EQ: usize = (R - L) + (L - R);

        /// Const assert hack
        pub(super) const GREATER: usize = L - R - 1;

        /// Const assert hack
        pub(super) const LESS: usize = R - L - 1;

        /// Const assert hack
        pub(super) const POWER_OF_TWO: usize = 0 - (L & (L - 1));
    }
}

mod vec {
    // copied from https://github.com/japaric/heapless/blob/39273893deda34fa741f79438134fd4d4d7ac3ae/src/vec.rs
    use core::{cmp::Ordering, convert::TryFrom, fmt, hash, iter::FromIterator, mem::MaybeUninit, ops, ptr, slice};
    use hash32;

    /// A fixed capacity [`Vec`](https://doc.rust-lang.org/std/vec/struct.Vec.html)
    pub(super) struct Vec<T, const N: usize> {
        // NOTE order is important for optimizations. the `len` first layout lets the compiler optimize
        // `new` to: reserve stack space and zero the first word. With the fields in the reverse order
        // the compiler optimizes `new` to `memclr`-ing the *entire* stack space, including the `buffer`
        // field which should be left uninitialized. Optimizations were last checked with Rust 1.60
        len: usize,

        buffer: [MaybeUninit<T>; N],
    }

    impl<T, const N: usize> Vec<T, N> {
        const ELEM: MaybeUninit<T> = MaybeUninit::uninit();
        const INIT: [MaybeUninit<T>; N] = [Self::ELEM; N]; // important for optimization of `new`

        /// Constructs a new, empty vector with a fixed capacity of `N`
        /// `Vec` `const` constructor; wrap the returned value in [`Vec`](../struct.Vec.html)
        pub(super) const fn new() -> Self {
            // Const assert N >= 0
            super::sealed::greater_than_eq_0::<N>();

            Self { len: 0, buffer: Self::INIT }
        }

        /// Constructs a new vector with a fixed capacity of `N` and fills it
        /// with the provided slice.
        #[inline]
        pub(super) fn from_slice(other: &[T]) -> Result<Self, ()>
        where
            T: Clone,
        {
            let mut v = Vec::new();
            v.extend_from_slice(other)?;
            Ok(v)
        }

        /// Clones a vec into a new vec
        pub(super) fn clone(&self) -> Self
        where
            T: Clone,
        {
            let mut new = Self::new();
            // avoid `extend_from_slice` as that introduces a runtime check / panicking branch
            for elem in self {
                unsafe {
                    new.push_unchecked(elem.clone());
                }
            }
            new
        }

        /// Extracts a slice containing the entire vector.
        ///
        /// Equivalent to `&s[..]`.
        pub(super) fn as_slice(&self) -> &[T] {
            // NOTE(unsafe) avoid bound checks in the slicing operation
            // &buffer[..self.len]
            unsafe { slice::from_raw_parts(self.buffer.as_ptr() as *const T, self.len) }
        }

        /// Returns the contents of the vector as an array of length `M` if the length
        /// of the vector is exactly `M`, otherwise returns `Err(self)`.
        pub(super) fn into_array<const M: usize>(self) -> Result<[T; M], Self> {
            if self.len() == M {
                // This is how the unstable `MaybeUninit::array_assume_init` method does it
                let array = unsafe { (&self.buffer as *const _ as *const [T; M]).read() };

                // We don't want `self`'s destructor to be called because that would drop all the
                // items in the array
                core::mem::forget(self);

                Ok(array)
            } else {
                Err(self)
            }
        }

        /// Extracts a mutable slice containing the entire vector.
        ///
        /// Equivalent to `&s[..]`.
        pub(super) fn as_mut_slice(&mut self) -> &mut [T] {
            // NOTE(unsafe) avoid bound checks in the slicing operation
            // &mut buffer[..self.len]
            unsafe { slice::from_raw_parts_mut(self.buffer.as_mut_ptr() as *mut T, self.len) }
        }

        /// Returns the maximum number of elements the vector can hold.
        pub(super) const fn capacity(&self) -> usize {
            N
        }

        /// Clears the vector, removing all values.
        pub(super) fn clear(&mut self) {
            self.truncate(0);
        }

        /// Extends the vec from an iterator.
        ///
        /// # Panic
        ///
        /// Panics if the vec cannot hold all elements of the iterator.
        pub(super) fn extend<I>(&mut self, iter: I)
        where
            I: IntoIterator<Item = T>,
        {
            for elem in iter {
                self.push(elem).ok().unwrap()
            }
        }

        /// Clones and appends all elements in a slice to the `Vec`.
        ///
        /// Iterates over the slice `other`, clones each element, and then appends
        /// it to this `Vec`. The `other` vector is traversed in-order.
        pub(super) fn extend_from_slice(&mut self, other: &[T]) -> Result<(), ()>
        where
            T: Clone,
        {
            if self.len + other.len() > self.capacity() {
                // won't fit in the `Vec`; don't modify anything and return an error
                Err(())
            } else {
                for elem in other {
                    unsafe {
                        self.push_unchecked(elem.clone());
                    }
                }
                Ok(())
            }
        }

        /// Removes the last element from a vector and returns it, or `None` if it's empty
        pub(super) fn pop(&mut self) -> Option<T> {
            if self.len != 0 {
                Some(unsafe { self.pop_unchecked() })
            } else {
                None
            }
        }

        /// Appends an `item` to the back of the collection
        ///
        /// Returns back the `item` if the vector is full
        pub(super) fn push(&mut self, item: T) -> Result<(), T> {
            if self.len < self.capacity() {
                unsafe { self.push_unchecked(item) }
                Ok(())
            } else {
                Err(item)
            }
        }

        /// Removes the last element from a vector and returns it
        ///
        /// # Safety
        ///
        /// This assumes the vec to have at least one element.
        pub(super) unsafe fn pop_unchecked(&mut self) -> T {
            debug_assert!(!self.is_empty());

            self.len -= 1;
            (self.buffer.get_unchecked_mut(self.len).as_ptr() as *const T).read()
        }

        /// Appends an `item` to the back of the collection
        ///
        /// # Safety
        ///
        /// This assumes the vec is not full.
        pub(super) unsafe fn push_unchecked(&mut self, item: T) {
            // NOTE(ptr::write) the memory slot that we are about to write to is uninitialized. We
            // use `ptr::write` to avoid running `T`'s destructor on the uninitialized memory
            debug_assert!(!self.is_full());

            *self.buffer.get_unchecked_mut(self.len) = MaybeUninit::new(item);

            self.len += 1;
        }

        /// Shortens the vector, keeping the first `len` elements and dropping the rest.
        pub(super) fn truncate(&mut self, len: usize) {
            // This is safe because:
            //
            // * the slice passed to `drop_in_place` is valid; the `len > self.len`
            //   case avoids creating an invalid slice, and
            // * the `len` of the vector is shrunk before calling `drop_in_place`,
            //   such that no value will be dropped twice in case `drop_in_place`
            //   were to panic once (if it panics twice, the program aborts).
            unsafe {
                // Note: It's intentional that this is `>` and not `>=`.
                //       Changing it to `>=` has negative performance
                //       implications in some cases. See rust-lang/rust#78884 for more.
                if len > self.len {
                    return;
                }
                let remaining_len = self.len - len;
                let s = ptr::slice_from_raw_parts_mut(self.as_mut_ptr().add(len), remaining_len);
                self.len = len;
                ptr::drop_in_place(s);
            }
        }

        /// Resizes the Vec in-place so that len is equal to new_len.
        ///
        /// If new_len is greater than len, the Vec is extended by the
        /// difference, with each additional slot filled with value. If
        /// new_len is less than len, the Vec is simply truncated.
        ///
        /// See also [`resize_default`](struct.Vec.html#method.resize_default).
        pub(super) fn resize(&mut self, new_len: usize, value: T) -> Result<(), ()>
        where
            T: Clone,
        {
            if new_len > self.capacity() {
                return Err(());
            }

            if new_len > self.len {
                while self.len < new_len {
                    self.push(value.clone()).ok();
                }
            } else {
                self.truncate(new_len);
            }

            Ok(())
        }

        /// Resizes the `Vec` in-place so that `len` is equal to `new_len`.
        ///
        /// If `new_len` is greater than `len`, the `Vec` is extended by the
        /// difference, with each additional slot filled with `Default::default()`.
        /// If `new_len` is less than `len`, the `Vec` is simply truncated.
        ///
        /// See also [`resize`](struct.Vec.html#method.resize).
        pub(super) fn resize_default(&mut self, new_len: usize) -> Result<(), ()>
        where
            T: Clone + Default,
        {
            self.resize(new_len, T::default())
        }

        /// Forces the length of the vector to `new_len`.
        ///
        /// This is a low-level operation that maintains none of the normal
        /// invariants of the type. Normally changing the length of a vector
        /// is done using one of the safe operations instead, such as
        /// [`truncate`], [`resize`], [`extend`], or [`clear`].
        ///
        /// [`truncate`]: #method.truncate
        /// [`resize`]: #method.resize
        /// [`extend`]: https://doc.rust-lang.org/stable/core/iter/trait.Extend.html#tymethod.extend
        /// [`clear`]: #method.clear
        ///
        /// # Safety
        ///
        /// - `new_len` must be less than or equal to [`capacity()`].
        /// - The elements at `old_len..new_len` must be initialized.
        ///
        /// [`capacity()`]: #method.capacity
        pub(super) unsafe fn set_len(&mut self, new_len: usize) {
            debug_assert!(new_len <= self.capacity());

            self.len = new_len
        }

        /// Removes an element from the vector and returns it.
        ///
        /// The removed element is replaced by the last element of the vector.
        ///
        /// This does not preserve ordering, but is O(1).
        ///
        /// # Panics
        ///
        /// Panics if `index` is out of bounds.
        pub(super) fn swap_remove(&mut self, index: usize) -> T {
            assert!(index < self.len);
            unsafe { self.swap_remove_unchecked(index) }
        }

        /// Removes an element from the vector and returns it.
        ///
        /// The removed element is replaced by the last element of the vector.
        ///
        /// This does not preserve ordering, but is O(1).
        ///
        /// # Safety
        ///
        ///  Assumes `index` within bounds.
        pub(super) unsafe fn swap_remove_unchecked(&mut self, index: usize) -> T {
            let length = self.len();
            debug_assert!(index < length);
            let value = ptr::read(self.as_ptr().add(index));
            let base_ptr = self.as_mut_ptr();
            ptr::copy(base_ptr.add(length - 1), base_ptr.add(index), 1);
            self.len -= 1;
            value
        }

        /// Returns true if the vec is full
        #[inline]
        pub(super) fn is_full(&self) -> bool {
            self.len == self.capacity()
        }

        /// Returns true if the vec is empty
        #[inline]
        pub(super) fn is_empty(&self) -> bool {
            self.len == 0
        }

        /// Returns `true` if `needle` is a prefix of the Vec.
        ///
        /// Always returns `true` if `needle` is an empty slice.
        #[inline]
        pub(super) fn starts_with(&self, needle: &[T]) -> bool
        where
            T: PartialEq,
        {
            let n = needle.len();
            self.len >= n && needle == &self[..n]
        }

        /// Returns `true` if `needle` is a suffix of the Vec.
        ///
        /// Always returns `true` if `needle` is an empty slice.
        #[inline]
        pub(super) fn ends_with(&self, needle: &[T]) -> bool
        where
            T: PartialEq,
        {
            let (v, n) = (self.len(), needle.len());
            v >= n && needle == &self[v - n..]
        }
    }

    // Trait implementations

    impl<T, const N: usize> Default for Vec<T, N> {
        fn default() -> Self {
            Self::new()
        }
    }

    impl<T, const N: usize> fmt::Debug for Vec<T, N>
    where
        T: fmt::Debug,
    {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            <[T] as fmt::Debug>::fmt(self, f)
        }
    }

    impl<const N: usize> fmt::Write for Vec<u8, N> {
        fn write_str(&mut self, s: &str) -> fmt::Result {
            match self.extend_from_slice(s.as_bytes()) {
                Ok(()) => Ok(()),
                Err(_) => Err(fmt::Error),
            }
        }
    }

    impl<T, const N: usize> Drop for Vec<T, N> {
        fn drop(&mut self) {
            // We drop each element used in the vector by turning into a &mut[T]
            unsafe {
                ptr::drop_in_place(self.as_mut_slice());
            }
        }
    }

    impl<'a, T: Clone, const N: usize> TryFrom<&'a [T]> for Vec<T, N> {
        type Error = ();

        fn try_from(slice: &'a [T]) -> Result<Self, Self::Error> {
            Vec::from_slice(slice)
        }
    }

    impl<T, const N: usize> Extend<T> for Vec<T, N> {
        fn extend<I>(&mut self, iter: I)
        where
            I: IntoIterator<Item = T>,
        {
            self.extend(iter)
        }
    }

    impl<'a, T, const N: usize> Extend<&'a T> for Vec<T, N>
    where
        T: 'a + Copy,
    {
        fn extend<I>(&mut self, iter: I)
        where
            I: IntoIterator<Item = &'a T>,
        {
            self.extend(iter.into_iter().cloned())
        }
    }

    impl<T, const N: usize> hash::Hash for Vec<T, N>
    where
        T: core::hash::Hash,
    {
        fn hash<H: hash::Hasher>(&self, state: &mut H) {
            <[T] as hash::Hash>::hash(self, state)
        }
    }

    impl<T, const N: usize> hash32::Hash for Vec<T, N>
    where
        T: hash32::Hash,
    {
        fn hash<H: hash32::Hasher>(&self, state: &mut H) {
            <[T] as hash32::Hash>::hash(self, state)
        }
    }

    impl<'a, T, const N: usize> IntoIterator for &'a Vec<T, N> {
        type Item = &'a T;
        type IntoIter = slice::Iter<'a, T>;

        fn into_iter(self) -> Self::IntoIter {
            self.iter()
        }
    }

    impl<'a, T, const N: usize> IntoIterator for &'a mut Vec<T, N> {
        type Item = &'a mut T;
        type IntoIter = slice::IterMut<'a, T>;

        fn into_iter(self) -> Self::IntoIter {
            self.iter_mut()
        }
    }

    impl<T, const N: usize> FromIterator<T> for Vec<T, N> {
        fn from_iter<I>(iter: I) -> Self
        where
            I: IntoIterator<Item = T>,
        {
            let mut vec = Vec::new();
            for i in iter {
                vec.push(i).ok().expect("Vec::from_iter overflow");
            }
            vec
        }
    }

    /// An iterator that moves out of an [`Vec`][`Vec`].
    ///
    /// This struct is created by calling the `into_iter` method on [`Vec`][`Vec`].
    ///
    /// [`Vec`]: (https://doc.rust-lang.org/std/vec/struct.Vec.html)
    ///
    pub(super) struct IntoIter<T, const N: usize> {
        vec: Vec<T, N>,
        next: usize,
    }

    impl<T, const N: usize> Iterator for IntoIter<T, N> {
        type Item = T;
        fn next(&mut self) -> Option<Self::Item> {
            if self.next < self.vec.len() {
                let item = unsafe { (self.vec.buffer.get_unchecked_mut(self.next).as_ptr() as *const T).read() };
                self.next += 1;
                Some(item)
            } else {
                None
            }
        }
    }

    impl<T, const N: usize> Clone for IntoIter<T, N>
    where
        T: Clone,
    {
        fn clone(&self) -> Self {
            let mut vec = Vec::new();

            if self.next < self.vec.len() {
                let s = unsafe {
                    slice::from_raw_parts(
                        (self.vec.buffer.as_ptr() as *const T).add(self.next),
                        self.vec.len() - self.next,
                    )
                };
                vec.extend_from_slice(s).ok();
            }

            Self { vec, next: 0 }
        }
    }

    impl<T, const N: usize> Drop for IntoIter<T, N> {
        fn drop(&mut self) {
            unsafe {
                // Drop all the elements that have not been moved out of vec
                ptr::drop_in_place(&mut self.vec.as_mut_slice()[self.next..]);
                // Prevent dropping of other elements
                self.vec.len = 0;
            }
        }
    }

    impl<T, const N: usize> IntoIterator for Vec<T, N> {
        type Item = T;
        type IntoIter = IntoIter<T, N>;

        fn into_iter(self) -> Self::IntoIter {
            IntoIter { vec: self, next: 0 }
        }
    }

    impl<A, B, const N1: usize, const N2: usize> PartialEq<Vec<B, N2>> for Vec<A, N1>
    where
        A: PartialEq<B>,
    {
        fn eq(&self, other: &Vec<B, N2>) -> bool {
            <[A]>::eq(self, &**other)
        }
    }

    // Vec<A, N> == [B]
    impl<A, B, const N: usize> PartialEq<[B]> for Vec<A, N>
    where
        A: PartialEq<B>,
    {
        fn eq(&self, other: &[B]) -> bool {
            <[A]>::eq(self, other)
        }
    }

    // Vec<A, N> == &[B]
    impl<A, B, const N: usize> PartialEq<&[B]> for Vec<A, N>
    where
        A: PartialEq<B>,
    {
        fn eq(&self, other: &&[B]) -> bool {
            <[A]>::eq(self, &other[..])
        }
    }

    // Vec<A, N> == &mut [B]
    impl<A, B, const N: usize> PartialEq<&mut [B]> for Vec<A, N>
    where
        A: PartialEq<B>,
    {
        fn eq(&self, other: &&mut [B]) -> bool {
            <[A]>::eq(self, &other[..])
        }
    }

    // Vec<A, N> == [B; M]
    // Equality does not require equal capacity
    impl<A, B, const N: usize, const M: usize> PartialEq<[B; M]> for Vec<A, N>
    where
        A: PartialEq<B>,
    {
        fn eq(&self, other: &[B; M]) -> bool {
            <[A]>::eq(self, &other[..])
        }
    }

    // Vec<A, N> == &[B; M]
    // Equality does not require equal capacity
    impl<A, B, const N: usize, const M: usize> PartialEq<&[B; M]> for Vec<A, N>
    where
        A: PartialEq<B>,
    {
        fn eq(&self, other: &&[B; M]) -> bool {
            <[A]>::eq(self, &other[..])
        }
    }

    // Implements Eq if underlying data is Eq
    impl<T, const N: usize> Eq for Vec<T, N> where T: Eq {}

    impl<T, const N1: usize, const N2: usize> PartialOrd<Vec<T, N2>> for Vec<T, N1>
    where
        T: PartialOrd,
    {
        fn partial_cmp(&self, other: &Vec<T, N2>) -> Option<Ordering> {
            PartialOrd::partial_cmp(&**self, &**other)
        }
    }

    impl<T, const N: usize> Ord for Vec<T, N>
    where
        T: Ord,
    {
        #[inline]
        fn cmp(&self, other: &Self) -> Ordering {
            Ord::cmp(&**self, &**other)
        }
    }

    impl<T, const N: usize> ops::Deref for Vec<T, N> {
        type Target = [T];

        fn deref(&self) -> &[T] {
            self.as_slice()
        }
    }

    impl<T, const N: usize> ops::DerefMut for Vec<T, N> {
        fn deref_mut(&mut self) -> &mut [T] {
            self.as_mut_slice()
        }
    }

    impl<T, const N: usize> AsRef<Vec<T, N>> for Vec<T, N> {
        #[inline]
        fn as_ref(&self) -> &Self {
            self
        }
    }

    impl<T, const N: usize> AsMut<Vec<T, N>> for Vec<T, N> {
        #[inline]
        fn as_mut(&mut self) -> &mut Self {
            self
        }
    }

    impl<T, const N: usize> AsRef<[T]> for Vec<T, N> {
        #[inline]
        fn as_ref(&self) -> &[T] {
            self
        }
    }

    impl<T, const N: usize> AsMut<[T]> for Vec<T, N> {
        #[inline]
        fn as_mut(&mut self) -> &mut [T] {
            self
        }
    }

    impl<T, const N: usize> Clone for Vec<T, N>
    where
        T: Clone,
    {
        fn clone(&self) -> Self {
            self.clone()
        }
    }

    #[cfg(test)]
    mod tests {
        use super::Vec;
        use core::fmt::Write;

        macro_rules! droppable {
            () => {
                static COUNT: core::sync::atomic::AtomicI32 = core::sync::atomic::AtomicI32::new(0);

                #[derive(Eq, Ord, PartialEq, PartialOrd)]
                struct Droppable(i32);
                impl Droppable {
                    fn new() -> Self {
                        COUNT.fetch_add(1, core::sync::atomic::Ordering::Relaxed);
                        Droppable(Self::count())
                    }

                    fn count() -> i32 {
                        COUNT.load(core::sync::atomic::Ordering::Relaxed)
                    }
                }
                impl Drop for Droppable {
                    fn drop(&mut self) {
                        COUNT.fetch_sub(1, core::sync::atomic::Ordering::Relaxed);
                    }
                }
            };
        }

        #[test]
        fn static_new() {
            static mut _V: Vec<i32, 4> = Vec::new();
        }

        #[test]
        fn stack_new() {
            let mut _v: Vec<i32, 4> = Vec::new();
        }

        #[test]
        fn is_full_empty() {
            let mut v: Vec<i32, 4> = Vec::new();

            assert!(v.is_empty());
            assert!(!v.is_full());

            v.push(1).unwrap();
            assert!(!v.is_empty());
            assert!(!v.is_full());

            v.push(1).unwrap();
            assert!(!v.is_empty());
            assert!(!v.is_full());

            v.push(1).unwrap();
            assert!(!v.is_empty());
            assert!(!v.is_full());

            v.push(1).unwrap();
            assert!(!v.is_empty());
            assert!(v.is_full());
        }

        #[test]
        fn drop() {
            droppable!();

            {
                let mut v: Vec<Droppable, 2> = Vec::new();
                v.push(Droppable::new()).ok().unwrap();
                v.push(Droppable::new()).ok().unwrap();
                v.pop().unwrap();
            }

            assert_eq!(Droppable::count(), 0);

            {
                let mut v: Vec<Droppable, 2> = Vec::new();
                v.push(Droppable::new()).ok().unwrap();
                v.push(Droppable::new()).ok().unwrap();
            }

            assert_eq!(Droppable::count(), 0);
        }

        #[test]
        fn eq() {
            let mut xs: Vec<i32, 4> = Vec::new();
            let mut ys: Vec<i32, 8> = Vec::new();

            assert_eq!(xs, ys);

            xs.push(1).unwrap();
            ys.push(1).unwrap();

            assert_eq!(xs, ys);
        }

        #[test]
        fn cmp() {
            let mut xs: Vec<i32, 4> = Vec::new();
            let mut ys: Vec<i32, 4> = Vec::new();

            assert_eq!(xs, ys);

            xs.push(1).unwrap();
            ys.push(2).unwrap();

            assert!(xs < ys);
        }

        #[test]
        fn cmp_heterogenous_size() {
            let mut xs: Vec<i32, 4> = Vec::new();
            let mut ys: Vec<i32, 8> = Vec::new();

            assert_eq!(xs, ys);

            xs.push(1).unwrap();
            ys.push(2).unwrap();

            assert!(xs < ys);
        }

        #[test]
        fn full() {
            let mut v: Vec<i32, 4> = Vec::new();

            v.push(0).unwrap();
            v.push(1).unwrap();
            v.push(2).unwrap();
            v.push(3).unwrap();

            assert!(v.push(4).is_err());
        }

        #[test]
        fn iter() {
            let mut v: Vec<i32, 4> = Vec::new();

            v.push(0).unwrap();
            v.push(1).unwrap();
            v.push(2).unwrap();
            v.push(3).unwrap();

            let mut items = v.iter();

            assert_eq!(items.next(), Some(&0));
            assert_eq!(items.next(), Some(&1));
            assert_eq!(items.next(), Some(&2));
            assert_eq!(items.next(), Some(&3));
            assert_eq!(items.next(), None);
        }

        #[test]
        fn iter_mut() {
            let mut v: Vec<i32, 4> = Vec::new();

            v.push(0).unwrap();
            v.push(1).unwrap();
            v.push(2).unwrap();
            v.push(3).unwrap();

            let mut items = v.iter_mut();

            assert_eq!(items.next(), Some(&mut 0));
            assert_eq!(items.next(), Some(&mut 1));
            assert_eq!(items.next(), Some(&mut 2));
            assert_eq!(items.next(), Some(&mut 3));
            assert_eq!(items.next(), None);
        }

        #[test]
        fn collect_from_iter() {
            let slice = &[1, 2, 3];
            let vec: Vec<i32, 4> = slice.iter().cloned().collect();
            assert_eq!(&vec, slice);
        }

        #[test]
        #[should_panic]
        fn collect_from_iter_overfull() {
            let slice = &[1, 2, 3];
            let _vec = slice.iter().cloned().collect::<Vec<_, 2>>();
        }

        #[test]
        fn iter_move() {
            let mut v: Vec<i32, 4> = Vec::new();
            v.push(0).unwrap();
            v.push(1).unwrap();
            v.push(2).unwrap();
            v.push(3).unwrap();

            let mut items = v.into_iter();

            assert_eq!(items.next(), Some(0));
            assert_eq!(items.next(), Some(1));
            assert_eq!(items.next(), Some(2));
            assert_eq!(items.next(), Some(3));
            assert_eq!(items.next(), None);
        }

        #[test]
        fn iter_move_drop() {
            droppable!();

            {
                let mut vec: Vec<Droppable, 2> = Vec::new();
                vec.push(Droppable::new()).ok().unwrap();
                vec.push(Droppable::new()).ok().unwrap();
                let mut items = vec.into_iter();
                // Move all
                let _ = items.next();
                let _ = items.next();
            }

            assert_eq!(Droppable::count(), 0);

            {
                let mut vec: Vec<Droppable, 2> = Vec::new();
                vec.push(Droppable::new()).ok().unwrap();
                vec.push(Droppable::new()).ok().unwrap();
                let _items = vec.into_iter();
                // Move none
            }

            assert_eq!(Droppable::count(), 0);

            {
                let mut vec: Vec<Droppable, 2> = Vec::new();
                vec.push(Droppable::new()).ok().unwrap();
                vec.push(Droppable::new()).ok().unwrap();
                let mut items = vec.into_iter();
                let _ = items.next(); // Move partly
            }

            assert_eq!(Droppable::count(), 0);
        }

        #[test]
        fn push_and_pop() {
            let mut v: Vec<i32, 4> = Vec::new();
            assert_eq!(v.len(), 0);

            assert_eq!(v.pop(), None);
            assert_eq!(v.len(), 0);

            v.push(0).unwrap();
            assert_eq!(v.len(), 1);

            assert_eq!(v.pop(), Some(0));
            assert_eq!(v.len(), 0);

            assert_eq!(v.pop(), None);
            assert_eq!(v.len(), 0);
        }

        #[test]
        fn resize_size_limit() {
            let mut v: Vec<u8, 4> = Vec::new();

            v.resize(0, 0).unwrap();
            v.resize(4, 0).unwrap();
            v.resize(5, 0).err().expect("full");
        }

        #[test]
        fn resize_length_cases() {
            let mut v: Vec<u8, 4> = Vec::new();

            assert_eq!(v.len(), 0);

            // Grow by 1
            v.resize(1, 0).unwrap();
            assert_eq!(v.len(), 1);

            // Grow by 2
            v.resize(3, 0).unwrap();
            assert_eq!(v.len(), 3);

            // Resize to current size
            v.resize(3, 0).unwrap();
            assert_eq!(v.len(), 3);

            // Shrink by 1
            v.resize(2, 0).unwrap();
            assert_eq!(v.len(), 2);

            // Shrink by 2
            v.resize(0, 0).unwrap();
            assert_eq!(v.len(), 0);
        }

        #[test]
        fn resize_contents() {
            let mut v: Vec<u8, 4> = Vec::new();

            // New entries take supplied value when growing
            v.resize(1, 17).unwrap();
            assert_eq!(v[0], 17);

            // Old values aren't changed when growing
            v.resize(2, 18).unwrap();
            assert_eq!(v[0], 17);
            assert_eq!(v[1], 18);

            // Old values aren't changed when length unchanged
            v.resize(2, 0).unwrap();
            assert_eq!(v[0], 17);
            assert_eq!(v[1], 18);

            // Old values aren't changed when shrinking
            v.resize(1, 0).unwrap();
            assert_eq!(v[0], 17);
        }

        #[test]
        fn resize_default() {
            let mut v: Vec<u8, 4> = Vec::new();

            // resize_default is implemented using resize, so just check the
            // correct value is being written.
            v.resize_default(1).unwrap();
            assert_eq!(v[0], 0);
        }

        #[test]
        fn write() {
            let mut v: Vec<u8, 4> = Vec::new();
            write!(v, "{:x}", 1234).unwrap();
            assert_eq!(&v[..], b"4d2");
        }

        #[test]
        fn extend_from_slice() {
            let mut v: Vec<u8, 4> = Vec::new();
            assert_eq!(v.len(), 0);
            v.extend_from_slice(&[1, 2]).unwrap();
            assert_eq!(v.len(), 2);
            assert_eq!(v.as_slice(), &[1, 2]);
            v.extend_from_slice(&[3]).unwrap();
            assert_eq!(v.len(), 3);
            assert_eq!(v.as_slice(), &[1, 2, 3]);
            assert!(v.extend_from_slice(&[4, 5]).is_err());
            assert_eq!(v.len(), 3);
            assert_eq!(v.as_slice(), &[1, 2, 3]);
        }

        #[test]
        fn from_slice() {
            // Successful construction
            let v: Vec<u8, 4> = Vec::from_slice(&[1, 2, 3]).unwrap();
            assert_eq!(v.len(), 3);
            assert_eq!(v.as_slice(), &[1, 2, 3]);

            // Slice too large
            assert!(Vec::<u8, 2>::from_slice(&[1, 2, 3]).is_err());
        }

        #[test]
        fn starts_with() {
            let v: Vec<_, 8> = Vec::from_slice(b"ab").unwrap();
            assert!(v.starts_with(&[]));
            assert!(v.starts_with(b""));
            assert!(v.starts_with(b"a"));
            assert!(v.starts_with(b"ab"));
            assert!(!v.starts_with(b"abc"));
            assert!(!v.starts_with(b"ba"));
            assert!(!v.starts_with(b"b"));
        }

        #[test]
        fn ends_with() {
            let v: Vec<_, 8> = Vec::from_slice(b"ab").unwrap();
            assert!(v.ends_with(&[]));
            assert!(v.ends_with(b""));
            assert!(v.ends_with(b"b"));
            assert!(v.ends_with(b"ab"));
            assert!(!v.ends_with(b"abc"));
            assert!(!v.ends_with(b"ba"));
            assert!(!v.ends_with(b"a"));
        }

        #[test]
        fn zero_capacity() {
            let mut v: Vec<u8, 0> = Vec::new();
            // Validate capacity
            assert_eq!(v.capacity(), 0);

            // Make sure there is no capacity
            assert!(v.push(1).is_err());

            // Validate length
            assert_eq!(v.len(), 0);

            // Validate pop
            assert_eq!(v.pop(), None);

            // Validate slice
            assert_eq!(v.as_slice(), &[]);

            // Validate empty
            assert!(v.is_empty());

            // Validate full
            assert!(v.is_full());
        }

        #[cfg(feature = "std")]
        #[test]
        fn struct_definition_doctest() {
            // A vector with a fixed capacity of 8 elements allocated on the stack
            let mut vec = Vec::<_, 8>::new();
            vec.push(1);
            vec.push(2);
            assert_eq!(vec.len(), 2);
            assert_eq!(vec[0], 1);
            assert_eq!(vec.pop(), Some(2));
            assert_eq!(vec.len(), 1);
            vec[0] = 7;
            assert_eq!(vec[0], 7);
            vec.extend([1, 2, 3].iter().cloned());
            for x in &vec {
                println!("{}", x);
            }
            assert_eq!(*vec, [7, 1, 2, 3]);
        }

        #[cfg(feature = "std")]
        #[test]
        fn new_doctest() {
            // allocate the vector on the stack
            let mut x: Vec<u8, 16> = Vec::new();

            // allocate the vector in a static variable
            static mut X: Vec<u8, 16> = Vec::new();
        }

        #[cfg(feature = "std")]
        #[test]
        fn as_slice_doctest() {
            let buffer: Vec<u8, 5> = Vec::from_slice(&[1, 2, 3, 5, 8]).unwrap();
            assert_eq!(buffer.as_slice(), &[1, 2, 3, 5, 8]);
        }

        #[cfg(feature = "std")]
        #[test]
        fn into_array_doctest() {
            let buffer: Vec<u8, 42> = Vec::from_slice(&[1, 2, 3, 5, 8]).unwrap();
            let array: [u8; 5] = buffer.into_array().unwrap();
            assert_eq!(array, [1, 2, 3, 5, 8]);
        }

        #[cfg(feature = "std")]
        #[test]
        fn as_mut_slice_doctest() {
            let mut buffer: Vec<u8, 5> = Vec::from_slice(&[1, 2, 3, 5, 8]).unwrap();
            buffer[0] = 9;
            assert_eq!(buffer.as_slice(), &[9, 2, 3, 5, 8]);
        }

        #[cfg(feature = "std")]
        #[test]
        fn extend_from_slice_doctest() {
            let mut vec = Vec::<u8, 8>::new();
            vec.push(1).unwrap();
            vec.extend_from_slice(&[2, 3, 4]).unwrap();
            assert_eq!(*vec, [1, 2, 3, 4]);
        }

        #[cfg(feature = "std")]
        #[test]
        fn swap_remove_doctest() {
            let mut v: Vec<_, 8> = Vec::new();
            v.push("foo").unwrap();
            v.push("bar").unwrap();
            v.push("baz").unwrap();
            v.push("qux").unwrap();
            assert_eq!(v.swap_remove(1), "bar");
            assert_eq!(&*v, ["foo", "qux", "baz"]);
            assert_eq!(v.swap_remove(0), "foo");
            assert_eq!(&*v, ["baz", "qux"]);
        }

        #[cfg(feature = "std")]
        #[test]
        fn swap_remove_unchecked_doctest() {
            let mut v: Vec<_, 8> = Vec::new();
            v.push("foo").unwrap();
            v.push("bar").unwrap();
            v.push("baz").unwrap();
            v.push("qux").unwrap();
            assert_eq!(unsafe { v.swap_remove_unchecked(1) }, "bar");
            assert_eq!(&*v, ["foo", "qux", "baz"]);
            assert_eq!(unsafe { v.swap_remove_unchecked(0) }, "foo");
            assert_eq!(&*v, ["baz", "qux"]);
        }

        #[cfg(feature = "std")]
        #[test]
        fn starts_with_doctest() {
            let v: Vec<_, 8> = Vec::from_slice(b"abc").unwrap();
            assert_eq!(v.starts_with(b""), true);
            assert_eq!(v.starts_with(b"ab"), true);
            assert_eq!(v.starts_with(b"bc"), false);
        }

        #[cfg(feature = "std")]
        #[test]
        fn ends_with_doctest() {
            let v: Vec<_, 8> = Vec::from_slice(b"abc").unwrap();
            assert_eq!(v.ends_with(b""), true);
            assert_eq!(v.ends_with(b"ab"), false);
            assert_eq!(v.ends_with(b"bc"), true);
        }
    }
}

// copied from https://github.com/japaric/heapless/blob/39273893deda34fa741f79438134fd4d4d7ac3ae/src/indexmap.rs

use core::{borrow::Borrow, fmt, iter::FromIterator, mem, num::NonZeroU32, ops, slice};

use hash32::{BuildHasher, BuildHasherDefault, FnvHasher, Hash, Hasher};

use vec::Vec;

/// A [`sprawl::indexmap::IndexMap`](./struct.IndexMap.html) using the default FNV hasher
///
/// A list of all Methods and Traits available for `FnvIndexMap` can be found in
/// the [`sprawl::indexmap::IndexMap`](./struct.IndexMap.html) documentation.
pub type FnvIndexMap<K, V, const N: usize> = IndexMap<K, V, BuildHasherDefault<FnvHasher>, N>;

#[derive(Clone, Copy, Eq, PartialEq)]
struct HashValue(u16);

impl HashValue {
    fn desired_pos(&self, mask: usize) -> usize {
        usize::from(self.0) & mask
    }

    fn probe_distance(&self, mask: usize, current: usize) -> usize {
        current.wrapping_sub(self.desired_pos(mask) as usize) & mask
    }
}

#[derive(Clone)]
struct Bucket<K, V> {
    hash: HashValue,
    key: K,
    value: V,
}

#[derive(Clone, Copy, PartialEq)]
struct Pos {
    // compact representation of `{ hash_value: u16, index: u16 }`
    // To get the most from `NonZero` we store the *value minus 1*. This way `None::Option<Pos>`
    // is equivalent to the very unlikely value of  `{ hash_value: 0xffff, index: 0xffff }` instead
    // the more likely of `{ hash_value: 0x00, index: 0x00 }`
    nz: NonZeroU32,
}

impl Pos {
    fn new(index: usize, hash: HashValue) -> Self {
        Pos { nz: unsafe { NonZeroU32::new_unchecked(((u32::from(hash.0) << 16) + index as u32).wrapping_add(1)) } }
    }

    fn hash(&self) -> HashValue {
        HashValue((self.nz.get().wrapping_sub(1) >> 16) as u16)
    }

    fn index(&self) -> usize {
        self.nz.get().wrapping_sub(1) as u16 as usize
    }
}

enum Insert<K, V> {
    Success(Inserted<V>),
    Full((K, V)),
}

struct Inserted<V> {
    index: usize,
    old_value: Option<V>,
}

macro_rules! probe_loop {
    ($probe_var: ident < $len: expr, $body: expr) => {
        loop {
            if $probe_var < $len {
                $body
                    $probe_var += 1;
            } else {
                $probe_var = 0;
            }
        }
    }
}

struct CoreMap<K, V, const N: usize> {
    entries: Vec<Bucket<K, V>, N>,
    indices: [Option<Pos>; N],
}

impl<K, V, const N: usize> CoreMap<K, V, N> {
    const fn new() -> Self {
        const INIT: Option<Pos> = None;

        CoreMap { entries: Vec::new(), indices: [INIT; N] }
    }
}

impl<K, V, const N: usize> CoreMap<K, V, N>
where
    K: Eq + Hash,
{
    fn capacity() -> usize {
        N
    }

    fn mask() -> usize {
        Self::capacity() - 1
    }

    fn find<Q>(&self, hash: HashValue, query: &Q) -> Option<(usize, usize)>
    where
        K: Borrow<Q>,
        Q: ?Sized + Eq,
    {
        let mut probe = hash.desired_pos(Self::mask());
        let mut dist = 0;

        probe_loop!(probe < self.indices.len(), {
            if let Some(pos) = self.indices[probe] {
                let entry_hash = pos.hash();
                // NOTE(i) we use unchecked indexing below
                let i = pos.index();
                debug_assert!(i < self.entries.len());

                if dist > entry_hash.probe_distance(Self::mask(), probe) {
                    // give up when probe distance is too long
                    return None;
                } else if entry_hash == hash && unsafe { self.entries.get_unchecked(i).key.borrow() == query } {
                    return Some((probe, i));
                }
            } else {
                return None;
            }

            dist += 1;
        });
    }

    fn insert(&mut self, hash: HashValue, key: K, value: V) -> Insert<K, V> {
        let mut probe = hash.desired_pos(Self::mask());
        let mut dist = 0;

        probe_loop!(probe < self.indices.len(), {
            let pos = &mut self.indices[probe];

            if let Some(pos) = *pos {
                let entry_hash = pos.hash();
                // NOTE(i) we use unchecked indexing below
                let i = pos.index();
                debug_assert!(i < self.entries.len());

                let their_dist = entry_hash.probe_distance(Self::mask(), probe);

                if their_dist < dist {
                    if self.entries.is_full() {
                        return Insert::Full((key, value));
                    }
                    // robin hood: steal the spot if it's better for us
                    let index = self.entries.len();
                    unsafe { self.entries.push_unchecked(Bucket { hash, key, value }) };
                    return Insert::Success(Inserted {
                        index: self.insert_phase_2(probe, Pos::new(index, hash)),
                        old_value: None,
                    });
                } else if entry_hash == hash && unsafe { self.entries.get_unchecked(i).key == key } {
                    return Insert::Success(Inserted {
                        index: i,
                        old_value: Some(mem::replace(unsafe { &mut self.entries.get_unchecked_mut(i).value }, value)),
                    });
                }
            } else {
                if self.entries.is_full() {
                    return Insert::Full((key, value));
                }
                // empty bucket, insert here
                let index = self.entries.len();
                *pos = Some(Pos::new(index, hash));
                unsafe { self.entries.push_unchecked(Bucket { hash, key, value }) };
                return Insert::Success(Inserted { index, old_value: None });
            }
            dist += 1;
        });
    }

    // phase 2 is post-insert where we forward-shift `Pos` in the indices.
    fn insert_phase_2(&mut self, mut probe: usize, mut old_pos: Pos) -> usize {
        probe_loop!(probe < self.indices.len(), {
            let pos = unsafe { self.indices.get_unchecked_mut(probe) };

            let mut is_none = true; // work around lack of NLL
            if let Some(pos) = pos.as_mut() {
                old_pos = mem::replace(pos, old_pos);
                is_none = false;
            }

            if is_none {
                *pos = Some(old_pos);
                return probe;
            }
        });
    }

    fn remove_found(&mut self, probe: usize, found: usize) -> (K, V) {
        // index `probe` and entry `found` is to be removed
        // use swap_remove, but then we need to update the index that points
        // to the other entry that has to move
        self.indices[probe] = None;
        let entry = unsafe { self.entries.swap_remove_unchecked(found) };

        // correct index that points to the entry that had to swap places
        if let Some(entry) = self.entries.get(found) {
            // was not last element
            // examine new element in `found` and find it in indices
            let mut probe = entry.hash.desired_pos(Self::mask());

            probe_loop!(probe < self.indices.len(), {
                if let Some(pos) = self.indices[probe] {
                    if pos.index() >= self.entries.len() {
                        // found it
                        self.indices[probe] = Some(Pos::new(found, entry.hash));
                        break;
                    }
                }
            });
        }

        self.backward_shift_after_removal(probe);

        (entry.key, entry.value)
    }

    fn backward_shift_after_removal(&mut self, probe_at_remove: usize) {
        // backward shift deletion in self.indices
        // after probe, shift all non-ideally placed indices backward
        let mut last_probe = probe_at_remove;
        let mut probe = probe_at_remove + 1;

        probe_loop!(probe < self.indices.len(), {
            if let Some(pos) = self.indices[probe] {
                let entry_hash = pos.hash();

                if entry_hash.probe_distance(Self::mask(), probe) > 0 {
                    unsafe { *self.indices.get_unchecked_mut(last_probe) = self.indices[probe] }
                    self.indices[probe] = None;
                } else {
                    break;
                }
            } else {
                break;
            }
            last_probe = probe;
        });
    }
}

impl<K, V, const N: usize> Clone for CoreMap<K, V, N>
where
    K: Eq + Hash + Clone,
    V: Clone,
{
    fn clone(&self) -> Self {
        Self { entries: self.entries.clone(), indices: self.indices }
    }
}

/// A view into an entry in the map
enum Entry<'a, K, V, const N: usize> {
    /// The entry corresponding to the key `K` exists in the map
    Occupied(OccupiedEntry<'a, K, V, N>),
    /// The entry corresponding to the key `K` does not exist in the map
    Vacant(VacantEntry<'a, K, V, N>),
}

/// An occupied entry which can be manipulated
struct OccupiedEntry<'a, K, V, const N: usize> {
    key: K,
    probe: usize,
    pos: usize,
    core: &'a mut CoreMap<K, V, N>,
}

impl<'a, K, V, const N: usize> OccupiedEntry<'a, K, V, N>
where
    K: Eq + Hash,
{
    /// Gets a reference to the key that this entity corresponds to
    fn key(&self) -> &K {
        &self.key
    }

    /// Removes this entry from the map and yields its corresponding key and value
    fn remove_entry(self) -> (K, V) {
        self.core.remove_found(self.probe, self.pos)
    }

    /// Gets a reference to the value associated with this entry
    fn get(&self) -> &V {
        // SAFETY: Already checked existence at instantiation and the only mutable reference
        // to the map is internally held.
        unsafe { &self.core.entries.get_unchecked(self.pos).value }
    }

    /// Gets a mutable reference to the value associated with this entry
    fn get_mut(&mut self) -> &mut V {
        // SAFETY: Already checked existence at instantiation and the only mutable reference
        // to the map is internally held.
        unsafe { &mut self.core.entries.get_unchecked_mut(self.pos).value }
    }

    /// Consumes this entry and yields a reference to the underlying value
    fn into_mut(self) -> &'a mut V {
        // SAFETY: Already checked existence at instantiation and the only mutable reference
        // to the map is internally held.
        unsafe { &mut self.core.entries.get_unchecked_mut(self.pos).value }
    }

    /// Overwrites the underlying map's value with this entry's value
    fn insert(self, value: V) -> V {
        // SAFETY: Already checked existence at instantiation and the only mutable reference
        // to the map is internally held.
        unsafe { mem::replace(&mut self.core.entries.get_unchecked_mut(self.pos).value, value) }
    }

    /// Removes this entry from the map and yields its value
    fn remove(self) -> V {
        self.remove_entry().1
    }
}

/// A view into an empty slot in the underlying map
struct VacantEntry<'a, K, V, const N: usize> {
    key: K,
    hash_val: HashValue,
    core: &'a mut CoreMap<K, V, N>,
}
impl<'a, K, V, const N: usize> VacantEntry<'a, K, V, N>
where
    K: Eq + Hash,
{
    /// Get the key associated with this entry
    fn key(&self) -> &K {
        &self.key
    }

    /// Consumes this entry to yield to key associated with it
    fn into_key(self) -> K {
        self.key
    }

    /// Inserts this entry into to underlying map, yields a mutable reference to the inserted value.
    /// If the map is at capacity the value is returned instead.
    fn insert(self, value: V) -> Result<&'a mut V, V> {
        if self.core.entries.is_full() {
            Err(value)
        } else {
            match self.core.insert(self.hash_val, self.key, value) {
                Insert::Success(inserted) => {
                    unsafe {
                        // SAFETY: Already checked existence at instantiation and the only mutable reference
                        // to the map is internally held.
                        Ok(&mut self.core.entries.get_unchecked_mut(inserted.index).value)
                    }
                }
                Insert::Full((_, v)) => Err(v),
            }
        }
    }
}

/// Fixed capacity [`IndexMap`](https://docs.rs/indexmap/1/indexmap/map/struct.IndexMap.html)
///
/// Note that you cannot use `IndexMap` directly, since it is generic around the hashing algorithm
/// in use. Pick a concrete instantiation like [`FnvIndexMap`](./type.FnvIndexMap.html) instead
/// or create your own.
///
/// Note that the capacity of the `IndexMap` must be a power of 2.
pub struct IndexMap<K, V, S, const N: usize> {
    core: CoreMap<K, V, N>,
    build_hasher: S,
}

impl<K, V, S, const N: usize> IndexMap<K, V, BuildHasherDefault<S>, N> {
    /// Creates an empty `IndexMap`.
    pub const fn new() -> Self {
        // Const assert
        sealed::greater_than_1::<N>();
        sealed::power_of_two::<N>();

        IndexMap { build_hasher: BuildHasherDefault::new(), core: CoreMap::new() }
    }
}

impl<K, V, S, const N: usize> IndexMap<K, V, S, N>
where
    K: Eq + Hash,
    S: BuildHasher,
{
    /* Public API */
    /// Returns the number of elements the map can hold
    fn capacity(&self) -> usize {
        N
    }

    /// Return an iterator over the keys of the map, in their order
    fn keys(&self) -> impl Iterator<Item = &K> {
        self.core.entries.iter().map(|bucket| &bucket.key)
    }

    /// Return an iterator over the values of the map, in their order
    fn values(&self) -> impl Iterator<Item = &V> {
        self.core.entries.iter().map(|bucket| &bucket.value)
    }

    /// Return an iterator over mutable references to the the values of the map, in their order
    fn values_mut(&mut self) -> impl Iterator<Item = &mut V> {
        self.core.entries.iter_mut().map(|bucket| &mut bucket.value)
    }

    /// Return an iterator over the key-value pairs of the map, in their order
    fn iter(&self) -> Iter<'_, K, V> {
        Iter { iter: self.core.entries.iter() }
    }

    /// Return an iterator over the key-value pairs of the map, in their order
    fn iter_mut(&mut self) -> IterMut<'_, K, V> {
        IterMut { iter: self.core.entries.iter_mut() }
    }

    /// Get the first key-value pair
    ///
    /// Computes in **O(1)** time
    fn first(&self) -> Option<(&K, &V)> {
        self.core.entries.first().map(|bucket| (&bucket.key, &bucket.value))
    }

    /// Get the first key-value pair, with mutable access to the value
    ///
    /// Computes in **O(1)** time
    fn first_mut(&mut self) -> Option<(&K, &mut V)> {
        self.core.entries.first_mut().map(|bucket| (&bucket.key, &mut bucket.value))
    }

    /// Get the last key-value pair
    ///
    /// Computes in **O(1)** time
    fn last(&self) -> Option<(&K, &V)> {
        self.core.entries.last().map(|bucket| (&bucket.key, &bucket.value))
    }

    /// Get the last key-value pair, with mutable access to the value
    ///
    /// Computes in **O(1)** time
    fn last_mut(&mut self) -> Option<(&K, &mut V)> {
        self.core.entries.last_mut().map(|bucket| (&bucket.key, &mut bucket.value))
    }

    /// Returns an entry for the corresponding key
    fn entry(&mut self, key: K) -> Entry<'_, K, V, N> {
        let hash_val = hash_with(&key, &self.build_hasher);
        if let Some((probe, pos)) = self.core.find(hash_val, &key) {
            Entry::Occupied(OccupiedEntry { key, probe, pos, core: &mut self.core })
        } else {
            Entry::Vacant(VacantEntry { key, hash_val, core: &mut self.core })
        }
    }

    /// Return the number of key-value pairs in the map.
    ///
    /// Computes in **O(1)** time.
    fn len(&self) -> usize {
        self.core.entries.len()
    }

    /// Returns true if the map contains no elements.
    ///
    /// Computes in **O(1)** time.
    fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Remove all key-value pairs in the map, while preserving its capacity.
    ///
    /// Computes in **O(n)** time.
    pub(crate) fn clear(&mut self) {
        self.core.entries.clear();
        for pos in self.core.indices.iter_mut() {
            *pos = None;
        }
    }

    /// Returns a reference to the value corresponding to the key.
    ///
    /// The key may be any borrowed form of the map's key type, but `Hash` and `Eq` on the borrowed
    /// form *must* match those for the key type.
    ///
    /// Computes in **O(1)** time (average).
    pub(crate) fn get<Q>(&self, key: &Q) -> Option<&V>
    where
        K: Borrow<Q>,
        Q: ?Sized + Hash + Eq,
    {
        self.find(key).map(|(_, found)| unsafe { &self.core.entries.get_unchecked(found).value })
    }

    /// Returns true if the map contains a value for the specified key.
    ///
    /// The key may be any borrowed form of the map's key type, but `Hash` and `Eq` on the borrowed
    /// form *must* match those for the key type.
    ///
    /// Computes in **O(1)** time (average).
    fn contains_key<Q>(&self, key: &Q) -> bool
    where
        K: Borrow<Q>,
        Q: ?Sized + Eq + Hash,
    {
        self.find(key).is_some()
    }

    /// Returns a mutable reference to the value corresponding to the key.
    ///
    /// The key may be any borrowed form of the map's key type, but `Hash` and `Eq` on the borrowed
    /// form *must* match those for the key type.
    ///
    /// Computes in **O(1)** time (average).
    fn get_mut<'v, Q>(&'v mut self, key: &Q) -> Option<&'v mut V>
    where
        K: Borrow<Q>,
        Q: ?Sized + Hash + Eq,
    {
        if let Some((_, found)) = self.find(key) {
            Some(unsafe { &mut self.core.entries.get_unchecked_mut(found).value })
        } else {
            None
        }
    }

    /// Inserts a key-value pair into the map.
    ///
    /// If an equivalent key already exists in the map: the key remains and retains in its place in
    /// the order, its corresponding value is updated with `value` and the older value is returned
    /// inside `Some(_)`.
    ///
    /// If no equivalent key existed in the map: the new key-value pair is inserted, last in order,
    /// and `None` is returned.
    ///
    /// Computes in **O(1)** time (average).
    ///
    /// See also entry if you you want to insert or modify or if you need to get the index of the
    /// corresponding key-value pair.
    pub(crate) fn insert(&mut self, key: K, value: V) -> Result<Option<V>, (K, V)> {
        let hash = hash_with(&key, &self.build_hasher);
        match self.core.insert(hash, key, value) {
            Insert::Success(inserted) => Ok(inserted.old_value),
            Insert::Full((k, v)) => Err((k, v)),
        }
    }

    /// Same as [`swap_remove`](struct.IndexMap.html#method.swap_remove)
    ///
    /// Computes in **O(1)** time (average).
    pub(crate) fn remove<Q>(&mut self, key: &Q) -> Option<V>
    where
        K: Borrow<Q>,
        Q: ?Sized + Hash + Eq,
    {
        self.swap_remove(key)
    }

    /// Remove the key-value pair equivalent to `key` and return its value.
    ///
    /// Like `Vec::swap_remove`, the pair is removed by swapping it with the last element of the map
    /// and popping it off. **This perturbs the postion of what used to be the last element!**
    ///
    /// Return `None` if `key` is not in map.
    ///
    /// Computes in **O(1)** time (average).
    fn swap_remove<Q>(&mut self, key: &Q) -> Option<V>
    where
        K: Borrow<Q>,
        Q: ?Sized + Hash + Eq,
    {
        self.find(key).map(|(probe, found)| self.core.remove_found(probe, found).1)
    }

    /* Private API */
    /// Return probe (indices) and position (entries)
    fn find<Q>(&self, key: &Q) -> Option<(usize, usize)>
    where
        K: Borrow<Q>,
        Q: ?Sized + Hash + Eq,
    {
        if self.is_empty() {
            return None;
        }
        let h = hash_with(key, &self.build_hasher);
        self.core.find(h, key)
    }
}

impl<'a, K, Q, V, S, const N: usize> ops::Index<&'a Q> for IndexMap<K, V, S, N>
where
    K: Eq + Hash + Borrow<Q>,
    Q: ?Sized + Eq + Hash,
    S: BuildHasher,
{
    type Output = V;

    fn index(&self, key: &Q) -> &V {
        self.get(key).expect("key not found")
    }
}

impl<'a, K, Q, V, S, const N: usize> ops::IndexMut<&'a Q> for IndexMap<K, V, S, N>
where
    K: Eq + Hash + Borrow<Q>,
    Q: ?Sized + Eq + Hash,
    S: BuildHasher,
{
    fn index_mut(&mut self, key: &Q) -> &mut V {
        self.get_mut(key).expect("key not found")
    }
}

impl<K, V, S, const N: usize> Clone for IndexMap<K, V, S, N>
where
    K: Eq + Hash + Clone,
    V: Clone,
    S: Clone,
{
    fn clone(&self) -> Self {
        Self { core: self.core.clone(), build_hasher: self.build_hasher.clone() }
    }
}

impl<K, V, S, const N: usize> fmt::Debug for IndexMap<K, V, S, N>
where
    K: Eq + Hash + fmt::Debug,
    V: fmt::Debug,
    S: BuildHasher,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_map().entries(self.iter()).finish()
    }
}

impl<K, V, S, const N: usize> Default for IndexMap<K, V, S, N>
where
    K: Eq + Hash,
    S: BuildHasher + Default,
{
    fn default() -> Self {
        IndexMap { build_hasher: <_>::default(), core: CoreMap::new() }
    }
}

impl<K, V, S, S2, const N: usize, const N2: usize> PartialEq<IndexMap<K, V, S2, N2>> for IndexMap<K, V, S, N>
where
    K: Eq + Hash,
    V: Eq,
    S: BuildHasher,
    S2: BuildHasher,
{
    fn eq(&self, other: &IndexMap<K, V, S2, N2>) -> bool {
        self.len() == other.len() && self.iter().all(|(key, value)| other.get(key).map_or(false, |v| *value == *v))
    }
}

impl<K, V, S, const N: usize> Eq for IndexMap<K, V, S, N>
where
    K: Eq + Hash,
    V: Eq,
    S: BuildHasher,
{
}

impl<K, V, S, const N: usize> Extend<(K, V)> for IndexMap<K, V, S, N>
where
    K: Eq + Hash,
    S: BuildHasher,
{
    fn extend<I>(&mut self, iterable: I)
    where
        I: IntoIterator<Item = (K, V)>,
    {
        for (k, v) in iterable {
            self.insert(k, v).ok().unwrap();
        }
    }
}

impl<'a, K, V, S, const N: usize> Extend<(&'a K, &'a V)> for IndexMap<K, V, S, N>
where
    K: Eq + Hash + Copy,
    V: Copy,
    S: BuildHasher,
{
    fn extend<I>(&mut self, iterable: I)
    where
        I: IntoIterator<Item = (&'a K, &'a V)>,
    {
        self.extend(iterable.into_iter().map(|(&key, &value)| (key, value)))
    }
}

impl<K, V, S, const N: usize> FromIterator<(K, V)> for IndexMap<K, V, S, N>
where
    K: Eq + Hash,
    S: BuildHasher + Default,
{
    fn from_iter<I>(iterable: I) -> Self
    where
        I: IntoIterator<Item = (K, V)>,
    {
        let mut map = IndexMap::default();
        map.extend(iterable);
        map
    }
}

#[derive(Clone)]
pub struct IntoIter<K, V, const N: usize> {
    entries: Vec<Bucket<K, V>, N>,
}

impl<K, V, const N: usize> Iterator for IntoIter<K, V, N> {
    type Item = (K, V);

    fn next(&mut self) -> Option<Self::Item> {
        self.entries.pop().map(|bucket| (bucket.key, bucket.value))
    }
}

impl<K, V, S, const N: usize> IntoIterator for IndexMap<K, V, S, N>
where
    K: Eq + Hash,
    S: BuildHasher,
{
    type Item = (K, V);
    type IntoIter = IntoIter<K, V, N>;

    fn into_iter(self) -> Self::IntoIter {
        IntoIter { entries: self.core.entries }
    }
}

impl<'a, K, V, S, const N: usize> IntoIterator for &'a IndexMap<K, V, S, N>
where
    K: Eq + Hash,
    S: BuildHasher,
{
    type Item = (&'a K, &'a V);
    type IntoIter = Iter<'a, K, V>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

impl<'a, K, V, S, const N: usize> IntoIterator for &'a mut IndexMap<K, V, S, N>
where
    K: Eq + Hash,
    S: BuildHasher,
{
    type Item = (&'a K, &'a mut V);
    type IntoIter = IterMut<'a, K, V>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter_mut()
    }
}

pub struct Iter<'a, K, V> {
    iter: slice::Iter<'a, Bucket<K, V>>,
}

impl<'a, K, V> Iterator for Iter<'a, K, V> {
    type Item = (&'a K, &'a V);

    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next().map(|bucket| (&bucket.key, &bucket.value))
    }
}

impl<'a, K, V> Clone for Iter<'a, K, V> {
    fn clone(&self) -> Self {
        Self { iter: self.iter.clone() }
    }
}

pub struct IterMut<'a, K, V> {
    iter: slice::IterMut<'a, Bucket<K, V>>,
}

impl<'a, K, V> Iterator for IterMut<'a, K, V> {
    type Item = (&'a K, &'a mut V);

    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next().map(|bucket| (&bucket.key, &mut bucket.value))
    }
}

fn hash_with<K, S>(key: &K, build_hasher: &S) -> HashValue
where
    K: ?Sized + Hash,
    S: BuildHasher,
{
    let mut h = build_hasher.build_hasher();
    key.hash(&mut h);
    HashValue(h.finish() as u16)
}

#[cfg(test)]
mod tests {
    use super::{Entry, FnvIndexMap};

    use core::mem;

    #[test]
    fn size() {
        const CAP: usize = 4;
        assert_eq!(
            mem::size_of::<FnvIndexMap<i16, u16, CAP>>(),
            CAP * mem::size_of::<u32>() + // indices
                CAP * (mem::size_of::<i16>() + // key
                     mem::size_of::<u16>() + // value
                     mem::size_of::<u16>() // hash
                ) + // buckets
                mem::size_of::<usize>() // entries.length
        )
    }

    #[test]
    fn partial_eq() {
        {
            let mut a: FnvIndexMap<_, _, 4> = FnvIndexMap::new();
            a.insert("k1", "v1").unwrap();

            let mut b: FnvIndexMap<_, _, 4> = FnvIndexMap::new();
            b.insert("k1", "v1").unwrap();

            assert!(a == b);

            b.insert("k2", "v2").unwrap();

            assert!(a != b);
        }

        {
            let mut a: FnvIndexMap<_, _, 4> = FnvIndexMap::new();
            a.insert("k1", "v1").unwrap();
            a.insert("k2", "v2").unwrap();

            let mut b: FnvIndexMap<_, _, 4> = FnvIndexMap::new();
            b.insert("k2", "v2").unwrap();
            b.insert("k1", "v1").unwrap();

            assert!(a == b);
        }
    }

    #[test]
    fn into_iter() {
        let mut src: FnvIndexMap<_, _, 4> = FnvIndexMap::new();
        src.insert("k1", "v1").unwrap();
        src.insert("k2", "v2").unwrap();
        src.insert("k3", "v3").unwrap();
        src.insert("k4", "v4").unwrap();
        let clone = src.clone();
        for (k, v) in clone.into_iter() {
            assert_eq!(v, *src.get(k).unwrap());
        }
    }

    #[test]
    fn insert_replaces_on_full_map() {
        let mut a: FnvIndexMap<_, _, 2> = FnvIndexMap::new();
        a.insert("k1", "v1").unwrap();
        a.insert("k2", "v2").unwrap();
        a.insert("k1", "v2").unwrap();
        assert_eq!(a.get("k1"), a.get("k2"));
    }

    const MAP_SLOTS: usize = 4096;
    fn almost_filled_map() -> FnvIndexMap<usize, usize, MAP_SLOTS> {
        let mut almost_filled = FnvIndexMap::new();
        for i in 1..MAP_SLOTS {
            almost_filled.insert(i, i).unwrap();
        }
        almost_filled
    }

    #[test]
    fn entry_find() {
        let key = 0;
        let value = 0;
        let mut src = almost_filled_map();
        let entry = src.entry(key);
        match entry {
            Entry::Occupied(_) => {
                panic!("Found entry without inserting");
            }
            Entry::Vacant(v) => {
                assert_eq!(&key, v.key());
                assert_eq!(key, v.into_key());
            }
        }
        src.insert(key, value).unwrap();
        let entry = src.entry(key);
        match entry {
            Entry::Occupied(mut o) => {
                assert_eq!(&key, o.key());
                assert_eq!(&value, o.get());
                assert_eq!(&value, o.get_mut());
                assert_eq!(&value, o.into_mut());
            }
            Entry::Vacant(_) => {
                panic!("Entry not found");
            }
        }
    }

    #[test]
    fn entry_vacant_insert() {
        let key = 0;
        let value = 0;
        let mut src = almost_filled_map();
        assert_eq!(MAP_SLOTS - 1, src.len());
        let entry = src.entry(key);
        match entry {
            Entry::Occupied(_) => {
                panic!("Entry found when empty");
            }
            Entry::Vacant(v) => {
                v.insert(value).unwrap();
            }
        };
        assert_eq!(value, *src.get(&key).unwrap())
    }

    #[test]
    fn entry_occupied_insert() {
        let key = 0;
        let value = 0;
        let value2 = 5;
        let mut src = almost_filled_map();
        assert_eq!(MAP_SLOTS - 1, src.len());
        src.insert(key, value).unwrap();
        let entry = src.entry(key);
        match entry {
            Entry::Occupied(o) => {
                assert_eq!(value, o.insert(value2));
            }
            Entry::Vacant(_) => {
                panic!("Entry not found");
            }
        };
        assert_eq!(value2, *src.get(&key).unwrap())
    }

    #[test]
    fn entry_remove_entry() {
        let key = 0;
        let value = 0;
        let mut src = almost_filled_map();
        src.insert(key, value).unwrap();
        assert_eq!(MAP_SLOTS, src.len());
        let entry = src.entry(key);
        match entry {
            Entry::Occupied(o) => {
                assert_eq!((key, value), o.remove_entry());
            }
            Entry::Vacant(_) => {
                panic!("Entry not found")
            }
        };
        assert_eq!(MAP_SLOTS - 1, src.len());
    }

    #[test]
    fn entry_remove() {
        let key = 0;
        let value = 0;
        let mut src = almost_filled_map();
        src.insert(key, value).unwrap();
        assert_eq!(MAP_SLOTS, src.len());
        let entry = src.entry(key);
        match entry {
            Entry::Occupied(o) => {
                assert_eq!(value, o.remove());
            }
            Entry::Vacant(_) => {
                panic!("Entry not found");
            }
        };
        assert_eq!(MAP_SLOTS - 1, src.len());
    }

    #[test]
    fn entry_roll_through_all() {
        let mut src: FnvIndexMap<usize, usize, MAP_SLOTS> = FnvIndexMap::new();
        for i in 0..MAP_SLOTS {
            match src.entry(i) {
                Entry::Occupied(_) => {
                    panic!("Entry found before insert");
                }
                Entry::Vacant(v) => {
                    v.insert(i).unwrap();
                }
            }
        }
        let add_mod = 99;
        for i in 0..MAP_SLOTS {
            match src.entry(i) {
                Entry::Occupied(o) => {
                    assert_eq!(i, o.insert(i + add_mod));
                }
                Entry::Vacant(_) => {
                    panic!("Entry not found after insert");
                }
            }
        }
        for i in 0..MAP_SLOTS {
            match src.entry(i) {
                Entry::Occupied(o) => {
                    assert_eq!((i, i + add_mod), o.remove_entry());
                }
                Entry::Vacant(_) => {
                    panic!("Entry not found after insert");
                }
            }
        }
        for i in 0..MAP_SLOTS {
            assert!(matches!(src.entry(i), Entry::Vacant(_)));
        }
        assert!(src.is_empty());
    }

    #[test]
    fn first_last() {
        let mut map = FnvIndexMap::<_, _, 4>::new();

        assert_eq!(None, map.first());
        assert_eq!(None, map.last());

        map.insert(0, 0).unwrap();
        map.insert(2, 2).unwrap();

        assert_eq!(Some((&0, &0)), map.first());
        assert_eq!(Some((&2, &2)), map.last());

        map.insert(1, 1).unwrap();

        assert_eq!(Some((&1, &1)), map.last());

        *map.first_mut().unwrap().1 += 1;
        *map.last_mut().unwrap().1 += 1;

        assert_eq!(Some((&0, &1)), map.first());
        assert_eq!(Some((&1, &2)), map.last());
    }

    #[cfg(feature = "std")]
    #[test]
    fn fnv_index_map_doctest() {
        // A hash map with a capacity of 16 key-value pairs allocated on the stack
        let mut book_reviews = FnvIndexMap::<_, _, 16>::new();
        // review some books.
        book_reviews.insert("Adventures of Huckleberry Finn", "My favorite book.").unwrap();
        book_reviews.insert("Grimms' Fairy Tales", "Masterpiece.").unwrap();
        book_reviews.insert("Pride and Prejudice", "Very enjoyable.").unwrap();
        book_reviews.insert("The Adventures of Sherlock Holmes", "Eye lyked it alot.").unwrap();
        // check for a specific one.
        if !book_reviews.contains_key("Les Misérables") {
            println!("We've got {} reviews, but Les Misérables ain't one.", book_reviews.len());
        }
        // oops, this review has a lot of spelling mistakes, let's delete it.
        book_reviews.remove("The Adventures of Sherlock Holmes");
        // look up the values associated with some keys.
        let to_find = ["Pride and Prejudice", "Alice's Adventure in Wonderland"];
        for book in &to_find {
            match book_reviews.get(book) {
                Some(review) => println!("{}: {}", book, review),
                None => println!("{} is unreviewed.", book),
            }
        }
        // iterate over everything.
        for (book, review) in &book_reviews {
            println!("{}: \"{}\"", book, review);
        }
    }

    #[cfg(feature = "std")]
    #[test]
    fn struct_definition_doctest() {
        // Since `IndexMap` cannot be used directly, we're using its `FnvIndexMap` instantiation
        // for this example.
        // A hash map with a capacity of 16 key-value pairs allocated on the stack
        let mut book_reviews = FnvIndexMap::<_, _, 16>::new();
        // review some books.
        book_reviews.insert("Adventures of Huckleberry Finn", "My favorite book.").unwrap();
        book_reviews.insert("Grimms' Fairy Tales", "Masterpiece.").unwrap();
        book_reviews.insert("Pride and Prejudice", "Very enjoyable.").unwrap();
        book_reviews.insert("The Adventures of Sherlock Holmes", "Eye lyked it alot.").unwrap();
        // check for a specific one.
        if !book_reviews.contains_key("Les Misérables") {
            println!("We've got {} reviews, but Les Misérables ain't one.", book_reviews.len());
        }
        // oops, this review has a lot of spelling mistakes, let's delete it.
        book_reviews.remove("The Adventures of Sherlock Holmes");
        // look up the values associated with some keys.
        let to_find = ["Pride and Prejudice", "Alice's Adventure in Wonderland"];
        for book in &to_find {
            match book_reviews.get(book) {
                Some(review) => println!("{}: {}", book, review),
                None => println!("{} is unreviewed.", book),
            }
        }
        // iterate over everything.
        for (book, review) in &book_reviews {
            println!("{}: \"{}\"", book, review);
        }
    }

    #[cfg(feature = "std")]
    #[test]
    fn keys_doctest() {
        let mut map = FnvIndexMap::<_, _, 16>::new();
        map.insert("a", 1).unwrap();
        map.insert("b", 2).unwrap();
        map.insert("c", 3).unwrap();
        for key in map.keys() {
            println!("{}", key);
        }
    }

    #[cfg(feature = "std")]
    #[test]
    fn values_doctest() {
        let mut map = FnvIndexMap::<_, _, 16>::new();
        map.insert("a", 1).unwrap();
        map.insert("b", 2).unwrap();
        map.insert("c", 3).unwrap();
        for val in map.values() {
            println!("{}", val);
        }
    }

    #[cfg(feature = "std")]
    #[test]
    fn values_mut_doctest() {
        let mut map = FnvIndexMap::<_, _, 16>::new();
        map.insert("a", 1).unwrap();
        map.insert("b", 2).unwrap();
        map.insert("c", 3).unwrap();
        for val in map.values_mut() {
            *val += 10;
        }
        for val in map.values() {
            println!("{}", val);
        }
    }

    #[cfg(feature = "std")]
    #[test]
    fn iter_doctest() {
        let mut map = FnvIndexMap::<_, _, 16>::new();
        map.insert("a", 1).unwrap();
        map.insert("b", 2).unwrap();
        map.insert("c", 3).unwrap();
        for (key, val) in map.iter() {
            println!("key: {} val: {}", key, val);
        }
    }

    #[cfg(feature = "std")]
    #[test]
    fn iter_mut_doctest() {
        let mut map = FnvIndexMap::<_, _, 16>::new();
        map.insert("a", 1).unwrap();
        map.insert("b", 2).unwrap();
        map.insert("c", 3).unwrap();
        for (_, val) in map.iter_mut() {
            *val = 2;
        }
        for (key, val) in &map {
            println!("key: {} val: {}", key, val);
        }
    }

    #[cfg(feature = "std")]
    #[test]
    fn entry_doctest() {
        let mut map = FnvIndexMap::<_, _, 16>::new();
        if let Entry::Vacant(v) = map.entry("a") {
            v.insert(1).unwrap();
        }
        if let Entry::Occupied(mut o) = map.entry("a") {
            println!("found {}", *o.get()); // Prints 1
            o.insert(2);
        }
        // Prints 2
        println!("val: {}", *map.get("a").unwrap());
    }

    #[cfg(feature = "std")]
    #[test]
    fn len_doctest() {
        let mut a = FnvIndexMap::<_, _, 16>::new();
        assert_eq!(a.len(), 0);
        a.insert(1, "a").unwrap();
        assert_eq!(a.len(), 1);
    }

    #[cfg(feature = "std")]
    #[test]
    fn is_empty_doctest() {
        let mut a = FnvIndexMap::<_, _, 16>::new();
        assert!(a.is_empty());
        a.insert(1, "a");
        assert!(!a.is_empty());
    }

    #[cfg(feature = "std")]
    #[test]
    fn clear_doctest() {
        let mut a = FnvIndexMap::<_, _, 16>::new();
        a.insert(1, "a");
        a.clear();
        assert!(a.is_empty());
    }

    #[cfg(feature = "std")]
    #[test]
    fn get_doctest() {
        let mut map = FnvIndexMap::<_, _, 16>::new();
        map.insert(1, "a").unwrap();
        assert_eq!(map.get(&1), Some(&"a"));
        assert_eq!(map.get(&2), None);
    }

    #[cfg(feature = "std")]
    #[test]
    fn contains_key_doctest() {
        let mut map = FnvIndexMap::<_, _, 8>::new();
        map.insert(1, "a").unwrap();
        assert_eq!(map.contains_key(&1), true);
        assert_eq!(map.contains_key(&2), false);
    }

    #[cfg(feature = "std")]
    #[test]
    fn get_mut_doctest() {
        let mut map = FnvIndexMap::<_, _, 8>::new();
        map.insert(1, "a").unwrap();
        if let Some(x) = map.get_mut(&1) {
            *x = "b";
        }
        assert_eq!(map[&1], "b");
    }

    #[cfg(feature = "std")]
    #[test]
    fn insert_doctest() {
        let mut map = FnvIndexMap::<_, _, 8>::new();
        assert_eq!(map.insert(37, "a"), Ok(None));
        assert_eq!(map.is_empty(), false);
        map.insert(37, "b");
        assert_eq!(map.insert(37, "c"), Ok(Some("b")));
        assert_eq!(map[&37], "c");
    }

    #[cfg(feature = "std")]
    #[test]
    fn remove_doctest() {
        let mut map = FnvIndexMap::<_, _, 8>::new();
        map.insert(1, "a").unwrap();
        assert_eq!(map.remove(&1), Some("a"));
        assert_eq!(map.remove(&1), None);
    }
}
