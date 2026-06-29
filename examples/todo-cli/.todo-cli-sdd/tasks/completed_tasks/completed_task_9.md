[AGENT-MANAGED FILE — DO NOT MODIFY MANUALLY]

# Completed task
## Title
Write project documentation

## Description
Complete `docs/README.md` with all sections required by the Rust documentation configuration:
project name and one-sentence description, Installation section, Building section showing
`cargo build --release`, Usage section with command examples for all four subcommands (`add`,
`list`, `done`, `delete`), Testing section showing `cargo test`, MSRV statement (Rust stable),
Contributing section, and License section. Add `///` doc comments to all public items in
`src/`. Verify documentation renders correctly.

## Completion criteria
- `docs/README.md` contains all required sections: description, Installation, Building, Usage
  (with examples for all four commands), Testing, MSRV, Contributing, License.
- All public items in `src/` have `///` doc comments.
- `cargo doc --no-deps` produces no errors or warnings.

## Commit
0c03d02
