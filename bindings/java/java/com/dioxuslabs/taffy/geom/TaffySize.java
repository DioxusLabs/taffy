package com.dioxuslabs.taffy.geom;

/**
 * The width and height of a {@link TaffyRect}
 * @param width The x extent of the rectangle
 * @param height The y extent of the rectangle
 */
public record TaffySize<T>(
        T width,
        T height
) {
}
