package com.dioxuslabs.taffy.geom.measure;

public class LengthPercentage {
    private final byte type;
    private final float value;

    private LengthPercentage(byte type, float value) {
        this.type = type;
        this.value = value;
    }

    public static LengthPercentage length(float value) {
        return new LengthPercentage((byte) 0, value);
    }

    public static LengthPercentage percent(float value) {
        return new LengthPercentage((byte) 1, value);
    }

    public static LengthPercentage zero() {
        return length(0);
    }

    @Override
    public boolean equals(Object obj) {
        if (!(obj instanceof LengthPercentage lpo)) {
            return false;
        }
        return type == lpo.type && value == lpo.value;
    }
}
