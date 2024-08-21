package com.dioxuslabs.taffy.style;

/**
 * Controls whether flex items are forced onto one line or can wrap onto multiple lines.
 * <p>
 * Defaults to {@link FlexWrap#NO_WRAP}
 * <p>
 * <a href="https://www.w3.org/TR/css-flexbox-1/#flex-wrap-property">Specification</a>
 */
public enum FlexWrap {
    /**
     * Items will not wrap and stay on a single line
     */
    NO_WRAP,
    /**
     * Items will wrap according to this item's {@link FlexDirection}
     */
    WRAP,
    /**
     * Items will wrap in the opposite direction to this item's {@link FlexDirection}
     */
    WRAP_REVERSE;

    private final int internal;

    FlexWrap() {
        internal = ordinal();
    }
}
