package com.dioxuslabs.taffy;

import com.dioxuslabs.taffy.geom.TaffyRect;
import com.dioxuslabs.taffy.geom.TaffySize;
import com.dioxuslabs.taffy.geom.measure.TaffyLengthPercentage;
import com.dioxuslabs.taffy.style.TaffyStyle;
import com.dioxuslabs.taffy.tree.TaffyLayout;

class Taffy {
    static {
        System.loadLibrary("jtaffy");
    }

    public static void main(String[] args) {
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
        System.out.println("Width: " + layout.size().width() + " = 200.0");
        System.out.println("Height: " + layout.size().height() + " = 200.0");
    }
}
