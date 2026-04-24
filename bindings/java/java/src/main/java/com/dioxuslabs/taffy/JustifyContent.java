package com.dioxuslabs.taffy;

/**
 * Main-axis distribution for flex containers. Has the same variants as
 * {@link AlignContent}; Taffy aliases them in Rust.
 */
public enum JustifyContent {
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
    JustifyContent(int tag) { this.tag = tag; }
    int tag() { return tag; }
}
