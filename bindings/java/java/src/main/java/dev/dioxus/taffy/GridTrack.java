package dev.dioxus.taffy;

/**
 * A single column/row track size for {@code grid-template-columns} /
 * {@code grid-template-rows}. Create via the static factory methods.
 *
 * <p>This is the minimal subset of Taffy's {@code TrackSizingFunction}
 * that the Java bindings expose: scalar min-max sizing functions.
 * Not supported yet: {@code minmax()}, {@code fit-content()},
 * {@code repeat()}, named lines.
 */
public final class GridTrack {

    // Tags mirror lib.rs.
    static final int TAG_LENGTH      = 0;
    static final int TAG_PERCENT     = 1;
    static final int TAG_FR          = 2;
    static final int TAG_AUTO        = 3;
    static final int TAG_MIN_CONTENT = 4;
    static final int TAG_MAX_CONTENT = 5;

    public static final GridTrack AUTO        = new GridTrack(TAG_AUTO, 0f);
    public static final GridTrack MIN_CONTENT = new GridTrack(TAG_MIN_CONTENT, 0f);
    public static final GridTrack MAX_CONTENT = new GridTrack(TAG_MAX_CONTENT, 0f);

    private final int tag;
    private final float value;

    private GridTrack(int tag, float value) { this.tag = tag; this.value = value; }

    public static GridTrack length(float px)     { return new GridTrack(TAG_LENGTH, px); }
    public static GridTrack percent(float ratio) { return new GridTrack(TAG_PERCENT, ratio); }
    public static GridTrack fr(float factor)     { return new GridTrack(TAG_FR, factor); }

    int tag() { return tag; }
    float value() { return value; }
}
