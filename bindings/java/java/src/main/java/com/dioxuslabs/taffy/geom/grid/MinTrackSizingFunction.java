package com.dioxuslabs.taffy.geom.grid;

import com.dioxuslabs.taffy.geom.measure.LengthPercentage;

/**
 * Minimum track sizing function
 * <p>
 * Specifies the minimum size of a grid track. A grid track will automatically size between it's minimum and maximum size based
 * on the size of it's contents, the amount of available space, and the sizing constraint the grid is being size under.
 * See <a href="https://developer.mozilla.org/en-US/docs/Web/CSS/grid-template-columns">...</a>
 */
public class MinTrackSizingFunction {
    private final byte type;
    private final LengthPercentage value;

    private MinTrackSizingFunction(byte type, LengthPercentage value) {
        this.type = type;
        this.value = value;
    }

    /**
     * Track minimum size should be a fixed length or percentage value
     */
    public static MinTrackSizingFunction fixed(LengthPercentage value) {
        return new MinTrackSizingFunction((byte) 0, value);
    }

    /**
     * Track minimum size should be content sized under a min-content constraint
     */
    public static MinTrackSizingFunction minContent() {
        return new MinTrackSizingFunction((byte) 1, null);
    }

    /**
     * Track minimum size should be content sized under a max-content constraint
     */
    public static MinTrackSizingFunction maxContent() {
        return new MinTrackSizingFunction((byte) 2, null);
    }

    /**
     * Track minimum size should be automatically sized
     */
    public static MinTrackSizingFunction auto() {
        return new MinTrackSizingFunction((byte) 3, null);
    }
}
