#### *RUST-SPECIFIC DOCUMENTATION CONFIGURATION AND INSTRUCTIONS*

- **Doc comments**. All public items (functions, structs, enums, traits, modules) must have
  `///` doc comments. Module-level documentation uses `//!` at the top of the file. Comments
  should describe what the item does and why, not restate its signature.
- **Doc examples**. Include a `# Examples` section with a fenced `rust` code block for every
  non-trivial public function. These examples are compiled and run by `cargo test --doc` and
  count as part of the test suite.
- **API doc preview**. Run `cargo doc --no-deps --open` to render and review the generated
  HTML documentation locally before considering a task complete.
- **MSRV**. `docs/README.md` must state the minimum supported Rust version (MSRV), e.g.
  "Requires Rust 1.70 or later." Update it whenever the MSRV changes.
- **`docs/README.md` structure**. In addition to the default template sections, Rust projects
  must include a "Building" section showing `cargo build --release` and a "Testing" section
  showing `cargo test`.
