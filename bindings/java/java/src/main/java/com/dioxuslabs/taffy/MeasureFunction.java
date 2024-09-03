package com.dioxuslabs.taffy;

import com.dioxuslabs.taffy.geom.Size;
import com.dioxuslabs.taffy.geom.measure.AvailableSpace;
import com.dioxuslabs.taffy.style.Style;

public interface MeasureFunction {
    Size<Float> measure(
            Size<Float> knownDimensions,
            Size<AvailableSpace> availableSpace,
            long nodeId,
            Object nodeContext,
            Style style
    );
}
