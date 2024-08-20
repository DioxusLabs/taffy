package com.dioxuslabs.taffy.geom.measure;

public class TaffyAvailableSpace {
    private final byte type;
    private final float value;

    private TaffyAvailableSpace(byte type, float value) {
        this.type = type;
        this.value = value;
    }

    public static TaffyAvailableSpace definite(float value) {
        return new TaffyAvailableSpace((byte) 0, value);
    }

    public static TaffyAvailableSpace minContent() {
        return new TaffyAvailableSpace((byte) 1, 0);
    }

    public static TaffyAvailableSpace maxContent() {
        return new TaffyAvailableSpace((byte) 2, 0);
    }

    @Override
    public boolean equals(Object obj) {
        if (!(obj instanceof TaffyAvailableSpace as)) {
            return false;
        }
        return type == as.type && value == as.value;
    }
}
