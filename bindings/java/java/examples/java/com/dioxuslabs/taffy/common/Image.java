package com.dioxuslabs.taffy.common;

import com.dioxuslabs.taffy.geom.Size;

public class Image {
    public record ImageContext(
            float width,
            float height
    ) {
    }

    public static Size<Float> imageMeasureFunction(Size<Float> knownDimensions, ImageContext imageContext) {
        Float width = knownDimensions.width();
        Float height = knownDimensions.height();

        if (width != null && height != null) {
            return new Size<>(width, height);
        } else if (width != null) {
            return new Size<>(width, (width / imageContext.width) * imageContext.height);
        } else if (height != null) {
            return new Size<>((height / imageContext.height) * imageContext.width, height);
        } else {
            return new Size<>(imageContext.width, imageContext.height);
        }
    }
}
