//! Generic CSS content size code that is shared between all CSS algorithms.
use crate::geometry::{Point, Rect, Size};
use crate::style::Overflow;

pub(crate) fn expand_scrollable_overflow(
    container_scrollable_overflow: &mut Rect<f32>,
    container_scroll_origin: Point<f32>,

    item_location: Point<f32>,
    item_size: Size<f32>,
    item_border: Rect<f32>,
    item_scrollbar_gutter: Size<f32>,
    item_overflow: Point<Overflow>,
    item_scrollable_overflow: Rect<f32>,
) {
    let item_rect = Rect::from_top_left_and_size(item_location, item_size);

    // From CSS Overflow 3, section 2.2:
    //   The scrollable overflow area is the union of:
    //     ...
    //   - The border boxes of all boxes for which it is the
    //     containing block and whose border boxes are positioned not
    //     wholly in the unreachable scrollable overflow region,
    //     accounting for transforms by projecting each box onto the
    //     plane of the element that establishes its 3D rendering
    //     context. [CSS3-TRANSFORMS]
    if item_rect.right < container_scroll_origin.x || item_rect.bottom < container_scroll_origin.y {
        return;
    }

    // From CSS Overflow 3, section 2.2:
    //   The scrollable overflow area is the union of:
    //     ...
    //     Border boxes with zero area do not affect the scrollable
    //     overflow area.
    if item_size.has_non_zero_area() {
        container_scrollable_overflow.union_with(item_rect);
    }

    // From CSS Overflow 3, section 2.2:
    //   The scrollable overflow area is the union of:
    //     ...
    //   - The margin areas of grid item and flex item boxes for
    //     which the box establishes a containing block.

    // TODO: open questions:
    // - what does this mean?
    // - is this predicated the same as the previous item?
    // - assuming the margins are positive, don't this subsume the
    //   previous item?

    // From CSS Overflow 3, section 2.2:
    //   The scrollable overflow area is the union of:
    //     ...
    //   - The scrollable overflow areas of all of the above boxes
    //     (including zero-area boxes and accounting for
    //     transforms as described above), provided they
    //     themselves have overflow: visible (i.e. do not
    //     themselves trap the overflow) and that scrollable
    //     overflow is not already clipped (e.g. by the clip
    //     property or the contain property).
    //
    // TODO: Account for clip and contain properties.

    let item_clip_rect = item_rect.shrunk_by(item_scrollbar_gutter).inset_by(item_border);

    let mut positioned_overflow = item_scrollable_overflow.offset_by(item_location.into());

    if match item_overflow {
        Point { x: Overflow::Visible, y: Overflow::Visible } => true,
        Point { x: Overflow::Visible, y: Overflow::Clip } => positioned_overflow.clip_against_y(item_clip_rect),
        Point { x: Overflow::Clip, y: Overflow::Visible } => positioned_overflow.clip_against_x(item_clip_rect),
        // From CSS Overflow 3, section 3.1:
        //   The visible/clip values of overflow compute to
        //   auto/hidden (respectively) if one of overflow-x or
        //   overflow-y is neither visible nor clip.
        //
        // Therefore all other pairs compute to auto, hidden,
        // or clip--and hence do not have overflow: visible.
        _ => false,
    } {
        container_scrollable_overflow.union_with(positioned_overflow);
    }
}
