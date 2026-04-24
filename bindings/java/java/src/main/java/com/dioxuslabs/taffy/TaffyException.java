package com.dioxuslabs.taffy;

/** Thrown when a Taffy operation fails (e.g. invalid node id). */
public class TaffyException extends RuntimeException {
    private static final long serialVersionUID = 1L;

    public TaffyException(String message) {
        super(message);
    }

    public TaffyException(String message, Throwable cause) {
        super(message, cause);
    }
}
