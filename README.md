# Stretch
[![CircleCI](https://circleci.com/gh/vislyhq/stretch.svg?style=svg)](https://circleci.com/gh/vislyhq/stretch)

Highly experimental implementation of Flexbox written in [Rust](https://www.rust-lang.org). The goal of stretch is to provide a solid foundation for layout across all platforms with a specific focus on mobile. Long term we want stretch to not only support flexbox but also many other layout algorithms. Stretch is not yet used in production as it is still missing some core functionality but we have been very pleased with the development progress to date and hope to deploy it in production systems very soon.

## Goals
Before using or contributing to stretch it is good to be aware of the core goals of the project. These are goals we are working towards, not necessarily features we currently support.

- High performance
- Cross platform
- Small binary size
- Support multiple layout systems
- Multi-threaded layout
- Language bindings for most common languages

## Usage
Stretch is still in its initial development phase so we currently don't publish any artifacts to package managers. If you want to use stretch today or start contributing to the development of stretch you will need to build it locally.

### Installation
If you don't have rust installed you have to do that first as well as install some components that we make use of to format and lint the codebase. For more on rust see their [website](https://www.rust-lang.org).

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

### Testing
Stretch is tested by validating that layouts written in stretch perform the same as in Chrome. This is done by rendering an equivalent layout in HTML and then generating a rust test case which asserts that the resulting layout is the same when run through stretch.

You can run these tests without setting up a webdriver environment but if you are looking to add any test case you will need to install [chromedriver](http://chromedriver.chromium.org). If you are developing on macOS this is easy to do through brew.

```bash
brew tap homebrew/cask
brew cask install chromedriver
```

[Selenium](https://www.seleniumhq.org) is bundled in the repo so no need to download it. However you must ensure to have java installed first. Once you have everything install you can re-generate all tests by running `cargo run --manifest-path scripts/gentest/Cargo.toml`. To add a new test case add another HTML file to `/test_fixtures` following the current tests these as a template for new tests.

## Contributing
Constributions are very welcome. Though we ask that you open an issue or pull request early in the process (before writing code) so we can discuss solutions and additions before you start spending time on implementing them. There are some specific areas where we would be extra happy to recieve contributions in.

- [ ] Binary size reduction
- [ ] Runtime performance
- [ ] Benchmark tests
- [ ] Ensure build / test environment works well on non macOS platforms
- [ ] Alternate layout systems
- [ ] Web compatibility tests
- [ ] RTL support
- [ ] Incremental layout support
- [ ] Android platform bindings
- [ ] iOS platform bindings
- [ ] JavaScript platform bindings

## Relationship to Yoga
[Yoga](https://www.yogalayout.com) is a cross-platform implementation of Flebox written in C. Yoga is a fantastic project but has some fundamental issues which we hope to resolve. Compared to Yoga we aim to have a stronger adherence to web standards, a flexible architecture supporting multiple layout algorithms, and performance improvements including multi-threaded layout. In addition to this we aim to use a safer language with a more modern codebase.

# LICENCE
```
Copyright (c) 2018 Visly Inc.

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all
copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
SOFTWARE.
```
