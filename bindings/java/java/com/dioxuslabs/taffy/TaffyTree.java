package com.dioxuslabs.taffy;

import com.dioxuslabs.taffy.geom.TaffySize;
import com.dioxuslabs.taffy.geom.measure.TaffyAvailableSpace;
import com.dioxuslabs.taffy.style.TaffyStyle;
import com.dioxuslabs.taffy.tree.TaffyLayout;

import java.util.List;

public class TaffyTree {
    public final long ptr;

    public TaffyTree() {
        this.ptr = newTaffyTree();
    }

    public long newLeaf(TaffyStyle style) {
        return newLeaf(this.ptr, style);
    }

    public long newWithChildren(TaffyStyle style, List<Long> children) {
        return newWithChildren(this.ptr, style, children);
    }

    public int childCount(long nodeId) {
        return childCount(this.ptr, nodeId);
    }

    public void computeLayout(long node, TaffySize<TaffyAvailableSpace> availableSize) {
        computeLayout(this.ptr, node, availableSize);
    }

    public TaffyLayout layout(long node) {
        return layout(this.ptr, node);
    }

    private static native long newTaffyTree();

    private static native long newLeaf(long pointer, TaffyStyle style);

    private static native long newWithChildren(long pointer, TaffyStyle style, List<Long> children);

    private static native int childCount(long pointer, long nodeId);

    private static native void computeLayout(long pointer, long node, TaffySize<TaffyAvailableSpace> availableSize);

    private static native TaffyLayout layout(long pointer, long node);
}
