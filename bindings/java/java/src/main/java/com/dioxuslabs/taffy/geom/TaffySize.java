package com.dioxuslabs.taffy.geom;

import com.dioxuslabs.taffy.geom.measure.TaffyAvailableSpace;
import com.dioxuslabs.taffy.geom.measure.TaffyDimension;
import com.dioxuslabs.taffy.geom.measure.TaffyLengthPercentage;
import com.dioxuslabs.taffy.geom.measure.TaffyLengthPercentageAuto;

/**
 * The width and height of a {@link TaffyRect}
 * @param width The x extent of the rectangle
 * @param height The y extent of the rectangle
 */
public record TaffySize<T>(
        T width,
        T height
) {
    /**
     * Dynamic function to create a {@link TaffySize} with a fixed width and height
     *
     * @param clazz The type of the {@link TaffySize}
     * @param width The width
     * @param height The height
     * @return A new {@link TaffySize} with the given width and height
     * @param <T> The type between {@link TaffyDimension}, {@link TaffyAvailableSpace}, {@link TaffyLengthPercentage} and {@link TaffyLengthPercentageAuto}
     */
    @SuppressWarnings("unchecked")
    public static <T> TaffySize<T> length(Class<T> clazz, float width, float height) {
        if (clazz == TaffyDimension.class) {
            return (TaffySize<T>) lengthDimension(width, height);
        } else if (clazz == TaffyAvailableSpace.class) {
            return (TaffySize<T>) definiteAvailableSize(width, height);
        } else if (clazz == TaffyLengthPercentage.class) {
            return (TaffySize<T>) lengthLengthPercentage(width, height);
        } else if (clazz == TaffyLengthPercentageAuto.class) {
            return (TaffySize<T>) lengthLengthPercentageAuto(width, height);
        }
        return null;
    }

    /**
     * Dynamic function to create a {@link TaffySize} with a percentage width and height
     *
     * @param clazz The type of the {@link TaffySize}
     * @param width The width
     * @param height The height
     * @return A new {@link TaffySize} with the given width and height
     * @param <T> The type between {@link TaffyDimension}, {@link TaffyLengthPercentage} and {@link TaffyLengthPercentageAuto}
     */
    @SuppressWarnings("unchecked")
    public static <T> TaffySize<T> percent(Class<T> clazz, float width, float height) {
        if (clazz == TaffyDimension.class) {
            return (TaffySize<T>) percentDimension(width, height);
        } else if (clazz == TaffyLengthPercentage.class) {
            return (TaffySize<T>) percentLengthPercentage(width, height);
        } else if (clazz == TaffyLengthPercentageAuto.class) {
            return (TaffySize<T>) percentLengthPercentageAuto(width, height);
        }
        return null;
    }


    /**
     * Dynamic function to create a {@link TaffySize} with auto as values
     *
     * @param clazz The type of the {@link TaffySize}
     * @return A new {@link TaffySize} with auto as width and height
     * @param <T> The type between {@link TaffyDimension} and {@link TaffyLengthPercentageAuto}
     */
    @SuppressWarnings("unchecked")
    public static <T> TaffySize<T> auto(Class<T> clazz) {
        if (clazz == TaffyDimension.class) {
            return (TaffySize<T>) autoDimension();
        } else if (clazz == TaffyLengthPercentageAuto.class) {
            return (TaffySize<T>) autoLengthPercentageAuto();
        }
        return null;
    }

    /**
     * Creates a {@link TaffyLengthPercentageAuto} {@link TaffySize} with a fixed width and height
     * @param width The width
     * @param height The height
     * @return A new {@link TaffySize} with the given width and height
     */
    public static TaffySize<TaffyLengthPercentageAuto> lengthLengthPercentageAuto(float width, float height) {
        return new TaffySize<>(TaffyLengthPercentageAuto.length(width), TaffyLengthPercentageAuto.length(height));
    }

    /**
     * Creates a {@link TaffyLengthPercentageAuto} {@link TaffySize} with a percentage width and height
     * @param width The width
     * @param height The height
     * @return A new {@link TaffySize} with the given width and height
     */
    public static TaffySize<TaffyLengthPercentageAuto> percentLengthPercentageAuto(float width, float height) {
        return new TaffySize<>(TaffyLengthPercentageAuto.percent(width), TaffyLengthPercentageAuto.percent(height));
    }

    /**
     * Creates a {@link TaffyLengthPercentageAuto} {@link TaffySize} with auto as width and height
     * @return A new {@link TaffySize} with auto as width and height
     */
    public static TaffySize<TaffyLengthPercentageAuto> autoLengthPercentageAuto() {
        return new TaffySize<>(TaffyLengthPercentageAuto.auto(), TaffyLengthPercentageAuto.auto());
    }

    /**
     * Creates a {@link TaffyLengthPercentage} {@link TaffySize} with a fixed width and height
     * @param width The width
     * @param height The height
     * @return A new {@link TaffySize} with the given width and height
     */
    public static TaffySize<TaffyLengthPercentage> lengthLengthPercentage(float width, float height) {
        return new TaffySize<>(TaffyLengthPercentage.length(width), TaffyLengthPercentage.length(height));
    }

    /**
     * Creates a {@link TaffyLengthPercentage} {@link TaffySize} with a percentage width and height
     * @param width The width
     * @param height The height
     * @return A new {@link TaffySize} with the given width and height
     */
    public static TaffySize<TaffyLengthPercentage> percentLengthPercentage(float width, float height) {
        return new TaffySize<>(TaffyLengthPercentage.percent(width), TaffyLengthPercentage.percent(height));
    }

    /**
     * Creates a {@link TaffyDimension} {@link TaffySize} with a fixed width and height
     * @param width The width
     * @param height The height
     * @return A new {@link TaffySize} with the given width and height
     */
    public static TaffySize<TaffyDimension> lengthDimension(float width, float height) {
        return new TaffySize<>(TaffyDimension.length(width), TaffyDimension.length(height));
    }

    /**
     * Creates a {@link TaffyDimension} {@link TaffySize} with a fixed width and height
     * @param width The width
     * @param height The height
     * @return A new {@link TaffySize} with the given width and height
     */
    public static TaffySize<TaffyDimension> percentDimension(float width, float height) {
        return new TaffySize<>(TaffyDimension.percent(width), TaffyDimension.percent(height));
    }

    /**
     * Creates a {@link TaffyDimension} {@link TaffySize} with auto as width and height
     * @return A new {@link TaffySize} with auto as width and height
     */
    public static TaffySize<TaffyDimension> autoDimension() {
        return new TaffySize<>(TaffyDimension.auto(), TaffyDimension.auto());
    }

    /**
     * Creates a {@link TaffyAvailableSpace} {@link TaffySize} with a fixed width and height
     * @param width The width
     * @param height The height
     * @return A new {@link TaffySize} with the given width and height
     */
    public static TaffySize<TaffyAvailableSpace> definiteAvailableSize(float width, float height) {
        return new TaffySize<>(TaffyAvailableSpace.definite(width), TaffyAvailableSpace.definite(height));
    }

    /**
     * Creates a {@link TaffyDimension} {@link TaffySize} with MinContent as width and height
     * @return A new {@link TaffySize} with MinContent as width and height
     */
    public static TaffySize<TaffyAvailableSpace> minContentAvailableSize() {
        return new TaffySize<>(TaffyAvailableSpace.minContent(), TaffyAvailableSpace.minContent());
    }

    /**
     * Creates a {@link TaffyDimension} {@link TaffySize} with MaxContent as width and height
     * @return A new {@link TaffySize} with MaxContent as width and height
     */
    public static TaffySize<TaffyAvailableSpace> maxContentAvailableSize() {
        return new TaffySize<>(TaffyAvailableSpace.maxContent(), TaffyAvailableSpace.maxContent());
    }
}
