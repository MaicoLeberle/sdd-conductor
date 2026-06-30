# sdd-conductor

**sdd-conductor** is an **AI agent-assisted software development orchestration system**.

It is a corpus of markdown files that an AI agent ingests as context, after which it can assist in
developing a separate, independently-maintained target project. The instruction set covers system
loading, project bootstrapping, tasks derivation and iterative execution, project specification
derivation, completion review, among others.

Collaboration between the user and the agent is central to the workflow: the user remains in
control at every step, reviewing outputs, making decisions and requesting changes, while the agent
implements tasks, runs quality checks, manages the development state, and commits results to git.

---

# What it does

#### Scope

sdd-conductor is _especially_ suited for small, single-language projects — prototypes, CLI tools, or
well-bounded libraries — whose full scope can be articulated as a specification upfront and whose
implementation can be decomposed into a linear sequence of tasks.

#### Main features

<ul>
<li>During bootstrap, a git-ignored <b>infrastructure directory</b> is created,
<code>.sdd-conductor</code>.</li>
<li><b>Derivation</b> of an ordered <b>task plan</b> from a <b>project specification</b> for
greenfield projects.</li>
<li><b>Agentic inference of specification</b> and architecture from a brownfield codebase.</li>
<li><b>Command set</b> with clear command preconditions.</li>
<li><b>Generation of code, test suite, and documentation</b>.</li>
<li>Default and language-specific <b>QA configuration</b>.</li>
<li><b>Schema-validated infrastructure artifacts</b>.</li>
<li><b>Session recovery</b> by means of <b>lifecycle stage</b> computation from stored
artifacts.</li>
<li><b>Specification-pivoting with full state archiving</b>.</li>
<li><b>User-controlled task plan review</b> and modification prior to execution.</li>
<li><b>Mid-execution task injection</b>, preventing insertion of inconsistencies while answering
user requests.</li>
<li><b>Task rollback</b> via (non-destructive) <code>git revert</code>.</li>
<li><b>Blocked state with explicit unblock flow</b>.</li>
<li><b>Tracking of sdd-conductor version</b> used on each project.</li>
</ul>

Current design does not yet fully support:
<ul>
<li> Multi-language projects (one language per project is preferred).
<li> Large or complex codebases where architectural decisions emerge during development / along the
project lifecycle.
<li> Projects requiring parallel task execution or team workflows.
<li> Exploratory or research-style work where requirements are inherently open-ended.
</ul>

---

#### Support for greenfield projects
The user defines goals in a structured specification file, possibly assisted
by the agent who is fully aware of what the specification file will later be used for. From that
specification, the agent derives an ordered task list. It then executes them one by one. This is
done following a tight execution plan, running quality checks and finally committing the result as a
separate git commit, all done autonomously by the agent and finally left to the user for
confirmation. This process is iterated over and over until the project is complete. The human
developer is always kept in the loop, allowed to introduce changes or secondary tasks where they
deem necessary. The agent pushes back if inconsistencies would thus be introduced.

#### Support for brownfield projects
The agent explores the existing codebase, infers the project
specification, documents the current architecture and module boundaries, and summarizes the current
development state. The user reviews and refines the inferred spec before task derivation begins.

#### Key capabilities

Among the features listed above, the following stand out for their contribution to fine-grained
development orchestration:

- **Lifecycle stage tracking**. The project's stage is computed at runtime from the filesystem
  state of `.sdd-conductor/`. That is, there is no stored state file, but the lifecycle stage is
  computable from the infrastructure state stored in `.sdd-conductor`. This enables session recovery
  and restricting commands by stage. Available stages: `uninitialized`, `bootstrapped`,
  `plan_is_ready`, `in_progress`, `blocked`, `complete`. See `lifecycle/main.md` for more.

- **Spec change versioning**. A snapshot of the specification is taken when tasks are derived. If
  the spec is later modified, the agent detects the divergence and offers two options: revert the
  spec to match the snapshot (in the case of an accidental change in the project specification file)
  or accept the change and archive the entire current development state (task history, architecture,
  modules, and old spec) into a versioned directory before re-deriving tasks.

- **Blocked state**. If a QA failure is unresolvable within the scope of a task, the agent sets the
  project to `blocked`, informs the user, and stops. Once the user resolves the underlying issue,
  running `unblock` re-runs all QA checks and clears the block if they all pass.

- **Task rollback**. Running `rollback_task` undoes the last completed task: the code commit is
  reverted and all orchestration state (`pending_tasks.md`, `current_state.md`, `architecture.md`,
  `modules.md`) is restored to the pre-task condition. Rollback is blocked when the spec has
  diverged from the snapshot, preventing operations that would cross a spec pivot boundary.

All technologies known by the agent are (theoretically) supported. Language-specific behaviour—code style, documentation, test suite, logging—may be explicitly configured. See "How to support
a new programming language" below.

---

# Commands

The user should proceed by issuing commands to the agent, given that the sdd-conductor context that
it loaded provides precise definitions of artifacts and commands.

