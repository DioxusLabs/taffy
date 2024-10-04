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

        Float abs = knownDimensions.getAbs(inlineAxis);
        float inlineSize;
        if (abs == null) {
            AvailableSpace as = availableSpace.getAbs(inlineAxis);
            if (as.isMinContent()) {
                inlineSize = minLineLength * fontMetrics.charWidth;
            } else if (as.isMaxContent()) {
                inlineSize = maxLineLength.get() * fontMetrics.charWidth;
            } else {
                inlineSize = Math.max(Math.min(as.val(), maxLineLength.get() * fontMetrics.charWidth), minLineLength * fontMetrics.charWidth);
            }
        } else {
            inlineSize = abs;
        }

        abs = knownDimensions.getAbs(blockAxis);
        float blockSize;
        if (abs == null) {
            int inlineLineLength = (int) Math.floor(inlineSize / fontMetrics.charWidth);
            int lineCount = 1;
            int currentLineLength = 0;

            for (String word : words) {
                if (currentLineLength == 0) {
                    // first word
                    currentLineLength = word.length();
                } else if (currentLineLength + word.length() + 1 > inlineLineLength) {
                    // every word past the first needs to check for line length including the space between words
                    // note: a real implementation of this should handle whitespace characters other than ' '
                    // and do something more sophisticated for long words
                    lineCount += 1;
                    currentLineLength = word.length();
                } else {
                    // add the word and a space
                    currentLineLength += word.length() + 1;
                }
            }

            blockSize = lineCount * fontMetrics.charHeight;
        } else {
            blockSize = abs;
        }

        if (textContext.writingMode == WritingMode.HORIZONTAL) {
            return new Size<>(inlineSize, blockSize);
        } else {
            return new Size<>(blockSize, inlineSize);
        }
    }
}
