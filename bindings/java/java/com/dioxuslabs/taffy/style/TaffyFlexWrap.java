package com.dioxuslabs.taffy.style;

/**
 * Controls whether flex items are forced onto one line or can wrap onto multiple lines.
 * <p>
 * Defaults to {@link TaffyFlexWrap#NO_WRAP}
 * <p>
 * <a href="https://www.w3.org/TR/css-flexbox-1/#flex-wrap-property">Specification</a>
 */
public enum TaffyFlexWrap {
    /**
     * Items will not wrap and stay on a single line
     */
    NO_WRAP,
    /**
     * Items will wrap according to this item's {@link TaffyFlexDirection}
     */
    WRAP,
    /**
     * Items will wrap in the opposite direction to this item's {@link TaffyFlexDirection}
     */
    WRAP_REVERSE
}
