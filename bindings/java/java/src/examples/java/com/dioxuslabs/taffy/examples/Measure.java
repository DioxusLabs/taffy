package com.dioxuslabs.taffy.examples;

import com.dioxuslabs.taffy.AvailableSpace;
import com.dioxuslabs.taffy.Dimension;
import com.dioxuslabs.taffy.Display;
import com.dioxuslabs.taffy.FlexDirection;
import com.dioxuslabs.taffy.MeasureFunction;
import com.dioxuslabs.taffy.Style;
import com.dioxuslabs.taffy.TaffyTree;

/**
 * Java port of {@code examples/measure.rs}. Two leaves with measure functions
 * (a block of text that wraps, a fixed-aspect image) are laid out in a column
 * and their computed sizes printed.
 */
public final class Measure {

    private Measure() {}

    private static final String LOREM = "Lorem ipsum dolor sit amet, consectetur adipiscing elit, "
            + "sed do eiusmod tempor incididunt ut labore et dolore magna aliqua.";
    private static final float CHAR_W = 10f;
    private static final float CHAR_H = 10f;

    /** A text block that wraps within the available width. */
    private static MeasureFunction text(String content) {
        return (kw, kh, awT, awV, ahT, ahV) -> {
            String[] words = content.split("\\s+");
            int maxWord = 0, totalChars = 0;
            for (String w : words) {
                maxWord = Math.max(maxWord, w.length());
                totalChars += w.length();
            }
            totalChars += Math.max(0, words.length - 1);

            float inline;
            if (!Float.isNaN(kw)) {
                inline = kw;
            } else if (awT == AvailableSpace.TAG_MIN_CONTENT) {
                inline = maxWord * CHAR_W;
            } else if (awT == AvailableSpace.TAG_MAX_CONTENT) {
                inline = totalChars * CHAR_W;
            } else {
                inline = Math.max(maxWord * CHAR_W, Math.min(totalChars * CHAR_W, awV));
            }

            float block;
            if (!Float.isNaN(kh)) {
                block = kh;
            } else {
                int lineLen = Math.max(1, (int) Math.floor(inline / CHAR_W));
                int lines = 1, cur = 0;
                for (String w : words) {
                    if (cur == 0) {
                        cur = w.length();
                    } else if (cur + w.length() + 1 > lineLen) {
                        lines++;
                        cur = w.length();
                    } else {
                        cur += w.length() + 1;
                    }
                }
                block = lines * CHAR_H;
            }
            return new float[] {inline, block};
        };
    }

    /** A fixed-aspect image: if either dimension is known, scale the other; otherwise natural size. */
    private static MeasureFunction image(float naturalW, float naturalH) {
        float aspect = naturalW / naturalH;
        return (kw, kh, awT, awV, ahT, ahV) -> {
            if (!Float.isNaN(kw) && !Float.isNaN(kh)) return new float[] {kw, kh};
            if (!Float.isNaN(kw)) return new float[] {kw, kw / aspect};
            if (!Float.isNaN(kh)) return new float[] {kh * aspect, kh};
            return new float[] {naturalW, naturalH};
        };
    }

    public static void main(String[] args) {
        try (TaffyTree tree = new TaffyTree()) {
            long textNode  = tree.newLeafWithMeasure(new Style(), text(LOREM));
            long imageNode = tree.newLeafWithMeasure(new Style(), image(400f, 300f));

            long root = tree.newWithChildren(Style.builder()
                            .display(Display.FLEX)
                            .flexDirection(FlexDirection.COLUMN)
                            .size(Dimension.length(200f), Dimension.AUTO),
                    textNode, imageNode);

            tree.computeLayout(root, AvailableSpace.MAX_CONTENT, AvailableSpace.MAX_CONTENT);

            System.out.println("root:  " + tree.layout(root));
            System.out.println("text:  " + tree.layout(textNode));
            System.out.println("image: " + tree.layout(imageNode));
        }
    }
}
