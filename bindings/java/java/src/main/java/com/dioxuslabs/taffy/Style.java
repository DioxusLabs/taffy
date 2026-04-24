package com.dioxuslabs.taffy;

/**
 * Mutable, fluent CSS-like style declaration. Mirrors the flex/block subset of
 * Taffy's Rust {@code Style} struct.
 *
 * <p>Create one with {@link #builder()} (or {@code new Style()}) and chain setters:
 * <pre>{@code
 *   Style s = Style.builder()
 *       .size(Dimension.length(100), Dimension.length(100))
 *       .flexDirection(FlexDirection.COLUMN)
 *       .justifyContent(JustifyContent.CENTER);
 * }</pre>
 *
 * <p>The Java object only stores values; the native Taffy {@code Style} is built
 * on-the-fly when the style is handed to {@link TaffyTree#newLeaf(Style)}.
 */
public final class Style {

    // --- Stored values (null = unset unless a default is documented) ---

    private Display display;
    private Position position;
    private BoxSizing boxSizing;
    private Overflow overflowX;
    private Overflow overflowY;
    private Float scrollbarWidth;

    private Dimension sizeW, sizeH;
    private Dimension minW, minH;
    private Dimension maxW, maxH;

    private LengthPercentageAuto marginL, marginR, marginT, marginB;
    private LengthPercentageAuto insetL, insetR, insetT, insetB;
    private LengthPercentage paddingL, paddingR, paddingT, paddingB;
    private LengthPercentage borderL, borderR, borderT, borderB;
    private LengthPercentage gapW, gapH;

    private FlexDirection flexDirection;
    private FlexWrap flexWrap;
    private Float flexGrow;
    private Float flexShrink;
    private Dimension flexBasis;

    private JustifyContent justifyContent;
    private AlignContent alignContent;
    private AlignItems alignItems;
    private AlignSelf alignSelf;
    private AlignItems justifyItems;
    private AlignSelf justifySelf;

    private Float aspectRatio;

    // Grid
    private GridTemplate[]     gridTemplateColumns;
    private GridTemplate[]     gridTemplateRows;
    private String[][]         gridTemplateColumnNames;
    private String[][]         gridTemplateRowNames;
    private GridTemplateArea[] gridTemplateAreas;
    private GridTrack[]        gridAutoColumns;
    private GridTrack[]        gridAutoRows;
    private GridPlacement      gridColumnStart, gridColumnEnd;
    private GridPlacement      gridRowStart, gridRowEnd;
    private GridAutoFlow       gridAutoFlow;

    public Style() {}

    /** Alias for {@code new Style()} for readability. */
    public static Style builder() { return new Style(); }

    // --- Fluent setters ---

    public Style display(Display v) { this.display = v; return this; }
    public Style position(Position v) { this.position = v; return this; }
    public Style boxSizing(BoxSizing v) { this.boxSizing = v; return this; }

    public Style overflow(Overflow xy) { this.overflowX = xy; this.overflowY = xy; return this; }
    public Style overflow(Overflow x, Overflow y) { this.overflowX = x; this.overflowY = y; return this; }
    public Style scrollbarWidth(float value) { this.scrollbarWidth = value; return this; }

    public Style size(Dimension width, Dimension height) {
        this.sizeW = width; this.sizeH = height; return this;
    }
    public Style minSize(Dimension width, Dimension height) {
        this.minW = width; this.minH = height; return this;
    }
    public Style maxSize(Dimension width, Dimension height) {
        this.maxW = width; this.maxH = height; return this;
    }

    public Style margin(LengthPercentageAuto all) { return margin(all, all, all, all); }
    public Style margin(LengthPercentageAuto left, LengthPercentageAuto right,
                        LengthPercentageAuto top, LengthPercentageAuto bottom) {
        this.marginL = left; this.marginR = right; this.marginT = top; this.marginB = bottom;
        return this;
    }

    public Style inset(LengthPercentageAuto all) { return inset(all, all, all, all); }
    public Style inset(LengthPercentageAuto left, LengthPercentageAuto right,
                       LengthPercentageAuto top, LengthPercentageAuto bottom) {
        this.insetL = left; this.insetR = right; this.insetT = top; this.insetB = bottom;
        return this;
    }

    public Style padding(LengthPercentage all) { return padding(all, all, all, all); }
    public Style padding(LengthPercentage left, LengthPercentage right,
                         LengthPercentage top, LengthPercentage bottom) {
        this.paddingL = left; this.paddingR = right; this.paddingT = top; this.paddingB = bottom;
        return this;
    }

