package com.dioxuslabs.taffy;

import org.junit.jupiter.api.Test;

import static org.junit.jupiter.api.Assertions.assertEquals;

class MeasureFunctionTest {

    /** A leaf with a fixed-size measure function reports that size through layout. */
    @Test
    void fixedSizeMeasureFunction() {
        try (TaffyTree tree = new TaffyTree()) {
            long imageNode = tree.newLeafWithMeasure(
                    new Style(),
                    (kw, kh, awT, awV, ahT, ahV) -> new float[] {40f, 30f});

            long root = tree.newWithChildren(Style.builder()
                            .flexDirection(FlexDirection.COLUMN)
                            // Default cross-axis align is stretch; override so the
                            // leaf keeps its measured width rather than filling
                            // the container.
                            .alignItems(AlignItems.FLEX_START)
                            .size(Dimension.length(200f), Dimension.AUTO),
                    imageNode);

            tree.computeLayout(root, AvailableSpace.MAX_CONTENT, AvailableSpace.MAX_CONTENT);

            Layout layout = tree.layout(imageNode);
            assertEquals(40f, layout.width);
            assertEquals(30f, layout.height);
        }
    }

    /** MAX_CONTENT gives full text width; a definite constraint wraps to multiple lines. */
    @Test
    void textMeasureRespondsToAvailableSpace() {
        try (TaffyTree tree = new TaffyTree()) {
            // 10 characters wide, 1 line tall at max-content, wraps otherwise.
            float charW = 10f, charH = 10f;
            String text = "one two three four";

            MeasureFunction text_measure = (kw, kh, awT, awV, ahT, ahV) -> {
                String[] words = text.split(" ");
                int maxWord = 0, totalChars = 0;
                for (String w : words) {
                    maxWord = Math.max(maxWord, w.length());
                    totalChars += w.length();
                }
                totalChars += words.length - 1; // inter-word spaces

                float inline;
                if (!Float.isNaN(kw)) {
                    inline = kw;
                } else if (awT == AvailableSpace.TAG_MAX_CONTENT) {
                    inline = totalChars * charW;
                } else if (awT == AvailableSpace.TAG_MIN_CONTENT) {
                    inline = maxWord * charW;
                } else {
                    inline = Math.max(maxWord * charW, Math.min(totalChars * charW, awV));
                }

                int lineLen = Math.max(1, (int) Math.floor(inline / charW));
                int lines = 1, cur = 0;
                for (String w : words) {
                    if (cur == 0) { cur = w.length(); }
                    else if (cur + w.length() + 1 > lineLen) { lines++; cur = w.length(); }
                    else { cur += w.length() + 1; }
                }
                return new float[] {inline, lines * charH};
            };

            long textNode = tree.newLeafWithMeasure(new Style(), text_measure);
            long root = tree.newWithChildren(Style.builder()
                            .flexDirection(FlexDirection.COLUMN)
                            .size(Dimension.length(60f), Dimension.AUTO),
                    textNode);

            tree.computeLayout(root, AvailableSpace.definite(60f), AvailableSpace.MAX_CONTENT);

            Layout tl = tree.layout(textNode);
            // "one two three four" -> 18 chars + 3 spaces = 21 chars of text at max-content
            // width constrained to 60 (6 chars), "three" (5) fits; wraps to 4 lines
            assertEquals(60f, tl.width);
            assertEquals(40f, tl.height); // 4 lines * 10
        }
    }

    /** Setting and clearing a measure function via setMeasure. */
    @Test
    void setMeasureReplacesCallback() {
        try (TaffyTree tree = new TaffyTree()) {
            long leaf = tree.newLeaf(new Style());

            tree.setMeasure(leaf, (kw, kh, awT, awV, ahT, ahV) -> new float[] {15f, 25f});

            long root = tree.newWithChildren(Style.builder()
                            .flexDirection(FlexDirection.COLUMN)
                            .alignItems(AlignItems.FLEX_START)
                            .size(Dimension.length(100f), Dimension.AUTO),
                    leaf);

            tree.computeLayout(root, AvailableSpace.MAX_CONTENT, AvailableSpace.MAX_CONTENT);
            assertEquals(15f, tree.layout(leaf).width);
            assertEquals(25f, tree.layout(leaf).height);

            // Clear and re-layout: leaf should measure as zero.
            tree.setMeasure(leaf, null);
            tree.markDirty(leaf);
            tree.computeLayout(root, AvailableSpace.MAX_CONTENT, AvailableSpace.MAX_CONTENT);
            assertEquals(0f, tree.layout(leaf).width);
            assertEquals(0f, tree.layout(leaf).height);
        }
    }
}
