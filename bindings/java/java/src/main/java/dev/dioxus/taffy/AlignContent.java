package dev.dioxus.taffy;

/**
 * Distribution of space between and around content items.
 * Tag {@code 0} means "unset" on the Rust side and is not exposed here;
 * pass {@code null} for an unset value.
 */
public enum AlignContent {
    START(1),
    END(2),
    FLEX_START(3),
    FLEX_END(4),
    CENTER(5),
    STRETCH(6),
    SPACE_BETWEEN(7),
    SPACE_EVENLY(8),
    SPACE_AROUND(9);

    private final int tag;
    AlignContent(int tag) { this.tag = tag; }
    int tag() { return tag; }
}
