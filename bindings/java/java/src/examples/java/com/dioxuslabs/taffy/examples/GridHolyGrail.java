package com.dioxuslabs.taffy.examples;

import com.dioxuslabs.taffy.Dimension;
import com.dioxuslabs.taffy.Display;
import com.dioxuslabs.taffy.GridPlacement;
import com.dioxuslabs.taffy.GridTrack;
import com.dioxuslabs.taffy.Style;
import com.dioxuslabs.taffy.TaffyTree;

/**
 * Java port of {@code examples/grid_holy_grail.rs} — the classic 3-column,
 * 3-row holy-grail layout: header, left sidebar, content, right sidebar,
 * footer.
 */
public final class GridHolyGrail {

    private GridHolyGrail() {}

    public static void main(String[] args) {
        try (TaffyTree tree = new TaffyTree()) {
            long header = tree.newLeaf(Style.builder()
                    .gridRow(GridPlacement.line(1), GridPlacement.AUTO)
                    .gridColumn(GridPlacement.span(3), GridPlacement.AUTO));

            long leftSidebar = tree.newLeaf(Style.builder()
                    .gridRow(GridPlacement.line(2), GridPlacement.AUTO)
                    .gridColumn(GridPlacement.line(1), GridPlacement.AUTO));

            long contentArea = tree.newLeaf(Style.builder()
                    .gridRow(GridPlacement.line(2), GridPlacement.AUTO)
                    .gridColumn(GridPlacement.line(2), GridPlacement.AUTO));

            long rightSidebar = tree.newLeaf(Style.builder()
                    .gridRow(GridPlacement.line(2), GridPlacement.AUTO)
                    .gridColumn(GridPlacement.line(3), GridPlacement.AUTO));

            long footer = tree.newLeaf(Style.builder()
                    .gridRow(GridPlacement.line(3), GridPlacement.AUTO)
                    .gridColumn(GridPlacement.span(3), GridPlacement.AUTO));

            long root = tree.newWithChildren(Style.builder()
                            .display(Display.GRID)
                            .size(Dimension.length(800f), Dimension.length(600f))
                            .gridTemplateColumns(GridTrack.length(250f), GridTrack.fr(1f), GridTrack.length(250f))
                            .gridTemplateRows(GridTrack.length(150f), GridTrack.fr(1f), GridTrack.length(150f)),
                    header, leftSidebar, contentArea, rightSidebar, footer);

            tree.computeLayout(root, 800f, 600f);

            System.out.println("root:         " + tree.layout(root));
            System.out.println("header:       " + tree.layout(header));
            System.out.println("leftSidebar:  " + tree.layout(leftSidebar));
            System.out.println("contentArea:  " + tree.layout(contentArea));
            System.out.println("rightSidebar: " + tree.layout(rightSidebar));
            System.out.println("footer:       " + tree.layout(footer));
        }
    }
}
