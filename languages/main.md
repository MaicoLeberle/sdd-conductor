<mark>THIS IS THE MAIN FILE OF ALL SUPPORTED PROGRAMMING LANGUAGES AND RELATED
LIBRARIES</mark>

**[AGENT READ-ONLY]** THIS FILE IS PART OF THE SDD-CONDUCTOR INFRASTRUCTURE.
THE AI AGENT MUST READ IT BUT MUST NEVER MODIFY IT UNDER ANY CIRCUMSTANCES.

**WARNING: THIS FILE SHOULD BE USED IN COMBINATION WITH ALL THE OTHER IMPORTED
FILES, NOT MADE TO OVERRIDE ANY PREVIOUSLY READ FILE. ANY INCONSISTENCIES
SHOULD BE REPORTED IMMEDIATELY, RESUMING WORK AFTER USER HAS CONFIRMED THEY WERE
CORRECTLY NOTIFIED.**

## Generic programming language defaults
The following guidelines apply to any programming language unless overridden by a
language-specific file:
- Prefer simple, readable, idiomatic code over clever or overly abstract solutions.
- Avoid unnecessary abstractions, generics, or additional modules unless there is a clear need.
- Keep functions and modules focused on a single responsibility.
- Minimize dependencies.
- Use the language's idiomatic error propagation mechanism; do not silently ignore errors.
- **QA checks**: run the language's appropriate type-checker, formatter, linter, and test runner.
  Use whichever tools are standard for the detected language and project structure.

## Supported programming languages
The project must specify the programming language for its development. If none is specified,
report this to the user and stop execution immediately.

If the target language has a corresponding file listed below, read it. Its instructions extend
the generic defaults above; where there is overlap, the language-specific file takes precedence.
Inform the user that language-specific configuration for that language was found and is in effect.

If no language-specific file exists for the target language, apply the generic defaults above
as-is. Inform the user that no language-specific configuration was found and that generic defaults
are in effect.
- Rust.md
