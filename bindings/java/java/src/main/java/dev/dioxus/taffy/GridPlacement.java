package dev.dioxus.taffy;

/**
 * A grid placement — one end of a {@code grid-row} or {@code grid-column}
 * line pair. Either {@link #AUTO}, an explicit line index (1-based, may
 * be negative to count from the end), or a span count.
 */
public final class GridPlacement {

    static final int TAG_AUTO = 0;
    static final int TAG_LINE = 1;
    static final int TAG_SPAN = 2;

    public static final GridPlacement AUTO = new GridPlacement(TAG_AUTO, 0);

    private final int tag;
    private final int value;

    private GridPlacement(int tag, int value) { this.tag = tag; this.value = value; }

    public static GridPlacement line(int index) { return new GridPlacement(TAG_LINE, index); }
    public static GridPlacement span(int count) { return new GridPlacement(TAG_SPAN, Math.max(1, count)); }

    int tag() { return tag; }
    int value() { return value; }
}
