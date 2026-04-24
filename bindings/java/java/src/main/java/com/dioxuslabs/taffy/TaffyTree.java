package com.dioxuslabs.taffy;

/**
 * A tree of Taffy nodes. Each node is identified by a {@code long} id that is
 * owned by (and only valid for) this particular tree.
 *
 * <p>This class owns a native handle; call {@link #close()} (or use
 * try-with-resources) to release it. The handle is also freed if the JVM shuts
 * down cleanly, but relying on finalization is not recommended.
 */
public final class TaffyTree implements AutoCloseable {

    private long handle;

    public TaffyTree() {
        this.handle = NativeBridge.treeNew();
    }

    /** Whether Taffy should round layout values to integers. Enabled by default. */
    public TaffyTree roundingEnabled(boolean enabled) {
        checkOpen();
        NativeBridge.treeEnableRounding(handle, enabled ? 1 : 0);
        return this;
    }

    public long newLeaf(Style style) {
        checkOpen();
        long styleHandle = style.toNative();
        try {
            return NativeBridge.treeNewLeaf(handle, styleHandle);
        } finally {
            NativeBridge.styleFree(styleHandle);
        }
    }

    /**
     * Create a leaf whose intrinsic size is computed by {@code measure}.
     * Used for text, images, or any other content whose size depends on
     * the available space rather than the style alone.
     */
    public long newLeafWithMeasure(Style style, MeasureFunction measure) {
        checkOpen();
        java.util.Objects.requireNonNull(measure, "measure");
        long styleHandle = style.toNative();
        try {
            return NativeBridge.treeNewLeafWithMeasure(handle, styleHandle, measure);
        } finally {
            NativeBridge.styleFree(styleHandle);
        }
    }

    /**
     * Install or replace the measure function for {@code node}. Passing
     * {@code null} clears it and the node will measure as zero-sized.
     */
    public void setMeasure(long node, MeasureFunction measure) {
        checkOpen();
        NativeBridge.treeSetMeasure(handle, node, measure);
    }

    public long newWithChildren(Style style, long... children) {
        checkOpen();
        long styleHandle = style.toNative();
        try {
            return NativeBridge.treeNewWithChildren(handle, styleHandle, children);
        } finally {
            NativeBridge.styleFree(styleHandle);
        }
    }

    public void setStyle(long node, Style style) {
        checkOpen();
        long styleHandle = style.toNative();
        try {
            NativeBridge.treeSetStyle(handle, node, styleHandle);
        } finally {
            NativeBridge.styleFree(styleHandle);
        }
    }

    public void addChild(long parent, long child) {
        checkOpen();
        NativeBridge.treeAddChild(handle, parent, child);
    }

    public void removeChild(long parent, long child) {
        checkOpen();
        NativeBridge.treeRemoveChild(handle, parent, child);
    }

    public int childCount(long node) {
        checkOpen();
        return NativeBridge.treeChildCount(handle, node);
    }

    public void markDirty(long node) {
        checkOpen();
        NativeBridge.treeMarkDirty(handle, node);
    }

    public void remove(long node) {
        checkOpen();
        NativeBridge.treeRemove(handle, node);
    }

    public void computeLayout(long root, AvailableSpace width, AvailableSpace height) {
        checkOpen();
        NativeBridge.treeComputeLayout(handle, root,
                width.tag(), width.value(),
                height.tag(), height.value());
    }

    /** Convenience: compute layout with a definite viewport size in pixels. */
    public void computeLayout(long root, float widthPx, float heightPx) {
        computeLayout(root, AvailableSpace.definite(widthPx), AvailableSpace.definite(heightPx));
    }

    public Layout layout(long node) {
        checkOpen();
        float[] buf = new float[19];
        NativeBridge.treeLayout(handle, node, buf);
        return new Layout(buf);
    }

