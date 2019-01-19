/// Determine if two borrowed pointers point to the same thing.
#[inline]
pub fn ref_eq<T>(thing: &T, other: &T) -> bool {
    (thing as *const T) == (other as *const T)
}