#### Syntax

Invoked using a double-slash prefix, namely `//<command>`. The double slash prefix explicitly
targets the sdd-conductor command set, bypassing any built-in command syntax the agent may
have—which typically starts with a single slash. Enter these commands in the prompt and the agent
should execute them.

#### Command reference

| Keyword | <nobr>Required stage(s)</nobr> | Description |
|---|---|---|
| `bootstrap` | `uninitialized` | Create `.sdd-conductor/` with all template files |
| `derive_project_spec` | `bootstrapped` | Infer spec and architecture from an existing codebase; pause for review |
| `derive_tasks` | `bootstrapped` | Validate spec, derive ordered task list, populate `tasks/` |
| `reset_plan` | `plan_is_ready` | Discard the current derived task plan and return to `bootstrapped` state for re-derivation |
| `show_next_task` | `plan_is_ready` <br> `in_progress` <br> `blocked` | Print the next task: title, description, completion criteria |
| `show_pending_tasks` | `plan_is_ready` <br> `in_progress` <br> `blocked` | Print the ordered list of pending task titles |
| `execute_next_task` | `plan_is_ready` <br> `in_progress` | Implement the next task, run QA checks, update state, commit |
| `inject_task` | `plan_is_ready` <br> `in_progress` | Classify a change request; prepend any preparatory tasks and the user-requested task to the queue |
| `unblock` | `blocked` | Re-run QA checks; clear the block if all pass, otherwise report remaining failures |
| `rollback_task` | `in_progress` <br> `complete` | Revert the last completed task's commit and restore all orchestration state to its pre-task condition |
| `show_stage` | (any) | Compute and report the current lifecycle stage |
| `help` | (any) | Print the full command reference table |

---

# Repository structure

`master.md` is the entry point. The agent reads it first and follows recursive inclusion
instructions until the full system is loaded. From that point it can accept commands.

The `.sdd-conductor/` directory is created by `bootstrap` inside the target project and is
gitignored. It holds the specification, architecture documentation, module boundaries, and all
task-state files.

---

<pre>
master.md                    Entry point. The agent reads this first and proceeds recursively
<b>lifecycle</b>
    main.md                  Lifecycle stage definitions, runtime computation, and session recovery
<b>commands</b>
    main.md                  Command registry: keywords, preconditions, and descriptions
    derive_project_spec.md   Instructions for the derive_project_spec command
    reset_plan.md            Discards derived task plan and restores bootstrapped stage
    unblock.md               Instructions for the unblock command
<b>tasks</b>
    main.md                  Task artifact definitions (pending tasks, current state, completed
                             tasks, etc.)
    derive_tasks.md          Instructions for the derive_tasks command
    task_execution.md        Instructions for the execute_next_task command
    rollback_task.md         Instructions for the rollback_task command
    inject_task.md           Instructions for the inject_task command
<b>schemas</b>                      Schemas that specific artifacts must satisfy
    main.md                  Schema-validation logic
    project.json             Schema for project.md
    current_state.json       Schema for current_state.md
<b>templates</b>                    Template files copied into .sdd-conductor during bootstrap
    architecture.md          Target project architecture template
    current_state.md         Current state artifact template
    main.md                  Bootstrap command definition (consisting mainly in copying templates
                             into .sdd-conductor/)
    meta.md                  Infrastructure metadata template
    modules.md               Module boundary definition template
    pending_tasks.md         Target project pending tasks template
    project.md               Target project specification template
    README.md                Target project public doc template
<b>languages</b>                    Supported programming languages and global configuration of code
                             style, dependency resolution, etc.
    main.md                  Global programming language code style, dependency resolution, etc.
    Rust.md                  Rust-specific code style, dependency resolution, etc.
    Haskell.md               Haskell-specific code style, dependency resolution, etc.
<b>tests</b>                        Test suite instructions (default + language-specific)
    main.md                  Default test suite instructions
    Rust.md                  Rust-specific test suite instructions
    Haskell.md               Haskell-specific test suite instructions
<b>documentation</b>                Documentation instructions (default + language-specific)
    main.md                  Default documentation instructions
    Rust.md                  Rust-specific documentation instructions
    Haskell.md               Haskell-specific documentation instructions
<b>logging</b>                      Logging configuration (default + language-specific)
    main.md                  Default logging configuration
    Rust.md                  Rust-specific logging configuration
    Haskell.md               Haskell-specific logging configuration
<b>git</b>
    main.md                  Git versioning rules
<b>examples</b>                     Examples developed using sdd-conductor (including .sdd-conductor dir)
    todo-cli                 todo-cli example
README.md                    sdd-conductor documentation
VERSION                      sdd-conductor version
</pre>

---

# Quickstart

**Prerequisites:**
<ul>
    <li> AI agent with file-read, file-write and shell-execution capabilities (like Claude Code)
    <li> git
</ul>
Developed and tested with Claude Code; any agent with file-read, file-write,
and shell-execution capabilities should work.

