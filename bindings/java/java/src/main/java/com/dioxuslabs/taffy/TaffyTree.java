package com.dioxuslabs.taffy;

import com.dioxuslabs.taffy.geom.Size;
import com.dioxuslabs.taffy.geom.measure.AvailableSpace;
import com.dioxuslabs.taffy.style.Style;
import com.dioxuslabs.taffy.tree.Layout;

import java.util.List;
import java.util.Optional;

public class TaffyTree {
    public final long ptr;

    public TaffyTree() {
        this.ptr = nvNewTaffyTree();
    }

    public TaffyTree(int capacity) {
        this.ptr = nvNewTaffyTreeWithCapacity(capacity);
    }

    public void enableRounding() {
        nvEnableRounding(this.ptr);
    }

    public void disableRounding() {
        nvDisableRounding(this.ptr);
    }

    public long newLeaf(Style style) {
        return nvNewLeaf(this.ptr, style);
    }

    public long newWithChildren(Style style, List<Long> children) {
        return nvNewWithChildren(this.ptr, style, children);
    }

    public int childCount(long nodeId) {
        return nvChildCount(this.ptr, nodeId);
    }

    public void clear() {
        nvClear(this.ptr);
    }

    public long remove(long nodeId) {
        return nvRemove(this.ptr, nodeId);
    }

    public void addChild(long parent, long child) {
        nvAddChild(this.ptr, parent, child);
    }

    public void insertChildAtIndex(long parent, int childIndex, int child) {
        nvInsertChildAtIndex(this.ptr, parent, childIndex, child);
    }

    public void setChildren(long parent, List<Long> children) {
        nvSetChildren(this.ptr, parent, children);
    }

    public long removeChild(long parent, long child) {
        return nvRemoveChild(this.ptr, parent, child);
    }

    public long removeChildAtIndex(long parent, int childIndex) {
        return nvRemoveChildAtIndex(this.ptr, parent, childIndex);
    }

    public long replaceChildAtIndex(long parent, int childIndex, long newChild) {
        return nvReplaceChildAtIndex(this.ptr, parent, childIndex, newChild);
    }

    public long childAtIndex(long parent, int childIndex) {
        return nvChildAtIndex(this.ptr, parent, childIndex);
    }

    /**
     * Returns the total number of nodes in the tree
     */
    public int totalNodeCount() {
        return nvTotalNodeCount(this.ptr);
    }

    public Optional<Long> parent(long nodeId) {
        throw new UnsupportedOperationException("Not implemented yet");
    }

    /**
     * Returns a list of children that belong to the parent node
     */
    public List<Long> children(long nodeId) {
        throw new UnsupportedOperationException("Not implemented yet");
    }

    public void setStyle(long node, Style style) {
        nvSetStyle(this.ptr, node, style);
    }

    public Style style(long node) {
        throw new UnsupportedOperationException("Not implemented yet");
    }

    public Layout layout(long node) {
        return nvLayout(this.ptr, node);
    }

    public void computeLayout(long node, Size<AvailableSpace> availableSize) {
        nvComputeLayout(this.ptr, node, availableSize);
    }

    public void printTree(long node) {
        nvPrintTree(this.ptr, node);
    }

    private static native long nvNewTaffyTree();

    private static native long nvNewTaffyTreeWithCapacity(int capacity);

    private static native void nvEnableRounding(long pointer);

    private static native void nvDisableRounding(long pointer);

    private static native long nvNewLeaf(long pointer, Style style);

    private static native long nvNewWithChildren(long pointer, Style style, List<Long> children);

    private static native int nvChildCount(long pointer, long nodeId);

    private static native void nvClear(long pointer);

    private static native long nvRemove(long pointer, long node);

    private static native void nvAddChild(long pointer, long parent, long child);

    private static native void nvInsertChildAtIndex(long pointer, long parent, int childIndex, long child);

    private static native void nvSetChildren(long pointer, long parent, List<Long> children);

    private static native long nvRemoveChild(long pointer, long parent, long child);

    private static native long nvRemoveChildAtIndex(long pointer, long parent, int childIndex);

    private static native long nvReplaceChildAtIndex(long pointer, long parent, int childIndex, long newChild);

    private static native long nvChildAtIndex(long pointer, long parent, int childIndex);

    private static native int nvTotalNodeCount(long pointer);

    private static native long nvParent(long pointer, long nodeId);

    private static native List<Long> nvChildren(long pointer, long nodeId);

    private static native void nvSetStyle(long pointer, long node, Style style);

    private static native Style nvStyle(long pointer, long node);

    private static native void nvComputeLayout(long pointer, long node, Size<AvailableSpace> availableSize);

    private static native void nvPrintTree(long pointer, long node);

    private static native Layout nvLayout(long pointer, long node);
}