    /**
     * Snapshot a node's current {@link Style}. Grid-track lists
     * ({@code grid-template-*}, {@code grid-auto-*}) are <em>not</em>
     * included in the snapshot — the returned {@code Style} carries only
     * the scalar style properties.
     */
    public Style getStyle(long node) {
        checkOpen();
        int[] ints = new int[NativeBridge.SNAPSHOT_INTS];
        float[] floats = new float[NativeBridge.SNAPSHOT_FLOATS];
        NativeBridge.treeGetStyle(handle, node, ints, floats);

        Style s = Style.builder()
                .display(Display.values()[ints[0]])
                .position(Position.values()[ints[1]])
                .boxSizing(BoxSizing.values()[ints[2]])
                .overflow(Overflow.values()[ints[3]], Overflow.values()[ints[4]])
                .flexDirection(FlexDirection.values()[ints[5]])
                .flexWrap(FlexWrap.values()[ints[6]])
                .gridAutoFlow(GridAutoFlow.values()[ints[13]]);

        // Align/justify are optional — tag value 0 means "unset".
        if (ints[7]  != 0) s.justifyContent(JustifyContent.values()[ints[7]  - 1]);
        if (ints[8]  != 0) s.alignContent(AlignContent.values()[ints[8]  - 1]);
        if (ints[9]  != 0) s.alignItems(AlignItems.values()[ints[9]  - 1]);
        if (ints[10] != 0) s.alignSelf(AlignSelf.values()[ints[10] - 1]);
        if (ints[11] != 0) s.justifyItems(AlignItems.values()[ints[11] - 1]);
        if (ints[12] != 0) s.justifySelf(AlignSelf.values()[ints[12] - 1]);

        s.flexGrow(floats[1]).flexShrink(floats[2]);
        if (!Float.isNaN(floats[0])) s.scrollbarWidth(floats[0]);
        if (!Float.isNaN(floats[3])) s.aspectRatio(floats[3]);

        s.size(Dimension.fromTag(ints[14], floats[4]), Dimension.fromTag(ints[15], floats[5]))
         .minSize(Dimension.fromTag(ints[16], floats[6]), Dimension.fromTag(ints[17], floats[7]))
         .maxSize(Dimension.fromTag(ints[18], floats[8]), Dimension.fromTag(ints[19], floats[9]))
         .flexBasis(Dimension.fromTag(ints[20], floats[10]));

        s.margin(LengthPercentageAuto.fromTag(ints[21], floats[11]),
                 LengthPercentageAuto.fromTag(ints[22], floats[12]),
                 LengthPercentageAuto.fromTag(ints[23], floats[13]),
                 LengthPercentageAuto.fromTag(ints[24], floats[14]))
         .inset(LengthPercentageAuto.fromTag(ints[25], floats[15]),
                LengthPercentageAuto.fromTag(ints[26], floats[16]),
                LengthPercentageAuto.fromTag(ints[27], floats[17]),
                LengthPercentageAuto.fromTag(ints[28], floats[18]));

        s.padding(LengthPercentage.fromTag(ints[29], floats[19]),
                  LengthPercentage.fromTag(ints[30], floats[20]),
                  LengthPercentage.fromTag(ints[31], floats[21]),
                  LengthPercentage.fromTag(ints[32], floats[22]))
         .border(LengthPercentage.fromTag(ints[33], floats[23]),
                 LengthPercentage.fromTag(ints[34], floats[24]),
                 LengthPercentage.fromTag(ints[35], floats[25]),
                 LengthPercentage.fromTag(ints[36], floats[26]))
         .gap(LengthPercentage.fromTag(ints[37], floats[27]),
              LengthPercentage.fromTag(ints[38], floats[28]));

        return s;
    }

    @Override
    public void close() {
        if (handle != 0L) {
            NativeBridge.treeFree(handle);
            handle = 0L;
        }
    }

    private void checkOpen() {
        if (handle == 0L) throw new IllegalStateException("TaffyTree has been closed");
    }
}
