package com.dioxuslabs.taffy;

/**
 * A soft constraint on the amount of space available when performing layout.
 * Passed to {@link TaffyTree#computeLayout}.
 */
public final class AvailableSpace {

    /** Tag value for "a specific amount of pixels is available" (pass with the pixel count). */
    public static final int TAG_DEFINITE    = NativeBridge.AVAIL_DEFINITE;
    /** Tag value for "minimum available space" (infinitely constrained). */
    public static final int TAG_MIN_CONTENT = NativeBridge.AVAIL_MIN_CONTENT;
    /** Tag value for "maximum available space" (infinite). */
    public static final int TAG_MAX_CONTENT = NativeBridge.AVAIL_MAX_CONTENT;

    /** The layout is infinite in this dimension. */
    public static final AvailableSpace MAX_CONTENT =
            new AvailableSpace(TAG_MAX_CONTENT, 0f);
    /** The layout has zero available space in this dimension. */
    public static final AvailableSpace MIN_CONTENT =
            new AvailableSpace(TAG_MIN_CONTENT, 0f);

    private final int tag;
    private final float value;

    private AvailableSpace(int tag, float value) {
        this.tag = tag;
        this.value = value;
    }

    /** A specific amount of pixels is available. */
    public static AvailableSpace definite(float pixels) {
        return new AvailableSpace(NativeBridge.AVAIL_DEFINITE, pixels);
    }

    int tag() { return tag; }
    float value() { return value; }
}
