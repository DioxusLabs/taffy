package com.dioxuslabs.taffy.style;

/**
 * The positioning strategy for this item.
 * <p>
 * This controls both how the origin is determined for the [`Style::position`] field,
 * and whether or not the item will be controlled by flexbox's layout algorithm.
 * <p>
 * WARNING: this enum follows the behavior of <a href="https://developer.mozilla.org/en-US/docs/Web/CSS/position">CSS's `position` property</a>,
 * which can be unintuitive.
 * <p>
 * {@link Position#RELATIVE} is the default value, in contrast to the default behavior in CSS.
 */
public enum Position {
    /**
     * The offset is computed relative to the final position given by the layout algorithm.
     * Offsets do not affect the position of any other items; they are effectively a correction factor applied at the end.
     */
    RELATIVE,
    /**
     * The offset is computed relative to this item's closest positioned ancestor, if any.
     * Otherwise, it is placed relative to the origin.
     * No space is created for the item in the page layout, and its size will not be altered.
     * <p>
     * WARNING: to opt-out of layouting entirely, you must use {@link Display#NONE} instead on your {@link Style} object.
     */
    ABSOLUTE;

    private final int internal;

    Position() {
        internal = ordinal();
    }
}
