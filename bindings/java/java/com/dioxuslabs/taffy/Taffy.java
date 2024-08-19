package com.dioxuslabs.taffy;

import com.dioxuslabs.taffy.geom.TaffyPoint;
import com.dioxuslabs.taffy.geom.TaffyRect;
import com.dioxuslabs.taffy.geom.measure.TaffyLengthPercentageAuto;
import com.dioxuslabs.taffy.style.TaffyOverflow;
import com.dioxuslabs.taffy.style.TaffyStyle;

class Taffy {
    static {
        System.loadLibrary("jtaffy");
    }

    public static void main(String[] args) {
        TaffyTree tree = new TaffyTree();
        System.out.println(tree.ptr);

        long id = tree.newLeaf(TaffyStyle.builder()
                .overflow(new TaffyPoint<>(TaffyOverflow.SCROLL, TaffyOverflow.HIDDEN))
                .inset(new TaffyRect<>(TaffyLengthPercentageAuto.auto(), TaffyLengthPercentageAuto.length(1), TaffyLengthPercentageAuto.length(1), TaffyLengthPercentageAuto.length(1)))
        );

        System.out.println("Leaf id: " + id);

        int children = tree.childCount(id);

        System.out.println("Child count: " + children);
    }
}
