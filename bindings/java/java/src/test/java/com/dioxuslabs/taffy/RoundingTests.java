package com.dioxuslabs.taffy;

import com.dioxuslabs.taffy.geom.TaffySize;
import com.dioxuslabs.taffy.geom.measure.TaffyDimension;
import com.dioxuslabs.taffy.style.TaffyAlignContent;
import com.dioxuslabs.taffy.style.TaffyStyle;
import com.dioxuslabs.taffy.tree.TaffyLayout;
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

        TaffySize<TaffyDimension> wSquare = TaffySize.lengthDimension(100.3f, 100.3f);

        long childA = taffy.newLeaf(TaffyStyle.builder().size(wSquare));
        long childB = taffy.newLeaf(TaffyStyle.builder().size(wSquare));

        long rootNode = taffy.newWithChildren(
                TaffyStyle.builder()
                        .size(TaffySize.length(TaffyDimension.class, 963.3333f, 1000f))
                        .justifyContent(TaffyAlignContent.CENTER),
                Arrays.asList(childA, childB)
        );

        taffy.computeLayout(rootNode, TaffySize.maxContentAvailableSize());

        TaffyLayout layoutA = taffy.layout(childA);
        TaffyLayout layoutB = taffy.layout(childB);

        assertEquals(layoutA.location().x() + layoutA.size().width(), layoutB.location().x());
    }
}
