package com.dioxuslabs.taffy.style;

import com.dioxuslabs.taffy.geom.TaffyLine;
import com.dioxuslabs.taffy.geom.TaffyPoint;
import com.dioxuslabs.taffy.geom.TaffyRect;
import com.dioxuslabs.taffy.geom.TaffySize;
import com.dioxuslabs.taffy.geom.grid.TaffyGenericGridPlacement;
import com.dioxuslabs.taffy.geom.grid.TaffyNonRepeatedTrackSizingFunction;
import com.dioxuslabs.taffy.geom.grid.TaffyTrackSizingFunction;
import com.dioxuslabs.taffy.geom.measure.TaffyDimension;
import com.dioxuslabs.taffy.geom.measure.TaffyLengthPercentage;
import com.dioxuslabs.taffy.geom.measure.TaffyLengthPercentageAuto;

import java.util.List;

@SuppressWarnings("all")
public class TaffyStyle {
    private TaffyDisplay display;
    private boolean itemIsTable;
    private TaffyBoxSizing boxSizing;

    private TaffyPoint<TaffyOverflow> overflow;
    private float scrollbarWidth;

    private TaffyPosition position;
    private TaffyRect<TaffyLengthPercentageAuto> inset;

    private TaffySize<TaffyDimension> size;
    private TaffySize<TaffyDimension> minSize;
    private TaffySize<TaffyDimension> maxSize;
    private Float aspectRatio;

    private TaffyRect<TaffyLengthPercentageAuto> margin;
    private TaffyRect<TaffyLengthPercentage> padding;
    private TaffyRect<TaffyLengthPercentage> border;

    private TaffyAlignItems alignItems;
    private TaffyAlignItems alignSelf;
    private TaffyAlignItems justifyItems;
    private TaffyAlignItems justifySelf;
    private TaffyAlignContent alignContent;
    private TaffyAlignContent justifyContent;
    private TaffySize<TaffyLengthPercentage> gap;

    private TaffyTextAlign textAlign;

    private TaffyFlexDirection flexDirection;
    private TaffyFlexWrap flexWrap;

    private TaffyDimension flexBasis;
    private float flexGrow;
    private float flexShrink;

    private List<TaffyTrackSizingFunction> gridTemplateRows;
    private List<TaffyTrackSizingFunction> gridTemplateColumns;
    private List<TaffyNonRepeatedTrackSizingFunction> gridAutoRows;
    private List<TaffyNonRepeatedTrackSizingFunction> gridAutoColumns;
    private TaffyGridAutoFlow gridAutoFlow;

    private TaffyLine<TaffyGenericGridPlacement> gridRow;
    private TaffyLine<TaffyGenericGridPlacement> gridColumn;

    public static TaffyStyle builder() {
        return new TaffyStyle();
    }

    public TaffyStyle display(TaffyDisplay display) {
        this.display = display;
        return this;
    }

    public TaffyStyle itemIsTable(boolean itemIsTable) {
        this.itemIsTable = itemIsTable;
        return this;
    }

    public TaffyStyle boxSizing(TaffyBoxSizing boxSizing) {
        this.boxSizing = boxSizing;
        return this;
    }

    public TaffyStyle overflow(TaffyPoint<TaffyOverflow> overflow) {
        this.overflow = overflow;
        return this;
    }

    public TaffyStyle scrollbarWidth(float scrollbarWidth) {
        this.scrollbarWidth = scrollbarWidth;
        return this;
    }

    public TaffyStyle position(TaffyPosition position) {
        this.position = position;
        return this;
    }

    public TaffyStyle inset(TaffyRect<TaffyLengthPercentageAuto> inset) {
        this.inset = inset;
        return this;
    }

    public TaffyStyle size(TaffySize<TaffyDimension> size) {
        this.size = size;
        return this;
    }

    public TaffyStyle minSize(TaffySize<TaffyDimension> minSize) {
        this.minSize = minSize;
        return this;
    }

    public TaffyStyle maxSize(TaffySize<TaffyDimension> maxSize) {
        this.maxSize = maxSize;
        return this;
    }

    public TaffyStyle aspectRatio(Float aspectRatio) {
        this.aspectRatio = aspectRatio;
        return this;
    }

    public TaffyStyle margin(TaffyRect<TaffyLengthPercentageAuto> margin) {
        this.margin = margin;
        return this;
    }

    public TaffyStyle padding(TaffyRect<TaffyLengthPercentage> padding) {
        this.padding = padding;
        return this;
    }

    public TaffyStyle border(TaffyRect<TaffyLengthPercentage> border) {
        this.border = border;
        return this;
    }

    public TaffyStyle alignItems(TaffyAlignItems alignItems) {
        this.alignItems = alignItems;
        return this;
    }

    public TaffyStyle alignSelf(TaffyAlignItems alignSelf) {
        this.alignSelf = alignSelf;
        return this;
    }

    public TaffyStyle justifyItems(TaffyAlignItems justifyItems) {
        this.justifyItems = justifyItems;
        return this;
    }

    public TaffyStyle justifySelf(TaffyAlignItems justifySelf) {
        this.justifySelf = justifySelf;
        return this;
    }

    public TaffyStyle alignContent(TaffyAlignContent alignContent) {
        this.alignContent = alignContent;
        return this;
    }

    public TaffyStyle justifyContent(TaffyAlignContent justifyContent) {
        this.justifyContent = justifyContent;
        return this;
    }

    public TaffyStyle gap(TaffySize<TaffyLengthPercentage> gap) {
        this.gap = gap;
        return this;
    }

    public TaffyStyle textAlign(TaffyTextAlign textAlign) {
        this.textAlign = textAlign;
        return this;
    }

    public TaffyStyle flexDirection(TaffyFlexDirection flexDirection) {
        this.flexDirection = flexDirection;
        return this;
    }

    public TaffyStyle flexWrap(TaffyFlexWrap flexWrap) {
        this.flexWrap = flexWrap;
        return this;
    }

    public TaffyStyle flexBasis(TaffyDimension flexBasis) {
        this.flexBasis = flexBasis;
        return this;
    }

    public TaffyStyle flexGrow(float flexGrow) {
        this.flexGrow = flexGrow;
        return this;
    }

    public TaffyStyle flexShrink(float flexShrink) {
        this.flexShrink = flexShrink;
        return this;
    }

    public TaffyStyle gridTemplateRows(List<TaffyTrackSizingFunction> gridTemplateRows) {
        this.gridTemplateRows = gridTemplateRows;
        return this;
    }

    public TaffyStyle gridTemplateColumns(List<TaffyTrackSizingFunction> gridTemplateColumns) {
        this.gridTemplateColumns = gridTemplateColumns;
        return this;
    }

    public TaffyStyle gridAutoRows(List<TaffyNonRepeatedTrackSizingFunction> gridAutoRows) {
        this.gridAutoRows = gridAutoRows;
        return this;
    }

    public TaffyStyle gridAutoColumns(List<TaffyNonRepeatedTrackSizingFunction> gridAutoColumns) {
        this.gridAutoColumns = gridAutoColumns;
        return this;
    }

    public TaffyStyle gridAutoFlow(TaffyGridAutoFlow gridAutoFlow) {
        this.gridAutoFlow = gridAutoFlow;
        return this;
    }

    public TaffyStyle gridRow(TaffyLine<TaffyGenericGridPlacement> gridRow) {
        this.gridRow = gridRow;
        return this;
    }

    public TaffyStyle gridColumn(TaffyLine<TaffyGenericGridPlacement> gridColumn) {
        this.gridColumn = gridColumn;
        return this;
    }
}
