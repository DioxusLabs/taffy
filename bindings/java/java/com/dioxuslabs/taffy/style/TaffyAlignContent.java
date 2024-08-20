package com.dioxuslabs.taffy.style;

/**
 * Sets the distribution of space between and around content items
 * For Flexbox it controls alignment in the cross axis
 * For Grid it controls alignment in the block axis
 * <p>
 * <a href="https://developer.mozilla.org/en-US/docs/Web/CSS/align-content">MDN</a>
 */
public enum TaffyAlignContent {
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
     * For flex containers with {@link TaffyFlexDirection#ROW_REVERSE} or {@link TaffyFlexDirection#COLUMN_REVERSE}
     * this is equivalent to {@link TaffyAlignContent#END}. In all other cases it is equivalent to
     * {@link TaffyAlignContent#START}.
     */
    FLEX_START,
    /**
     * Items are packed towards the flex-relative end of the axis.
     * <p>
     * For flex containers with {@link TaffyFlexDirection#ROW_REVERSE} or {@link TaffyFlexDirection#COLUMN_REVERSE}
     * this is equivalent to {@link TaffyAlignContent#START}. In all other cases it is equivalent to
     * {@link TaffyAlignContent#END}.
     */
    FLEX_END,
    /**
     * Items are centered along the middle of the cross axis
     */
    CENTER,
    /**
     * Items are stretched to fill the container
     */
    STRETCH,
    /**
     * The first and last items are aligned flush with the edges of the container (no gap)
     * The gap between items is distributed evenly.
     */
    SPACE_BETWEEN,
    /**
     * The gap between the first and last items is exactly <b>THE SAME</b> as the gap between items.
     * The gaps are distributed evenly
     */
    SPACE_EVENLY,
    /**
     * The gap between the first and last items is exactly <b>HALF</b> the gap between items.
     * The gaps are distributed evenly in proportion to these ratios.
     */
    SPACE_AROUND;

    private final int internal;

    TaffyAlignContent() {
        internal = ordinal();
    }
}
