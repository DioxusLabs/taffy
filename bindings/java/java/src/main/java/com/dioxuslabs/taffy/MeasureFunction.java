package com.dioxuslabs.taffy;

/**
 * Callback invoked by Taffy to measure the intrinsic size of a leaf node
 * (e.g. a block of text or an image). Install one via
 * {@link TaffyTree#newLeafWithMeasure(Style, MeasureFunction)} or
 * {@link TaffyTree#setMeasure(long, MeasureFunction)}.
 *
 * <p>Values are passed as primitives to avoid per-call allocation across the
 * JNI boundary.
 *
 * <h2>Parameters</h2>
 * <ul>
 *   <li>{@code knownWidth} / {@code knownHeight} — the constraint already
 *   imposed on this axis, or {@link Float#NaN} if the axis is unconstrained.
 *   Implementations should return the known value unchanged on that axis.</li>
 *   <li>{@code availWidthTag}, {@code availWidthValue} — the
 *   {@link AvailableSpace} encoded as {@code (tag, value)}. See the
 *   {@code AVAIL_*} constants in {@code NativeBridge}.</li>
 * </ul>
 *
 * <h2>Return</h2>
 * A 2-element {@code float[]} of {@code {width, height}}. Exceptions thrown
 * by the callback are caught on the native side and treated as
 * {@code {0, 0}} — they do not abort layout.
 */
@FunctionalInterface
public interface MeasureFunction {

    float[] measure(float knownWidth, float knownHeight,
                    int availWidthTag, float availWidthValue,
                    int availHeightTag, float availHeightValue);
}
