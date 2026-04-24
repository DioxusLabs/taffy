package dev.dioxus.taffy;

import org.junit.jupiter.api.Test;

import static org.junit.jupiter.api.Assertions.assertEquals;
import static org.junit.jupiter.api.Assertions.assertTrue;

/** Mirror of {@code examples/basic.rs} exercised through the Java bindings. */
class BasicLayoutTest {

    @Test
    void flexCenteredChild() {
        try (TaffyTree tree = new TaffyTree()) {
            long child = tree.newLeaf(Style.builder()
                    .size(Dimension.percent(0.5f), Dimension.AUTO));

            long root = tree.newWithChildren(Style.builder()
                            .size(Dimension.length(100f), Dimension.length(100f))
                            .justifyContent(JustifyContent.CENTER),
                    child);

            tree.computeLayout(root, 100f, 100f);

            Layout rootLayout = tree.layout(root);
            Layout childLayout = tree.layout(child);

            assertEquals(100f, rootLayout.width);
            assertEquals(100f, rootLayout.height);

            // The child has width 50% of its parent (100) -> 50.
            assertEquals(50f, childLayout.width);
            // Child is centered horizontally: x = (100 - 50) / 2 = 25.
            assertEquals(25f, childLayout.x);
        }
    }

    @Test
    void flexGrowFillsRemainingSpace() {
        try (TaffyTree tree = new TaffyTree()) {
            long header = tree.newLeaf(Style.builder()
                    .size(Dimension.length(800f), Dimension.length(100f)));

            long body = tree.newLeaf(Style.builder()
                    .size(Dimension.length(800f), Dimension.AUTO)
                    .flexGrow(1f));

            long root = tree.newWithChildren(Style.builder()
                            .flexDirection(FlexDirection.COLUMN)
                            .size(Dimension.length(800f), Dimension.length(600f)),
                    header, body);

            tree.computeLayout(root, 800f, 600f);

            assertEquals(800f, tree.layout(root).width);
            assertEquals(600f, tree.layout(root).height);
            assertEquals(800f, tree.layout(header).width);
            assertEquals(100f, tree.layout(header).height);
            assertEquals(800f, tree.layout(body).width);
            assertEquals(500f, tree.layout(body).height);
        }
    }

    @Test
    void maxContentViewport() {
        try (TaffyTree tree = new TaffyTree()) {
            long root = tree.newLeaf(Style.builder()
                    .size(Dimension.length(200f), Dimension.length(200f)));
            tree.computeLayout(root, AvailableSpace.MAX_CONTENT, AvailableSpace.MAX_CONTENT);
            Layout layout = tree.layout(root);
            assertEquals(200f, layout.width);
            assertEquals(200f, layout.height);
        }
    }

    @Test
    void childCountReflectsTree() {
        try (TaffyTree tree = new TaffyTree()) {
            long a = tree.newLeaf(new Style());
            long b = tree.newLeaf(new Style());
            long root = tree.newWithChildren(new Style(), a, b);
            assertEquals(2, tree.childCount(root));
            assertEquals(0, tree.childCount(a));
            assertTrue(root != 0L);
        }
    }
}