    public Style border(LengthPercentage all) { return border(all, all, all, all); }
    public Style border(LengthPercentage left, LengthPercentage right,
                        LengthPercentage top, LengthPercentage bottom) {
        this.borderL = left; this.borderR = right; this.borderT = top; this.borderB = bottom;
        return this;
    }

    public Style gap(LengthPercentage both) { this.gapW = both; this.gapH = both; return this; }
    public Style gap(LengthPercentage columnGap, LengthPercentage rowGap) {
        this.gapW = columnGap; this.gapH = rowGap; return this;
    }

    public Style flexDirection(FlexDirection v) { this.flexDirection = v; return this; }
    public Style flexWrap(FlexWrap v) { this.flexWrap = v; return this; }
    public Style flexGrow(float v) { this.flexGrow = v; return this; }
    public Style flexShrink(float v) { this.flexShrink = v; return this; }
    public Style flexBasis(Dimension v) { this.flexBasis = v; return this; }

    public Style justifyContent(JustifyContent v) { this.justifyContent = v; return this; }
    public Style alignContent(AlignContent v) { this.alignContent = v; return this; }
    public Style alignItems(AlignItems v) { this.alignItems = v; return this; }
    public Style alignSelf(AlignSelf v) { this.alignSelf = v; return this; }
    public Style justifyItems(AlignItems v) { this.justifyItems = v; return this; }
    public Style justifySelf(AlignSelf v) { this.justifySelf = v; return this; }

    /** Width / height ratio, where the width is along the item's main axis. */
    public Style aspectRatio(float value) { this.aspectRatio = value; return this; }

    // Grid

    /** Flat list of tracks — equivalent to wrapping each in {@link GridTemplate#track}. */
    public Style gridTemplateColumns(GridTrack... tracks) {
        this.gridTemplateColumns = wrapTracks(tracks);
        return this;
    }
    public Style gridTemplateRows(GridTrack... tracks) {
        this.gridTemplateRows = wrapTracks(tracks);
        return this;
    }
    /** Full template supporting {@code repeat()} via {@link GridTemplate#repeat}, etc. */
    public Style gridTemplateColumns(GridTemplate... components) {
        this.gridTemplateColumns = components;
        return this;
    }
    public Style gridTemplateRows(GridTemplate... components) {
        this.gridTemplateRows = components;
        return this;
    }
    public Style gridAutoColumns(GridTrack... tracks) { this.gridAutoColumns = tracks; return this; }
    public Style gridAutoRows(GridTrack... tracks)    { this.gridAutoRows = tracks; return this; }

    /**
     * Line names for {@code grid-template-columns}. One inner array per line
     * position: slot {@code i} is the set of names at the line <em>before</em>
     * track {@code i}, and the final slot is the line after the last track.
     * Inner arrays may be {@code null} or empty for unnamed lines.
     */
    public Style gridTemplateColumnNames(String[]... namesPerLine) {
        this.gridTemplateColumnNames = namesPerLine;
        return this;
    }
    public Style gridTemplateRowNames(String[]... namesPerLine) {
        this.gridTemplateRowNames = namesPerLine;
        return this;
    }

    /** Declare named rectangular {@code grid-template-areas}. */
    public Style gridTemplateAreas(GridTemplateArea... areas) {
        this.gridTemplateAreas = areas;
        return this;
    }

    private static GridTemplate[] wrapTracks(GridTrack[] tracks) {
        if (tracks == null) return null;
        GridTemplate[] out = new GridTemplate[tracks.length];
        for (int i = 0; i < tracks.length; i++) out[i] = GridTemplate.track(tracks[i]);
        return out;
    }
    public Style gridColumn(GridPlacement start, GridPlacement end) {
        this.gridColumnStart = start; this.gridColumnEnd = end; return this;
    }
    public Style gridRow(GridPlacement start, GridPlacement end) {
        this.gridRowStart = start; this.gridRowEnd = end; return this;
    }
    public Style gridAutoFlow(GridAutoFlow flow) { this.gridAutoFlow = flow; return this; }

    // --- Fluent getters -----------------------------------------------------
    // Return the value previously supplied to the corresponding setter, or
    // {@code null} if the field was never set. Reading an unset value does not
    // materialize a native default — that only happens when the style is
    // passed to {@link TaffyTree}.

