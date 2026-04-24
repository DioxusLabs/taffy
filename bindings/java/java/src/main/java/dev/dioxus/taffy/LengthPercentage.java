package dev.dioxus.taffy;

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
}
