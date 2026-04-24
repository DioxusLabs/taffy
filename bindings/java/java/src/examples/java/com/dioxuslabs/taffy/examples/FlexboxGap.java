package com.dioxuslabs.taffy.examples;

import com.dioxuslabs.taffy.AvailableSpace;
import com.dioxuslabs.taffy.Dimension;
import com.dioxuslabs.taffy.LengthPercentage;
import com.dioxuslabs.taffy.Style;
import com.dioxuslabs.taffy.TaffyTree;

/**
 * Java port of {@code examples/flexbox_gap.rs}.
 *
 * <p>Creates three 20x20 children, evenly spaced 10px apart. The container
 * therefore computes as 80x20.
 */
public final class FlexboxGap {

    private FlexboxGap() {}

    public static void main(String[] args) {
        try (TaffyTree tree = new TaffyTree()) {
            Style childStyle = Style.builder()
                    .size(Dimension.length(20f), Dimension.length(20f));

            long child0 = tree.newLeaf(childStyle);
            long child1 = tree.newLeaf(childStyle);
            long child2 = tree.newLeaf(childStyle);

            long root = tree.newWithChildren(Style.builder()
                            .gap(LengthPercentage.length(10f), LengthPercentage.ZERO),
                    child0, child1, child2);

            tree.computeLayout(root, AvailableSpace.MAX_CONTENT, AvailableSpace.MAX_CONTENT);

            System.out.println("root:   " + tree.layout(root));
            System.out.println("child0: " + tree.layout(child0));
            System.out.println("child1: " + tree.layout(child1));
            System.out.println("child2: " + tree.layout(child2));
        }
    }
}
