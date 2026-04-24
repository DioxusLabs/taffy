package com.dioxuslabs.taffy;

/**
 * Raw JNI entry points into the Taffy cdylib. Every handle is a native
 * pointer encoded as a {@code long}; a value of {@code 0L} means "null".
 *
 * <p>This class is intentionally package-private and low-level: consumers
 * should use {@link TaffyTree} and {@link Style} instead.
 */
final class NativeBridge {

    static {
        NativeLoader.ensureLoaded();
    }

    private NativeBridge() {}

    // ---- Tag constants (mirror the Rust side) ----

    static final int DIM_LENGTH = 0;
    static final int DIM_PERCENT = 1;
    static final int DIM_AUTO = 2;

    static final int AVAIL_DEFINITE = 0;
    static final int AVAIL_MIN_CONTENT = 1;
    static final int AVAIL_MAX_CONTENT = 2;

    // ---- Tree lifecycle ----

    static native long treeNew();
    static native void treeFree(long handle);
    static native void treeEnableRounding(long handle, int enable);

    // ---- Node creation / mutation ----

    static native long treeNewLeaf(long tree, long style);
    static native long treeNewLeafWithMeasure(long tree, long style, MeasureFunction measure);
    static native long treeNewWithChildren(long tree, long style, long[] children);
    /** Pass {@code null} to clear a previously-installed measure function. */
    static native void treeSetMeasure(long tree, long node, MeasureFunction measure);
    static native void treeSetStyle(long tree, long node, long style);
    static native void treeAddChild(long tree, long parent, long child);
    static native void treeRemoveChild(long tree, long parent, long child);
    static native int  treeChildCount(long tree, long node);
    static native void treeMarkDirty(long tree, long node);
    static native void treeRemove(long tree, long node);

    // ---- Layout ----

    static native void treeComputeLayout(long tree, long node,
                                         int widthTag, float widthValue,
                                         int heightTag, float heightValue);
    /** Fills {@code out} (length >= 19) with layout values. See Layout.unpack. */
    static native void treeLayout(long tree, long node, float[] out);

    // Style snapshot buffer sizes — see lib.rs for the slot layout.
    static final int SNAPSHOT_INTS   = 39;
    static final int SNAPSHOT_FLOATS = 29;
    /** Fills {@code ints} and {@code floats} with every scalar style property. */
    static native void treeGetStyle(long tree, long node, int[] ints, float[] floats);

    // ---- Style ----

    static native long styleNew();
    static native void styleFree(long handle);

    static native void styleSetDisplay(long handle, int tag);
    static native void styleSetPosition(long handle, int tag);
    static native void styleSetBoxSizing(long handle, int tag);
    static native void styleSetOverflow(long handle, int xTag, int yTag);
    static native void styleSetScrollbarWidth(long handle, float value);

    static native void styleSetSize(long handle, int wTag, float wVal, int hTag, float hVal);
    static native void styleSetMinSize(long handle, int wTag, float wVal, int hTag, float hVal);
    static native void styleSetMaxSize(long handle, int wTag, float wVal, int hTag, float hVal);

    static native void styleSetMargin(long handle,
                                      int lTag, float lVal, int rTag, float rVal,
                                      int tTag, float tVal, int bTag, float bVal);
    static native void styleSetInset(long handle,
                                     int lTag, float lVal, int rTag, float rVal,
                                     int tTag, float tVal, int bTag, float bVal);
    static native void styleSetPadding(long handle,
                                       int lTag, float lVal, int rTag, float rVal,
                                       int tTag, float tVal, int bTag, float bVal);
    static native void styleSetBorder(long handle,
                                      int lTag, float lVal, int rTag, float rVal,
                                      int tTag, float tVal, int bTag, float bVal);
    static native void styleSetGap(long handle, int wTag, float wVal, int hTag, float hVal);

    static native void styleSetFlexDirection(long handle, int tag);
    static native void styleSetFlexWrap(long handle, int tag);
    static native void styleSetFlexGrow(long handle, float value);
    static native void styleSetFlexShrink(long handle, float value);
    static native void styleSetFlexBasis(long handle, int tag, float value);

    static native void styleSetJustifyContent(long handle, int tag);
    static native void styleSetAlignContent(long handle, int tag);
    static native void styleSetAlignItems(long handle, int tag);
    static native void styleSetAlignSelf(long handle, int tag);
    static native void styleSetJustifyItems(long handle, int tag);
    static native void styleSetJustifySelf(long handle, int tag);

    /** Pass {@link Float#NaN} to unset. */
    static native void styleSetAspectRatio(long handle, float value);

    // ---- Grid ----
    // Track kind tags: 0=length, 1=percent, 2=fr, 3=auto, 4=min-content, 5=max-content
    // Placement tags:  0=auto, 1=line(value as int), 2=span(value as int)
    // Auto-flow tags:  0=row, 1=column, 2=row-dense, 3=column-dense
    //
    // Each track is carried as four parallel arrays (minTag, minVal, maxTag,
    // maxVal). For single-valued tracks min and max are identical; for
    // minmax(a, b) they differ.

    static native void styleSetGridTemplateColumns(long handle,
                                                   int[] headers,
                                                   int[] minTags, float[] minVals,
                                                   int[] maxTags, float[] maxVals);
    static native void styleSetGridTemplateRows(long handle,
                                                int[] headers,
                                                int[] minTags, float[] minVals,
                                                int[] maxTags, float[] maxVals);
    static native void styleSetGridAutoColumns(long handle,
                                               int[] minTags, float[] minVals,
                                               int[] maxTags, float[] maxVals);
    static native void styleSetGridAutoRows(long handle,
                                            int[] minTags, float[] minVals,
                                            int[] maxTags, float[] maxVals);
    static native void styleSetGridColumn(long handle,
                                          int startTag, int startVal, String startName,
                                          int endTag,   int endVal,   String endName);
    static native void styleSetGridRow(long handle,
                                       int startTag, int startVal, String startName,
                                       int endTag,   int endVal,   String endName);
    static native void styleSetGridAutoFlow(long handle, int tag);

    /** Line names for grid-template-{columns,rows}. Parallel arrays:
     *  {@code counts[i]} is the number of names at line position {@code i},
     *  and the next {@code counts[i]} entries of {@code flatNames} belong
     *  to that line. */
    static native void styleSetGridTemplateColumnNames(long handle, int[] counts, String[] flatNames);
    static native void styleSetGridTemplateRowNames(long handle, int[] counts, String[] flatNames);

    /** Parallel arrays describing CSS {@code grid-template-areas} entries. */
    static native void styleSetGridTemplateAreas(long handle,
                                                 String[] names,
                                                 int[] rowStarts, int[] rowEnds,
                                                 int[] columnStarts, int[] columnEnds);
}
