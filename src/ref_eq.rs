/// Determine if two borrowed pointers point to the same thing.
#[inline]
pub fn ref_eq<'a, 'b, T>(thing: &'a T, other: &'b T) -> bool {
    (thing as *const T) == (other as *const T)
}
