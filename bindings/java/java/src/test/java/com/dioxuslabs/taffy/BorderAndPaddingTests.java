package com.dioxuslabs.taffy;

import com.dioxuslabs.taffy.geom.TaffyRect;
import com.dioxuslabs.taffy.geom.TaffySize;
import com.dioxuslabs.taffy.geom.measure.TaffyLengthPercentage;
import com.dioxuslabs.taffy.style.TaffyStyle;
import com.dioxuslabs.taffy.tree.TaffyLayout;
import org.junit.jupiter.api.Test;

import java.util.Arrays;

import static com.dioxuslabs.taffy.TaffyTests.assertEquals;

public class BorderAndPaddingTests {
    static {
        System.loadLibrary("jtaffy");
    }

    public <T> TaffyRect<T> arrToRect(T[] items) {
        return new TaffyRect<>(items[0], items[1], items[2], items[3]);
    }

    @Test
    public void borderOnASingleAxisDoesntIncreaseSize() {
        for (int i = 0; i < 4; i++) {
            TaffyTree taffy = new TaffyTree();

            TaffyLengthPercentage[] lengths = new TaffyLengthPercentage[4];
            Arrays.fill(lengths, TaffyLengthPercentage.length(0f));
            lengths[i] = TaffyLengthPercentage.length(10f);
            TaffyRect<TaffyLengthPercentage> rect = arrToRect(lengths);

            long node = taffy.newLeaf(TaffyStyle.builder()
                    .border(rect)
            );

            taffy.computeLayout(node, TaffySize.definiteAvailableSize(100, 100));

            TaffyLayout layout = taffy.layout(node);

            assertEquals(layout.size().width() * layout.size().height(), 0.0f);
        }
    }

    @Test
    public void paddingOnASingleAxisDoesntIncreaseSize() {
        for (int i = 0; i < 4; i++) {
            TaffyTree taffy = new TaffyTree();

            TaffyLengthPercentage[] lengths = new TaffyLengthPercentage[4];
            Arrays.fill(lengths, TaffyLengthPercentage.length(0f));
            lengths[i] = TaffyLengthPercentage.length(10f);
            TaffyRect<TaffyLengthPercentage> rect = arrToRect(lengths);

            long node = taffy.newLeaf(TaffyStyle.builder()
                    .padding(rect)
            );

            taffy.computeLayout(node, TaffySize.definiteAvailableSize(100, 100));

            TaffyLayout layout = taffy.layout(node);

            assertEquals(layout.size().width() * layout.size().height(), 0.0f);
        }
    }

    @Test
    public void borderAndPaddingOnASingleAxisDoesntIncreaseSize() {
        for (int i = 0; i < 4; i++) {
            TaffyTree taffy = new TaffyTree();

            TaffyLengthPercentage[] lengths = new TaffyLengthPercentage[4];
            Arrays.fill(lengths, TaffyLengthPercentage.length(0f));
            lengths[i] = TaffyLengthPercentage.length(10f);
            TaffyRect<TaffyLengthPercentage> rect = arrToRect(lengths);

            long node = taffy.newLeaf(TaffyStyle.builder()
                    .border(rect)
                    .padding(rect)
            );

            taffy.computeLayout(node, TaffySize.definiteAvailableSize(100, 100));

            TaffyLayout layout = taffy.layout(node);

            assertEquals(layout.size().width() * layout.size().height(), 0.0f);
        }
    }

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

        TaffyLayout layout = taffy.layout(node);

        assertEquals(layout.size().width(), 200.0f);
        assertEquals(layout.size().height(), 200.0f);
    }
}
