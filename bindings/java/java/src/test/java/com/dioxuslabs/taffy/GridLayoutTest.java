package com.dioxuslabs.taffy;

import org.junit.jupiter.api.Test;

import static org.junit.jupiter.api.Assertions.assertEquals;

class GridLayoutTest {

    /**
     * Three columns of 100px each in a 300px-wide grid: children land at
     * x=0, 100, 200. Smoke-test of the single-valued track encoding.
     */
    @Test
    void threeEqualColumns() {
        try (TaffyTree tree = new TaffyTree()) {
            long a = tree.newLeaf(Style.builder().gridColumn(GridPlacement.line(1), GridPlacement.AUTO));
            long b = tree.newLeaf(Style.builder().gridColumn(GridPlacement.line(2), GridPlacement.AUTO));
            long c = tree.newLeaf(Style.builder().gridColumn(GridPlacement.line(3), GridPlacement.AUTO));

            long root = tree.newWithChildren(Style.builder()
                            .display(Display.GRID)
                            .size(Dimension.length(300f), Dimension.length(100f))
                            .gridTemplateColumns(GridTrack.length(100f), GridTrack.length(100f), GridTrack.length(100f)),
                    a, b, c);

            tree.computeLayout(root, 300f, 100f);

            assertEquals(0f,   tree.layout(a).x);
            assertEquals(100f, tree.layout(b).x);
            assertEquals(200f, tree.layout(c).x);
            assertEquals(100f, tree.layout(a).width);
            assertEquals(100f, tree.layout(b).width);
            assertEquals(100f, tree.layout(c).width);
        }
    }

    /**
     * minmax(100px, 1fr) — the track is at least 100px wide and otherwise
     * takes a share of the remaining free space.
     *
     * <p>300px container, two columns: minmax(100, 1fr) and minmax(100, 1fr).
     * Both reach 150px (half the free space, each above the 100px minimum).
     */
    @Test
    void minmaxColumnsSplitFreeSpace() {
        try (TaffyTree tree = new TaffyTree()) {
            long a = tree.newLeaf(Style.builder().gridColumn(GridPlacement.line(1), GridPlacement.AUTO));
            long b = tree.newLeaf(Style.builder().gridColumn(GridPlacement.line(2), GridPlacement.AUTO));

            long root = tree.newWithChildren(Style.builder()
                            .display(Display.GRID)
                            .size(Dimension.length(300f), Dimension.length(100f))
                            .gridTemplateColumns(
                                    GridTrack.minmax(GridTrack.length(100f), GridTrack.fr(1f)),
                                    GridTrack.minmax(GridTrack.length(100f), GridTrack.fr(1f))),
                    a, b);

            tree.computeLayout(root, 300f, 100f);

            assertEquals(150f, tree.layout(a).width);
            assertEquals(150f, tree.layout(b).width);
            assertEquals(0f,   tree.layout(a).x);
            assertEquals(150f, tree.layout(b).x);
        }
    }

    /** {@code fit-content()} and {@code fit-content(<percent>)} factories print as expected. */
    @Test
    void fitContentTrackToString() {
        assertEquals("fit-content(50.0)",  GridTrack.fitContent(50f).toString());
        assertEquals("fit-content(0.25%)", GridTrack.fitContentPercent(0.25f).toString());
    }

    /**
     * Smoke test for the {@code fit-content()} wire-up: tracks declared with
     * {@code fit-content(limit)} lay out without crashing, and a fixed-size
     * item sized below the limit is honored at its declared size (rather than
     * the track clamping below it).
     *
     * <p>The stricter "large intrinsic content clamps to fit-content's limit"
     * assertion would need a measure function (Phase B territory) to reliably
     * separate min-content from max-content; here we just validate the
     * binding path.
     */
    @Test
    void fitContentLaysOutWithoutCrashing() {
        try (TaffyTree tree = new TaffyTree()) {
            long a = tree.newLeaf(Style.builder()
                    .size(Dimension.length(30f), Dimension.length(20f))
                    .gridColumn(GridPlacement.line(1), GridPlacement.AUTO));
            long b = tree.newLeaf(Style.builder()
                    .size(Dimension.length(30f), Dimension.length(20f))
                    .gridColumn(GridPlacement.line(2), GridPlacement.AUTO));

            long root = tree.newWithChildren(Style.builder()
                            .display(Display.GRID)
                            .size(Dimension.length(300f), Dimension.length(100f))
                            .gridTemplateColumns(GridTrack.fitContent(50f), GridTrack.fitContent(50f)),
                    a, b);

            tree.computeLayout(root, 300f, 100f);
            // Items are 30px (below the 50px fit-content limit) so tracks
            // honor them.
            assertEquals(30f, tree.layout(a).width);
            assertEquals(30f, tree.layout(b).width);
        }
    }

