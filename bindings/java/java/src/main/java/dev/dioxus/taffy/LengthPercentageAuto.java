package dev.dioxus.taffy;

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
}