### New project

**1. Load context**

Open Claude Code in this repository and say:

```
Read master.md and follow all recursive inclusion instructions until the full system is loaded.
```

Wait for the agent to confirm it has ingested the full tree.

**2. Bootstrap**

Navigate to your target project directory and tell the agent:

```
//bootstrap
```

The agent creates `.sdd-conductor/` with all required template files and adds it to `.gitignore`.

**3. Define your spec**

Open `.sdd-conductor/project.md` and fill in the `*GOALS*`, `*NON-GOALS*`, `*CONSTRAINTS*`, and
`*SUCCESS CRITERIA*` sections. Additional sections may be added as needed.

**4. Derive tasks**

```
//derive_tasks
```

The agent validates the spec, then populates `.sdd-conductor/tasks/` with the task list. All tasks
are listed in order in `.sdd-conductor/tasks/pending_tasks.md`.

**5. Execute tasks**

```
//execute_next_task
```

The agent presents a summary of the next task and asks for permission to proceed. After execution
it runs QA checks, updates state files, shows a summary, prompts for commit confirmation, and
commits. Repeat after reviewing each result.

**6. Completion**

When `.sdd-conductor/tasks/pending_tasks.md` has no task entries, all tasks have been completed.
Manually run and review the target project to confirm it meets the goals defined in
`.sdd-conductor/project.md`.

---

### Pre-existing project

**1–2. Load context and bootstrap** — same as above.

**3. Derive the spec from the existing codebase**

```
//derive_project_spec
```

The agent explores the codebase (source files, manifests, git log), infers goals, constraints, and
success criteria, documents the existing architecture and module boundaries, and summarizes the
current development state. It then presents the inferred `project.md`, `architecture.md`, and
`modules.md` for your review.

Edit those files as needed, then continue:

**4–6.** Same as steps 4–6 of the new project flow above.

---

## Examples

<blockquote>
<b>Note on <code>.sdd-conductor/</code></b>: in a real project the <code>.sdd-conductor/</code>
directory would be listed in <code>.gitignore</code> and never committed—this is the standard
sdd-conductor workflow, as described in <code>templates/main.md</code> and <code>git/main.md</code>.
It is intentionally included in each of the examples, artificially removing the
<code>.gitignore</code> entry for it, so that the full development infrastructure with all its
artifacts are visible.
</blockquote>

#### todo-cli

`examples/todo-cli` is a Rust CLI todo manager built end-to-end using sdd-conductor. It stores tasks
as JSON, supports four subcommands (`list`, `add`, `done`, `delete`), and is split across two
modules (`tasks.rs` for the data model and storage layer, `main.rs` for CLI dispatch).

The session was straightforward: the spec was defined upfront, tasks were derived once without any
pivots, and each task was executed in order without rollbacks or blocked states. Mid-session changes
were requested by the user, first asking for a task split prior to executing the first task (`done`
and `delete` were originally a single task and were separated), and then injecting a task mid-way
(requesting a module refactor, creating the `tasks` module — see `completed_task_5.md`).
Beyond that, the project proceeded linearly from `bootstrapped` to `complete` across nine committed
tasks.

---

## Supported languages

#### Currently supported programming languages:
- **Rust**
- **Haskell**

#### How to support a new programming language:
<ul>
<li> Any language can be used without needing to provide specific configuration files for it. The
agent applies generic defaults for code style, error handling, QA checks, tests, documentation, and
logging—inferred from standard practice for the detected language.
<li> To add explicit configuration for language <code>A</code>, three levels are available:
    <ul>
    <li> <b>No files</b>: generic defaults apply for everything.
    <li> <b>Minimal language-specific configuration</b>: <code>languages/A.md</code> is
    provided and included in the list in <code>languages/main.md</code>. This should include
    explicit code style, error handling, and QA check instructions (specific tool names and
    commands) for this specific programming language <code>A</code>.
    <li><b>Extending language-specific configuration</b>. If <code>languages/A.md</code> exists
    and is added to the list in <code>languages/main.md</code>, more specific configuration can be
    given in relation to <code>A</code> by adding any of the following:
        <ul>
        <li> <b><code>tests/A.md</code></b>: language-specific test suite instructions—must be
        listed in <code>tests/main.md</code>.
        <li> <b><code>documentation/A.md</code></b>: language-specific documentation
        instructions—must be listed in <code>documentation/main.md</code>.
        <li> <b><code>logging/A.md</code></b>: language-specific logging instructions—must be listed
        in <code>logging/main.md</code>.
        </ul>
    </ul>
</ul>
<br><blockquote><b>NOTE</b>: Each provided language-specific file extends the default configuration,
taking precedence where there is an overlap, but in those dimensions alone. The agent informs the
user which configuration files were found and are in effect at load time.</blockquote>

---

## Requirements

Any AI agent with file-read, file-write, and shell-execution capabilities, and `git`.

---

## Contributing

All contribution suggestions are encouraged and welcome.
