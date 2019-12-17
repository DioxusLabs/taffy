#[cfg(feature = "std")]
mod std {
    pub type Box<A> = ::std::boxed::Box<A>;
    pub type Map<K, V> = ::std::collections::HashMap<K, V>;
    pub type Vec<A> = ::std::vec::Vec<A>;
    pub type ChildrenVec<A> = ::std::vec::Vec<A>;
    pub type ParentsVec<A> = ::std::vec::Vec<A>;

    pub fn new_map_with_capacity<K, V>(capacity: usize) -> Map<K, V>
    where
        K: Eq + ::std::hash::Hash,
    {
        Map::with_capacity(capacity)
    }

    pub fn new_vec_with_capacity<A>(capacity: usize) -> Vec<A> {
        Vec::with_capacity(capacity)
    }

    #[inline]
    pub fn round(value: f32) -> f32 {
        value.round()
    }

    #[inline]
    pub fn abs(value: f32) -> f32 {
        value.abs()
    }
}

#[cfg(feature = "alloc")]
mod alloc {
    pub type Box<A> = ::alloc::boxed::Box<A>;
    pub type Map<K, V> = ::hashbrown::HashMap<K, V>;
    pub type Vec<A> = ::alloc::vec::Vec<A>;
    pub type ChildrenVec<A> = ::alloc::vec::Vec<A>;
    pub type ParentsVec<A> = ::alloc::vec::Vec<A>;

    pub fn new_map_with_capacity<K, V>(capacity: usize) -> Map<K, V> {
        Map::with_capacity(capacity)
    }

    pub fn new_vec_with_capacity<A>(capacity: usize) -> Vec<A> {
        Vec::with_capacity(capacity)
    }

    #[inline]
    pub fn round(value: f32) -> f32 {
        num_traits::float::FloatCore::round(value)
    }

    #[inline]
    pub fn abs(value: f32) -> f32 {
        num_traits::float::FloatCore::abs(value)
    }
}

#[cfg(all(not(feature = "alloc"), not(feature = "std")))]
mod core {
    use typenum::marker_traits::Unsigned;

    type MaxNodeCount = heapless::consts::U256;
    type MaxChildCount = heapless::consts::U16;
    type MaxParentsCount = heapless::consts::U1;

    pub type Map<K, V> = ::heapless::FnvIndexMap<K, V, MaxNodeCount>;
    pub type Vec<A> = ::arrayvec::ArrayVec<[A; MaxNodeCount::USIZE]>;
    pub type ChildrenVec<A> = ::arrayvec::ArrayVec<[A; MaxChildCount::USIZE]>;
    pub type ParentsVec<A> = ::arrayvec::ArrayVec<[A; MaxParentsCount::USIZE]>;

    pub fn new_map_with_capacity<K, V>(_capacity: usize) -> Map<K, V>
    where
        K: Eq + ::hash32::Hash,
    {
        Map::new()
    }

    pub fn new_vec_with_capacity<T, A>(_capacity: usize) -> ::arrayvec::ArrayVec<A>
    where
        A: ::arrayvec::Array<Item = T>,
    {
        ::arrayvec::ArrayVec::new()
    }

    #[inline]
    pub fn round(value: f32) -> f32 {
        num_traits::float::FloatCore::round(value)
    }

    #[inline]
    pub fn abs(value: f32) -> f32 {
        num_traits::float::FloatCore::abs(value)
    }
}

#[cfg(feature = "alloc")]
pub use self::alloc::*;
#[cfg(all(not(feature = "alloc"), not(feature = "std")))]
pub use self::core::*;
#[cfg(feature = "std")]
pub use self::std::*;
