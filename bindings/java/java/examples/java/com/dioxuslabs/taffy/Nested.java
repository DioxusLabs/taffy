package com.dioxuslabs.taffy;

import com.dioxuslabs.taffy.geom.Size;
import com.dioxuslabs.taffy.geom.measure.Dimension;
import com.dioxuslabs.taffy.style.Style;

import java.util.List;

/**
 * Equivalent of taffy/examples/nested.rs
 */
public class Nested {
    public static void main(String[] args) {
        TaffyTree taffy = new TaffyTree();

        // left
        long childT1 = taffy.newLeaf(
                Style.builder().size(Size.lengthDimension(5, 5))
        );

        long div1 = taffy.newWithChildren(
                Style.builder().size(new Size<>(Dimension.percent(0.5f), Dimension.percent(1f))),
                List.of(childT1)
        );

        // right
        long childT2 = taffy.newLeaf(
                Style.builder().size(Size.lengthDimension(5, 5))
        );

        long div2 = taffy.newWithChildren(
                Style.builder().size(new Size<>(Dimension.percent(0.5f), Dimension.percent(1f))),
                List.of(childT2)
        );

        long container = taffy.newWithChildren(
                Style.builder().size(Size.percent(Dimension.class, 1, 1)),
                List.of(div1, div2)
        );

        taffy.computeLayout(
                container,
                Size.definiteAvailableSize(100, 100)
        );
        System.out.println("node: " + taffy.layout(container).toString());

        System.out.println("div1: " + taffy.layout(div1).toString());
        System.out.println("div2: " + taffy.layout(div2).toString());

        System.out.println("child1: " + taffy.layout(childT1).toString());
        System.out.println("child: " + taffy.layout(childT2).toString());
    }
}
