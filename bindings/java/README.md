# Taffy Java bindings

JNI bindings to the [Taffy](../../README.md) layout engine.

Status: **WIP / MVP**. The flex and block subsets of the API are wired up;
CSS Grid, measure functions, node context, and `calc()` are not yet exposed.

## Layout

- `rust/` — a `cdylib` Rust crate (`taffy_java`) containing the JNI entry
  points. It is **not** part of the main Taffy workspace so a normal
  `cargo build` in the repo root does not require a JDK.
- `java/` — a Gradle project that compiles to `taffy-java-<version>.jar`
  and runs JUnit tests against the freshly built cdylib.

## Requirements

- Rust 1.71+ (matches Taffy's MSRV)
- JDK 8+ for consumers, JDK 17+ to run Gradle
- Gradle 8+ (tested with 9.4)

## Building

From `bindings/java/java/`:

```sh
gradle build       # compiles cargo + javac, packages jar
gradle test        # runs JUnit tests against the local cdylib
```

`:cargoBuild` shells out to `cargo build --release` in `../rust` and wires
the output directory into `java.library.path` for tests via the
`taffy.native.dir` system property.

## Using the library

```java
import dev.dioxus.taffy.*;

try (TaffyTree tree = new TaffyTree()) {
    long child = tree.newLeaf(Style.builder()
            .size(Dimension.percent(0.5f), Dimension.AUTO));

    long root = tree.newWithChildren(Style.builder()
            .size(Dimension.length(100f), Dimension.length(100f))
            .justifyContent(JustifyContent.CENTER),
            child);

    tree.computeLayout(root, 100f, 100f);

    Layout rootLayout  = tree.layout(root);
    Layout childLayout = tree.layout(child);
}
```

Every `TaffyTree` owns a native handle and must be closed (try-with-resources
is the idiomatic pattern). Nodes are plain `long` ids scoped to the tree
that produced them.

## Loading the native library

`NativeLoader` looks for the cdylib in this order:

1. `-Dtaffy.native.path=/abs/path/to/libtaffy_java.dylib` — explicit file
2. `-Dtaffy.native.dir=/abs/path/to/dir` — directory; the name is derived from
   `System.mapLibraryName("taffy_java")`
3. `/native/<os>-<arch>/<mappedName>` on the classpath (for shipping a jar
   with bundled binaries — not produced by the default build yet)
4. `System.loadLibrary("taffy_java")` using `java.library.path`

## Scope (MVP)

Exposed today:

- `TaffyTree`: create, drop, enable/disable rounding, `newLeaf`,
  `newWithChildren`, `setStyle`, `addChild`, `removeChild`, `childCount`,
  `markDirty`, `remove`, `computeLayout`, `layout`.
- `Style`: `display`, `position`, `boxSizing`, `overflow`, `scrollbarWidth`,
  `size` / `minSize` / `maxSize`, `margin`, `inset`, `padding`, `border`,
  `gap`, `flexDirection`, `flexWrap`, `flexGrow`, `flexShrink`, `flexBasis`,
  `justifyContent`, `alignContent`, `alignItems`, `alignSelf`, `aspectRatio`.
- Value types: `Dimension`, `LengthPercentage`, `LengthPercentageAuto`,
  `AvailableSpace`.
- Enums: `Display`, `Position`, `BoxSizing`, `Overflow`, `FlexDirection`,
  `FlexWrap`, `JustifyContent`, `AlignContent`, `AlignItems`, `AlignSelf`.

Not yet exposed:

- CSS Grid (`grid-template-rows`/`columns`, `grid-auto-*`, placement, gaps).
- Block-specific properties (`text-align`, float/clear).
- Measure functions / node context (needed for text nodes).
- `calc()` values.
- Detailed layout info (grid track info, etc.).
- Packaged binaries per OS/arch in the published jar.

## Design notes

- **Handles.** The Java side stores opaque `long` pointers for the tree and
  for transient native `Style` objects. All handles are owned — the JVM
  frees them deterministically, not via finalization.
- **Style materialization.** `Style` is a Java-side bag of optional fields.
  When a node is created it spins up a fresh native `Style` via JNI setters
  and discards it immediately after. This favors clarity over raw throughput;
  if needed we can later add a batched/serialized path.
- **Error reporting.** Taffy errors (`TaffyError`) are thrown as
  `dev.dioxus.taffy.TaffyException` across the JNI boundary.
- **Tagged value types.** `Dimension` and friends are encoded as a
  `(tag, float)` pair instead of a subclass hierarchy; this keeps the JNI
  ABI simple — every setter takes primitives only.
