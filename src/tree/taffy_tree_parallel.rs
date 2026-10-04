//! Parallel layout of a [`TaffyTree`].
//!
//! # PROTOTYPE
//!
//! This module is a prototype and it is not sound to expose as a safe API yet:
//!
//!  - It relies on the tree being well-formed: each node must be in the children list of at most one node,
//!    and at most once. `TaffyTree`'s mutation methods do not currently guarantee this (for example `add_child`
//!    does not detach the child from its previous parent). This is only checked in debug builds.
//!  - It relies on `Style` being `Sync`, which is only true if `calc()` values point to data that is `Sync`.
//!
//! # How it works
//!
//! The layout algorithms compute the layouts of the children of a node in batches of independent jobs
//! (see [`LayoutPartialTree::compute_child_layouts`]). The view in this module computes the jobs of a batch
//! on Rayon's thread pool if the subtrees they lay out are large enough for that to be worthwhile.
//!
//! Each task that lays out a node writes to the data of the nodes in that node's subtree, and no other task
//! accesses that data until the task has finished, so tasks that lay out sibling subtrees access disjoint data:
//!
//!  - A node's cache and layout are only accessed by the task that is laying out its parent (or, for an
//!    out-of-flow node, its containing block).
//!  - A node's hoisted children, detailed layout info and node context are only accessed by the task that is
//!    laying out the node itself.
//!  - Styles and the structure of the tree are only read.
#![allow(unsafe_code)]

use super::*;
use crate::tree::ChildLayoutJob;
use core::marker::PhantomData;
use rayon::prelude::*;

/// The default minimum total number of nodes in the subtrees that a batch of jobs would
/// lay out for the jobs to be computed in parallel
pub(super) const DEFAULT_MIN_BATCH_WEIGHT: u32 = 256;

/// The index of the slot that a node is stored in
#[inline(always)]
fn slot(node_id: NodeId) -> usize {
    (u64::from(node_id) & 0xffff_ffff) as usize
}

/// State that is shared between all of the tasks of a parallel layout run
struct SharedTree<'t, NodeContext, MeasureFunction> {
    /// A pointer to the data of each node (indexed by slot). Null for vacant slots.
    nodes: Vec<*mut NodeData>,
    /// A pointer to the context of each node (indexed by slot). Null for nodes without a context.
    contexts: Vec<*mut NodeContext>,
    /// The number of nodes in the subtree rooted at each node (indexed by slot)
    subtree_sizes: Vec<u32>,
    /// The children of each node
    children: &'t SlotMap<DefaultKey, ChildrenVec<NodeId>>,
    /// The parent of each node
    #[cfg(debug_assertions)]
    parents: &'t SlotMap<DefaultKey, Option<NodeId>>,
    /// Whether each node is currently being laid out by a task (indexed by slot)
    #[cfg(debug_assertions)]
    in_layout: Vec<core::sync::atomic::AtomicBool>,
    /// The function that measures leaf nodes
    measure_function: &'t MeasureFunction,
    /// Whether any node in the tree is floated
    #[cfg(feature = "block_layout")]
    may_contain_floats: bool,
    /// The minimum total subtree size of a batch of jobs for the jobs to be computed in parallel
    min_batch_weight: u32,
    /// The pointers in `nodes` and `contexts` are derived from a mutable borrow of the tree
    marker: PhantomData<&'t mut TaffyTree<NodeContext>>,
}

// SAFETY (PROTOTYPE: see the module docs for the conditions this relies on that are not yet enforced):
// tasks running on different threads only write to the data of disjoint sets of nodes, and each node context
// is only accessed by one task at a time (but may be accessed from different threads over time).
unsafe impl<NodeContext: Send, MeasureFunction: Sync> Sync for SharedTree<'_, NodeContext, MeasureFunction> {}

