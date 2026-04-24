gentest:
  cargo run --release --package gentest --

import-yoga-tests:
  cargo run --package import-yoga-tests --

format-fixtures:
  cargo run --package format-fixtures --

[working-directory: 'benches']
bench *ARGS:
  cargo bench {{ARGS}}

clippy:
  cargo +nightly clippy --workspace

fmt:
  cargo fmt --all