package com.dioxuslabs.taffy;

import com.dioxuslabs.taffy.geom.Size;
import com.dioxuslabs.taffy.geom.measure.LengthPercentage;
import com.dioxuslabs.taffy.style.Style;

import java.util.List;

/**
 * Equivalent of taffy/examples/flexbox_gap.rs
 */
public class FlexboxGap {
    public static void main(String[] args) {
        TaffyTree taffy = new TaffyTree();

        Style childStyle = Style.builder()
                .size(Size.lengthDimension(20, 20));
        long child0 = taffy.newLeaf(childStyle);
        long child1 = taffy.newLeaf(childStyle);
        long child2 = taffy.newLeaf(childStyle);

        long root = taffy.newWithChildren(
                Style.builder()
                        .gap(new Size<>(LengthPercentage.length(10), LengthPercentage.zero())),
                List.of(child0, child1, child2)
        );

        // Compute layout and print result
        taffy.computeLayout(root, Size.maxContentAvailableSize());
        taffy.printTree(root);
    }
}
