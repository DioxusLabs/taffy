package com.dioxuslabs.taffy.geom;

/**
 * A 2-dimensional coordinate.
 * <p>
 * When used in association with a {@link Rect}, represents the top-left corner.
 */
public class Point<T> {
    /**
     * The x-coordinate
     */
    private T x;
    /**
     * The y-coordinate
     */
    private T y;

    public Point(T x, T y) {
        this.x = x;
        this.y = y;
    }

    public T x() {
        return x;
    }

    public T y() {
        return y;
    }

    public void x(T x) {
        this.x = x;
    }

    public void y(T y) {
        this.y = y;
    }
}
