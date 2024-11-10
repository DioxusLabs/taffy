gentest:
  cargo run --release --package gentest --

import-yoga-tests:
  cargo run --package import-yoga-tests --

format-fixtures:
  cargo run --package format-fixtures --

bench:
  cargo bench --package taffy_benchmarks

clippy:
  cargo +nightly clippy --workspace

fmt:
  cargo fmt --all