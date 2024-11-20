package com.dioxuslabs.taffy.geom.measure;

public class LengthPercentageAuto {
    private final byte type;
    private final float value;

    private LengthPercentageAuto(byte type, float value) {
        this.type = type;
        this.value = value;
    }

    public static LengthPercentageAuto length(float value) {
        return new LengthPercentageAuto((byte) 0, value);
    }

    public static LengthPercentageAuto percent(float value) {
        return new LengthPercentageAuto((byte) 1, value);
    }

    public static LengthPercentageAuto auto() {
        return new LengthPercentageAuto((byte) 2, 0);
    }

    @Override
    public boolean equals(Object obj) {
        if (!(obj instanceof LengthPercentageAuto lpo)) {
            return false;
        }
        return type == lpo.type && value == lpo.value;
    }
}