    public Display       getDisplay()        { return display; }
    public Position      getPosition()       { return position; }
    public BoxSizing     getBoxSizing()      { return boxSizing; }
    public Overflow      getOverflowX()      { return overflowX; }
    public Overflow      getOverflowY()      { return overflowY; }
    public Float         getScrollbarWidth() { return scrollbarWidth; }

    public Dimension getWidth()     { return sizeW; }
    public Dimension getHeight()    { return sizeH; }
    public Dimension getMinWidth()  { return minW; }
    public Dimension getMinHeight() { return minH; }
    public Dimension getMaxWidth()  { return maxW; }
    public Dimension getMaxHeight() { return maxH; }

    public LengthPercentageAuto getMarginLeft()   { return marginL; }
    public LengthPercentageAuto getMarginRight()  { return marginR; }
    public LengthPercentageAuto getMarginTop()    { return marginT; }
    public LengthPercentageAuto getMarginBottom() { return marginB; }

    public LengthPercentageAuto getInsetLeft()   { return insetL; }
    public LengthPercentageAuto getInsetRight()  { return insetR; }
    public LengthPercentageAuto getInsetTop()    { return insetT; }
    public LengthPercentageAuto getInsetBottom() { return insetB; }

    public LengthPercentage getPaddingLeft()   { return paddingL; }
    public LengthPercentage getPaddingRight()  { return paddingR; }
    public LengthPercentage getPaddingTop()    { return paddingT; }
    public LengthPercentage getPaddingBottom() { return paddingB; }

    public LengthPercentage getBorderLeft()   { return borderL; }
    public LengthPercentage getBorderRight()  { return borderR; }
    public LengthPercentage getBorderTop()    { return borderT; }
    public LengthPercentage getBorderBottom() { return borderB; }

    public LengthPercentage getColumnGap() { return gapW; }
    public LengthPercentage getRowGap()    { return gapH; }

    public FlexDirection getFlexDirection() { return flexDirection; }
    public FlexWrap      getFlexWrap()      { return flexWrap; }
    public Float         getFlexGrow()      { return flexGrow; }
    public Float         getFlexShrink()    { return flexShrink; }
    public Dimension     getFlexBasis()     { return flexBasis; }

    public JustifyContent getJustifyContent() { return justifyContent; }
    public AlignContent   getAlignContent()   { return alignContent; }
    public AlignItems     getAlignItems()     { return alignItems; }
    public AlignSelf      getAlignSelf()      { return alignSelf; }
    public AlignItems     getJustifyItems()   { return justifyItems; }
    public AlignSelf      getJustifySelf()    { return justifySelf; }

    public Float getAspectRatio() { return aspectRatio; }

    public GridTemplate[] getGridTemplateColumns() {
        return gridTemplateColumns == null ? null : gridTemplateColumns.clone();
    }
    public GridTemplate[] getGridTemplateRows() {
        return gridTemplateRows == null ? null : gridTemplateRows.clone();
    }
    public GridTrack[] getGridAutoColumns() {
        return gridAutoColumns == null ? null : gridAutoColumns.clone();
    }
    public GridTrack[] getGridAutoRows() {
        return gridAutoRows == null ? null : gridAutoRows.clone();
    }
    public GridPlacement getGridColumnStart() { return gridColumnStart; }
    public GridPlacement getGridColumnEnd()   { return gridColumnEnd; }
    public GridPlacement getGridRowStart()    { return gridRowStart; }
    public GridPlacement getGridRowEnd()      { return gridRowEnd; }
    public GridAutoFlow  getGridAutoFlow()    { return gridAutoFlow; }

    public String[][] getGridTemplateColumnNames() { return cloneNames(gridTemplateColumnNames); }
    public String[][] getGridTemplateRowNames()    { return cloneNames(gridTemplateRowNames); }
    public GridTemplateArea[] getGridTemplateAreas() {
        return gridTemplateAreas == null ? null : gridTemplateAreas.clone();
    }

    private static String[][] cloneNames(String[][] src) {
        if (src == null) return null;
        String[][] out = new String[src.length][];
        for (int i = 0; i < src.length; i++) {
            out[i] = src[i] == null ? null : src[i].clone();
        }
        return out;
    }

    // --- Internals ----------------------------------------------------------

