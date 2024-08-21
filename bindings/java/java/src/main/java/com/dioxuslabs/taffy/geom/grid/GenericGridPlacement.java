package com.dioxuslabs.taffy.geom.grid;

import com.dioxuslabs.taffy.style.GridAutoFlow;

/**
 * A grid line placement specification which is generic over the coordinate system that it uses to define
 * grid line positions.
 * <p>
 * GenericGridPlacement<GridLine> is aliased as GridPlacement and is exposed to users of Taffy to define styles.
 * GenericGridPlacement<OriginZeroLine> is aliased as OriginZeroGridPlacement and is used internally for placement computations.
 * <p>
 * See [`crate::compute::grid::type::coordinates`] for documentation on the different coordinate systems.
 */
public class GenericGridPlacement {
    private final byte type;
    private final short value;

    private GenericGridPlacement(byte type, short value) {
        this.type = type;
        this.value = value;
    }

    /**
     * Place item according to the auto-placement algorithm, and the parent's {@link GridAutoFlow} property
     */
    public static GenericGridPlacement auto() {
        return new GenericGridPlacement((byte) 0, (short) 0);
    }

    /**
     * Place item at specified line (column or row) index
     */
    public static GenericGridPlacement line(short value) {
        return new GenericGridPlacement((byte) 1, value);
    }

    /**
     * Item should span specified number of tracks (columns or rows)
     */
    public static GenericGridPlacement span(short value) {
        return new GenericGridPlacement((byte) 2, value);
    }

    @Override
    public boolean equals(Object obj) {
        if (!(obj instanceof GenericGridPlacement gpg)) {
            return false;
        }
        return type == gpg.type && value == gpg.value;
    }
}