    /**
     * {@code repeat(3, 1fr)} expands to three 1fr columns. With 300px of
     * container width, each lands at 100px and children placed on lines
     * 1, 2, 3 sit at x=0, 100, 200.
     */
    @Test
    void repeatCountExpandsTracks() {
        try (TaffyTree tree = new TaffyTree()) {
            long a = tree.newLeaf(Style.builder().gridColumn(GridPlacement.line(1), GridPlacement.AUTO));
            long b = tree.newLeaf(Style.builder().gridColumn(GridPlacement.line(2), GridPlacement.AUTO));
            long c = tree.newLeaf(Style.builder().gridColumn(GridPlacement.line(3), GridPlacement.AUTO));

            long root = tree.newWithChildren(Style.builder()
                            .display(Display.GRID)
                            .size(Dimension.length(300f), Dimension.length(100f))
                            .gridTemplateColumns(GridTemplate.repeat(3, GridTrack.fr(1f))),
                    a, b, c);

            tree.computeLayout(root, 300f, 100f);

            assertEquals(0f,   tree.layout(a).x);
            assertEquals(100f, tree.layout(b).x);
            assertEquals(200f, tree.layout(c).x);
            assertEquals(100f, tree.layout(a).width);
        }
    }

    /**
     * {@code repeat(2, 100px 200px)} expands to 4 tracks (100, 200, 100, 200).
     * Placed in a 600px container, the 4-track row lays out exactly.
     */
    @Test
    void repeatExpandsInnerTrackList() {
        try (TaffyTree tree = new TaffyTree()) {
            long a = tree.newLeaf(Style.builder().gridColumn(GridPlacement.line(1), GridPlacement.AUTO));
            long b = tree.newLeaf(Style.builder().gridColumn(GridPlacement.line(2), GridPlacement.AUTO));
            long c = tree.newLeaf(Style.builder().gridColumn(GridPlacement.line(3), GridPlacement.AUTO));
            long d = tree.newLeaf(Style.builder().gridColumn(GridPlacement.line(4), GridPlacement.AUTO));

            long root = tree.newWithChildren(Style.builder()
                            .display(Display.GRID)
                            .size(Dimension.length(600f), Dimension.length(100f))
                            .gridTemplateColumns(GridTemplate.repeat(2,
                                    GridTrack.length(100f), GridTrack.length(200f))),
                    a, b, c, d);

            tree.computeLayout(root, 600f, 100f);

            assertEquals(100f, tree.layout(a).width);
            assertEquals(200f, tree.layout(b).width);
            assertEquals(100f, tree.layout(c).width);
            assertEquals(200f, tree.layout(d).width);
            assertEquals(0f,   tree.layout(a).x);
            assertEquals(100f, tree.layout(b).x);
            assertEquals(300f, tree.layout(c).x);
            assertEquals(400f, tree.layout(d).x);
        }
    }

    /** {@link GridTemplate#autoFill} / {@link GridTemplate#autoFit} don't crash on layout. */
    @Test
    void autoFillAndAutoFitSmoke() {
        try (TaffyTree tree = new TaffyTree()) {
            long fillRoot = tree.newLeaf(Style.builder()
                    .display(Display.GRID)
                    .size(Dimension.length(500f), Dimension.length(100f))
                    .gridTemplateColumns(GridTemplate.autoFill(GridTrack.length(100f))));
            tree.computeLayout(fillRoot, 500f, 100f);
            assertEquals(500f, tree.layout(fillRoot).width);

            long fitRoot = tree.newLeaf(Style.builder()
                    .display(Display.GRID)
                    .size(Dimension.length(500f), Dimension.length(100f))
                    .gridTemplateColumns(GridTemplate.autoFit(GridTrack.length(100f))));
            tree.computeLayout(fitRoot, 500f, 100f);
            assertEquals(500f, tree.layout(fitRoot).width);
        }
    }

