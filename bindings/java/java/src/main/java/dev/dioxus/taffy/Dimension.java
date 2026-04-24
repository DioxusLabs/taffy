package dev.dioxus.taffy;

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
}
