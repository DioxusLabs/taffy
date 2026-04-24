package com.dioxuslabs.taffy.examples;

import com.dioxuslabs.taffy.AvailableSpace;
import com.dioxuslabs.taffy.Dimension;
import com.dioxuslabs.taffy.JustifyContent;
import com.dioxuslabs.taffy.Style;
import com.dioxuslabs.taffy.TaffyTree;

/** Java port of {@code examples/basic.rs}. */
public final class Basic {

    private Basic() {}

    public static void main(String[] args) {
        try (TaffyTree tree = new TaffyTree()) {
            long child = tree.newLeaf(Style.builder()
                    .size(Dimension.percent(0.5f), Dimension.AUTO));

            long root = tree.newWithChildren(Style.builder()
                            .size(Dimension.length(100f), Dimension.length(100f))
                            .justifyContent(JustifyContent.CENTER),
                    child);

            System.out.println("Compute layout with 100x100 viewport:");
            tree.computeLayout(root, 100f, 100f);
            System.out.println("node:  " + tree.layout(root));
            System.out.println("child: " + tree.layout(child));

            System.out.println();
            System.out.println("Compute layout with undefined (MAX_CONTENT) viewport:");
            tree.computeLayout(root, AvailableSpace.MAX_CONTENT, AvailableSpace.MAX_CONTENT);
            System.out.println("node:  " + tree.layout(root));
            System.out.println("child: " + tree.layout(child));
        }
    }
}
