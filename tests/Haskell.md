#### *HASKELL-SPECIFIC TEST SUITE INSTRUCTIONS*

- **Test framework**. Use `HUnit` for unit tests. If property-based testing is needed in
  addition, add `QuickCheck`. Avoid heavier frameworks (e.g. `tasty`) unless the test suite
  spans multiple test types that justify the extra dependency.
- **Test placement**. Tests live in a top-level `test/` directory. The entry point is
  `test/Spec.hs`. The `.cabal` file must declare a `test-suite` stanza pointing to
  `test/Spec.hs` with `type: exitcode-stdio-1.0`.
- **Test naming**. Name test cases as descriptive strings, e.g.
  `"parse returns error on empty input" ~: ...`. Names should read as plain English
  descriptions.
- **Test isolation**. Each test must be self-contained: set up its own state, make no
  assumptions about execution order, and clean up any temporary files it creates
  (via `System.Directory.removeFile`).
- **Running tests**. Use `cabal test` for the full suite. Pass
  `--test-show-details=direct` to see individual test case results.
