package com.dioxuslabs.taffy.geom.measure;

public class AvailableSpace {
    private final byte type;
    private final float value;

    private AvailableSpace(byte type, float value) {
        this.type = type;
        this.value = value;
    }

    public static AvailableSpace definite(float value) {
        return new AvailableSpace((byte) 0, value);
    }

    public static AvailableSpace minContent() {
        return new AvailableSpace((byte) 1, 0);
    }

    public static AvailableSpace maxContent() {
        return new AvailableSpace((byte) 2, 0);
    }

    @Override
    public boolean equals(Object obj) {
        if (!(obj instanceof AvailableSpace as)) {
            return false;
        }
        return type == as.type && value == as.value;
    }
}
