package com.dioxuslabs.taffy;

public enum Position {
    RELATIVE(0),
    ABSOLUTE(1);

    private final int tag;
    Position(int tag) { this.tag = tag; }
    int tag() { return tag; }
}
