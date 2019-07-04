# Stretch
[![CircleCI](https://circleci.com/gh/vislyhq/stretch.svg?style=svg)](https://circleci.com/gh/vislyhq/stretch)
[![Cargo](https://img.shields.io/crates/v/stretch.svg)](https://crates.io/crates/stretch)
[![npm](https://img.shields.io/npm/v/stretch-layout.svg)](https://www.npmjs.com/package/stretch-layout)
[![cocoapods](https://img.shields.io/cocoapods/v/StretchKit.svg)](https://cocoapods.org/pods/StretchKit)
[![bintray](https://api.bintray.com/packages/visly/maven/stretch-kotlin-bindings/images/download.svg)](https://bintray.com/visly/maven/stretch-kotlin-bindings)

Stretch is an implementation of Flexbox written in [Rust](https://www.rust-lang.org). The goal of stretch is to provide a solid foundation for layout across all platforms with a specific focus on mobile. Long term we want stretch to not only support flexbox but also many other layout algorithms such as grid layout. Stretch was made for and powers https://visly.app.

## Goals
Before using or contributing to stretch it is good to be aware of the core goals of the project. These are goals we are working towards, not necessarily features we currently support.

- High performance
- Cross platform
- Small binary size
- Support multiple layout systems
- Multi-threaded layout
- Language bindings for most common languages

## Supported Platforms

- Rust
- Android
- iOS
- JavaScript / TypeScript

## Usage
Stretch is built in Rust but comes with bindings to multiple languages and platforms so you can use it in a way that feels natural to your project.

### Rust
```toml
# Cargo.toml

[dependencies]
stretch = "0.3.2"
```

```rust
// main.rs

use stretch::geometry::Size;
use stretch::style::*;

fn main() -> Result<(), stretch::Error> {
    let mut stretch = stretch::node::Stretch::new();
    
    let child = stretch.new_node(
        Style { size: Size { width: Dimension::Percent(0.5), height: Dimension::Auto }, ..Default::default() },
        vec![],
    )?;

    let node = stretch.new_node(
        Style {
            size: Size { width: Dimension::Points(100.0), height: Dimension::Points(100.0) },
            justify_content: JustifyContent::Center,
            ..Default::default()
        },
        vec![child],
    )?;

    stretch.compute_layout(node, Size::undefined())?;
    dbg!(stretch.layout(node)?);
}

```

### Android
```groovy
// Build.gradle

android {
    splits {
        abi {
            enable true
        }
    }
}

dependencies {
    implementation 'app.visly.stretch:stretch:0.3.2'
}
```

```kotlin
// MainActivity.kt

val node = Node(
  Style(size = Size(Dimension.Points(100f), Dimension.Points(100f)), justifyContent = JustifyContent.Center), 
  listOf(
    Node(Style(size = Size(Dimension.Percent(0.5f), Dimension.Percent(0.5f))), listOf())
  ))

val layout = node.computeLayout(Size(null, null))
Log.d(TAG, "width: ${layout.width}, height: ${layout.height}")
```

### iOS
```ruby
# Podfile

pod 'StretchKit', '~> 0.3.2'
```

```swift
// ViewController.swift
 
let node = Node(
  style: Style(size: Size(width: .points(100.0), height: .points(100.0)), justifyContent: .center), 
  children: [
    Node(style: Style(size: Size(width: .percent(0.5), height: .percent(0.5))), children: [])
  ])
  
let layout = node.computeLayout(thatFits: Size(width: nil, height: nil))
print("width: \(layout.width), height: \(layout.height)")
```

### JavaScript
```bash
> npm install --save stretch-layout
```

```javascript
// index.js

import { Allocator, Node, JustifyContent } from 'stretch-layout';

const allocator = new Allocator();
const node = new Node(allocator, {width: 100, height: 100, justifyContent: JustifyContent.Center});
node.addChild(new Node(allocator, {width: '50%', height: '50%'}));
const layout = node.computeLayout();

console.log(layout.width, layout.height);
```

## Contributing
Contributions are very welcome. Though we ask that you open an issue or pull request early in the process (before writing code) so we can discuss solutions and additions before you start spending time on implementing them. There are some specific areas where we would be extra happy to receive contributions in.

- Binary size reduction
- Runtime performance
- Ensure build / test environment works well on non macOS platforms
- Alternate layout systems (grid layout perhaps?)
- Web compatibility tests
- RTL support
- Platform bindings
- API improvements
- Documentation & Examples

### Installation
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
Stretch is tested by validating that layouts written in stretch perform the same as in Chrome. This is done by rendering an equivalent layout in HTML and then generating a Rust test case which asserts that the resulting layout is the same when run through stretch.

You can run these tests without setting up a webdriver environment but if you are looking to add any test case you will need to install [chromedriver](http://chromedriver.chromium.org). If you are developing on macOS this is easy to do through brew.

```bash
brew tap homebrew/cask
brew cask install chromedriver
```

Once you have chromedriver installed and available in `PATH` you can re-generate all tests by running `cargo run --package gentest`.

To add a new test case add another HTML file to `/test_fixtures` following the current tests as a template for new tests.

### Benchmarking
Benchmarks build on the same infrastructure as testing, and actually benchmarks are automatically generated from test fixtures just like tests. Run `cargo bench` to run benchmarks locally.

## Relationship to Yoga
[Yoga](https://www.yogalayout.com) is a cross-platform implementation of Flexbox written in C. Yoga is a fantastic project but has some fundamental issues which we hope to resolve. Compared to Yoga we aim to have a stronger adherence to web standards, a flexible architecture eventually supporting multiple layout algorithms, and future performance improvements including multi-threaded layout. In addition to this we aim to use a safer language with a more modern codebase.

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
