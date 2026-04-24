package com.dioxuslabs.taffy.examples;

import com.dioxuslabs.taffy.Dimension;
import com.dioxuslabs.taffy.Style;
import com.dioxuslabs.taffy.TaffyTree;

/** Java port of {@code examples/nested.rs}. */
public final class Nested {

    private Nested() {}

    public static void main(String[] args) {
        try (TaffyTree tree = new TaffyTree()) {
            // left column
            long leftChild = tree.newLeaf(Style.builder()
                    .size(Dimension.length(5f), Dimension.length(5f)));
            long div1 = tree.newWithChildren(Style.builder()
                            .size(Dimension.percent(0.5f), Dimension.percent(1f)),
                    leftChild);

            // right column
            long rightChild = tree.newLeaf(Style.builder()
                    .size(Dimension.length(5f), Dimension.length(5f)));
            long div2 = tree.newWithChildren(Style.builder()
                            .size(Dimension.percent(0.5f), Dimension.percent(1f)),
                    rightChild);

            long container = tree.newWithChildren(Style.builder()
                            .size(Dimension.percent(1f), Dimension.percent(1f)),
                    div1, div2);

            tree.computeLayout(container, 100f, 100f);

            System.out.println("container: " + tree.layout(container));
            System.out.println("div1:      " + tree.layout(div1));
            System.out.println("div2:      " + tree.layout(div2));
            System.out.println("leftChild:  " + tree.layout(leftChild));
            System.out.println("rightChild: " + tree.layout(rightChild));
        }
    }
}
