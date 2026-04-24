package dev.dioxus.taffy;

public enum BoxSizing {
    BORDER_BOX(0),
    CONTENT_BOX(1);

    private final int tag;
    BoxSizing(int tag) { this.tag = tag; }
    int tag() { return tag; }
}
