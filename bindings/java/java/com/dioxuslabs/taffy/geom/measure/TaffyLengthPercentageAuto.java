package com.dioxuslabs.taffy.geom.measure;

public class TaffyLengthPercentageAuto {
    private final byte type;
    private final float value;

    private TaffyLengthPercentageAuto(byte type, float value) {
        this.type = type;
        this.value = value;
    }

    public static TaffyLengthPercentageAuto length(float value) {
        return new TaffyLengthPercentageAuto((byte) 0, value);
    }

    public static TaffyLengthPercentageAuto percent(float value) {
        return new TaffyLengthPercentageAuto((byte) 1, value);
    }

    public static TaffyLengthPercentageAuto auto() {
        return new TaffyLengthPercentageAuto((byte) 2, 0);
    }

    @Override
    public boolean equals(Object obj) {
        if (!(obj instanceof TaffyLengthPercentageAuto lpo)) {
            return false;
        }
        return type == lpo.type && value == lpo.value;
    }
}
