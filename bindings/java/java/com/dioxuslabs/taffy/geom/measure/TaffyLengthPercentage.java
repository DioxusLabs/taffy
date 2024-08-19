package com.dioxuslabs.taffy.geom.measure;

public class TaffyLengthPercentage {
    private final byte type;
    private final float value;

    private TaffyLengthPercentage(byte type, float value) {
        this.type = type;
        this.value = value;
    }

    public static TaffyLengthPercentage length(float value) {
        return new TaffyLengthPercentage((byte) 0, value);
    }

    public static TaffyLengthPercentage percentage(float value) {
        return new TaffyLengthPercentage((byte) 1, value);
    }

    @Override
    public boolean equals(Object obj) {
        if (!(obj instanceof TaffyLengthPercentage lpo)) {
            return false;
        }
        return type == lpo.type && value == lpo.value;
    }
}
