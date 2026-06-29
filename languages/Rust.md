<mark>THIS IS THE FILE CONTAINING ALL THE INSTRUCTIONS THAT MUST BE FOLLOWED
WHILE WRITING RUST CODE</mark>

**[AGENT READ-ONLY]** THIS FILE IS PART OF THE SDD-CONDUCTOR INFRASTRUCTURE.
THE AI AGENT MUST READ IT BUT MUST NEVER MODIFY IT UNDER ANY CIRCUMSTANCES.

**WARNING: THIS FILE SHOULD BE USED IN COMBINATION WITH ALL THE OTHER IMPORTED
FILES, NOT MADE TO OVERRIDE ANY PREVIOUSLY READ FILE. ANY INCONSISTENCIES
SHOULD BE REPORTED IMMEDIATELY, RESUMING WORK AFTER USER HAS CONFIRMED THEY WERE
CORRECTLY NOTIFIED.**


#### *General*
- You are an expert Rust software engineer.
- Your primary objective is to help develop software incrementally, safely, and
with minimal complexity, ensuring quality code and documentation.
- Prefer simple, idiomatic Rust.
- Avoid unnecessary abstractions.
- Avoid introducing traits, generics, macros, or additional modules unless there
is a clear need.
- Prefer composition over inheritance-like patterns.
- Keep functions and modules focused on a single responsibility.
- Minimize dependencies.


#### *Dependency management*
- **`Cargo.lock`**: commit it for binary/application crates (ensures reproducible builds); do not
commit it for library crates (lets downstream consumers resolve versions).
- **Semver**: prefer `^` ranges for new dependencies; avoid `*` wildcards; only enable crate
feature flags that are actually needed — do not blindly accept `default-features`.
- **Vetting**: before adding a dependency, verify it is actively maintained, has reasonable
download counts on crates.io, and has minimal transitive dependencies. Prefer crates from
well-established authors or the Rust ecosystem's standard set (e.g. `tokio`, `serde`, `clap`)
over unknown alternatives.


#### **QA checks**
- *Type-checking*. Start by running recursively `cargo check` until all bugs have been fixed. If
any changes were introduced due to `cargo check` reporting errors, make sure the expected
specification of the target project has been kept after the changes.
- *Code style*
        - After valid type-checking of the target project, run `cargo fmt` and
        `cargo clippy --all-targets --all-features -- -D warnings`. Follow standard Rust formatting
        (rustfmt). In particular, abide by a maximum of 100 column unless absolutely impossible to
        break the line into a number of columns not exceeding the 100 columns.
        - Prefer explicit, readable code over clever code.
        - Avoid excessive comments; code should be self-explanatory whenever possible.
        - Public APIs should have documentation comments.
        - *Error handling*
                - Use `Result` for recoverable errors.
                - Avoid `unwrap` and `expect` outside tests and prototypes.
                - Propagate errors instead of hiding them.
- *Testing*
        - There must be a unit test for each new functionality, especially core business logic.
        - Prefer unit tests close to the code being tested.
        - Keep tests simple and focused.
        - Make sure `cargo test` passes all tests.
- *Security audit*. Run `cargo audit` to check for known vulnerabilities in the dependency tree.
  Requires `cargo install cargo-audit` if not already installed. Resolve any findings before
  proceeding.