impl<'t, NodeContext, MeasureFunction> SharedTree<'t, NodeContext, MeasureFunction> {
    /// Create the shared state for laying out the subtree rooted at `root`
    fn new(taffy: &'t mut TaffyTree<NodeContext>, root: NodeId, measure_function: &'t MeasureFunction) -> Self {
        #[cfg(feature = "float_layout")]
        let floated_node_count = taffy.floated_node_count;
        let TaffyTree { nodes, node_context_data, children, parents, config, .. } = taffy;

        let node_count = nodes.len();
        let mut node_ptrs: Vec<*mut NodeData> = Vec::with_capacity(nodes.capacity() + 1);
        for (key, data) in nodes.iter_mut() {
            let slot = slot(key.into());
            if slot >= node_ptrs.len() {
                node_ptrs.resize(slot + 1, core::ptr::null_mut());
            }
            node_ptrs[slot] = data;
        }
        let mut context_ptrs: Vec<*mut NodeContext> = vec![core::ptr::null_mut(); node_ptrs.len()];
        for (key, context) in node_context_data.iter_mut() {
            context_ptrs[slot(key.into())] = context;
        }

        // Compute the size of each subtree: list the nodes so that each node comes after its parent,
        // then add the size of each node's subtree to its parent's in reverse order
        let mut subtree_sizes: Vec<u32> = vec![0; node_ptrs.len()];
        let mut order: Vec<(u32, u32)> = Vec::with_capacity(node_count);
        let mut stack: Vec<(NodeId, u32)> = Vec::new();
        stack.push((root, u32::MAX));
        while let Some((node_id, parent_slot)) = stack.pop() {
            let node_slot = slot(node_id) as u32;
            order.push((node_slot, parent_slot));
            stack.extend(children[node_id.into()].iter().map(|child| (*child, node_slot)));
        }
        for (node_slot, parent_slot) in order.into_iter().rev() {
            subtree_sizes[node_slot as usize] += 1;
            if parent_slot != u32::MAX {
                subtree_sizes[parent_slot as usize] += subtree_sizes[node_slot as usize];
            }
        }

        #[cfg(not(debug_assertions))]
        let _ = parents;

        Self {
            #[cfg(debug_assertions)]
            in_layout: (0..node_ptrs.len()).map(|_| core::sync::atomic::AtomicBool::new(false)).collect(),
            nodes: node_ptrs,
            contexts: context_ptrs,
            subtree_sizes,
            children,
            #[cfg(debug_assertions)]
            parents,
            measure_function,
            #[cfg(all(feature = "block_layout", feature = "float_layout"))]
            may_contain_floats: floated_node_count > 0,
            #[cfg(all(feature = "block_layout", not(feature = "float_layout")))]
            may_contain_floats: false,
            min_batch_weight: config.parallel_min_batch_weight,
            marker: PhantomData,
        }
    }
}

/// A view over a [`TaffyTree`] that implements the layout traits and computes batches of child layouts in parallel.
///
/// Each task has its own view, so the layout traits' `&mut self` methods do not require exclusive access to the tree.
struct ParTaffyView<'a, 't, NodeContext, MeasureFunction> {
    /// The state shared between all tasks
    shared: &'a SharedTree<'t, NodeContext, MeasureFunction>,
    /// The node that this view's task is currently computing the layout of
    #[cfg(debug_assertions)]
    current: Option<NodeId>,
}

impl<NodeContext, MeasureFunction> ParTaffyView<'_, '_, NodeContext, MeasureFunction> {
    /// A pointer to the node's data
    #[inline(always)]
    fn node(&self, node_id: NodeId) -> *mut NodeData {
        self.shared.nodes[slot(node_id)]
    }

    /// The node's style
    #[inline(always)]
    fn style(&self, node_id: NodeId) -> &Style {
        // SAFETY: styles are not written to during layout
        unsafe { &(*self.node(node_id)).style }
    }
}

