package com.dioxuslabs.taffy;

import org.junit.jupiter.api.Test;

import static org.junit.jupiter.api.Assertions.assertEquals;
import static org.junit.jupiter.api.Assertions.assertNotNull;
import static org.junit.jupiter.api.Assertions.assertNull;

/** Read-your-writes: what you apply to a node you can read back via getStyle. */
class TreeGetStyleTest {

    @Test
    void dimensionsAndFlexRoundTrip() {
        try (TaffyTree tree = new TaffyTree()) {
            long node = tree.newLeaf(Style.builder()
                    .display(Display.FLEX)
                    .flexDirection(FlexDirection.COLUMN)
                    .size(Dimension.length(100f), Dimension.percent(0.5f))
                    .flexGrow(1.5f)
                    .aspectRatio(2f));

            Style s = tree.getStyle(node);

            assertEquals(Display.FLEX,           s.getDisplay());
            assertEquals(FlexDirection.COLUMN,   s.getFlexDirection());
            assertEquals(Dimension.length(100f), s.getWidth());
            assertEquals(Dimension.percent(0.5f), s.getHeight());
            assertEquals(1.5f,  s.getFlexGrow());
            assertEquals(2f,    s.getAspectRatio());
        }
    }

    @Test
    void edgeRectsRoundTrip() {
        try (TaffyTree tree = new TaffyTree()) {
            long node = tree.newLeaf(Style.builder()
                    .padding(
                            LengthPercentage.length(1f),
                            LengthPercentage.length(2f),
                            LengthPercentage.length(3f),
                            LengthPercentage.length(4f))
                    .margin(
                            LengthPercentageAuto.length(5f),
                            LengthPercentageAuto.AUTO,
                            LengthPercentageAuto.percent(0.1f),
                            LengthPercentageAuto.length(7f)));

            Style s = tree.getStyle(node);

            assertEquals(LengthPercentage.length(1f), s.getPaddingLeft());
            assertEquals(LengthPercentage.length(2f), s.getPaddingRight());
            assertEquals(LengthPercentage.length(3f), s.getPaddingTop());
            assertEquals(LengthPercentage.length(4f), s.getPaddingBottom());

            assertEquals(LengthPercentageAuto.length(5f),    s.getMarginLeft());
            assertEquals(LengthPercentageAuto.AUTO,          s.getMarginRight());
            assertEquals(LengthPercentageAuto.percent(0.1f), s.getMarginTop());
            assertEquals(LengthPercentageAuto.length(7f),    s.getMarginBottom());
        }
    }

    @Test
    void optionalEnumsStayUnsetWhenNeverApplied() {
        try (TaffyTree tree = new TaffyTree()) {
            long node = tree.newLeaf(new Style());
            Style s = tree.getStyle(node);

            // These are Option<_> on the taffy side and default to None.
            assertNull(s.getJustifyContent());
            assertNull(s.getAlignContent());
            assertNull(s.getAlignItems());
            assertNull(s.getAlignSelf());

            // These always have a concrete default.
            assertNotNull(s.getDisplay());
            assertNotNull(s.getPosition());
            assertNotNull(s.getFlexDirection());
        }
    }

    @Test
    void alignEnumsRoundTripWhenSet() {
        try (TaffyTree tree = new TaffyTree()) {
            long node = tree.newLeaf(Style.builder()
                    .justifyContent(JustifyContent.SPACE_BETWEEN)
                    .alignContent(AlignContent.SPACE_AROUND)
                    .alignItems(AlignItems.BASELINE)
                    .alignSelf(AlignSelf.STRETCH));

            Style s = tree.getStyle(node);
            assertEquals(JustifyContent.SPACE_BETWEEN, s.getJustifyContent());
            assertEquals(AlignContent.SPACE_AROUND,    s.getAlignContent());
            assertEquals(AlignItems.BASELINE,          s.getAlignItems());
            assertEquals(AlignSelf.STRETCH,            s.getAlignSelf());
        }
    }
}
