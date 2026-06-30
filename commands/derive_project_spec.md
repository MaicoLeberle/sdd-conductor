# Derive project spec

This command is intended for projects that already have existing code at the time of bootstrapping.
It infers the project specification from the current state of the codebase, writes it into
`.sdd-conductor/project.md`, and summarises what has already been built into
`.sdd-conductor/tasks/current_state.md`. It then pauses for user review before any task derivation.

## Steps

1. **Assert stage**. Compute the current lifecycle stage per `lifecycle/main.md`. Assert it is
   `bootstrapped`. If not, inform the user of the current stage and stop execution immediately.

2. **Explore the codebase**. Investigate the target project to understand what has been built:
   - Read the directory and file structure.
   - Read source files, prioritising entry points, core modules, and any existing documentation
     (README, inline comments, doc files).
   - Read any package manifests (e.g. `Cargo.toml`, `package.json`, `pyproject.toml`) for
     declared dependencies, project name, and description.
   - If a git history exists, read the commit log for additional intent signals.

3. **Infer and write the project specification**. Based on the exploration above, fill in
   `.sdd-conductor/project.md`:
   - **GOALS**: what the project is clearly trying to accomplish.
   - **NON-GOALS**: what the project is explicitly not doing, or what is out of scope given the
     current implementation direction.
   - **CONSTRAINTS**: technical or design constraints evident from the codebase (language choice,
     dependency restrictions, architectural decisions already in place, etc.).
   - **SUCCESS CRITERIA**: observable conditions that would confirm the project goals are met.
   The filled spec must satisfy `schemas/project.json`.

4. **Derive architectural context**. Based on the codebase exploration above, populate:
   - `.sdd-conductor/architecture.md`: describe the high-level architecture already in place (layers,
     subsystems, key design patterns in use, and how data flows between them).
   - `.sdd-conductor/modules.md`: describe the module boundaries present in the codebase (modules,
     crates, packages, or equivalent units, with their responsibilities and inter-module interfaces).

5. **Summarise the current development state**. Based on the exploration above, write a concise
   narrative description of what has already been implemented into the DESCRIPTION section of
   `.sdd-conductor/tasks/current_state.md`. The STATUS section must be set to `NOT BLOCKED`.
   The file must satisfy `schemas/current_state.json`.

6. **Present and pause**. Show the user the generated `.sdd-conductor/project.md`,
   `.sdd-conductor/architecture.md`, and `.sdd-conductor/modules.md`, with a brief explanation of the
   reasoning behind each inferred section and architectural decision. Ask the user to review and
   edit those files as needed. Inform the user to invoke `derive_tasks` once satisfied.
   Stop execution immediately.
