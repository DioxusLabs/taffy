package com.dioxuslabs.taffy.geom.grid;

/**
 * The first argument to a repeated track definition. This type represents the type of automatic repetition to perform.
 * <p>
 * See <a href="https://www.w3.org/TR/css-grid-1/#auto-repeat">...</a> for an explanation of how auto-repeated
 * track definitions work and the difference between AutoFit and AutoFill.
 */
public class TaffyGridTrackRepetition {
    private final byte type;
    private final short value;

    private TaffyGridTrackRepetition(byte type, short value) {
        this.type = type;
        this.value = value;
    }

    /**
     * Auto-repeating tracks should be generated to fit the container
     * See: <a href="https://developer.mozilla.org/en-US/docs/Web/CSS/repeat#auto-fill">...</a>
     */
    public static TaffyGridTrackRepetition autoFill() {
        return new TaffyGridTrackRepetition((byte) 0, (short) 0);
    }

    /**
     * Auto-repeating tracks should be generated to fit the container
     * See: <a href="https://developer.mozilla.org/en-US/docs/Web/CSS/repeat#auto-fit">...</a>
     */
    public static TaffyGridTrackRepetition autoFit() {
        return new TaffyGridTrackRepetition((byte) 1, (short) 0);
    }

    /**
     * The specified tracks should be repeated exacts N times
     */
    public static TaffyGridTrackRepetition count(short count) {
        return new TaffyGridTrackRepetition((byte) 2, count);
    }

    @Override
    public boolean equals(Object obj) {
        if (!(obj instanceof TaffyGridTrackRepetition lpo)) {
            return false;
        }
        return type == lpo.type && value == lpo.value;
    }
}
