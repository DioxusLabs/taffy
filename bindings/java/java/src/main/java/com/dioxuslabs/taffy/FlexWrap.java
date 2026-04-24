package com.dioxuslabs.taffy;

public enum FlexWrap {
    NO_WRAP(0),
    WRAP(1),
    WRAP_REVERSE(2);

    private final int tag;
    FlexWrap(int tag) { this.tag = tag; }
    int tag() { return tag; }
}
