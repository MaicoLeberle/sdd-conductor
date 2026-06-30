#### *HASKELL-SPECIFIC LOGGING CONFIGURATION*

- **Library selection**.
  - For simple CLI tools: use `System.IO.hPutStrLn stderr` for error and diagnostic output.
    No logging library is required.
  - For applications that need explicit log levels: use the `hslogger` package
    (`System.Log.Logger`). Initialize with `updateGlobalLogger rootLoggerName
    (setLevel WARNING)` at the start of `main`.
- **Log levels**. Use `debugM`, `infoM`, `warningM`, `errorM` from `System.Log.Logger`.
  For stderr-only output without `hslogger`, prefix messages manually with `[ERROR]` or
  `[WARN]` for clarity.
- **Debug-only instrumentation**. Do not use `Debug.Trace` in production code. Guard
  verbose diagnostic output behind a verbosity flag in the options/`Params` record, not at
  compile time.
- **Security**. Never log values that may contain secrets: API keys, passwords, tokens, or
  personally identifiable information. This applies to all log levels.
