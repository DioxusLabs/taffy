package com.dioxuslabs.taffy.tree;

import com.dioxuslabs.taffy.geom.Point;
import com.dioxuslabs.taffy.geom.Rect;
import com.dioxuslabs.taffy.geom.Size;

/**
 * The final result of a layout algorithm for a single node.
 * @param order The relative ordering of the node
 *              <p>
 *              Nodes with a higher order should be rendered on top of those with a lower order. This is
 *              effectively a topological sort of each tree.
 * @param location The top-left corner of the node
 * @param size The width and height of the node
 * @param contentSize The width and height of the content inside the node. This may be larger than the size
 *                    of the node in the case of overflowing content and is useful for computing a "scroll
 *                    width/height" for scrollable nodes
 * @param scrollbarSize The size of the scrollbars in each dimension. If there is no scrollbar then the
 *                      size will be zero.
 * @param border The size of the borders of the node
 * @param padding The size of the padding of the node
 * @param margin The size of the margin of the node
 */
public record Layout(
        int order,
        Point<Float> location,
        Size<Float> size,
        Size<Float> contentSize,
        Size<Float> scrollbarSize,
        Rect<Float> border,
        Rect<Float> padding,
        Rect<Float> margin
) {
}
