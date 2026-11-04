# Import Yoga Tests script

This is a rough and ready script for the purpose of syncing test fixtures from the [Yoga](https://github.com/facebook/yoga/) which Taffy's predecessor Stretch was originally descended from, and whose tests are compatible with Taffy's generated test infrastructure.

## Usage

- Clone the [yoga](https://github.com/facebook/yoga/) repo locally
- Set the `YOGA_FIXTURE_DIR` environment variable to the absolute path of `gentest/fixtures` directory within your local copy of the repo
- Run `cargo run` in this directory (or `just import-yoga-tests` in the root directory of the Taffy repo)
- Manually inspect the imported tests (if any - there may not be any new tests since the last sync) to check that they make sense for Taffy (and that they're not for example testing features we don't implement)
- Run `just gentest` in the root directory of the Taffy repo to generate actual tests from the fixtures
- Run `cargo test` to ensure that the new tests pass
- If any tests do not pass, either fix them or prefix the fixture file with an `x` to disable it (e.g. `align_content_stretch.html` would become `xalign_content_stretch.html`) and rerun `cargo gentest` to automatically remove the generated tests for that fixture.

Fixtures are searched recursively (including disabled fixtures) and new fixtures are written to
`test_fixtures/flex/`. The importer preserves Yoga's column direction, zero flex shrink and
`align-content: flex-start` defaults explicitly; Taffy's base stylesheet already supplies relative
positioning. Existing inline declarations, including the `flex` shorthand, take precedence.
The historical gap renames (`row_gap` → `gap_row_gap`, `column_gap` → `gap_column_gap`) are retained.

Review tests with text, root offsets, or a static root: the test harness uses a synthetic text
measure function and treats the layout root as its initial containing block. Unsupported fixtures
should remain disabled rather than changing their expected values to match Taffy.
