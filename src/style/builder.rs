//! A builder pattern interface to facilitate the creation of [`super::Style`]
use super::{
    AlignContent, AlignItems, AlignSelf, BoxSizing, Dimension, Display, JustifyContent, LengthPercentage,
    LengthPercentageAuto, Overflow, Position, Style,
};
use crate::sys::Rc;
use crate::{util::sys::Vec, Line, NodeId, Point, Rect, Size};
use core::cell::Cell;

#[cfg(feature = "flexbox")]
use super::{FlexDirection, FlexWrap};
#[cfg(feature = "taffy_tree")]
use crate::{TaffyResult, TaffyTree};
#[cfg(feature = "grid")]
use {
    super::{GridAutoFlow, GridPlacement, NonRepeatedTrackSizingFunction, TrackSizingFunction},
    crate::sys::GridTrackVec,
};

/// `NodeIdRef` can be passed to a [`StyleBuilder`] so that caller can later
/// retrieve the [`NodeId`] of a built tree node.
#[derive(Debug, Clone, Default)]
pub struct NodeIdRef(Rc<Cell<Option<NodeId>>>);

impl NodeIdRef {
    /// Create an empty [`NodeIdRef`].
    pub fn new() -> Self {
        Self(Rc::new(Cell::new(None)))
    }

    /// Set the [`NodeId`].
    fn set(&self, node_id: NodeId) {
        self.0.set(Some(node_id));
    }

    /// Get a copy of the inner [`NodeId`], if any is present.
    pub fn get(&self) -> Option<NodeId> {
        self.0.get()
    }
}

