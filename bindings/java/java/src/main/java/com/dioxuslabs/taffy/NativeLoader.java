package com.dioxuslabs.taffy;

import java.io.IOException;
import java.io.InputStream;
import java.nio.file.Files;
import java.nio.file.Path;
import java.nio.file.Paths;
import java.nio.file.StandardCopyOption;

/**
 * Loads the {@code taffy_java} native library.
 *
 * <p>Resolution order:
 * <ol>
 *   <li>{@code taffy.native.path} system property — absolute path to the library file.</li>
 *   <li>{@code taffy.native.dir} system property — directory containing the library; name is
 *       derived via {@link System#mapLibraryName(String)}.</li>
 *   <li>A bundled copy on the classpath at {@code /native/<os>-<arch>/<mappedName>}. If present
 *       it is extracted to a temp file and loaded.</li>
 *   <li>{@link System#loadLibrary(String)} fallback using the standard {@code java.library.path}.</li>
 * </ol>
 */
final class NativeLoader {

    private static final String LIB_NAME = "taffy_java";
    private static boolean loaded = false;

    private NativeLoader() {}

    static synchronized void ensureLoaded() {
        if (loaded) return;

        String explicitPath = System.getProperty("taffy.native.path");
        if (explicitPath != null && !explicitPath.isEmpty()) {
            System.load(explicitPath);
            loaded = true;
            return;
        }

        String explicitDir = System.getProperty("taffy.native.dir");
        if (explicitDir != null && !explicitDir.isEmpty()) {
            Path p = Paths.get(explicitDir).resolve(System.mapLibraryName(LIB_NAME));
            System.load(p.toAbsolutePath().toString());
            loaded = true;
            return;
        }

        if (tryLoadFromClasspath()) {
            loaded = true;
            return;
        }

        System.loadLibrary(LIB_NAME);
        loaded = true;
    }

    private static boolean tryLoadFromClasspath() {
        String resource = "/native/" + detectTriple() + "/" + System.mapLibraryName(LIB_NAME);
        try (InputStream in = NativeLoader.class.getResourceAsStream(resource)) {
            if (in == null) return false;
            Path tmp = Files.createTempFile("taffy_java", libExtension());
            tmp.toFile().deleteOnExit();
            Files.copy(in, tmp, StandardCopyOption.REPLACE_EXISTING);
            System.load(tmp.toAbsolutePath().toString());
            return true;
        } catch (IOException e) {
            throw new TaffyException("failed to extract bundled native library: " + resource, e);
        }
    }

    private static String detectTriple() {
        String os = System.getProperty("os.name", "").toLowerCase();
        String arch = System.getProperty("os.arch", "").toLowerCase();
        String osPart;
        if (os.contains("mac") || os.contains("darwin")) osPart = "macos";
        else if (os.contains("win")) osPart = "windows";
        else osPart = "linux";
        String archPart;
        if (arch.contains("aarch64") || arch.contains("arm64")) archPart = "aarch64";
        else if (arch.contains("64")) archPart = "x86_64";
        else archPart = arch;
        return osPart + "-" + archPart;
    }

    private static String libExtension() {
        String os = System.getProperty("os.name", "").toLowerCase();
        if (os.contains("mac") || os.contains("darwin")) return ".dylib";
        if (os.contains("win")) return ".dll";
        return ".so";
    }
}
