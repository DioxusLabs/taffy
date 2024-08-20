package com.dioxuslabs.taffy.geom.measure;

public class TaffyDimension {
    private final byte type;
    private final float value;

    private TaffyDimension(byte type, float value) {
        this.type = type;
        this.value = value;
    }

    public static TaffyDimension length(float value) {
        return new TaffyDimension((byte) 0, value);
    }

    public static TaffyDimension percent(float value) {
        return new TaffyDimension((byte) 1, value);
    }

    public static TaffyDimension auto() {
        return new TaffyDimension((byte) 2, 0);
    }

    @Override
    public boolean equals(Object obj) {
        if (!(obj instanceof TaffyDimension lpo)) {
            return false;
        }
        return type == lpo.type && value == lpo.value;
    }
}