    /**
     * Materialize this style into a fresh native Style handle. The caller owns
     * the handle and must eventually call {@link NativeBridge#styleFree(long)}.
     */
    long toNative() {
        long h = NativeBridge.styleNew();
        try {
            if (display != null) NativeBridge.styleSetDisplay(h, display.tag());
            if (position != null) NativeBridge.styleSetPosition(h, position.tag());
            if (boxSizing != null) NativeBridge.styleSetBoxSizing(h, boxSizing.tag());
            if (overflowX != null || overflowY != null) {
                int xt = overflowX != null ? overflowX.tag() : Overflow.VISIBLE.tag();
                int yt = overflowY != null ? overflowY.tag() : Overflow.VISIBLE.tag();
                NativeBridge.styleSetOverflow(h, xt, yt);
            }
            if (scrollbarWidth != null) NativeBridge.styleSetScrollbarWidth(h, scrollbarWidth);

            if (sizeW != null || sizeH != null) {
                Dimension w = sizeW != null ? sizeW : Dimension.AUTO;
                Dimension hD = sizeH != null ? sizeH : Dimension.AUTO;
                NativeBridge.styleSetSize(h, w.tag(), w.value(), hD.tag(), hD.value());
            }
            if (minW != null || minH != null) {
                Dimension w = minW != null ? minW : Dimension.AUTO;
                Dimension hD = minH != null ? minH : Dimension.AUTO;
                NativeBridge.styleSetMinSize(h, w.tag(), w.value(), hD.tag(), hD.value());
            }
            if (maxW != null || maxH != null) {
                Dimension w = maxW != null ? maxW : Dimension.AUTO;
                Dimension hD = maxH != null ? maxH : Dimension.AUTO;
                NativeBridge.styleSetMaxSize(h, w.tag(), w.value(), hD.tag(), hD.value());
            }

            if (marginL != null) applyRectLpa(NativeBridgeSetter.MARGIN, h, marginL, marginR, marginT, marginB);
            if (insetL != null)  applyRectLpa(NativeBridgeSetter.INSET,  h, insetL, insetR, insetT, insetB);
            if (paddingL != null) applyRectLp(NativeBridgeSetter.PADDING, h, paddingL, paddingR, paddingT, paddingB);
            if (borderL  != null) applyRectLp(NativeBridgeSetter.BORDER,  h, borderL,  borderR,  borderT,  borderB);

            if (gapW != null || gapH != null) {
                LengthPercentage w = gapW != null ? gapW : LengthPercentage.ZERO;
                LengthPercentage hD = gapH != null ? gapH : LengthPercentage.ZERO;
                NativeBridge.styleSetGap(h, w.tag(), w.value(), hD.tag(), hD.value());
            }

            if (flexDirection != null) NativeBridge.styleSetFlexDirection(h, flexDirection.tag());
            if (flexWrap != null)      NativeBridge.styleSetFlexWrap(h, flexWrap.tag());
            if (flexGrow != null)      NativeBridge.styleSetFlexGrow(h, flexGrow);
            if (flexShrink != null)    NativeBridge.styleSetFlexShrink(h, flexShrink);
            if (flexBasis != null)     NativeBridge.styleSetFlexBasis(h, flexBasis.tag(), flexBasis.value());

            if (justifyContent != null) NativeBridge.styleSetJustifyContent(h, justifyContent.tag());
            if (alignContent != null)   NativeBridge.styleSetAlignContent(h, alignContent.tag());
            if (alignItems != null)     NativeBridge.styleSetAlignItems(h, alignItems.tag());
            if (alignSelf != null)      NativeBridge.styleSetAlignSelf(h, alignSelf.tag());
            if (justifyItems != null)   NativeBridge.styleSetJustifyItems(h, justifyItems.tag());
            if (justifySelf != null)    NativeBridge.styleSetJustifySelf(h, justifySelf.tag());

            if (aspectRatio != null) NativeBridge.styleSetAspectRatio(h, aspectRatio);

            if (gridTemplateColumns != null) {
                GridTemplateBuffers b = GridTemplateBuffers.pack(gridTemplateColumns);
                NativeBridge.styleSetGridTemplateColumns(h, b.headers, b.minTags, b.minVals, b.maxTags, b.maxVals);
            }
            if (gridTemplateRows != null) {
                GridTemplateBuffers b = GridTemplateBuffers.pack(gridTemplateRows);
                NativeBridge.styleSetGridTemplateRows(h, b.headers, b.minTags, b.minVals, b.maxTags, b.maxVals);
            }
            if (gridAutoColumns != null) {
                GridTrackBuffers b = GridTrackBuffers.pack(gridAutoColumns);
                NativeBridge.styleSetGridAutoColumns(h, b.minTags, b.minVals, b.maxTags, b.maxVals);
            }
            if (gridAutoRows != null) {
                GridTrackBuffers b = GridTrackBuffers.pack(gridAutoRows);
                NativeBridge.styleSetGridAutoRows(h, b.minTags, b.minVals, b.maxTags, b.maxVals);
            }
            if (gridColumnStart != null || gridColumnEnd != null) {
                GridPlacement s = gridColumnStart != null ? gridColumnStart : GridPlacement.AUTO;
                GridPlacement e = gridColumnEnd   != null ? gridColumnEnd   : GridPlacement.AUTO;
                NativeBridge.styleSetGridColumn(h,
                        s.tag(), s.value(), s.name(),
                        e.tag(), e.value(), e.name());
            }
            if (gridRowStart != null || gridRowEnd != null) {
                GridPlacement s = gridRowStart != null ? gridRowStart : GridPlacement.AUTO;
                GridPlacement e = gridRowEnd   != null ? gridRowEnd   : GridPlacement.AUTO;
                NativeBridge.styleSetGridRow(h,
                        s.tag(), s.value(), s.name(),
                        e.tag(), e.value(), e.name());
            }
            if (gridAutoFlow != null) NativeBridge.styleSetGridAutoFlow(h, gridAutoFlow.tag());

            if (gridTemplateColumnNames != null) {
                LineNameBuffers b = LineNameBuffers.pack(gridTemplateColumnNames);
                NativeBridge.styleSetGridTemplateColumnNames(h, b.counts, b.flatNames);
            }
            if (gridTemplateRowNames != null) {
                LineNameBuffers b = LineNameBuffers.pack(gridTemplateRowNames);
                NativeBridge.styleSetGridTemplateRowNames(h, b.counts, b.flatNames);
            }
            if (gridTemplateAreas != null) {
                AreaBuffers b = AreaBuffers.pack(gridTemplateAreas);
                NativeBridge.styleSetGridTemplateAreas(h, b.names, b.rowStarts, b.rowEnds, b.colStarts, b.colEnds);
            }

            return h;
        } catch (RuntimeException | Error t) {
            NativeBridge.styleFree(h);
            throw t;
        }
    }

