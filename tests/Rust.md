<mark>THIS IS THE FILE CONTAINING ALL TEST SUITE INSTRUCTIONS FOR PROJECT USING
RUST. </mark>

**[AGENT READ-ONLY]** THIS FILE IS PART OF THE SDD-CONDUCTOR INFRASTRUCTURE.
THE AI AGENT MUST READ IT BUT MUST NEVER MODIFY IT UNDER ANY CIRCUMSTANCES.

**WARNING: THIS MAY OVERRIDE THE DEFAULT CONFIGURATION FOR TEST SUITES IN THE
MAIN FILE IN THIS SAME DIRECTORY. IN SUCH CASE, YOU MUST INFORM THE USER ABOUT
THE OVERRIDDEN CONFIGURATION.**

#### *RUST-SPECIFIC TEST SUITE INSTRUCTIONS*

- **Unit test placement**. Unit tests live in a `#[cfg(test)]` module at the bottom of the
  source file they test (`mod tests { use super::*; ... }`). Do not place unit tests in
  separate files.
- **Integration tests**. Tests that exercise the public API end-to-end belong in a top-level
  `tests/` directory. Each file in `tests/` is compiled as a separate crate. Shared helpers
  go in `tests/common/mod.rs` (not in `src/`).
- **Doc tests**. Code examples in `///` documentation comments are compiled and executed by
  `cargo test --doc`. Every non-trivial public function should have at least one doc example,
  and it must compile and produce the shown output.
- **Test naming**. Use the convention `test_<function_or_behaviour>_<scenario>`, e.g.
  `test_parse_returns_error_on_empty_input`. Names should read as plain English descriptions.
- **Test isolation**. Each test must be self-contained: set up its own state, make no
  assumptions about execution order, and clean up any temporary files or directories it
  creates.
- **Running tests**. Use `cargo test` for the full suite. Use `cargo test --doc` to run doc
  tests separately. Pass `-- --nocapture` to see `println!` output while debugging a failing
  test.
