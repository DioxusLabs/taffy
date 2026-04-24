package dev.dioxus.taffy;

public enum Display {
    BLOCK(0),
    FLEX(1),
    /** Currently treated as FLEX until the grid feature is enabled in the bindings. */
    GRID(2),
    NONE(3);

    private final int tag;
    Display(int tag) { this.tag = tag; }
    int tag() { return tag; }
}
