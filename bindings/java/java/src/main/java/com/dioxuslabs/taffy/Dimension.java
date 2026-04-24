package com.dioxuslabs.taffy;

/**
 * A CSS-like length that is either an absolute length, a percentage, or
 * {@code auto}.
 *
 * <p>Percentages are expressed as a fraction of 1.0 (e.g. {@code 0.5} = 50%).
 */
public final class Dimension {

    /** The {@code auto} length. */
    public static final Dimension AUTO = new Dimension(NativeBridge.DIM_AUTO, 0f);
    /** Zero absolute length. */
    public static final Dimension ZERO = new Dimension(NativeBridge.DIM_LENGTH, 0f);

    private final int tag;
    private final float value;

    private Dimension(int tag, float value) {
        this.tag = tag;
        this.value = value;
    }

    /** An absolute length (in the application's chosen units). */
    public static Dimension length(float value) {
        return new Dimension(NativeBridge.DIM_LENGTH, value);
    }

    /** A percentage length ({@code 0.5} = 50%). */
    public static Dimension percent(float value) {
        return new Dimension(NativeBridge.DIM_PERCENT, value);
    }

    int tag() { return tag; }
    float value() { return value; }

    static Dimension fromTag(int tag, float value) {
        switch (tag) {
            case NativeBridge.DIM_LENGTH:  return length(value);
            case NativeBridge.DIM_PERCENT: return percent(value);
            default:                       return AUTO;
        }
    }

    @Override
    public boolean equals(Object o) {
        if (this == o) return true;
        if (!(o instanceof Dimension)) return false;
        Dimension d = (Dimension) o;
        return tag == d.tag && Float.compare(value, d.value) == 0;
    }

    @Override
    public int hashCode() {
        return 31 * tag + Float.floatToIntBits(value);
    }

    @Override
    public String toString() {
        switch (tag) {
            case NativeBridge.DIM_LENGTH:  return "length(" + value + ")";
            case NativeBridge.DIM_PERCENT: return "percent(" + value + ")";
            default:                       return "auto";
        }
    }
}
