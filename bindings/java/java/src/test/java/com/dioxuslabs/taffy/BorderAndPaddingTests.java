package com.dioxuslabs.taffy;

import com.dioxuslabs.taffy.geom.TaffyRect;
import com.dioxuslabs.taffy.geom.TaffySize;
import com.dioxuslabs.taffy.geom.measure.TaffyLengthPercentage;
import com.dioxuslabs.taffy.style.TaffyStyle;
import com.dioxuslabs.taffy.tree.TaffyLayout;
import org.junit.jupiter.api.Assertions;
import org.junit.jupiter.api.Test;

public class BorderAndPaddingTests {
    @Test
    public void verticalBorderAndPaddingPercentageValuesUseAvailableSpaceCorrectly() {
        TaffyTree taffy = new TaffyTree();
        long node = taffy.newLeaf(TaffyStyle.builder()
                .padding(new TaffyRect<>(
                        TaffyLengthPercentage.percentage(1f),
                        TaffyLengthPercentage.length(0f),
                        TaffyLengthPercentage.percentage(1f),
                        TaffyLengthPercentage.length(0f)
                ))
        );

        taffy.computeLayout(node, TaffySize.definiteAvailableSize(200, 100));

        System.out.println("Getting the layout of nodes");

        TaffyLayout layout = taffy.layout(node);
        Assertions.assertEquals(200.0f, layout.size().width());
        Assertions.assertEquals(200.0f, layout.size().height());
    }
}