/// Given a builder name and associated fields, generate the following :
/// * A struct of the given name, with the following fields
///     * `children`: a vec of child builder
///     * `node_id_ref`: a field holding a [`Option<NodeIdRef>`], wich allow for retrieving the [`NodeId`] of the built node
///     * A [`Option<_>`] field for each provided field
/// * An `impl`` block containing the following :
///     * A method named after the provided field, used to set said field
///     * A `build_style` method, used to generate a [`Style`](super::Style) based on data stored in the builder
macro_rules! gen_builder {
    ($builder:ident, $(($field:ident: $type:ty $(, cfg: $($cfg:tt)+)?)),* $(,)?) => {
        /// Use [`StyleBuilder`] to construct a tree of nested style nodes.
        ///
        /// Example :
        /// ```rust
        /// # use taffy::prelude::*;
        /// let mut builder_tree: TaffyTree<()> = TaffyTree::new();
        /// let header_node_handle = NodeIdRef::new();
        /// let body_node_handle = NodeIdRef::new();
        ///
        /// let builder_root_node = StyleBuilder::new()
        ///     .flex_direction(FlexDirection::Column)
        ///     .size(Size { width: length(800.0), height: length(600.0) })
        ///     .child(
        ///         StyleBuilder::new().width(length(800.0)).height(length(100.0)).node_id_ref(header_node_handle.clone()),
        ///     )
        ///     .child(
        ///         StyleBuilder::new()
        ///             .width(length(800.0))
        ///             .height(auto())
        ///             .flex_grow(1.0)
        ///             .node_id_ref(body_node_handle.clone()),
        ///     )
        ///     .build(&mut builder_tree)
        ///     .unwrap();
        ///
        /// builder_tree.compute_layout(builder_root_node, Size::MAX_CONTENT).unwrap();
        /// ```
        #[derive(Debug, Default)]
        pub struct $builder<'a> {
            children: Vec<&'a StyleBuilder<'a>>,
            node_id_ref: Option<NodeIdRef>,
            $(
                $(#[cfg($($cfg)+)])?
                $field: Option<$type>,
            )*
        }

        impl<'a> $builder<'a> {
            $(
                $(#[cfg($($cfg)+)])?
                #[doc = concat!("Will set the `", stringify!($field), "` field to the provided value in the")]
                #[doc = "\nresulting [`Style`](super::Style) when the [`build`](StyleBuilder::build) method is called."]
                #[doc = concat!("\n\nSee [`Style::", stringify!($field), "`](super::Style::", stringify!($field), ").")]
                pub fn $field(&mut self, $field: impl Into<$type>) -> &mut Self {
                    self.$field = Some($field.into());
                    self
                }
            )*

            /// Build an [`Style`](super::Style) based on provided cconfiguration.
            /// Calling this without setting any field results in
            /// [`Style::default()`](super::Style::default)
            pub fn build_style(&self) -> Style {
                let mut style = Style::default();

                $(
                    $(#[cfg($($cfg)+)])?
                    if let Some(ref value) = self.$field {
                        style.$field = Clone::clone(value);
                    }
                )*

                style
            }
        }
    };
}

gen_builder!(
    StyleBuilder,
    (display: Display),
    (item_is_table: bool),
    (box_sizing: BoxSizing),
    (overflow: Point<Overflow>),
    (scrollbar_width: f32),
    (position: Position),
    (inset: Rect<LengthPercentageAuto>),
    (size: Size<Dimension>),
    (min_size: Size<Dimension>),
    (max_size: Size<Dimension>),
    (aspect_ratio: Option<f32>),
    (margin: Rect<LengthPercentageAuto>),
    (padding: Rect<LengthPercentage>),
    (border: Rect<LengthPercentage>),
    (align_items: Option<AlignItems>, cfg: any(feature = "flexbox", feature = "grid")),
    (align_self: Option<AlignSelf>, cfg: any(feature = "flexbox", feature = "grid")),
    (justify_items: Option<AlignItems>, cfg: feature = "grid"),
    (justify_self: Option<AlignSelf>, cfg: feature = "grid"),
    (align_content: Option<AlignContent>, cfg: any(feature = "flexbox", feature = "grid")),
    (justify_content: Option<JustifyContent>, cfg: any(feature = "flexbox", feature = "grid")),
    (gap: Size<LengthPercentage>, cfg: any(feature = "flexbox", feature = "grid")),
    (text_align: super::TextAlign, cfg: feature = "block_layout"),
    (flex_direction: FlexDirection, cfg: feature = "flexbox"),
    (flex_wrap: FlexWrap, cfg: feature = "flexbox"),
    (flex_basis: Dimension, cfg: feature = "flexbox"),
    (flex_grow: f32, cfg: feature = "flexbox"),
    (flex_shrink: f32, cfg: feature = "flexbox"),
    (grid_template_rows: GridTrackVec<TrackSizingFunction>, cfg: feature = "grid"),
    (grid_template_columns: GridTrackVec<TrackSizingFunction>, cfg: feature = "grid"),
    (grid_auto_rows: GridTrackVec<NonRepeatedTrackSizingFunction>, cfg: feature = "grid"),
    (grid_auto_columns: GridTrackVec<NonRepeatedTrackSizingFunction>, cfg: feature = "grid"),
    (grid_auto_flow: GridAutoFlow, cfg: feature = "grid"),
    (grid_row: Line<GridPlacement>, cfg: feature = "grid"),
    (grid_column: Line<GridPlacement>, cfg: feature = "grid"),
);

impl<'a> StyleBuilder<'a> {
    /// Create a new [`StyleBuilder`].
    pub fn new() -> Self {
        Self::default()
    }

    #[cfg(feature = "flexbox")]
    /// Create a new [`StyleBuilder`], pre-configured to use [`FlexDirection::Row`]
    pub fn row() -> Self {
        let mut row = Self::new();
        row.flex_direction(FlexDirection::Row);
        row
    }

    #[cfg(feature = "flexbox")]
    /// Create a new [`StyleBuilder`], pre-configured to use [`FlexDirection::Column`]
    pub fn column() -> Self {
        let mut column = Self::new();
        column.flex_direction(FlexDirection::Column);
        column
    }

    /// Add a child [`StyleBuilder`] to this builder. Calling this method does not result
    /// in the child [`StyleBuilder`] being built until the [`StyleBuilder::build`] method
    /// is invoke on this builder.
    pub fn child(&'a mut self, style_builder: &'a StyleBuilder) -> &'a mut StyleBuilder<'a> {
        self.children.push(style_builder);
        self
    }

    #[cfg(feature = "taffy_tree")]
    /// Create a new node for this builder and all child builder stored within.
    /// This is done by creating a new node in the provided [`TaffyTree`].
    /// Return a [`TaffyResult<NodeId>`] for the root node. Child [`NodeId`] can be
    /// retrieved once [`build`](StyleBuilder::build) is invoked via setting a [`NodeIdRef`]
    /// in each of the desired child [`StyleBuilder`]
    pub fn build(&self, tree: &mut TaffyTree) -> TaffyResult<NodeId> {
        let style = self.build_style();
        let node_id = tree.new_leaf(style)?;

        if let Some(node_id_ref) = self.node_id_ref.as_ref() {
            node_id_ref.set(node_id);
        }

        let children_node_ids = self.children.iter().map(|child| child.build(tree)).collect::<Result<Vec<_>, _>>()?;

        tree.set_children(node_id, &children_node_ids)?;

        Ok(node_id)
    }

    /// This setter can be used to set a [`NodeIdRef`]. If this is set,
    /// the [`NodeIdRef`] can be used to retrieved to [`NodeId`] of the node
    /// built via the [`build`](StyleBuilder::build) method
    /// Example:
    /// ```rust
    /// # use taffy::prelude::*;
    ///
    /// let mut tree: TaffyTree<()> = TaffyTree::new();
    /// let child_node_id_ref = NodeIdRef::new();
    ///
    /// let root_node_id = StyleBuilder::new()
    ///     .display(Display::Block)
    ///     .child(
    ///         StyleBuilder::new()
    ///             .display(Display::Block)
    ///             .node_id_ref(child_node_id_ref.clone())
    ///     )
    ///     .build(&mut tree)
    ///     .unwrap();
    ///
    /// tree.compute_layout(root_node_id, Size::MAX_CONTENT).unwrap();
    ///
    /// assert!(
    ///     matches!(
    ///         child_node_id_ref.get(),
    ///         Some(_)
    ///     )
    /// );
    ///
    /// tree.layout(child_node_id_ref.get().unwrap()).unwrap();
    /// ```
    pub fn node_id_ref(&'a mut self, node_id_ref: NodeIdRef) -> &'a mut StyleBuilder<'a> {
        self.node_id_ref = Some(node_id_ref);
        self
    }

    /// Shorthand method to set the width of the resulting [`Style`]
    pub fn width(&'a mut self, width: Dimension) -> &'a mut StyleBuilder<'a> {
        match self.size {
            Some(size) => {
                self.size = Some(Size { width, ..size });
            }
            None => self.size = Some(Size { width, ..Style::DEFAULT.size }),
        }
        self
    }

    /// Shorthand method to set the height of the resulting [`Style`]
    pub fn height(&'a mut self, height: Dimension) -> &'a mut StyleBuilder<'a> {
        match self.size {
            Some(size) => {
                self.size = Some(Size { height, ..size });
            }
            None => self.size = Some(Size { height, ..Style::DEFAULT.size }),
        }
        self
    }
}

#[cfg(test)]
mod test {
    #[cfg(feature = "flexbox")]
    use crate::FlexDirection;

    use crate::{
        prelude::{auto, length, TaffyMaxContent},
        style::builder::NodeIdRef,
        Size, TaffyTree,
    };

    use super::{Style, StyleBuilder};

    #[test]
    fn builder_defaults_match_defaults() {
        assert_eq!(StyleBuilder::default().build_style(), Style::default())
    }

    #[test]
    #[cfg(feature = "flexbox")]
    fn readme_example() {
        let mut tree: TaffyTree<()> = TaffyTree::new();
        let header_node = tree
            .new_leaf(Style { size: Size { width: length(800.0), height: length(100.0) }, ..Default::default() })
            .unwrap();

        let body_node = tree
            .new_leaf(Style {
                size: Size { width: length(800.0), height: auto() },
                flex_grow: 1.0,
                ..Default::default()
            })
            .unwrap();

        let root_node = tree
            .new_with_children(
                Style {
                    flex_direction: FlexDirection::Column,
                    size: Size { width: length(800.0), height: length(600.0) },
                    ..Default::default()
                },
                &[header_node, body_node],
            )
            .unwrap();

        tree.compute_layout(root_node, Size::MAX_CONTENT).unwrap();

        let mut builder_tree: TaffyTree<()> = TaffyTree::new();
        let header_node_handle = NodeIdRef::new();
        let body_node_handle = NodeIdRef::new();

        let builder_root_node = StyleBuilder::new()
            .flex_direction(FlexDirection::Column)
            .size(Size { width: length(800.0), height: length(600.0) })
            .child(
                StyleBuilder::new().width(length(800.0)).height(length(100.0)).node_id_ref(header_node_handle.clone()),
            )
            .child(
                StyleBuilder::new()
                    .width(length(800.0))
                    .height(auto())
                    .flex_grow(1.0)
                    .node_id_ref(body_node_handle.clone()),
            )
            .build(&mut builder_tree)
            .unwrap();

        builder_tree.compute_layout(builder_root_node, Size::MAX_CONTENT).unwrap();

        assert_eq!(
            tree.layout(root_node).unwrap().size.width,
            builder_tree.layout(builder_root_node).unwrap().size.width
        );
        assert_eq!(
            tree.layout(root_node).unwrap().size.height,
            builder_tree.layout(builder_root_node).unwrap().size.height
        );
        assert_eq!(
            tree.layout(header_node).unwrap().size.width,
            builder_tree.layout(header_node_handle.get().unwrap()).unwrap().size.width
        );
        assert_eq!(
            tree.layout(header_node).unwrap().size.height,
            builder_tree.layout(header_node_handle.get().unwrap()).unwrap().size.height
        );
        assert_eq!(
            tree.layout(body_node).unwrap().size.width,
            builder_tree.layout(body_node_handle.get().unwrap()).unwrap().size.width
        );
        assert_eq!(
            tree.layout(body_node).unwrap().size.height,
            builder_tree.layout(body_node_handle.get().unwrap()).unwrap().size.height
        );
    }

    #[test]
    fn row() {
        assert_eq!(
            StyleBuilder::row().build_style(),
            Style { flex_direction: FlexDirection::Row, ..Default::default() }
        )
    }

    #[test]
    fn column() {
        assert_eq!(
            StyleBuilder::column().build_style(),
            Style { flex_direction: FlexDirection::Column, ..Default::default() }
        )
    }
}
