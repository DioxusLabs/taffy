# Stretch2

[![GitHub CI](https://github.com/DioxusLabs/stretch/actions/workflows/ci.yml/badge.svg)](https://github.com/DioxusLabs/stretch/actions/workflows/ci.yml)
[![crates.io](https://img.shields.io/crates/v/stretch2.svg)](https://crates.io/crates/stretch2)

> THIS IS A FORK OF THE ORIGINAL STRETCH. CURRENTLY MAINTAINED BY DIOXUS.

Stretch2 is an implementation of Flexbox written in [Rust](https://www.rust-lang.org).
The goal of stretch2 is to provide a solid foundation for layout across all platforms with a specific focus on mobile.
Long term we want stretch2 to not only support flexbox but also many other layout algorithms such as grid layout.
Stretch was made for and powers <https://visly.app>.

## Goals

Before using or contributing to stretch2 it is good to be aware of the core goals of the project. These are goals we are working towards, not necessarily features we currently support.

- High performance
- Cross platform
- Small binary size
- Support multiple layout systems
- Multi-threaded layout

## Usage

```toml
# Cargo.toml

[dependencies]
stretch2 = "0.4.3"
```

If you don't have Rust installed you have to do that first as well as install some components that we make use of to format and lint the codebase. For more on Rust see their [website](https://www.rust-lang.org).

```bash
curl https://sh.rustup.rs -sSf | sh
rustup component add rustfmt
rustup component add clippy
```

Once that is done you can clone the repo and do some sanity checks to make sure everything is working correctly.

```bash
git clone https://github.com/vislyhq/stretch.git
cd stretch
cargo test
```

If you have made any changes to the API you should also update and run tests for all the platform bindings located in `/bindings/*`.

### Testing

Stretch2 is tested by validating that layouts written in stretch2 perform the same as in Chrome.
This is done by rendering an equivalent layout in HTML and then generating a Rust test case which asserts that the resulting layout is the same when run through stretch.

You can run these tests without setting up a webdriver environment but if you are looking to add any test case you will need to install [chromedriver](http://chromedriver.chromium.org).
If you are developing on macOS this is easy to do through brew.

```bash
brew tap homebrew/cask
brew cask install chromedriver
```

Once you have chromedriver installed and available in `PATH` you can re-generate all tests by running `cargo run --package gentest`.

To add a new test case add another HTML file to `/test_fixtures` following the current tests as a template for new tests.

### Benchmarking

Benchmarks build on the same infrastructure as testing, and actually benchmarks are automatically generated from test fixtures just like tests.
Run `cargo bench` to run benchmarks locally.
