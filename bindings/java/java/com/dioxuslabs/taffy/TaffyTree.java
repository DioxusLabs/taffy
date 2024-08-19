package com.dioxuslabs.taffy;

import com.dioxuslabs.taffy.style.TaffyStyle;

public class TaffyTree {
    public final long ptr;

    public TaffyTree() {
        this.ptr = newTaffyTree();
    }

    public long newLeaf(TaffyStyle style) {
        return newLeaf(this.ptr, style);
    }

    public int childCount(long nodeId) {
        return childCount(this.ptr, nodeId);
    }

    private static native long newTaffyTree();
    private static native long newLeaf(long pointer, TaffyStyle style);
    private static native int childCount(long pointer, long nodeId);
}
