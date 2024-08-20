package com.dioxuslabs.taffy.geom;

import com.dioxuslabs.taffy.geom.measure.TaffyAvailableSpace;
import com.dioxuslabs.taffy.geom.measure.TaffyDimension;

/**
 * The width and height of a {@link TaffyRect}
 * @param width The x extent of the rectangle
 * @param height The y extent of the rectangle
 */
public record TaffySize<T>(
        T width,
        T height
) {
    public static TaffySize<TaffyDimension> lengthDimension(float width, float height) {
        return new TaffySize<>(TaffyDimension.length(width), TaffyDimension.length(height));
    }

    public static TaffySize<TaffyDimension> percentDimension(float width, float height) {
        return new TaffySize<>(TaffyDimension.percent(width), TaffyDimension.percent(height));
    }

    public static TaffySize<TaffyDimension> autoDimension() {
        return new TaffySize<>(TaffyDimension.auto(), TaffyDimension.auto());
    }

    public static TaffySize<TaffyAvailableSpace> definiteAvailableSize(float width, float height) {
        return new TaffySize<>(TaffyAvailableSpace.definite(width), TaffyAvailableSpace.definite(height));
    }

    public static TaffySize<TaffyAvailableSpace> minContentAvailableSize() {
        return new TaffySize<>(TaffyAvailableSpace.minContent(), TaffyAvailableSpace.minContent());
    }

    public static TaffySize<TaffyAvailableSpace> maxContentAvailableSize() {
        return new TaffySize<>(TaffyAvailableSpace.maxContent(), TaffyAvailableSpace.maxContent());
    }
}
