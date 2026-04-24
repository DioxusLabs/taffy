package dev.dioxus.taffy;

import java.util.Objects;

/** The result of a layout computation for a single node. */
public final class Layout {

    public final float x;
    public final float y;
    public final float width;
    public final float height;
    public final float contentWidth;
    public final float contentHeight;
    public final float paddingLeft;
    public final float paddingRight;
    public final float paddingTop;
    public final float paddingBottom;
    public final float borderLeft;
    public final float borderRight;
    public final float borderTop;
    public final float borderBottom;
    public final float marginLeft;
    public final float marginRight;
    public final float marginTop;
    public final float marginBottom;
    public final int order;

    Layout(float[] b) {
        this.x = b[0];
        this.y = b[1];
        this.width = b[2];
        this.height = b[3];
        this.contentWidth = b[4];
        this.contentHeight = b[5];
        this.paddingLeft = b[6];
        this.paddingRight = b[7];
        this.paddingTop = b[8];
        this.paddingBottom = b[9];
        this.borderLeft = b[10];
        this.borderRight = b[11];
        this.borderTop = b[12];
        this.borderBottom = b[13];
        this.marginLeft = b[14];
        this.marginRight = b[15];
        this.marginTop = b[16];
        this.marginBottom = b[17];
        this.order = (int) b[18];
    }

    @Override
    public String toString() {
        return "Layout{x=" + x + ", y=" + y
                + ", width=" + width + ", height=" + height
                + ", contentWidth=" + contentWidth + ", contentHeight=" + contentHeight
                + ", order=" + order + '}';
    }

    @Override
    public boolean equals(Object o) {
        if (this == o) return true;
        if (!(o instanceof Layout)) return false;
        Layout l = (Layout) o;
        return Float.compare(l.x, x) == 0
                && Float.compare(l.y, y) == 0
                && Float.compare(l.width, width) == 0
                && Float.compare(l.height, height) == 0
                && order == l.order;
    }

    @Override
    public int hashCode() {
        return Objects.hash(x, y, width, height, order);
    }
}
