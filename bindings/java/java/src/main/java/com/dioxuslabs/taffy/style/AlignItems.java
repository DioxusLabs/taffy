package com.dioxuslabs.taffy.style;

/**
 * Used to control how child nodes are aligned.
 * For Flexbox it controls alignment in the cross axis
 * For Grid it controls alignment in the block axis
 * <p>
 * <a href="https://developer.mozilla.org/en-US/docs/Web/CSS/align-items">MDN</a>
 */
public enum AlignItems {
    /**
     * Items are packed toward the start of the axis
     */
    START,
    /**
     * Items are packed toward the end of the axis
     */
    END,
    /**
     * Items are packed towards the flex-relative start of the axis.
     * <p>
     * For flex containers with {@link FlexDirection#ROW_REVERSE} or {@link FlexDirection#COLUMN_REVERSE}
     * this is equivalent to {@link AlignItems#END}. In all other cases it is equivalent to
     * {@link AlignItems#START}.
     */
    FLEX_START,
    /**
     * Items are packed towards the flex-relative end of the axis.
     * <p>
     * For flex containers with {@link FlexDirection#ROW_REVERSE} or {@link FlexDirection#COLUMN_REVERSE}
     * this is equivalent to {@link AlignItems#START}. In all other cases it is equivalent to
     * {@link AlignItems#END}.
     */
    FLEX_END,
    /**
     * Items are packed along the center of the cross axis
     */
    CENTER,
    /**
     * Items are aligned such as their baselines align
     */
    BASELINE,
    /**
     * Stretch to fill the container
     */
    STRETCH;

    private final int internal;

    AlignItems() {
        internal = ordinal();
    }
}
