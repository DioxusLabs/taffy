package com.dioxuslabs.taffy;

public enum FlexDirection {
    ROW(0),
    COLUMN(1),
    ROW_REVERSE(2),
    COLUMN_REVERSE(3);

    private final int tag;
    FlexDirection(int tag) { this.tag = tag; }
    int tag() { return tag; }
}
