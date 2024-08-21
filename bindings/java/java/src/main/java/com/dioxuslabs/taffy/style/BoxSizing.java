package com.dioxuslabs.taffy.style;

/**
 * Specifies whether size styles for this node are assigned to the node's "content box" or "border box"
 * <p>
 * - The "content box" is the node's inner size excluding padding, border and margin
 * - The "border box" is the node's outer size including padding and border (but still excluding margin)
 * <p>
 * This property modifies the application of the following styles:
 * <p>
 *   - `size`
 *   - `min_size`
 *   - `max_size`
 *   - `flex_basis`
 * <p
 * See <a href="https://developer.mozilla.org/en-US/docs/Web/CSS/box-sizing">...</a>
 */
public enum BoxSizing {
    /**
     * Size styles such size, min_size, max_size specify the box's "content box" (the size excluding padding/border/margin)
     */
    BORDER_BOX,
    /**
     * Size styles such size, min_size, max_size specify the box's "border box" (the size excluding margin but including padding/border)
     */
    CONTENT_BOX;

    private final int internal;

    BoxSizing() {
        internal = ordinal();
    }
}
