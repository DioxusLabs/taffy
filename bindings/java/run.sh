cargo build &&
cd java &&
javac $(find . -name "*.java") &&
cd com/dioxuslabs/taffy &&
javac -h . ./**/*.java &&
cd ../../.. &&
java -Djava.library.path="../../../target/debug" com.dioxuslabs.taffy.Taffy
