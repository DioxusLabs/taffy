package com.dioxuslabs.taffy;

/**
 * A single column/row track size for {@code grid-template-columns} /
 * {@code grid-template-rows} / {@code grid-auto-columns} / {@code grid-auto-rows}.
 *
 * <p>Internally every track carries a <em>min</em> and a <em>max</em>
 * sizing function; the usual factories ({@link #length}, {@link #fr}, etc.)
 * set both sides to the same function, and {@link #minmax} lets you
 * specify them independently as in CSS {@code minmax(100px, 1fr)}.
 *
 * <p>Not yet supported: {@code repeat()}, named lines.
 */
public final class GridTrack {

    // Tags mirror lib.rs.
    static final int TAG_LENGTH              = 0;
    static final int TAG_PERCENT             = 1;
    static final int TAG_FR                  = 2;
    static final int TAG_AUTO                = 3;
    static final int TAG_MIN_CONTENT         = 4;
    static final int TAG_MAX_CONTENT         = 5;
    static final int TAG_FIT_CONTENT_LENGTH  = 6;
    static final int TAG_FIT_CONTENT_PERCENT = 7;

    public static final GridTrack AUTO        = single(TAG_AUTO, 0f);
    public static final GridTrack MIN_CONTENT = single(TAG_MIN_CONTENT, 0f);
    public static final GridTrack MAX_CONTENT = single(TAG_MAX_CONTENT, 0f);

    private final int   minTag;
    private final float minValue;
    private final int   maxTag;
    private final float maxValue;

    private GridTrack(int minTag, float minValue, int maxTag, float maxValue) {
        this.minTag = minTag;
        this.minValue = minValue;
        this.maxTag = maxTag;
        this.maxValue = maxValue;
    }

    private static GridTrack single(int tag, float value) {
        return new GridTrack(tag, value, tag, value);
    }

    public static GridTrack length(float px)     { return single(TAG_LENGTH,  px); }
    public static GridTrack percent(float ratio) { return single(TAG_PERCENT, ratio); }
    /** A fraction of the remaining free space ({@code 1fr} in CSS). Max-only; on the
     *  min side it behaves as {@link #AUTO}. */
    public static GridTrack fr(float factor)     { return single(TAG_FR, factor); }

    /** CSS {@code fit-content(<length>)} — a max track sizing function that
     *  clamps the content size by the given length. Max-only; on the min side
     *  it behaves as {@link #AUTO}. */
    public static GridTrack fitContent(float px) { return single(TAG_FIT_CONTENT_LENGTH, px); }
    /** CSS {@code fit-content(<percentage>)} — same as {@link #fitContent(float)}
     *  but the limit is a percentage ({@code 0.5} = 50%). */
    public static GridTrack fitContentPercent(float ratio) { return single(TAG_FIT_CONTENT_PERCENT, ratio); }

    /**
     * A track whose min and max sizing functions differ, as in CSS
     * {@code minmax(min, max)}. Tags invalid for the min side (such as
     * {@link #fr}) are treated as {@link #AUTO} by the layout engine.
     */
    public static GridTrack minmax(GridTrack min, GridTrack max) {
        return new GridTrack(min.minTag, min.minValue, max.maxTag, max.maxValue);
    }

    int minTag()   { return minTag; }
    float minValue() { return minValue; }
    int maxTag()   { return maxTag; }
    float maxValue() { return maxValue; }

    @Override
    public boolean equals(Object o) {
        if (this == o) return true;
        if (!(o instanceof GridTrack)) return false;
        GridTrack g = (GridTrack) o;
        return minTag == g.minTag && maxTag == g.maxTag
                && Float.compare(minValue, g.minValue) == 0
                && Float.compare(maxValue, g.maxValue) == 0;
    }

    @Override
    public int hashCode() {
        int r = minTag;
        r = 31 * r + Float.floatToIntBits(minValue);
        r = 31 * r + maxTag;
        r = 31 * r + Float.floatToIntBits(maxValue);
        return r;
    }

    @Override
    public String toString() {
        if (minTag == maxTag && Float.compare(minValue, maxValue) == 0) {
            return tagToString(minTag, minValue);
        }
        return "minmax(" + tagToString(minTag, minValue) + ", " + tagToString(maxTag, maxValue) + ")";
    }

    private static String tagToString(int tag, float value) {
        switch (tag) {
            case TAG_LENGTH:              return "length(" + value + ")";
            case TAG_PERCENT:             return "percent(" + value + ")";
            case TAG_FR:                  return "fr(" + value + ")";
            case TAG_MIN_CONTENT:         return "min-content";
            case TAG_MAX_CONTENT:         return "max-content";
            case TAG_FIT_CONTENT_LENGTH:  return "fit-content(" + value + ")";
            case TAG_FIT_CONTENT_PERCENT: return "fit-content(" + value + "%)";
            default:                      return "auto";
        }
    }
}