    /**
     * Parallel packed arrays for a grid-template list. Each component
     * contributes 3 ints to {@code headers} — {@code [kind, repKind, repCount]}
     * — plus 1 more int for its track count, for a total of 4 per component.
     * The track arrays are flat across every component.
     */
    private static final class GridTemplateBuffers {
        final int[]   headers; // 4 ints per component
        final int[]   minTags;
        final float[] minVals;
        final int[]   maxTags;
        final float[] maxVals;

        private GridTemplateBuffers(int comps, int totalTracks) {
            this.headers = new int[4 * comps];
            this.minTags = new int[totalTracks];
            this.minVals = new float[totalTracks];
            this.maxTags = new int[totalTracks];
            this.maxVals = new float[totalTracks];
        }

        static GridTemplateBuffers pack(GridTemplate[] comps) {
            int total = 0;
            for (GridTemplate c : comps) total += c.tracks().length;

            GridTemplateBuffers b = new GridTemplateBuffers(comps.length, total);
            int ti = 0;
            for (int i = 0; i < comps.length; i++) {
                GridTemplate c = comps[i];
                b.headers[4 * i]     = c.kind();
                b.headers[4 * i + 1] = c.repKind();
                b.headers[4 * i + 2] = c.repCount();
                b.headers[4 * i + 3] = c.tracks().length;
                for (GridTrack t : c.tracks()) {
                    b.minTags[ti] = t.minTag();
                    b.minVals[ti] = t.minValue();
                    b.maxTags[ti] = t.maxTag();
                    b.maxVals[ti] = t.maxValue();
                    ti++;
                }
            }
            return b;
        }
    }

    /** Counts + flattened names for a nested {@code String[][]}. */
    private static final class LineNameBuffers {
        final int[]    counts;
        final String[] flatNames;

        private LineNameBuffers(int[] counts, String[] flatNames) {
            this.counts = counts;
            this.flatNames = flatNames;
        }

