//! This "benchmark" exists solely to provide a nicer error message for users running "cargo bench"
//! from the root of the repo. The correct way to run benchmarks is to run "cargo bench" from this
//! benches directory

fn main() {
    eprintln!();
    eprintln!("Error: Taffy's benchmarks are contained in a separate crate");
    eprintln!("Please run 'cargo bench' from within the 'benches' directory, or use the alias 'cargo xbench'");
    eprintln!();
    std::process::exit(1);
}
