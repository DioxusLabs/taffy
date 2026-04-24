package dev.dioxus.taffy;

public enum AlignItems {
    START(1),
    END(2),
    FLEX_START(3),
    FLEX_END(4),
    CENTER(5),
    BASELINE(6),
    STRETCH(7);

    private final int tag;
    AlignItems(int tag) { this.tag = tag; }
    int tag() { return tag; }
}
