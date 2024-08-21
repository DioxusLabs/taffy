package com.dioxuslabs.taffy.geom.grid;

import com.dioxuslabs.taffy.geom.measure.LengthPercentage;

/**
 * Maximum track sizing function
 * <p>
 * Specifies the maximum size of a grid track. A grid track will automatically size between it's minimum and maximum size based
 * on the size of it's contents, the amount of available space, and the sizing constraint the grid is being size under.
 * See <a href="https://developer.mozilla.org/en-US/docs/Web/CSS/grid-template-columns">...</a>
 */
public class MaxTrackSizingFunction {
    private final byte type;
    private final Object value;

    private MaxTrackSizingFunction(byte type, LengthPercentage value) {
        this.type = type;
        this.value = value;
    }

    private MaxTrackSizingFunction(byte type, float value) {
        this.type = type;
        this.value = value;
    }

    /**
     * Track maximum size should be a fixed length or percentage value
     */
    public static MaxTrackSizingFunction fixed(LengthPercentage value) {
        return new MaxTrackSizingFunction((byte) 0, value);
    }

    /**
     * Track maximum size should be content sized under a min-content constraint
     */
    public static MaxTrackSizingFunction minContent() {
        return new MaxTrackSizingFunction((byte) 1, null);
    }

    /**
     * Track maximum size should be content sized under a max-content constraint
     */
    public static MaxTrackSizingFunction maxContent() {
        return new MaxTrackSizingFunction((byte) 2, null);
    }

    /**
     * Track maximum size should be sized according to the fit-content formula
     */
    public static MaxTrackSizingFunction fitContent(LengthPercentage value) {
        return new MaxTrackSizingFunction((byte) 3, value);
    }

    /**
     * Track maximum size should be automatically sized
     */
    public static MaxTrackSizingFunction auto() {
        return new MaxTrackSizingFunction((byte) 4, null);
    }

    /**
     * The dimension as a fraction of the total available grid space (`fr` units in CSS)
     * Specified value is the numerator of the fraction. Denominator is the sum of all fraction specified in that grid dimension
     * Spec: <a href="https://www.w3.org/TR/css3-grid-layout/#fr-unit">Spec</a>
     */
    public static MaxTrackSizingFunction fraction(float value) {
        return new MaxTrackSizingFunction((byte) 5, value);
    }
}
