package com.dioxuslabs.taffy.geom;

import com.dioxuslabs.taffy.enums.AbsoluteAxis;
import com.dioxuslabs.taffy.geom.measure.AvailableSpace;
import com.dioxuslabs.taffy.geom.measure.Dimension;
import com.dioxuslabs.taffy.geom.measure.LengthPercentage;
import com.dioxuslabs.taffy.geom.measure.LengthPercentageAuto;

/**
 * The width and height of a {@link Rect}
 *
 * @param width  The x extent of the rectangle
 * @param height The y extent of the rectangle
 */
public record Size<T>(
        T width,
        T height
) {
    /**
     * Dynamic function to create a {@link Size} with a fixed width and height
     *
     * @param clazz  The type of the {@link Size}
     * @param width  The width
     * @param height The height
     * @param <T>    The type between {@link Dimension}, {@link AvailableSpace}, {@link LengthPercentage} and {@link LengthPercentageAuto}
     * @return A new {@link Size} with the given width and height
     */
    @SuppressWarnings("unchecked")
    public static <T> Size<T> length(Class<T> clazz, float width, float height) {
        if (clazz == Dimension.class) {
            return (Size<T>) lengthDimension(width, height);
        } else if (clazz == AvailableSpace.class) {
            return (Size<T>) definiteAvailableSize(width, height);
        } else if (clazz == LengthPercentage.class) {
            return (Size<T>) lengthLengthPercentage(width, height);
        } else if (clazz == LengthPercentageAuto.class) {
            return (Size<T>) lengthLengthPercentageAuto(width, height);
        }
        return null;
    }

    /**
     * Dynamic function to create a {@link Size} with fixed zero width and height
     *
     * @param clazz  The type of the {@link Size}
     * @param <T>    The type between {@link Dimension}, {@link AvailableSpace}, {@link LengthPercentage} and {@link LengthPercentageAuto}
     * @return A new {@link Size} with the given width and height
     */
    public static <T> Size<T> zero(Class<T> clazz) {
        return length(clazz, 0, 0);
    }

    /**
     * Dynamic function to create a {@link Size} with a percentage width and height
     *
     * @param clazz  The type of the {@link Size}
     * @param width  The width
     * @param height The height
     * @param <T>    The type between {@link Dimension}, {@link LengthPercentage} and {@link LengthPercentageAuto}
     * @return A new {@link Size} with the given width and height
     */
    @SuppressWarnings("unchecked")
    public static <T> Size<T> percent(Class<T> clazz, float width, float height) {
        if (clazz == Dimension.class) {
            return (Size<T>) percentDimension(width, height);
        } else if (clazz == LengthPercentage.class) {
            return (Size<T>) percentLengthPercentage(width, height);
        } else if (clazz == LengthPercentageAuto.class) {
            return (Size<T>) percentLengthPercentageAuto(width, height);
        }
        return null;
    }

    /**
     * Dynamic function to create a {@link Size} with auto as values
     *
     * @param clazz The type of the {@link Size}
     * @param <T>   The type between {@link Dimension} and {@link LengthPercentageAuto}
     * @return A new {@link Size} with auto as width and height
     */
    @SuppressWarnings("unchecked")
    public static <T> Size<T> auto(Class<T> clazz) {
        if (clazz == Dimension.class) {
            return (Size<T>) autoDimension();
        } else if (clazz == LengthPercentageAuto.class) {
            return (Size<T>) autoLengthPercentageAuto();
        }
        return null;
    }

    /**
     * Creates a {@link LengthPercentageAuto} {@link Size} with a fixed width and height
     *
     * @param width  The width
     * @param height The height
     * @return A new {@link Size} with the given width and height
     */
    public static Size<LengthPercentageAuto> lengthLengthPercentageAuto(float width, float height) {
        return new Size<>(LengthPercentageAuto.length(width), LengthPercentageAuto.length(height));
    }

    /**
     * Creates a {@link LengthPercentageAuto} {@link Size} with a percentage width and height
     *
     * @param width  The width
     * @param height The height
     * @return A new {@link Size} with the given width and height
     */
    public static Size<LengthPercentageAuto> percentLengthPercentageAuto(float width, float height) {
        return new Size<>(LengthPercentageAuto.percent(width), LengthPercentageAuto.percent(height));
    }

    /**
     * Creates a {@link LengthPercentageAuto} {@link Size} with auto as width and height
     *
     * @return A new {@link Size} with auto as width and height
     */
    public static Size<LengthPercentageAuto> autoLengthPercentageAuto() {
        return new Size<>(LengthPercentageAuto.auto(), LengthPercentageAuto.auto());
    }

    /**
     * Creates a {@link LengthPercentage} {@link Size} with a fixed width and height
     *
     * @param width  The width
     * @param height The height
     * @return A new {@link Size} with the given width and height
     */
    public static Size<LengthPercentage> lengthLengthPercentage(float width, float height) {
        return new Size<>(LengthPercentage.length(width), LengthPercentage.length(height));
    }

    /**
     * Creates a {@link LengthPercentage} {@link Size} with a percentage width and height
     *
     * @param width  The width
     * @param height The height
     * @return A new {@link Size} with the given width and height
     */
    public static Size<LengthPercentage> percentLengthPercentage(float width, float height) {
        return new Size<>(LengthPercentage.percent(width), LengthPercentage.percent(height));
    }

    /**
     * Creates a {@link Dimension} {@link Size} with a fixed width and height
     *
     * @param width  The width
     * @param height The height
     * @return A new {@link Size} with the given width and height
     */
    public static Size<Dimension> lengthDimension(float width, float height) {
        return new Size<>(Dimension.length(width), Dimension.length(height));
    }

    /**
     * Creates a {@link Dimension} {@link Size} with a fixed width and height
     *
     * @param width  The width
     * @param height The height
     * @return A new {@link Size} with the given width and height
     */
    public static Size<Dimension> percentDimension(float width, float height) {
        return new Size<>(Dimension.percent(width), Dimension.percent(height));
    }

    /**
     * Creates a {@link Dimension} {@link Size} with auto as width and height
     *
     * @return A new {@link Size} with auto as width and height
     */
    public static Size<Dimension> autoDimension() {
        return new Size<>(Dimension.auto(), Dimension.auto());
    }

    /**
     * Creates a {@link AvailableSpace} {@link Size} with a fixed width and height
     *
     * @param width  The width
     * @param height The height
     * @return A new {@link Size} with the given width and height
     */
    public static Size<AvailableSpace> definiteAvailableSize(float width, float height) {
        return new Size<>(AvailableSpace.definite(width), AvailableSpace.definite(height));
    }

    /**
     * Creates a {@link Dimension} {@link Size} with MinContent as width and height
     *
     * @return A new {@link Size} with MinContent as width and height
     */
    public static Size<AvailableSpace> minContentAvailableSize() {
        return new Size<>(AvailableSpace.minContent(), AvailableSpace.minContent());
    }

    /**
     * Creates a {@link Dimension} {@link Size} with MaxContent as width and height
     *
     * @return A new {@link Size} with MaxContent as width and height
     */
    public static Size<AvailableSpace> maxContentAvailableSize() {
        return new Size<>(AvailableSpace.maxContent(), AvailableSpace.maxContent());
    }

    @Override
    public String toString() {
        return String.format("""
                        Size {
                            width: %s,
                            height: %s,
                        }""",
                width, height);
    }

    public T getAbs(AbsoluteAxis axis) {
        return axis == AbsoluteAxis.HORIZONTAL ? width : height;
    }
}
