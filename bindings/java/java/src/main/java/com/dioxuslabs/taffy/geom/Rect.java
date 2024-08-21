package com.dioxuslabs.taffy.geom;

/**
 * An axis-aligned UI rectangle
 */
public class Rect<T>{
    /**
     * This can represent either the x-coordinate of the starting edge,
     *  or the amount of padding on the starting side.
     * <p>
     * The starting edge is the left edge when working with LTR text,
     * and the right edge when working with RTL text.
     */
    private T left;
    /**
     * This can represent either the x-coordinate of the ending edge,
     * or the amount of padding on the ending side.
     */
    private T right;
    /**
     * This can represent either the y-coordinate of the top edge,
     * or the amount of padding on the top side.
     */
    private T top;
    /**
     * This can represent either the y-coordinate of the bottom edge,
     * or the amount of padding on the bottom side.
     */
    private T bottom;

    public Rect(T left, T right, T top, T bottom) {
        this.left = left;
        this.right = right;
        this.top = top;
        this.bottom = bottom;
    }

    public T left() {
        return left;
    }

    public T right() {
        return right;
    }

    public T top() {
        return top;
    }

    public T bottom() {
        return bottom;
    }

    public void left(T left) {
        this.left = left;
    }

    public void right(T right) {
        this.right = right;
    }

    public void top(T top) {
        this.top = top;
    }

    public void bottom(T bottom) {
        this.bottom = bottom;
    }
}
