package com.dioxuslabs.taffy;

import com.dioxuslabs.taffy.enums.AlignContent;
import com.dioxuslabs.taffy.geom.Size;
import com.dioxuslabs.taffy.geom.measure.Dimension;
import com.dioxuslabs.taffy.style.Style;

import java.util.List;

/**
 * Equivalent of taffy/examples/basic.rs
 */
public class Basic {
    public static void main(String[] args) {
        TaffyTree taffy = new TaffyTree();

        long child = taffy.newLeaf(
                Style.builder().size(new Size<>(Dimension.percent(0.5f), Dimension.auto()))
        );

        long node = taffy.newWithChildren(
                Style.builder().size(Size.length(Dimension.class, 100, 100))
                        .justifyContent(AlignContent.CENTER),
                List.of(child)
        );

        System.out.println("Compute layout with 100x100 viewport:");
        taffy.computeLayout(
                node,
                Size.definiteAvailableSize(100, 100)
        );
        System.out.println("node: " + taffy.layout(node).toString());
        System.out.println("child: " + taffy.layout(child).toString());

        System.out.println("Compute layout with undefined (infinite) viewport:");
        taffy.computeLayout(node, Size.maxContentAvailableSize());
        System.out.println("node: " + taffy.layout(node).toString());
        System.out.println("child: " + taffy.layout(child).toString());
    }
}