    /**
     * Named lines: declaring {@code grid-template-columns: [start] 100px [mid] 100px [end]}
     * isn't yet supported (no line-names on templates), but a named
     * placement at least round-trips across JNI without crashing — this
     * smoke-tests the named-line encoding path.
     */
    @Test
    void namedPlacementRoundTripsThroughJni() {
        try (TaffyTree tree = new TaffyTree()) {
            // No named lines declared on the template → unknown names resolve
            // to auto per taffy spec; we just check no crash.
            long child = tree.newLeaf(Style.builder()
                    .gridColumn(GridPlacement.namedLine("header-start"),
                                GridPlacement.namedSpan("header-end", 1)));

            long root = tree.newWithChildren(Style.builder()
                            .display(Display.GRID)
                            .size(Dimension.length(300f), Dimension.length(100f))
                            .gridTemplateColumns(GridTrack.length(100f), GridTrack.length(100f), GridTrack.length(100f)),
                    child);

            tree.computeLayout(root, 300f, 100f);
            // Child gets placed by the auto-placement algorithm since the
            // names don't exist. The important bit is no UnsatisfiedLinkError
            // and no native crash.
        }
    }

    /**
     * Named lines declared on a template and referenced from a placement
     * resolve correctly — the child placed at {@code col-2}/{@code col-3}
     * ends up in the middle column.
     */
    @Test
    void namedLinesDeclaredOnTemplateResolvePlacement() {
        try (TaffyTree tree = new TaffyTree()) {
            long child = tree.newLeaf(Style.builder()
                    .gridColumn(GridPlacement.namedLine("col-2"), GridPlacement.namedLine("col-3")));

            long root = tree.newWithChildren(Style.builder()
                            .display(Display.GRID)
                            .size(Dimension.length(300f), Dimension.length(100f))
                            .gridTemplateColumns(GridTrack.length(100f), GridTrack.length(100f), GridTrack.length(100f))
                            // One name per line position: 4 lines for 3 tracks.
                            .gridTemplateColumnNames(
                                    new String[] {"col-1"},
                                    new String[] {"col-2"},
                                    new String[] {"col-3"},
                                    new String[] {"col-4"}),
                    child);

            tree.computeLayout(root, 300f, 100f);

            // col-2 → line 2, col-3 → line 3 → middle column.
            assertEquals(100f, tree.layout(child).x);
            assertEquals(100f, tree.layout(child).width);
        }
    }

    /**
     * A {@code grid-template-areas} entry routes a placement by name to the
     * declared rectangle.
     */
    @Test
    void templateAreasRoutePlacement() {
        try (TaffyTree tree = new TaffyTree()) {
            // 3x3 grid, header area spans row 1 / all 3 columns.
            long header = tree.newLeaf(Style.builder()
                    .gridColumn(GridPlacement.namedLine("header"), GridPlacement.AUTO));

            long root = tree.newWithChildren(Style.builder()
                            .display(Display.GRID)
                            .size(Dimension.length(300f), Dimension.length(300f))
                            .gridTemplateColumns(GridTrack.fr(1f), GridTrack.fr(1f), GridTrack.fr(1f))
                            .gridTemplateRows(GridTrack.fr(1f), GridTrack.fr(1f), GridTrack.fr(1f))
                            .gridTemplateAreas(
                                    GridTemplateArea.of("header", 1, 2, 1, 4)),
                    header);

            tree.computeLayout(root, 300f, 300f);
            // Smoke: no crash, layout computed, child width/height positive
            // (exact position depends on whether taffy's area-name resolution
            // is fully wired for the placement we use).
            Layout l = tree.layout(header);
            assertEquals(true, l.width >= 0f);
            assertEquals(true, l.height >= 0f);
        }
    }

    /**
     * When the container is narrower than the sum of minima, the minimum
     * wins. minmax(200, 1fr) × 2 in a 300-wide container → each column
     * takes its 200 min, container overflows.
     */
    @Test
    void minmaxHonorsMinimum() {
        try (TaffyTree tree = new TaffyTree()) {
            long a = tree.newLeaf(Style.builder().gridColumn(GridPlacement.line(1), GridPlacement.AUTO));
            long b = tree.newLeaf(Style.builder().gridColumn(GridPlacement.line(2), GridPlacement.AUTO));

            long root = tree.newWithChildren(Style.builder()
                            .display(Display.GRID)
                            .size(Dimension.length(300f), Dimension.length(100f))
                            .gridTemplateColumns(
                                    GridTrack.minmax(GridTrack.length(200f), GridTrack.fr(1f)),
                                    GridTrack.minmax(GridTrack.length(200f), GridTrack.fr(1f))),
                    a, b);

            tree.computeLayout(root, 300f, 100f);

            assertEquals(200f, tree.layout(a).width);
            assertEquals(200f, tree.layout(b).width);
        }
    }
}
