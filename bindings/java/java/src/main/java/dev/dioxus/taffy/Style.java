package dev.dioxus.taffy;

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
    private GridTrack[] gridTemplateColumns;
    private GridTrack[] gridTemplateRows;
    private GridTrack[] gridAutoColumns;
    private GridTrack[] gridAutoRows;
    private GridPlacement gridColumnStart, gridColumnEnd;
    private GridPlacement gridRowStart, gridRowEnd;
    private GridAutoFlow gridAutoFlow;

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
    public Style gridTemplateColumns(GridTrack... tracks) { this.gridTemplateColumns = tracks; return this; }
    public Style gridTemplateRows(GridTrack... tracks)    { this.gridTemplateRows = tracks; return this; }
    public Style gridAutoColumns(GridTrack... tracks)     { this.gridAutoColumns = tracks; return this; }
    public Style gridAutoRows(GridTrack... tracks)        { this.gridAutoRows = tracks; return this; }
    public Style gridColumn(GridPlacement start, GridPlacement end) {
        this.gridColumnStart = start; this.gridColumnEnd = end; return this;
    }
    public Style gridRow(GridPlacement start, GridPlacement end) {
        this.gridRowStart = start; this.gridRowEnd = end; return this;
    }
    public Style gridAutoFlow(GridAutoFlow flow) { this.gridAutoFlow = flow; return this; }

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
                int[] tags = new int[gridTemplateColumns.length];
                float[] vals = new float[gridTemplateColumns.length];
                for (int i = 0; i < gridTemplateColumns.length; i++) {
                    tags[i] = gridTemplateColumns[i].tag();
                    vals[i] = gridTemplateColumns[i].value();
                }
                NativeBridge.styleSetGridTemplateColumns(h, tags, vals);
            }
            if (gridTemplateRows != null) {
                int[] tags = new int[gridTemplateRows.length];
                float[] vals = new float[gridTemplateRows.length];
                for (int i = 0; i < gridTemplateRows.length; i++) {
                    tags[i] = gridTemplateRows[i].tag();
                    vals[i] = gridTemplateRows[i].value();
                }
                NativeBridge.styleSetGridTemplateRows(h, tags, vals);
            }
            if (gridAutoColumns != null) {
                int[] tags = new int[gridAutoColumns.length];
                float[] vals = new float[gridAutoColumns.length];
                for (int i = 0; i < gridAutoColumns.length; i++) {
                    tags[i] = gridAutoColumns[i].tag();
                    vals[i] = gridAutoColumns[i].value();
                }
                NativeBridge.styleSetGridAutoColumns(h, tags, vals);
            }
            if (gridAutoRows != null) {
                int[] tags = new int[gridAutoRows.length];
                float[] vals = new float[gridAutoRows.length];
                for (int i = 0; i < gridAutoRows.length; i++) {
                    tags[i] = gridAutoRows[i].tag();
                    vals[i] = gridAutoRows[i].value();
                }
                NativeBridge.styleSetGridAutoRows(h, tags, vals);
            }
            if (gridColumnStart != null || gridColumnEnd != null) {
                GridPlacement s = gridColumnStart != null ? gridColumnStart : GridPlacement.AUTO;
                GridPlacement e = gridColumnEnd   != null ? gridColumnEnd   : GridPlacement.AUTO;
                NativeBridge.styleSetGridColumn(h, s.tag(), s.value(), e.tag(), e.value());
            }
            if (gridRowStart != null || gridRowEnd != null) {
                GridPlacement s = gridRowStart != null ? gridRowStart : GridPlacement.AUTO;
                GridPlacement e = gridRowEnd   != null ? gridRowEnd   : GridPlacement.AUTO;
                NativeBridge.styleSetGridRow(h, s.tag(), s.value(), e.tag(), e.value());
            }
            if (gridAutoFlow != null) NativeBridge.styleSetGridAutoFlow(h, gridAutoFlow.tag());

            return h;
        } catch (RuntimeException | Error t) {
            NativeBridge.styleFree(h);
            throw t;
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
