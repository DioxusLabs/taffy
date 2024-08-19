package com.dioxuslabs.taffy.geom.grid;

import java.util.List;

/**
 * The sizing function for a grid track (row/column)
 * See <a href="https://developer.mozilla.org/en-US/docs/Web/CSS/grid-template-columns">...</a>
 */
public abstract class TaffyTrackSizingFunction {
    private byte type;

    private TaffyTrackSizingFunction(byte type) {
        this.type = type;
    }

    /**
     * A single non-repeated track
     */
    public static TaffyTrackSizingFunction single(TaffyNonRepeatedTrackSizingFunction func) {
        return new TaffyTrackSingleSizingFunction(func);
    }

    /**
     * Automatically generate grid tracks to fit the available space using the specified definite track lengths
     * Only valid if every track in template (not just the repetition) has a fixed size.
     */
    public static TaffyTrackSizingFunction repeat(TaffyGridTrackRepetition reps, List<TaffyNonRepeatedTrackSizingFunction> list) {
        return new TaffyTrackRepeatSizingFunction(reps, list);
    }

    private static class TaffyTrackSingleSizingFunction extends TaffyTrackSizingFunction {
        private final TaffyNonRepeatedTrackSizingFunction func;

        private TaffyTrackSingleSizingFunction(TaffyNonRepeatedTrackSizingFunction func) {
            super((byte) 0);
            this.func = func;
        }
    }

    private static class TaffyTrackRepeatSizingFunction extends TaffyTrackSizingFunction {
        private final TaffyGridTrackRepetition repetitions;
        private final List<TaffyNonRepeatedTrackSizingFunction> functions;

        private TaffyTrackRepeatSizingFunction(TaffyGridTrackRepetition repetitions,
                                               List<TaffyNonRepeatedTrackSizingFunction> functions) {
            super((byte) 1);
            this.repetitions = repetitions;
            this.functions = functions;
        }
    }
}
