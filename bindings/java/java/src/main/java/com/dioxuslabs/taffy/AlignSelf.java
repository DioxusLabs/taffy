package com.dioxuslabs.taffy;

/** Alias enum for {@link AlignItems}; Taffy uses the same type for both in Rust. */
public enum AlignSelf {
    START(1),
    END(2),
    FLEX_START(3),
    FLEX_END(4),
    CENTER(5),
    BASELINE(6),
    STRETCH(7);

    private final int tag;
    AlignSelf(int tag) { this.tag = tag; }
    int tag() { return tag; }
}
