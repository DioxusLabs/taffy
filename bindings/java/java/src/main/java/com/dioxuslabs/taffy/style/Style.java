package com.dioxuslabs.taffy.style;

import com.dioxuslabs.taffy.geom.Line;
import com.dioxuslabs.taffy.geom.Point;
import com.dioxuslabs.taffy.geom.Rect;
import com.dioxuslabs.taffy.geom.Size;
import com.dioxuslabs.taffy.geom.grid.GenericGridPlacement;
import com.dioxuslabs.taffy.geom.grid.NonRepeatedTrackSizingFunction;
import com.dioxuslabs.taffy.geom.grid.TrackSizingFunction;
import com.dioxuslabs.taffy.geom.measure.Dimension;
import com.dioxuslabs.taffy.geom.measure.LengthPercentage;
import com.dioxuslabs.taffy.geom.measure.LengthPercentageAuto;

import java.util.List;

@SuppressWarnings("all")
public class Style {
    private Display display;
    private boolean itemIsTable;
    private BoxSizing boxSizing;

    private Point<Overflow> overflow;
    private float scrollbarWidth;

    private Position position;
    private Rect<LengthPercentageAuto> inset;

    private Size<Dimension> size;
    private Size<Dimension> minSize;
    private Size<Dimension> maxSize;
    private Float aspectRatio;

    private Rect<LengthPercentageAuto> margin;
    private Rect<LengthPercentage> padding;
    private Rect<LengthPercentage> border;

    private AlignItems alignItems;
    private AlignItems alignSelf;
    private AlignItems justifyItems;
    private AlignItems justifySelf;
    private AlignContent alignContent;
    private AlignContent justifyContent;
    private Size<LengthPercentage> gap;

    private TextAlign textAlign;

    private FlexDirection flexDirection;
    private FlexWrap flexWrap;

    private Dimension flexBasis;
    private float flexGrow;
    private float flexShrink;

    private List<TrackSizingFunction> gridTemplateRows;
    private List<TrackSizingFunction> gridTemplateColumns;
    private List<NonRepeatedTrackSizingFunction> gridAutoRows;
    private List<NonRepeatedTrackSizingFunction> gridAutoColumns;
    private GridAutoFlow gridAutoFlow;

    private Line<GenericGridPlacement> gridRow;
    private Line<GenericGridPlacement> gridColumn;

    public static Style builder() {
        return new Style();
    }

    public Style display(Display display) {
        this.display = display;
        return this;
    }

    public Style itemIsTable(boolean itemIsTable) {
        this.itemIsTable = itemIsTable;
        return this;
    }

    public Style boxSizing(BoxSizing boxSizing) {
        this.boxSizing = boxSizing;
        return this;
    }

    public Style overflow(Point<Overflow> overflow) {
        this.overflow = overflow;
        return this;
    }

    public Style scrollbarWidth(float scrollbarWidth) {
        this.scrollbarWidth = scrollbarWidth;
        return this;
    }

    public Style position(Position position) {
        this.position = position;
        return this;
    }

    public Style inset(Rect<LengthPercentageAuto> inset) {
        this.inset = inset;
        return this;
    }

    public Style size(Size<Dimension> size) {
        this.size = size;
        return this;
    }

    public Style minSize(Size<Dimension> minSize) {
        this.minSize = minSize;
        return this;
    }

    public Style maxSize(Size<Dimension> maxSize) {
        this.maxSize = maxSize;
        return this;
    }

    public Style aspectRatio(Float aspectRatio) {
        this.aspectRatio = aspectRatio;
        return this;
    }

    public Style margin(Rect<LengthPercentageAuto> margin) {
        this.margin = margin;
        return this;
    }

    public Style padding(Rect<LengthPercentage> padding) {
        this.padding = padding;
        return this;
    }

    public Style border(Rect<LengthPercentage> border) {
        this.border = border;
        return this;
    }

    public Style alignItems(AlignItems alignItems) {
        this.alignItems = alignItems;
        return this;
    }

    public Style alignSelf(AlignItems alignSelf) {
        this.alignSelf = alignSelf;
        return this;
    }

    public Style justifyItems(AlignItems justifyItems) {
        this.justifyItems = justifyItems;
        return this;
    }

    public Style justifySelf(AlignItems justifySelf) {
        this.justifySelf = justifySelf;
        return this;
    }

    public Style alignContent(AlignContent alignContent) {
        this.alignContent = alignContent;
        return this;
    }

    public Style justifyContent(AlignContent justifyContent) {
        this.justifyContent = justifyContent;
        return this;
    }

    public Style gap(Size<LengthPercentage> gap) {
        this.gap = gap;
        return this;
    }

    public Style textAlign(TextAlign textAlign) {
        this.textAlign = textAlign;
        return this;
    }

    public Style flexDirection(FlexDirection flexDirection) {
        this.flexDirection = flexDirection;
        return this;
    }

    public Style flexWrap(FlexWrap flexWrap) {
        this.flexWrap = flexWrap;
        return this;
    }

    public Style flexBasis(Dimension flexBasis) {
        this.flexBasis = flexBasis;
        return this;
    }

    public Style flexGrow(float flexGrow) {
        this.flexGrow = flexGrow;
        return this;
    }

    public Style flexShrink(float flexShrink) {
        this.flexShrink = flexShrink;
        return this;
    }

    public Style gridTemplateRows(List<TrackSizingFunction> gridTemplateRows) {
        this.gridTemplateRows = gridTemplateRows;
        return this;
    }

    public Style gridTemplateColumns(List<TrackSizingFunction> gridTemplateColumns) {
        this.gridTemplateColumns = gridTemplateColumns;
        return this;
    }

    public Style gridAutoRows(List<NonRepeatedTrackSizingFunction> gridAutoRows) {
        this.gridAutoRows = gridAutoRows;
        return this;
    }

    public Style gridAutoColumns(List<NonRepeatedTrackSizingFunction> gridAutoColumns) {
        this.gridAutoColumns = gridAutoColumns;
        return this;
    }

    public Style gridAutoFlow(GridAutoFlow gridAutoFlow) {
        this.gridAutoFlow = gridAutoFlow;
        return this;
    }

    public Style gridRow(Line<GenericGridPlacement> gridRow) {
        this.gridRow = gridRow;
        return this;
    }

    public Style gridColumn(Line<GenericGridPlacement> gridColumn) {
        this.gridColumn = gridColumn;
        return this;
    }
}
