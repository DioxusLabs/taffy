package com.dioxuslabs.taffy;

/** A length that is either an absolute length or a percentage (no {@code auto}). */
public final class LengthPercentage {

    public static final LengthPercentage ZERO = new LengthPercentage(NativeBridge.DIM_LENGTH, 0f);

    private final int tag;
    private final float value;

    private LengthPercentage(int tag, float value) {
        this.tag = tag;
        this.value = value;
    }

    public static LengthPercentage length(float value) {
        return new LengthPercentage(NativeBridge.DIM_LENGTH, value);
    }

    public static LengthPercentage percent(float value) {
        return new LengthPercentage(NativeBridge.DIM_PERCENT, value);
    }

    int tag() { return tag; }
    float value() { return value; }

    static LengthPercentage fromTag(int tag, float value) {
        return tag == NativeBridge.DIM_PERCENT ? percent(value) : length(value);
    }

    @Override
    public boolean equals(Object o) {
        if (this == o) return true;
        if (!(o instanceof LengthPercentage)) return false;
        LengthPercentage l = (LengthPercentage) o;
        return tag == l.tag && Float.compare(value, l.value) == 0;
    }

    @Override
    public int hashCode() {
        return 31 * tag + Float.floatToIntBits(value);
    }

    @Override
    public String toString() {
        return tag == NativeBridge.DIM_PERCENT
                ? "percent(" + value + ")"
                : "length(" + value + ")";
    }
}