impl<'a, 't, NodeContext, MeasureFunction> ParTaffyView<'a, 't, NodeContext, MeasureFunction>
where
    NodeContext: Send,
    MeasureFunction: Fn(LayoutInput, NodeId, Option<&mut NodeContext>, &Style) -> LayoutOutput + Sync,
{
    /// Create a view for a task that is computing the layout of `current`
    #[inline(always)]
    fn new(shared: &'a SharedTree<'t, NodeContext, MeasureFunction>, current: Option<NodeId>) -> Self {
        #[cfg(not(debug_assertions))]
        let _ = current;
        Self {
            shared,
            #[cfg(debug_assertions)]
            current,
        }
    }

    /// Check that the node whose layout this view's task is about to compute is in the subtree of the node that
    /// the task is currently computing the layout of (a child, or an out-of-flow descendant being laid out by its
    /// containing block), and that no other task is computing it.
    #[cfg(debug_assertions)]
    fn enter(&mut self, node_id: NodeId) -> Option<NodeId> {
        if let Some(current) = self.current {
            let mut ancestor = self.shared.parents[node_id.into()];
            while ancestor.is_some() && ancestor != Some(current) {
                ancestor = self.shared.parents[ancestor.unwrap().into()];
            }
            assert!(ancestor.is_some(), "{node_id:?} is being laid out by {current:?}, which is not its ancestor");
        }
        let was_in_layout = self.shared.in_layout[slot(node_id)].swap(true, core::sync::atomic::Ordering::SeqCst);
        assert!(!was_in_layout, "{node_id:?} is being laid out by two tasks at once");
        self.current.replace(node_id)
    }

    /// Undo [`Self::enter`]
    #[cfg(debug_assertions)]
    fn exit(&mut self, node_id: NodeId, previous: Option<NodeId>) {
        self.shared.in_layout[slot(node_id)].store(false, core::sync::atomic::Ordering::SeqCst);
        self.current = previous;
    }

    /// Compute the layout of a node without consulting (or storing the result to) the node's cache
    fn compute_uncached_layout(
        &mut self,
        node_id: NodeId,
        inputs: LayoutInput,
        #[cfg(feature = "block_layout")] block_ctx: Option<&mut BlockContext<'_>>,
    ) -> LayoutOutput {
        #[cfg(debug_assertions)]
        let previous = self.enter(node_id);

        let display_mode = self.style(node_id).display;
        let has_children = self.child_count(node_id) > 0;

        // Dispatch to a layout algorithm based on the node's display style and whether the node has children or not.
        let mut output = match (display_mode, has_children) {
            (Display::None, _) => compute_hidden_layout(self, node_id),
            #[cfg(feature = "block_layout")]
            (Display::Block, true) => compute_block_layout(self, node_id, inputs, block_ctx),
            #[cfg(feature = "block_layout")]
            (Display::FlowRoot, true) => compute_block_layout(self, node_id, inputs, None),
            #[cfg(feature = "flexbox")]
            (Display::Flex, true) => compute_flexbox_layout(self, node_id, inputs),
            #[cfg(feature = "grid")]
            (Display::Grid, true) => compute_grid_layout(self, node_id, inputs),
            (_, false) => {
                let shared = self.shared;
                // SAFETY: styles are not written to during layout
                let style = unsafe { &(*shared.nodes[slot(node_id)]).style };
                // SAFETY: a node's context is only accessed by the task that is computing the node's layout
                let node_context = unsafe { shared.contexts[slot(node_id)].as_mut() };
                (shared.measure_function)(inputs, node_id, node_context, style)
            }
        };

        // Lay out any out-of-flow candidates for which this node is the containing block (see `TaffyView`)
        if inputs.run_mode == RunMode::PerformLayout {
            compute_oof_layout(self, node_id, &mut output);
        }

        #[cfg(debug_assertions)]
        self.exit(node_id, previous);

        output
    }

    /// Unified implementation that both `LayoutPartialTree::compute_child_layout`
    /// and `LayoutBlockContainer::compute_block_child_layout` delegate to.
    #[inline(always)]
    fn compute_child_layout_inner(
        &mut self,
        node_id: NodeId,
        inputs: LayoutInput,
        #[cfg(feature = "block_layout")] block_ctx: Option<&mut BlockContext<'_>>,
    ) -> LayoutOutput {
        if inputs.run_mode == RunMode::PerformHiddenLayout {
            return self.compute_hidden_child_layout(node_id);
        }
        compute_cached_layout(self, node_id, inputs, |tree, node_id, inputs| {
            tree.compute_uncached_layout(
                node_id,
                inputs,
                #[cfg(feature = "block_layout")]
                block_ctx,
            )
        })
    }

    /// Lay out a node whose ancestor is `Display::None` using hidden layout, regardless of its own display style
    fn compute_hidden_child_layout(&mut self, node_id: NodeId) -> LayoutOutput {
        #[cfg(debug_assertions)]
        let previous = self.enter(node_id);
        let output = compute_hidden_layout(self, node_id);
        #[cfg(debug_assertions)]
        self.exit(node_id, previous);
        output
    }

    /// Compute the layout for a job, without consulting (or storing the result to) the node's cache
    #[inline(always)]
    fn compute_uncached_job(&mut self, job: &ChildLayoutJob) -> LayoutOutput {
        #[cfg(feature = "block_layout")]
        if job.is_in_parent_bfc {
            let mut bfc = crate::BlockFormattingContext::float_free();
            let mut block_ctx = bfc.detached_block_context();
            return self.compute_uncached_layout(job.node, job.input, Some(&mut block_ctx));
        }
        self.compute_uncached_layout(
            job.node,
            job.input,
            #[cfg(feature = "block_layout")]
            None,
        )
    }

    /// Compute a batch of child layouts, in parallel if the subtrees that need to be laid out are large enough.
    ///
    /// The caches of the children are read before any tasks are spawned (so that no task is spawned for a job
    /// whose result is cached), and written after all of the tasks have finished.
    fn compute_batch(&mut self, parent_node_id: NodeId, jobs: &mut [ChildLayoutJob]) {
        #[cfg(debug_assertions)]
        {
            let mut nodes: Vec<NodeId> = jobs.iter().map(|job| job.node).collect();
            nodes.sort_by_key(|node| u64::from(*node));
            assert!(nodes.windows(2).all(|pair| pair[0] != pair[1]), "Two jobs in a batch are for the same node");
            assert_eq!(self.current, Some(parent_node_id), "Batch is for a node that is not being laid out");
        }
        #[cfg(not(debug_assertions))]
        let _ = parent_node_id;

        let shared = self.shared;
        let mut uncached_jobs: Vec<&mut ChildLayoutJob> = Vec::new();
        let mut weight: u32 = 0;
        for job in jobs.iter_mut() {
            if job.input.run_mode == RunMode::PerformHiddenLayout {
                job.output = self.compute_hidden_child_layout(job.node);
            } else if let Some(output) = self.cache_get(job.node, &job.input) {
                job.output = output;
            } else {
                weight = weight.saturating_add(shared.subtree_sizes[slot(job.node)]);
                uncached_jobs.push(job);
            }
        }

        if uncached_jobs.len() >= 2 && weight >= shared.min_batch_weight {
            // Group small jobs so that the subtrees laid out by each task are (on average)
            // at least a quarter of the size of the smallest batch that is run in parallel
            let min_task_weight = (shared.min_batch_weight / 4).max(1) as usize;
            let min_len = (min_task_weight * uncached_jobs.len() / (weight.max(1) as usize)).max(1);
            #[cfg(debug_assertions)]
            let current = self.current;
            #[cfg(not(debug_assertions))]
            let current = None;
            uncached_jobs.par_iter_mut().with_min_len(min_len).for_each(|job| {
                let mut view = ParTaffyView::new(shared, current);
                job.output = view.compute_uncached_job(job);
            });
        } else {
            for job in uncached_jobs.iter_mut() {
                job.output = self.compute_uncached_job(job);
            }
        }

        for job in uncached_jobs {
            self.cache_store(job.node, &job.input, job.output.clone());
        }
    }
}

