package com.dioxuslabs.taffy.style;

/**
 * Sets the layout used for the children of this node
 * <p>
 * The default values depends on on which feature flags are enabled. The order of precedence is: Flex, Grid, Block, None.
 */
public enum TaffyDisplay {
    /**
     * The children will follow the block layout algorithm
     */
    BLOCK,
    /**
     * The children will follow the flexbox layout algorithm
     */
    FLEX,
    /**
     * The children will follow the CSS Grid layout algorithm
     */
    GRID,
    /**
     * The node is hidden, and it's children will also be hidden
     */
    NONE;

    private final int internal;

    TaffyDisplay() {
        internal = ordinal();
    }
}
