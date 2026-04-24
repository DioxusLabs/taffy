plugins {
    `java-library`
}

group = "com.dioxuslabs"
version = "0.1.0-SNAPSHOT"

java {
    sourceCompatibility = JavaVersion.VERSION_1_8
    targetCompatibility = JavaVersion.VERSION_1_8
}

repositories {
    mavenCentral()
}

dependencies {
    testImplementation(platform("org.junit:junit-bom:5.10.2"))
    testImplementation("org.junit.jupiter:junit-jupiter")
    testRuntimeOnly("org.junit.platform:junit-platform-launcher")
}

// ---- Rust build integration ------------------------------------------------

val rustProjectDir = layout.projectDirectory.dir("../rust")
val rustTargetDir  = rustProjectDir.dir("target/release")

val libFileName: String = run {
    val osName = System.getProperty("os.name").lowercase()
    when {
        osName.contains("mac") || osName.contains("darwin") -> "libtaffy_java.dylib"
        osName.contains("windows") -> "taffy_java.dll"
        else -> "libtaffy_java.so"
    }
}

val cargoBuild by tasks.registering(Exec::class) {
    group = "build"
    description = "Compile the taffy_java Rust cdylib (release)."
    workingDir = rustProjectDir.asFile
    commandLine("cargo", "build", "--release")
    inputs.dir(rustProjectDir.dir("src"))
    inputs.file(rustProjectDir.file("Cargo.toml"))
    outputs.file(rustTargetDir.file(libFileName))
}

tasks.named<Test>("test") {
    dependsOn(cargoBuild)
    useJUnitPlatform()
    // Point NativeLoader at the freshly built cdylib.
    systemProperty("taffy.native.dir", rustTargetDir.asFile.absolutePath)
    // JDK 22+ emits a warning when System.load is called without explicit
    // native-access enablement. Opt in here so tests run clean.
    jvmArgs("--enable-native-access=ALL-UNNAMED")
    testLogging {
        events("passed", "failed", "skipped")
        showStandardStreams = true
    }
}

// ---- Examples --------------------------------------------------------------
// Each file under src/examples/java/com/dioxuslabs/taffy/examples/<Name>.java
// becomes a `./gradlew example<Name>` task, analogous to `cargo run --example`.

val examples by sourceSets.creating {
    java.srcDir("src/examples/java")
    compileClasspath += sourceSets["main"].output
    runtimeClasspath += sourceSets["main"].output
}

listOf("Basic", "FlexboxGap", "Nested", "Measure", "GridHolyGrail").forEach { name ->
    tasks.register<JavaExec>("example$name") {
        group = "examples"
        description = "Run the $name example."
        dependsOn(cargoBuild, tasks.named("examplesClasses"))
        classpath = examples.runtimeClasspath
        mainClass.set("com.dioxuslabs.taffy.examples.$name")
        systemProperty("taffy.native.dir", rustTargetDir.asFile.absolutePath)
        jvmArgs("--enable-native-access=ALL-UNNAMED")
    }
}
