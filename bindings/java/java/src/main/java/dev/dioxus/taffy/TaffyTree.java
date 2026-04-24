package dev.dioxus.taffy;

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
