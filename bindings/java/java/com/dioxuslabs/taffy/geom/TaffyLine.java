package com.dioxuslabs.taffy.geom;

/**
 * An abstract "line". Represents any type that has a start and an end
 * @param start The start position of a line
 * @param end The end position of a line
 */
public record TaffyLine<T>(
        T start,
        T end
) {
}
