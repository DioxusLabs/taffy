package com.dioxuslabs.taffy.geom.grid;

import java.util.List;

/**
 * The sizing function for a grid track (row/column)
 * See <a href="https://developer.mozilla.org/en-US/docs/Web/CSS/grid-template-columns">...</a>
 */
public abstract class TrackSizingFunction {
    private byte type;

    private TrackSizingFunction(byte type) {
        this.type = type;
    }

    /**
     * A single non-repeated track
     */
    public static TrackSizingFunction single(NonRepeatedTrackSizingFunction func) {
        return new TrackSingleSizingFunction(func);
    }

    /**
     * Automatically generate grid tracks to fit the available space using the specified definite track lengths
     * Only valid if every track in template (not just the repetition) has a fixed size.
     */
    public static TrackSizingFunction repeat(GridTrackRepetition reps, List<NonRepeatedTrackSizingFunction> list) {
        return new TrackRepeatSizingFunction(reps, list);
    }

    private static class TrackSingleSizingFunction extends TrackSizingFunction {
        private final NonRepeatedTrackSizingFunction func;

        private TrackSingleSizingFunction(NonRepeatedTrackSizingFunction func) {
            super((byte) 0);
            this.func = func;
        }
    }

    private static class TrackRepeatSizingFunction extends TrackSizingFunction {
        private final GridTrackRepetition repetitions;
        private final List<NonRepeatedTrackSizingFunction> functions;

        private TrackRepeatSizingFunction(GridTrackRepetition repetitions,
                                          List<NonRepeatedTrackSizingFunction> functions) {
            super((byte) 1);
            this.repetitions = repetitions;
            this.functions = functions;
        }
    }
}
