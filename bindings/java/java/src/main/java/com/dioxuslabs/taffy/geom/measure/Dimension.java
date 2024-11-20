package com.dioxuslabs.taffy.geom.measure;

public class Dimension {
    private final byte type;
    private final float value;

    private Dimension(byte type, float value) {
        this.type = type;
        this.value = value;
    }

    public static Dimension length(float value) {
        return new Dimension((byte) 0, value);
    }

    public static Dimension percent(float value) {
        return new Dimension((byte) 1, value);
    }

    public static Dimension auto() {
        return new Dimension((byte) 2, 0);
    }

    @Override
    public boolean equals(Object obj) {
        if (!(obj instanceof Dimension lpo)) {
            return false;
        }
        return type == lpo.type && value == lpo.value;
    }
}