impl<NodeContext, MeasureFunction> TraversePartialTree for ParTaffyView<'_, '_, NodeContext, MeasureFunction> {
    type ChildIter<'a>
        = TaffyTreeChildIter<'a>
    where
        Self: 'a;

    #[inline(always)]
    fn child_ids(&self, parent_node_id: NodeId) -> Self::ChildIter<'_> {
        TaffyTreeChildIter(self.shared.children[parent_node_id.into()].iter())
    }

    #[inline(always)]
    fn child_count(&self, parent_node_id: NodeId) -> usize {
        self.shared.children[parent_node_id.into()].len()
    }

    #[inline(always)]
    fn get_child_id(&self, parent_node_id: NodeId, child_index: usize) -> NodeId {
        self.shared.children[parent_node_id.into()][child_index]
    }
}

impl<NodeContext, MeasureFunction> LayoutPartialTree for ParTaffyView<'_, '_, NodeContext, MeasureFunction>
where
    NodeContext: Send,
    MeasureFunction: Fn(LayoutInput, NodeId, Option<&mut NodeContext>, &Style) -> LayoutOutput + Sync,
{
    type CoreContainerStyle<'a>
        = &'a Style
    where
        Self: 'a;

    type CustomIdent = DefaultCheapStr;

    #[inline(always)]
    fn get_core_container_style(&self, node_id: NodeId) -> Self::CoreContainerStyle<'_> {
        self.style(node_id)
    }

    #[inline(always)]
    fn set_unrounded_layout(&mut self, node_id: NodeId, layout: &Layout) {
        // SAFETY: a node's layout is only accessed by the task that is laying out its parent or containing block
        unsafe { (*self.node(node_id)).unrounded_layout = *layout };
    }

    #[inline(always)]
    fn resolve_calc_value(&self, _val: *const (), _basis: f32) -> f32 {
        0.0
    }

    #[inline(always)]
    fn compute_child_layout(&mut self, node_id: NodeId, inputs: LayoutInput) -> LayoutOutput {
        self.compute_child_layout_inner(
            node_id,
            inputs,
            #[cfg(feature = "block_layout")]
            None,
        )
    }

    #[inline(always)]
    fn compute_child_layouts(&mut self, parent_node_id: NodeId, jobs: &mut [ChildLayoutJob]) {
        self.compute_batch(parent_node_id, jobs)
    }
}

