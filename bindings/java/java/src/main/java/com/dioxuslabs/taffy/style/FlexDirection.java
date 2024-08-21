package com.dioxuslabs.taffy.style;

/**
 * The direction of the flexbox layout main axis.
 * <p>
 * There are always two perpendicular layout axes: main (or primary) and cross (or secondary).
 * Adding items will cause them to be positioned adjacent to each other along the main axis.
 * By varying this value throughout your tree, you can create complex axis-aligned layouts.
 * <p>
 * Items are always aligned relative to the cross axis, and justified relative to the main axis.
 * <p>
 * The default behavior is {@link FlexDirection#ROW}.
 * <p>
 * <a href="https://www.w3.org/TR/css-flexbox-1/#flex-direction-property">Specification</a>
 */
public enum FlexDirection {
    /**
     * Defines +x as the main axis
     * <p>
     * Items will be added from left to right in a row.
     */
    ROW,
    /**
     * Defines +y as the main axis
     * <p>
     * Items will be added from top to bottom in a column.
     */
    COLUMN,
    /**
     * Defines -x as the main axis
     * <p>
     * Items will be added from right to left in a row.
     */
    ROW_REVERSE,
    /**
     * Defines -y as the main axis
     * <p>
     * Items will be added from bottom to top in a column.
     */
    COLUMN_REVERSE;

    private final int internal;

    FlexDirection() {
        internal = ordinal();
    }
}
