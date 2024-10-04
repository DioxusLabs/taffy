package com.dioxuslabs.taffy;

import com.dioxuslabs.taffy.common.Image.ImageContext;
import com.dioxuslabs.taffy.common.Text;
import com.dioxuslabs.taffy.common.Text.FontMetrics;
import com.dioxuslabs.taffy.common.Text.TextContext;
import com.dioxuslabs.taffy.enums.Display;
import com.dioxuslabs.taffy.enums.FlexDirection;
import com.dioxuslabs.taffy.geom.Size;
import com.dioxuslabs.taffy.geom.measure.AvailableSpace;
import com.dioxuslabs.taffy.geom.measure.Dimension;
import com.dioxuslabs.taffy.style.Style;

import java.util.List;

import static com.dioxuslabs.taffy.common.Image.imageMeasureFunction;
import static com.dioxuslabs.taffy.common.Text.LOREM_IPSUM;
import static com.dioxuslabs.taffy.common.Text.textMeasureFunction;

/**
 * Equivalent of taffy/examples/measure.rs
 */
public class Measure {
    public static Size<Float> measureFunction(
            Size<Float> knownDimensions,
            Size<AvailableSpace> availableSpace,
            Object nodeContext,
            FontMetrics fontMetrics
    ) {
        if (knownDimensions.bothAxisDefined()) {
            return new Size<>(knownDimensions.width(), knownDimensions.height());
        }

        if (nodeContext instanceof TextContext nc) {
            return textMeasureFunction(knownDimensions, availableSpace, nc, fontMetrics);
        } else if (nodeContext instanceof ImageContext nc) {
            return imageMeasureFunction(knownDimensions, nc);
        } else {
            return Size.zero(Float.class);
        }
    }

    public static void main(String[] args) {
        TaffyTree taffy = new TaffyTree();

        FontMetrics fontMetrics = new FontMetrics(10f, 10f);

        long textNode = taffy.newLeafWithContext(
                Style.def(),
                new TextContext(LOREM_IPSUM, Text.WritingMode.HORIZONTAL)
        );

        long imageNode = taffy.newLeafWithContext(
                Style.def(),
                new ImageContext(400f, 300f)
        );

        long root = taffy.newWithChildren(
                Style.builder()
                        .display(Display.FLEX)
                        .flexDirection(FlexDirection.COLUMN)
                        .size(new Size<>(Dimension.length(200), Dimension.auto())),
                List.of(textNode, imageNode)
        );

        // Compute layout and print result
        taffy.computeLayoutWithMeasure(
                root,
                Size.maxContentAvailableSize()
                // todo measure func
        );

        taffy.printTree(root);
    }
}