impl<NodeContext, MeasureFunction> LayoutContainingBlock for ParTaffyView<'_, '_, NodeContext, MeasureFunction>
where
    NodeContext: Send,
    MeasureFunction: Fn(LayoutInput, NodeId, Option<&mut NodeContext>, &Style) -> LayoutOutput + Sync,
{
    type OofItemStyle<'a>
        = &'a Style
    where
        Self: 'a;

    #[inline(always)]
    fn get_oof_item_style(&self, node_id: NodeId) -> Self::OofItemStyle<'_> {
        self.style(node_id)
    }

    #[inline(always)]
    fn clear_hoisted_children(&mut self, node_id: NodeId) {
        // SAFETY: a node's hoisted children are only accessed by the task that is laying out the node
        unsafe { (*self.node(node_id)).hoisted_children.clear() };
    }

    #[inline(always)]
    fn add_hoisted_children(&mut self, node_id: NodeId, hoisted: &[NodeId]) {
        // SAFETY: a node's hoisted children are only accessed by the task that is laying out the node
        unsafe { (*self.node(node_id)).hoisted_children.extend_from_slice(hoisted) };
    }

    #[inline(always)]
    fn get_detailed_layout_info(&self, node_id: NodeId) -> &DetailedLayoutInfo {
        // SAFETY: a node's detailed layout info is only accessed by the task that is laying out the node
        unsafe { &(*self.node(node_id)).detailed_layout_info }
    }
}

impl<NodeContext, MeasureFunction> CacheTree for ParTaffyView<'_, '_, NodeContext, MeasureFunction> {
    #[inline(always)]
    fn cache_get(&mut self, node_id: NodeId, input: &LayoutInput) -> Option<LayoutOutput> {
        // SAFETY: a node's cache is only accessed by the task that is laying out its parent or containing block
        unsafe { (*self.node(node_id)).cache.get(input) }
    }

    #[inline(always)]
    fn cache_store(&mut self, node_id: NodeId, input: &LayoutInput, layout_output: LayoutOutput) {
        // SAFETY: a node's cache is only accessed by the task that is laying out its parent or containing block
        unsafe { (*self.node(node_id)).cache.store(input, layout_output) }
    }

    #[inline(always)]
    fn cache_clear(&mut self, node_id: NodeId) {
        // SAFETY: a node's cache is only accessed by the task that is laying out its parent or containing block
        // (hidden layout clears the cache of the node that it is laying out, on behalf of that task)
        unsafe { (*self.node(node_id)).cache.clear() };
    }
}

#[cfg(feature = "block_layout")]
impl<NodeContext, MeasureFunction> LayoutBlockContainer for ParTaffyView<'_, '_, NodeContext, MeasureFunction>
where
    NodeContext: Send,
    MeasureFunction: Fn(LayoutInput, NodeId, Option<&mut NodeContext>, &Style) -> LayoutOutput + Sync,
{
    type BlockContainerStyle<'a>
        = &'a Style
    where
        Self: 'a;
    type BlockItemStyle<'a>
        = &'a Style
    where
        Self: 'a;

    #[inline(always)]
    fn get_block_container_style(&self, node_id: NodeId) -> Self::BlockContainerStyle<'_> {
        self.style(node_id)
    }

    #[inline(always)]
    fn get_block_child_style(&self, child_node_id: NodeId) -> Self::BlockItemStyle<'_> {
        self.style(child_node_id)
    }

    #[inline(always)]
    fn compute_block_child_layout(
        &mut self,
        node_id: NodeId,
        inputs: LayoutInput,
        block_ctx: Option<&mut BlockContext<'_>>,
    ) -> LayoutOutput {
        self.compute_child_layout_inner(node_id, inputs, block_ctx)
    }

    #[inline(always)]
    fn bfc_may_contain_floats(&self, _bfc_root_node_id: NodeId) -> bool {
        self.shared.may_contain_floats
    }

    #[inline(always)]
    fn compute_block_child_layouts(&mut self, parent_node_id: NodeId, jobs: &mut [ChildLayoutJob]) {
        self.compute_batch(parent_node_id, jobs)
    }
}

