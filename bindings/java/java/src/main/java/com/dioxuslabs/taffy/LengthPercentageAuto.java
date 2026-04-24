package com.dioxuslabs.taffy;

/** A length that is either an absolute length, a percentage, or {@code auto}. */
public final class LengthPercentageAuto {

    public static final LengthPercentageAuto AUTO =
            new LengthPercentageAuto(NativeBridge.DIM_AUTO, 0f);
    public static final LengthPercentageAuto ZERO =
            new LengthPercentageAuto(NativeBridge.DIM_LENGTH, 0f);

    private final int tag;
    private final float value;

    private LengthPercentageAuto(int tag, float value) {
        this.tag = tag;
        this.value = value;
    }

    public static LengthPercentageAuto length(float value) {
        return new LengthPercentageAuto(NativeBridge.DIM_LENGTH, value);
    }

    public static LengthPercentageAuto percent(float value) {
        return new LengthPercentageAuto(NativeBridge.DIM_PERCENT, value);
    }

    int tag() { return tag; }
    float value() { return value; }

    static LengthPercentageAuto fromTag(int tag, float value) {
        switch (tag) {
            case NativeBridge.DIM_LENGTH:  return length(value);
            case NativeBridge.DIM_PERCENT: return percent(value);
            default:                       return AUTO;
        }
    }

    @Override
    public boolean equals(Object o) {
        if (this == o) return true;
        if (!(o instanceof LengthPercentageAuto)) return false;
        LengthPercentageAuto l = (LengthPercentageAuto) o;
        return tag == l.tag && Float.compare(value, l.value) == 0;
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
