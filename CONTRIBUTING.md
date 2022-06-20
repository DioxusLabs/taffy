# Contributing

This is a cross-team project, aiming to build solid foundations for Rust UI libraries of all sorts.
New contributions are extremely welcome!

The basic process is simple:

1. Pick an [issue](https://github.com/DioxusLabs/taffy/issues?q=is%3Aissue+is%3Aopen+label%3A%22good+first+issue%22), or [file a new one](https://github.com/DioxusLabs/taffy/issues/new).
2. Comment in the issue that you plan to tackle it, and the team will assign the task to you.
3. Submit a PR.
4. Respond to feedback from reviewers and make sure CI passes.

The PR review process is completely open:  help us by commenting on, testing and approving PRs.

If you'd like to help on a consistent basis or are interested in project management, create a Discussions post, and we'll be happy to hand out triage rights.

## Testing

### Running tests

Flexbox layouts are tested by validating that layouts written in this crate perform the same as in Chrome.
This is done by rendering an equivalent layout in HTML and then generating a Rust test case which asserts that the resulting layout is the same when run through our layout engine.

You can run these tests without setting up a webdriver environment but if you are looking to add any test case you will need to install [chromedriver](http://chromedriver.chromium.org) and [Chrome](https://www.google.com/chrome/).
If you are developing on macOS this is easy to do through brew.

```bash
brew install chromedriver
```

Once you have chromedriver installed and available in `PATH` you can re-generate all tests by running `cargo run --package gentest`. You should not manually update the tests in `tests/generated`. Instead, fix the script in `scripts/gentest/` and re-generate them. This can happen after a refactor. It can be helpful to commit the updated tests in a dedicated commit so that they can be easier to ignore during review.

To add a new test case add another HTML file to `/test_fixtures` following the current tests as a template for new tests.

### Writing tests

1. All tests should be wrapped in a module called `tests`, to ensure they are not compiled unless testing

    ```rs
    #[cfg(test)]
    mod tests {
        // Place tests here
    }
    ```

2. For unit-testing this should be placed within the file of the definition they are testing
    1. If file becomes significantly long, tests should be split out into its own file under the same module

    ```rs
    // file: ./src/my_struct.rs
    struct MyStruct;

    impl MyStruct {
        fn some_method() { .. }
    }

    #[cfg(test)]
    mod tests {
        #[test]
        fn test_of_some_method() { .. }
    }
    ```

3. For integration tests this should be placed within the `./tests` folder

    ```rs
    // file: ./tests/my_integration_test.rs
    #[cfg(test)]
    mod tests {
        #[test]
        fn integration_test_one() { .. }
        #[test]
        fn integration_test_two() { .. }
    }
    ```

4. Each test should have a clear intent
    1. It should be evident what is being tested (naming, code, comments)
    2. When this test fails, it should be easy to understand what went wrong

## Benchmarking

Benchmarks build on the same infrastructure as testing, and actually benchmarks are automatically generated from test fixtures just like tests.
Run `cargo bench` to run benchmarks locally.
