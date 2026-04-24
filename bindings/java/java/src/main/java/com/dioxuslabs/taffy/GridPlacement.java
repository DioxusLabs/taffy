package com.dioxuslabs.taffy;

/**
 * A grid placement — one end of a {@code grid-row} or {@code grid-column}
 * line pair. Either {@link #AUTO}, an explicit line index (1-based, may be
 * negative to count from the end), a span count, or a named variant
 * referencing a line from {@code grid-template-*-names}.
 */
public final class GridPlacement {

    static final int TAG_AUTO        = 0;
    static final int TAG_LINE        = 1;
    static final int TAG_SPAN        = 2;
    static final int TAG_NAMED_LINE  = 3;
    static final int TAG_NAMED_SPAN  = 4;

    public static final GridPlacement AUTO = new GridPlacement(TAG_AUTO, 0, null);

    private final int    tag;
    private final int    value;
    private final String name; // null for un-named variants

    private GridPlacement(int tag, int value, String name) {
        this.tag = tag;
        this.value = value;
        this.name = name;
    }

    public static GridPlacement line(int index) { return new GridPlacement(TAG_LINE, index, null); }
    public static GridPlacement span(int count) { return new GridPlacement(TAG_SPAN, Math.max(1, count), null); }

    /** The first line with the given name (equivalent to {@code namedLine(name, 1)}). */
    public static GridPlacement namedLine(String name) { return namedLine(name, 1); }

    /** The {@code index}-th line with the given name (1-based; negative counts from the end). */
    public static GridPlacement namedLine(String name, int index) {
        java.util.Objects.requireNonNull(name, "name");
        return new GridPlacement(TAG_NAMED_LINE, index, name);
    }

    /** Span until the first line with the given name (equivalent to {@code namedSpan(name, 1)}). */
    public static GridPlacement namedSpan(String name) { return namedSpan(name, 1); }

    /** Span until the {@code count}-th line with the given name. */
    public static GridPlacement namedSpan(String name, int count) {
        java.util.Objects.requireNonNull(name, "name");
        return new GridPlacement(TAG_NAMED_SPAN, Math.max(1, count), name);
    }

    int    tag()   { return tag; }
    int    value() { return value; }
    String name()  { return name; }
}
