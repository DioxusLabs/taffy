package com.dioxuslabs.taffy;

/**
 * A single component of a {@code grid-template-columns} / {@code grid-template-rows}
 * value: either a non-repeated track ({@link #track}) or a {@code repeat()}
 * expression.
 *
 * <p>Repeats come in three flavors:
 * <ul>
 *   <li>{@link #repeat(int, GridTrack...)} — {@code repeat(N, tracks…)}</li>
 *   <li>{@link #autoFill(GridTrack...)} — {@code repeat(auto-fill, tracks…)}</li>
 *   <li>{@link #autoFit(GridTrack...)}   — {@code repeat(auto-fit, tracks…)}</li>
 * </ul>
 *
 * <p>{@code grid-auto-*} properties do not use this type — they take a
 * flat {@link GridTrack} list.
 */
public final class GridTemplate {

    // Kinds (mirror lib.rs).
    static final int KIND_SINGLE = 0;
    static final int KIND_REPEAT = 1;

    // Repetition kinds (mirror lib.rs).
    static final int REP_AUTO_FILL = 0;
    static final int REP_AUTO_FIT  = 1;
    static final int REP_COUNT     = 2;

    private final int kind;
    private final int repKind;
    private final int repCount;
    private final GridTrack[] tracks;

    private GridTemplate(int kind, int repKind, int repCount, GridTrack[] tracks) {
        this.kind = kind;
        this.repKind = repKind;
        this.repCount = repCount;
        this.tracks = tracks;
    }

    /** Wrap a single {@link GridTrack} as a template component. */
    public static GridTemplate track(GridTrack t) {
        java.util.Objects.requireNonNull(t, "t");
        return new GridTemplate(KIND_SINGLE, 0, 0, new GridTrack[] {t});
    }

    /** CSS {@code repeat(<count>, <tracks…>)}. {@code count} must be positive. */
    public static GridTemplate repeat(int count, GridTrack... tracks) {
        if (count <= 0) throw new IllegalArgumentException("repeat count must be positive");
        if (tracks == null || tracks.length == 0) {
            throw new IllegalArgumentException("repeat needs at least one track");
        }
        return new GridTemplate(KIND_REPEAT, REP_COUNT, count, tracks.clone());
    }

    /** CSS {@code repeat(auto-fill, <tracks…>)}. */
    public static GridTemplate autoFill(GridTrack... tracks) {
        if (tracks == null || tracks.length == 0) {
            throw new IllegalArgumentException("repeat needs at least one track");
        }
        return new GridTemplate(KIND_REPEAT, REP_AUTO_FILL, 0, tracks.clone());
    }

    /** CSS {@code repeat(auto-fit, <tracks…>)}. */
    public static GridTemplate autoFit(GridTrack... tracks) {
        if (tracks == null || tracks.length == 0) {
            throw new IllegalArgumentException("repeat needs at least one track");
        }
        return new GridTemplate(KIND_REPEAT, REP_AUTO_FIT, 0, tracks.clone());
    }

    int         kind()     { return kind; }
    int         repKind()  { return repKind; }
    int         repCount() { return repCount; }
    GridTrack[] tracks()   { return tracks; }
}
