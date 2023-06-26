use core::hash::Hasher;
use ordered_float::OrderedFloat;
use std::cell::RefCell;
use std::hash::Hash;

use crate::{
    prelude::{NodeId, Size},
    style::AvailableSpace,
    tree::{RunMode, SizeBaselinesAndMargins, SizingMode},
};

thread_local! {
    static SHOULD_USE_CACHE: RefCell<bool> = RefCell::new(false);
    static CACHE_STACK_FRAME_COUNT: RefCell<usize> = RefCell::new(0);
    static CALL_STACK_CACHE: core::cell::RefCell<hashbrown::HashMap<CacheKey, CachedItem>>  = core::cell::RefCell::new(hashbrown::HashMap::new());
}

#[derive(Hash, PartialEq, Eq, Debug)]
struct CacheKey {
    node: NodeId,
    known_dimensions: Size<Option<OrderedFloat<f32>>>,
    available_space: Size<CachedAvailableSpace>,
    parent_size: Size<Option<OrderedFloat<f32>>>,
    run_mode: RunMode,
    sizing_mode: SizingMode,
}

impl CacheKey {
    fn new(
        node: NodeId,
        known_dimensions: &Size<Option<f32>>,
        available_space: &Size<AvailableSpace>,
        parent_size: &Size<Option<f32>>,
        run_mode: RunMode,
        sizing_mode: SizingMode,
    ) -> Self {
        let known_dimensions = known_dimensions.to_cache_variant();
        let parent_size = parent_size.to_cache_variant();
        Self { node, known_dimensions, available_space: (*available_space).into(), parent_size, run_mode, sizing_mode }
    }
}

struct CachedItem {
    cached_size_and_baselines: SizeBaselinesAndMargins,
}

impl CachedItem {
    fn new(cached_size_and_baselines: SizeBaselinesAndMargins) -> Self {
        Self { cached_size_and_baselines }
    }
}

/// call this when you know the algorithm is recursive
fn set_should_use_cache(should_use_cache: bool) {
    SHOULD_USE_CACHE.with(|cache| {
        *cache.borrow_mut() = should_use_cache;
    });
}

fn should_use_cache() -> bool {
    SHOULD_USE_CACHE.with(|cache| *cache.borrow())
}

/// Together with the `decrement_cache_call_stack_frame_count` function, this function is responsible for clearing the thread local cache
/// this must be called anytime a recursive call to `compute` is made
fn increment_cache_call_stack_frame_count() {
    CACHE_STACK_FRAME_COUNT.with(|stack| {
        let mut stack = stack.borrow_mut();
        *stack += 1;
    });
}

/// this must be called anytime a recursive call to `compute` returns
fn decrement_cache_call_stack_frame_count() {
    CACHE_STACK_FRAME_COUNT.with(|stack| {
        let mut stack = stack.borrow_mut();
        *stack -= 1;
        if *stack == 0 {
            CALL_STACK_CACHE.with(|cache| {
                cache.borrow_mut().clear();
            });
            set_should_use_cache(false);
        }
    });
}

fn store_in_thread_local_cache(
    node: NodeId,
    parent_size: Size<Option<f32>>,
    known_dimensions: Size<Option<f32>>,
    available_space: Size<AvailableSpace>,
    computed_size_and_baselines: SizeBaselinesAndMargins,
    run_mode: RunMode,
    sizing_mode: SizingMode,
) {
    if !StackFrameCache::should_use() {
        return;
    }
    CALL_STACK_CACHE.with(|cache| {
        let cache_key = CacheKey::new(node, &known_dimensions, &available_space, &parent_size, run_mode, sizing_mode);
        cache.borrow_mut().entry(cache_key).or_insert_with(|| CachedItem::new(computed_size_and_baselines));
        // println!("cache size: {}", cache.borrow().len());
    });
}

fn get_from_thread_local_cache(
    node: NodeId,
    parent_size: &Size<Option<f32>>,
    known_dimensions: &Size<Option<f32>>,
    available_space: &Size<AvailableSpace>,
    run_mode: RunMode,
    sizing_mode: SizingMode,
) -> Option<SizeBaselinesAndMargins> {
    if !StackFrameCache::should_use() {
        return None;
    }
    CALL_STACK_CACHE.with(|cache| {
        let cache_key = CacheKey::new(node, known_dimensions, available_space, parent_size, run_mode, sizing_mode);
        cache.borrow().get(&cache_key).map(|item| item.cached_size_and_baselines)
    })
}

pub(crate) struct StackFrameCache;

impl StackFrameCache {
    pub fn set_should_use(should_use_cache: bool) {
        set_should_use_cache(should_use_cache);
    }

    fn should_use() -> bool {
        should_use_cache()
    }

    pub fn get(
        node: NodeId,
        parent_size: &Size<Option<f32>>,
        known_dimensions: &Size<Option<f32>>,
        available_space: &Size<AvailableSpace>,
        run_mode: RunMode,
        sizing_mode: SizingMode,
    ) -> Option<SizeBaselinesAndMargins> {
        get_from_thread_local_cache(node, parent_size, known_dimensions, available_space, run_mode, sizing_mode)
    }

    pub fn insert(
        node: NodeId,
        parent_size: Size<Option<f32>>,
        known_dimensions: Size<Option<f32>>,
        available_space: Size<AvailableSpace>,
        computed_size_and_baselines: SizeBaselinesAndMargins,
        run_mode: RunMode,
        sizing_mode: SizingMode,
    ) {
        store_in_thread_local_cache(
            node,
            parent_size,
            known_dimensions,
            available_space,
            computed_size_and_baselines,
            run_mode,
            sizing_mode,
        )
    }

    pub fn decrement_frame_count() {
        decrement_cache_call_stack_frame_count();
    }

    pub fn increment_frame_count() {
        increment_cache_call_stack_frame_count();
    }
}

fn round_to_6dp(value: f32) -> f32 {
    (value * 100000.).round() / 100000.
}

impl From<AvailableSpace> for CachedAvailableSpace {
    fn from(available_space: AvailableSpace) -> Self {
        match available_space {
            AvailableSpace::Definite(n) => CachedAvailableSpace::Definite(round_to_6dp(n).into()),
            AvailableSpace::MinContent => CachedAvailableSpace::MinContent,
            AvailableSpace::MaxContent => CachedAvailableSpace::MaxContent,
        }
    }
}

impl From<Size<AvailableSpace>> for Size<CachedAvailableSpace> {
    fn from(available_space: Size<AvailableSpace>) -> Self {
        Size { width: available_space.width.into(), height: available_space.height.into() }
    }
}

impl Hash for Size<Option<OrderedFloat<f32>>> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.width.hash(state);
        self.height.hash(state);
    }
}

impl Size<Option<f32>> {
    fn to_cache_variant(&self) -> Size<Option<OrderedFloat<f32>>> {
        Size { width: self.width.map(|w| round_to_6dp(w).into()), height: self.height.map(|h| round_to_6dp(h).into()) }
    }
}

impl Hash for Size<CachedAvailableSpace> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.width.hash(state);
        self.height.hash(state);
    }
}

#[derive(Hash, PartialEq, Eq, Debug)]
enum CachedAvailableSpace {
    /// The amount of space available is the specified number of pixels
    Definite(OrderedFloat<f32>),
    /// The amount of space available is indefinite and the node should be laid out under a min-content constraint
    MinContent,
    /// The amount of space available is indefinite and the node should be laid out under a max-content constraint
    MaxContent,
}
