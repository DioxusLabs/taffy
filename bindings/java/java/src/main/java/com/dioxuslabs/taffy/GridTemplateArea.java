package com.dioxuslabs.taffy;

/**
 * One named rectangle in a {@code grid-template-areas} value.
 *
 * <p>Row and column indices are 1-based line numbers (matching CSS
 * {@code grid-row-start} semantics). {@code rowEnd} / {@code columnEnd}
 * are <em>exclusive</em> line numbers — an area spanning one track from
 * line 1 to the next line is {@code (rowStart=1, rowEnd=2)}.
 */
public final class GridTemplateArea {

    private final String name;
    private final int rowStart;
    private final int rowEnd;
    private final int columnStart;
    private final int columnEnd;

    private GridTemplateArea(String name, int rowStart, int rowEnd, int columnStart, int columnEnd) {
        this.name        = java.util.Objects.requireNonNull(name, "name");
        this.rowStart    = rowStart;
        this.rowEnd      = rowEnd;
        this.columnStart = columnStart;
        this.columnEnd   = columnEnd;
    }

    public static GridTemplateArea of(String name, int rowStart, int rowEnd, int columnStart, int columnEnd) {
        return new GridTemplateArea(name, rowStart, rowEnd, columnStart, columnEnd);
    }

    public String name()        { return name; }
    public int    rowStart()    { return rowStart; }
    public int    rowEnd()      { return rowEnd; }
    public int    columnStart() { return columnStart; }
    public int    columnEnd()   { return columnEnd; }

    @Override
    public String toString() {
        return "GridTemplateArea{" + name + " @ rows " + rowStart + "-" + rowEnd
                + ", cols " + columnStart + "-" + columnEnd + '}';
    }
}
