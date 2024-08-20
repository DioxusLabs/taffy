package com.dioxuslabs.taffy.geom.grid;

/**
 * The sizing function for a grid track (row/column) (either auto-track or template track)
 * May either be a MinMax variant which specifies separate values for the min-/max- track sizing functions
 * or a scalar value which applies to both track sizing functions.
 *
 * @param min The value representing the minimum
 * @param max The value representing the maximum
 */
public record TaffyNonRepeatedTrackSizingFunction(
        TaffyMinTrackSizingFunction min,
        TaffyMaxTrackSizingFunction max
) {
}
