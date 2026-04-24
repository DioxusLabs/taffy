package com.dioxuslabs.taffy;

import org.junit.jupiter.api.Test;

import static org.junit.jupiter.api.Assertions.assertEquals;
import static org.junit.jupiter.api.Assertions.assertNotEquals;
import static org.junit.jupiter.api.Assertions.assertNull;

/** Round-trip: what you set via a builder setter, you read via the getter. */
class StyleGettersTest {

    @Test
    void unsetFieldsReturnNull() {
        Style s = new Style();
        assertNull(s.getDisplay());
        assertNull(s.getWidth());
        assertNull(s.getFlexGrow());
        assertNull(s.getPaddingLeft());
        assertNull(s.getGridAutoFlow());
    }

    @Test
    void scalarRoundTrip() {
        Style s = Style.builder()
                .display(Display.FLEX)
                .position(Position.ABSOLUTE)
                .flexDirection(FlexDirection.COLUMN)
                .flexGrow(2.5f)
                .aspectRatio(1.5f);

        assertEquals(Display.FLEX, s.getDisplay());
        assertEquals(Position.ABSOLUTE, s.getPosition());
        assertEquals(FlexDirection.COLUMN, s.getFlexDirection());
        assertEquals(2.5f, s.getFlexGrow());
        assertEquals(1.5f, s.getAspectRatio());
    }

    @Test
    void sizeRoundTrip() {
        Style s = Style.builder().size(Dimension.length(100f), Dimension.percent(0.5f));
        assertEquals(Dimension.length(100f), s.getWidth());
        assertEquals(Dimension.percent(0.5f), s.getHeight());
        // And percent != length of same value.
        assertNotEquals(Dimension.length(0.5f), s.getHeight());
    }

    @Test
    void edgeRectRoundTrip() {
        Style s = Style.builder().padding(
                LengthPercentage.length(1f),
                LengthPercentage.length(2f),
                LengthPercentage.length(3f),
                LengthPercentage.length(4f));
        assertEquals(LengthPercentage.length(1f), s.getPaddingLeft());
        assertEquals(LengthPercentage.length(2f), s.getPaddingRight());
        assertEquals(LengthPercentage.length(3f), s.getPaddingTop());
        assertEquals(LengthPercentage.length(4f), s.getPaddingBottom());
    }

    @Test
    void overflowShorthandSetsBoth() {
        Style s = Style.builder().overflow(Overflow.HIDDEN);
        assertEquals(Overflow.HIDDEN, s.getOverflowX());
        assertEquals(Overflow.HIDDEN, s.getOverflowY());
    }

    @Test
    void dimensionEqualityAndToString() {
        assertEquals(Dimension.length(42f), Dimension.length(42f));
        assertNotEquals(Dimension.length(42f), Dimension.percent(42f));
        assertEquals(Dimension.AUTO, Dimension.AUTO);
        assertEquals("auto", Dimension.AUTO.toString());
        assertEquals("length(42.0)", Dimension.length(42f).toString());
        assertEquals("percent(0.5)", Dimension.percent(0.5f).toString());
    }
}
