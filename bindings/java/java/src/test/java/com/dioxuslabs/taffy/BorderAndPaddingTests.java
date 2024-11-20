package com.dioxuslabs.taffy;

import com.dioxuslabs.taffy.geom.Rect;
import com.dioxuslabs.taffy.geom.Size;
import com.dioxuslabs.taffy.geom.measure.LengthPercentage;
import com.dioxuslabs.taffy.style.Style;
import com.dioxuslabs.taffy.tree.Layout;
import org.junit.jupiter.api.Test;

import java.util.Arrays;

import static com.dioxuslabs.taffy.TaffyTests.assertEquals;

public class BorderAndPaddingTests {
    static {
        System.loadLibrary("jtaffy");
    }

    public <T> Rect<T> arrToRect(T[] items) {
        return new Rect<>(items[0], items[1], items[2], items[3]);
    }

    @Test
    public void borderOnASingleAxisDoesntIncreaseSize() {
        for (int i = 0; i < 4; i++) {
            TaffyTree taffy = new TaffyTree();

            LengthPercentage[] lengths = new LengthPercentage[4];
            Arrays.fill(lengths, LengthPercentage.length(0f));
            lengths[i] = LengthPercentage.length(10f);
            Rect<LengthPercentage> rect = arrToRect(lengths);

            long node = taffy.newLeaf(Style.builder()
                    .border(rect)
            );

            taffy.computeLayout(node, Size.definiteAvailableSize(100, 100));

            Layout layout = taffy.layout(node);

            assertEquals(layout.size().width() * layout.size().height(), 0.0f);
        }
    }

    @Test
    public void paddingOnASingleAxisDoesntIncreaseSize() {
        for (int i = 0; i < 4; i++) {
            TaffyTree taffy = new TaffyTree();

            LengthPercentage[] lengths = new LengthPercentage[4];
            Arrays.fill(lengths, LengthPercentage.length(0f));
            lengths[i] = LengthPercentage.length(10f);
            Rect<LengthPercentage> rect = arrToRect(lengths);

            long node = taffy.newLeaf(Style.builder()
                    .padding(rect)
            );

            taffy.computeLayout(node, Size.definiteAvailableSize(100, 100));

            Layout layout = taffy.layout(node);

            assertEquals(layout.size().width() * layout.size().height(), 0.0f);
        }
    }

    @Test
    public void borderAndPaddingOnASingleAxisDoesntIncreaseSize() {
        for (int i = 0; i < 4; i++) {
            TaffyTree taffy = new TaffyTree();

            LengthPercentage[] lengths = new LengthPercentage[4];
            Arrays.fill(lengths, LengthPercentage.length(0f));
            lengths[i] = LengthPercentage.length(10f);
            Rect<LengthPercentage> rect = arrToRect(lengths);

            long node = taffy.newLeaf(Style.builder()
                    .border(rect)
                    .padding(rect)
            );

            taffy.computeLayout(node, Size.definiteAvailableSize(100, 100));

            Layout layout = taffy.layout(node);

            assertEquals(layout.size().width() * layout.size().height(), 0.0f);
        }
    }

    @Test
    public void verticalBorderAndPaddingPercentageValuesUseAvailableSpaceCorrectly() {
        TaffyTree taffy = new TaffyTree();
        long node = taffy.newLeaf(Style.builder()
                .padding(new Rect<>(
                        LengthPercentage.percent(1f),
                        LengthPercentage.length(0f),
                        LengthPercentage.percent(1f),
                        LengthPercentage.length(0f)
                ))
        );

        taffy.computeLayout(node, Size.definiteAvailableSize(200, 100));

        Layout layout = taffy.layout(node);

        assertEquals(layout.size().width(), 200.0f);
        assertEquals(layout.size().height(), 200.0f);
    }
}
