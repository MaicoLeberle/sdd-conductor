<mark>THIS IS THE FILE CONTAINING ALL LOGGING INSTRUCTIONS FOR PROJECTS USING
RUST. THIS MAY OVERRIDE THE LOGGING DEFAULT CONFIGURATION IN THE MAIN FILE IN
THIS SAME DIRECTORY</mark>

**[AGENT READ-ONLY]** THIS FILE IS PART OF THE SDD-CONDUCTOR INFRASTRUCTURE.
THE AI AGENT MUST READ IT BUT MUST NEVER MODIFY IT UNDER ANY CIRCUMSTANCES.

**WARNING: THIS MAY OVERRIDE THE LOGGING DEFAULT CONFIGURATION IN THE MAIN FILE
IN THIS SAME DIRECTORY. IN SUCH CASE, YOU MUST INFORM THE USER ABOUT THE
OVERRIDDEN CONFIGURATION.**


#### *RUST-SPECIFIC LOGGING CONFIGURATION*

- **Crate selection**.
  - For CLI tools and simple applications: use the `log` facade crate with `env_logger` as
    the backend. Initialize with `env_logger::init()` at the top of `main()`.
  - For libraries, async applications, or anything using `tokio`: use `tracing` with
    `tracing-subscriber` as the backend. Initialize with
    `tracing_subscriber::fmt::init()` at the top of `main()`. Libraries must only depend on
    the `tracing` facade — never on `tracing-subscriber`.
- **Log macros**. Use the appropriate level macros: `debug!`, `info!`, `warn!`, `error!`.
  Prefer structured fields over string interpolation where `tracing` is in use, e.g.
  `tracing::debug!(path = %file_path, "opening file")`.
- **Debug-only instrumentation**. Gate debug logging that is too verbose for production with
  `#[cfg(debug_assertions)]`. Do not use `cfg!(debug_assertions)` at runtime for logging
  decisions — use the log level filtering provided by `env_logger` or `tracing-subscriber`
  instead.
- **Security**. Never log values that may contain secrets: API keys, passwords, tokens,
  private keys, or personally identifiable information. This applies to all log levels,
  including `debug!`.