        static LineNameBuffers pack(String[][] nested) {
            int total = 0;
            for (String[] inner : nested) {
                if (inner != null) total += inner.length;
            }
            int[] counts = new int[nested.length];
            String[] flat = new String[total];
            int fi = 0;
            for (int i = 0; i < nested.length; i++) {
                String[] inner = nested[i];
                if (inner == null) { counts[i] = 0; continue; }
                counts[i] = inner.length;
                for (String name : inner) {
                    flat[fi++] = name;
                }
            }
            return new LineNameBuffers(counts, flat);
        }
    }

    private static final class AreaBuffers {
        final String[] names;
        final int[]    rowStarts;
        final int[]    rowEnds;
        final int[]    colStarts;
        final int[]    colEnds;

        private AreaBuffers(int n) {
            this.names     = new String[n];
            this.rowStarts = new int[n];
            this.rowEnds   = new int[n];
            this.colStarts = new int[n];
            this.colEnds   = new int[n];
        }

        static AreaBuffers pack(GridTemplateArea[] areas) {
            AreaBuffers b = new AreaBuffers(areas.length);
            for (int i = 0; i < areas.length; i++) {
                GridTemplateArea a = areas[i];
                b.names[i]     = a.name();
                b.rowStarts[i] = a.rowStart();
                b.rowEnds[i]   = a.rowEnd();
                b.colStarts[i] = a.columnStart();
                b.colEnds[i]   = a.columnEnd();
            }
            return b;
        }
    }

    /** Parallel packed arrays for a grid-track list. */
    private static final class GridTrackBuffers {
        final int[] minTags;
        final float[] minVals;
        final int[] maxTags;
        final float[] maxVals;

        private GridTrackBuffers(int n) {
            this.minTags = new int[n];
            this.minVals = new float[n];
            this.maxTags = new int[n];
            this.maxVals = new float[n];
        }

        static GridTrackBuffers pack(GridTrack[] tracks) {
            GridTrackBuffers b = new GridTrackBuffers(tracks.length);
            for (int i = 0; i < tracks.length; i++) {
                b.minTags[i] = tracks[i].minTag();
                b.minVals[i] = tracks[i].minValue();
                b.maxTags[i] = tracks[i].maxTag();
                b.maxVals[i] = tracks[i].maxValue();
            }
            return b;
        }
    }

    private enum NativeBridgeSetter { MARGIN, INSET, PADDING, BORDER }

    private static void applyRectLpa(NativeBridgeSetter which, long h,
                                     LengthPercentageAuto l, LengthPercentageAuto r,
                                     LengthPercentageAuto t, LengthPercentageAuto b) {
        LengthPercentageAuto ll = l != null ? l : LengthPercentageAuto.ZERO;
        LengthPercentageAuto rr = r != null ? r : LengthPercentageAuto.ZERO;
        LengthPercentageAuto tt = t != null ? t : LengthPercentageAuto.ZERO;
        LengthPercentageAuto bb = b != null ? b : LengthPercentageAuto.ZERO;
        switch (which) {
            case MARGIN:
                NativeBridge.styleSetMargin(h,
                        ll.tag(), ll.value(), rr.tag(), rr.value(),
                        tt.tag(), tt.value(), bb.tag(), bb.value());
                break;
            case INSET:
                NativeBridge.styleSetInset(h,
                        ll.tag(), ll.value(), rr.tag(), rr.value(),
                        tt.tag(), tt.value(), bb.tag(), bb.value());
                break;
            default:
                throw new IllegalStateException();
        }
    }

    private static void applyRectLp(NativeBridgeSetter which, long h,
                                    LengthPercentage l, LengthPercentage r,
                                    LengthPercentage t, LengthPercentage b) {
        LengthPercentage ll = l != null ? l : LengthPercentage.ZERO;
        LengthPercentage rr = r != null ? r : LengthPercentage.ZERO;
        LengthPercentage tt = t != null ? t : LengthPercentage.ZERO;
        LengthPercentage bb = b != null ? b : LengthPercentage.ZERO;
        switch (which) {
            case PADDING:
                NativeBridge.styleSetPadding(h,
                        ll.tag(), ll.value(), rr.tag(), rr.value(),
                        tt.tag(), tt.value(), bb.tag(), bb.value());
                break;
            case BORDER:
                NativeBridge.styleSetBorder(h,
                        ll.tag(), ll.value(), rr.tag(), rr.value(),
                        tt.tag(), tt.value(), bb.tag(), bb.value());
                break;
            default:
                throw new IllegalStateException();
        }
    }
}
