<mark>THIS IS THE ENTRY FILE FOR ALL INSTRUCTIONS RELATED TO DOCUMENTATION
</mark>

**[AGENT READ-ONLY]** THIS FILE IS PART OF THE SDD-CONDUCTOR INFRASTRUCTURE.
THE AI AGENT MUST READ IT BUT MUST NEVER MODIFY IT UNDER ANY CIRCUMSTANCES.

**WARNING: THIS FILE SHOULD BE USED IN COMBINATION WITH ALL THE OTHER IMPORTED
FILES, NOT MADE TO OVERRIDE ANY PREVIOUSLY READ FILE. ANY INCONSISTENCIES
SHOULD BE REPORTED IMMEDIATELY, RESUMING WORK AFTER USER HAS CONFIRMED THEY WERE
CORRECTLY NOTIFIED.**

## Default documentation configuration
- Every project must include a `docs` directory with project-related
documentation.
- Unless specified otherwise, the documentation for the project must follow the
following global guidelines:
    - Create `docs` directory in the target project repository if it does not exist. Copy the
    `templates/README.md` template into `docs/README.md` in the target project repository.
    - Documentation must be human-readable, concise, and clear.
    - Documentation must cover EXACTLY (and nothing else) everything necessary
    for humans to understand globally what the project is about, what it can do,
    any relevant technical details, etc.
    - All relevant technical information about the project must also be
    documented, including instructions for execution, testing, architecture,
    module boundaries, API descriptions, etc.

## Language-specific documentation configuration
If a language-specific file exists for the target language (see list below), read it. Its
instructions extend the default documentation configuration above; where there is overlap, the
language-specific file takes precedence. Inform the user that language-specific documentation
configuration was found and is in effect. If no file exists for the target language, the
defaults above apply as-is.
- documentation/Rust.md
- documentation/Haskell.md