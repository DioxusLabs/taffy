package com.dioxuslabs.taffy.style;

/**
 * Controls whether grid items are placed row-wise or column-wise. And whether the sparse or dense packing algorithm is used.
 * <p>
 * The "dense" packing algorithm attempts to fill in holes earlier in the grid, if smaller items come up later. This may cause items to appear out-of-order, when doing so would fill in holes left by larger items.
 * <p>
 * Defaults to [`GridAutoFlow::Row`]
 * <p>
 * <a href="https://developer.mozilla.org/en-US/docs/Web/CSS/grid-auto-flow">MDN</a>
 */
public enum TaffyGridAutoFlow {
    /**
     * Items are placed by filling each row in turn, adding new rows as necessary
     */
    ROW,
    /**
     * Items are placed by filling each column in turn, adding new columns as necessary.
     */
    COLUMN,
    /**
     * Combines ROW with the dense packing algorithm.
     */
    ROW_DENSE,
    /**
     * Combines COLUMN with the dense packing algorithm.
     */
    COLUMN_DENSE
}
