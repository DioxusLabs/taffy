package dev.dioxus.taffy;

public enum Overflow {
    VISIBLE(0),
    CLIP(1),
    HIDDEN(2),
    SCROLL(3);

    private final int tag;
    Overflow(int tag) { this.tag = tag; }
    int tag() { return tag; }
}
