package dev.dioxus.taffy;

public enum GridAutoFlow {
    ROW(0),
    COLUMN(1),
    ROW_DENSE(2),
    COLUMN_DENSE(3);

    private final int tag;
    GridAutoFlow(int tag) { this.tag = tag; }
    int tag() { return tag; }
}
