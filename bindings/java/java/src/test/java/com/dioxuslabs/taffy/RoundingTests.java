package com.dioxuslabs.taffy;

import com.dioxuslabs.taffy.geom.Size;
import com.dioxuslabs.taffy.geom.measure.Dimension;
import com.dioxuslabs.taffy.style.AlignContent;
import com.dioxuslabs.taffy.style.Style;
import com.dioxuslabs.taffy.tree.Layout;
import org.junit.jupiter.api.Test;

import java.util.Arrays;

import static com.dioxuslabs.taffy.TaffyTests.assertEquals;

public class RoundingTests {
    static {
        System.loadLibrary("jtaffy");
    }

    @Test
    public void roundingDoesntLeaveGaps() {
        // First create an instance of TaffyTree
        TaffyTree taffy = new TaffyTree();

        Size<Dimension> wSquare = Size.lengthDimension(100.3f, 100.3f);

        long childA = taffy.newLeaf(Style.builder().size(wSquare));
        long childB = taffy.newLeaf(Style.builder().size(wSquare));

        long rootNode = taffy.newWithChildren(
                Style.builder()
                        .size(Size.length(Dimension.class, 963.3333f, 1000f))
                        .justifyContent(AlignContent.CENTER),
                Arrays.asList(childA, childB)
        );

        taffy.computeLayout(rootNode, Size.maxContentAvailableSize());
        taffy.printTree(rootNode);

        Layout layoutA = taffy.layout(childA);
        Layout layoutB = taffy.layout(childB);

        assertEquals(layoutA.location().x() + layoutA.size().width(), layoutB.location().x());
    }
}