#[cfg(feature = "flexbox")]
impl<NodeContext, MeasureFunction> LayoutFlexboxContainer for ParTaffyView<'_, '_, NodeContext, MeasureFunction>
where
    NodeContext: Send,
    MeasureFunction: Fn(LayoutInput, NodeId, Option<&mut NodeContext>, &Style) -> LayoutOutput + Sync,
{
    type FlexboxContainerStyle<'a>
        = &'a Style
    where
        Self: 'a;
    type FlexboxItemStyle<'a>
        = &'a Style
    where
        Self: 'a;

    #[inline(always)]
    fn get_flexbox_container_style(&self, node_id: NodeId) -> Self::FlexboxContainerStyle<'_> {
        self.style(node_id)
    }

    #[inline(always)]
    fn get_flexbox_child_style(&self, child_node_id: NodeId) -> Self::FlexboxItemStyle<'_> {
        self.style(child_node_id)
    }
}

#[cfg(feature = "grid")]
impl<NodeContext, MeasureFunction> LayoutGridContainer for ParTaffyView<'_, '_, NodeContext, MeasureFunction>
where
    NodeContext: Send,
    MeasureFunction: Fn(LayoutInput, NodeId, Option<&mut NodeContext>, &Style) -> LayoutOutput + Sync,
{
    type GridContainerStyle<'a>
        = &'a Style
    where
        Self: 'a;
    type GridItemStyle<'a>
        = &'a Style
    where
        Self: 'a;

    #[inline(always)]
    fn get_grid_container_style(&self, node_id: NodeId) -> Self::GridContainerStyle<'_> {
        self.style(node_id)
    }

    #[inline(always)]
    fn get_grid_child_style(&self, child_node_id: NodeId) -> Self::GridItemStyle<'_> {
        self.style(child_node_id)
    }

    #[inline(always)]
    fn set_detailed_grid_info(&mut self, node_id: NodeId, detailed_grid_info: DetailedGridInfo) {
        // SAFETY: a node's detailed layout info is only accessed by the task that is laying out the node
        unsafe { (*self.node(node_id)).detailed_layout_info = DetailedLayoutInfo::Grid(Box::new(detailed_grid_info)) };
    }
}

impl<NodeContext: Send> TaffyTree<NodeContext> {
    /// Updates the stored layout of the provided `node` and its children, laying out sibling subtrees in parallel
    /// on the current Rayon thread pool where they are large enough for that to be worthwhile.
    ///
    /// The result is the same as that of [`compute_layout_with_measure`](Self::compute_layout_with_measure).
    ///
    /// **This is a prototype**. It must only be called on trees in which each node is the child of at most one
    /// node, and appears in that node's list of children only once.
    pub fn compute_layout_with_measure_parallel<MeasureFunction>(
        &mut self,
        node_id: NodeId,
        available_space: Size<AvailableSpace>,
        measure_function: MeasureFunction,
    ) -> Result<(), TaffyError>
    where
        MeasureFunction: Fn(LayoutInput, NodeId, Option<&mut NodeContext>, &Style) -> LayoutOutput + Sync,
    {
        let use_rounding = self.config.use_rounding;
        {
            let shared = SharedTree::new(self, node_id, &measure_function);
            let mut view = ParTaffyView::new(&shared, None);
            compute_root_layout(&mut view, node_id, available_space);
        }
        if use_rounding {
            let mut taffy_view = TaffyView { taffy: self, measure_function };
            round_layout(&mut taffy_view, node_id);
        }
        Ok(())
    }

    /// Updates the stored layout of the provided `node` and its children, laying out sibling subtrees in parallel.
    /// See [`compute_layout_with_measure_parallel`](Self::compute_layout_with_measure_parallel).
    pub fn compute_layout_parallel(
        &mut self,
        node: NodeId,
        available_space: Size<AvailableSpace>,
    ) -> Result<(), TaffyError> {
        self.compute_layout_with_measure_parallel(node, available_space, |inputs, _, _, style| {
            compute_leaf_layout(inputs, style, |_, _| 0.0, |_, _| Size::ZERO)
        })
    }

    /// Set the minimum total number of nodes in the subtrees that a batch of child layouts would lay out
    /// for the batch to be computed in parallel. Setting this to zero runs every batch in parallel.
    #[doc(hidden)]
    pub fn set_parallel_layout_threshold(&mut self, min_batch_weight: u32) {
        self.config.parallel_min_batch_weight = min_batch_weight;
    }
}
