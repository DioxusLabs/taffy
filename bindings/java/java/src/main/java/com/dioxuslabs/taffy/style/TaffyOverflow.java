package com.dioxuslabs.taffy.style;

/**
 * How children overflowing their container should affect layout
 * <p>
 * In CSS the primary effect of this property is to control whether contents of a parent container that overflow that container should
 * be displayed anyway, be clipped, or trigger the container to become a scroll container. However it also has secondary effects on layout,
 * the main ones being:
 * <p>
 *   - The automatic minimum size Flexbox/CSS Grid items with non-`Visible` overflow is `0` rather than being content based
 *   - `Overflow::Scroll` nodes have space in the layout reserved for a scrollbar (width controlled by the `scrollbar_width` property)
 * <p>
 * In Taffy, we only implement the layout related secondary effects as we are not concerned with drawing/painting. The amount of space reserved for
 * a scrollbar is controlled by the `scrollbar_width` property. If this is `0` then `Scroll` behaves identically to `Hidden`.
 * <p>
 * <a href="https://developer.mozilla.org/en-US/docs/Web/CSS/overflow">...</a>
 */
public enum TaffyOverflow {
    /**
     * The automatic minimum size of this node as a flexbox/grid item should be based on the size of its content.
     * Content that overflows this node *should* contribute to the scroll region of its parent.
     */
    VISIBLE,
    /**
     * The automatic minimum size of this node as a flexbox/grid item should be based on the size of its content.
     * Content that overflows this node should *not* contribute to the scroll region of its parent.
     */
    CLIP,
    /**
     * The automatic minimum size of this node as a flexbox/grid item should be `0`.
     * Content that overflows this node should *not* contribute to the scroll region of its parent.
     */
    HIDDEN,
    /**
     * The automatic minimum size of this node as a flexbox/grid item should be `0`. Additionally, space should be reserved
     * for a scrollbar. The amount of space reserved is controlled by the `scrollbar_width` property.
     * Content that overflows this node should *not* contribute to the scroll region of its parent.
     */
    SCROLL;

    private final int internal;

    TaffyOverflow() {
        internal = ordinal();
    }
}
