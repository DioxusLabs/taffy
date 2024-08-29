package com.dioxuslabs.taffy.common;

import com.dioxuslabs.taffy.enums.AbsoluteAxis;
import com.dioxuslabs.taffy.geom.Size;
import com.dioxuslabs.taffy.geom.measure.AvailableSpace;

import java.util.Arrays;
import java.util.concurrent.atomic.AtomicInteger;

public class Text {
    public static String LOREM_IPSUM = "Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo consequat. Duis aute irure dolor in reprehenderit in voluptate velit esse cillum dolore eu fugiat nulla pariatur. Excepteur sint occaecat cupidatat non proident, sunt in culpa qui officia deserunt mollit anim id est laborum.";

    public record FontMetrics(
            float charWidth,
            float charHeight
    ) {
    }

    public enum WritingMode {
        HORIZONTAL,
        VERTICAL
    }

    public record TextContext(
            String textContent,
            WritingMode writingMode
    ) {
    }

    public static Size<Float> textMeasureFunction(
            Size<Float> knownDimensions,
            Size<AvailableSpace> availableSpace,
            TextContext textContext,
            FontMetrics fontMetrics
    ) {
        AbsoluteAxis inlineAxis = textContext.writingMode == WritingMode.HORIZONTAL ? AbsoluteAxis.HORIZONTAL : AbsoluteAxis.VERTICAL;
        AbsoluteAxis blockAxis = inlineAxis == AbsoluteAxis.HORIZONTAL ? AbsoluteAxis.VERTICAL : AbsoluteAxis.HORIZONTAL;

        String[] words = textContext.textContent.split(" ");

        if (words.length == 0) {
            return Size.zero(Float.class);
        }

        int minLineLength = Arrays.stream(words).map(String::length).max(Integer::compareTo).orElse(0);
        AtomicInteger maxLineLength = new AtomicInteger();
        Arrays.stream(words).forEach(line -> maxLineLength.addAndGet(line.length()));

        float inlineSize =
                knownDimensions.getAbs(inlineAxis)
    }
}
