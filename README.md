# sdd-conductor

sdd-conductor is an AI agent-assisted software development orchestration system. It is a corpus
of markdown files that an AI agent ingests as context, after which it can assist in developing a
separate, independently-maintained target project. The instructions cover system loading, project
bootstrapping, development plan tasks derivation and iterative execution, specification derivation,
and completion review.

Collaboration between the user and the agent is central to the workflow: the user remains in
control at every step, reviewing outputs, making decisions and requesting changes, while the agent
implements tasks, runs quality checks, manages the development state, and commits results to git.

---

## Scope

sdd-conductor is an agent-agnostic development assistant **especially** suited for small,
single-language projects — prototypes, CLI tools, or well-bounded libraries — whose
full scope can be articulated as a specification upfront and whose implementation
can be decomposed into a linear sequence of tasks.

Features include:
- During bootstrap, a git-ignored infrastructure dir is created, `.sdd-conductor`.
- Derivation of an ordered task plan from a project specification
(greenfield), or agentic inference of specification and architecture from an existing
codebase (brownfield).
- Command set with clear command preconditions.
- Generation of code, test suite, and documentation.
- Default and language-specific QA configuration.
- Schema-validated infrastructure artifacts.
- Lifecycle stage computation from artifacts (enabling session recovery).
- Specification-pivoting with full state archiving.
- User-controlled task plan review and modification prior to execution.
- Mid-execution task injection via `//inject_task`: classify the request, optionally extend the spec
minimally, design any preparatory tasks needed to reconcile with completed work, and prepend
everything to the task queue.
- Task rollback via non-destructive `git revert`.
- Blocked state with explicit unblock flow.
- Version tracking.

Current design does not yet fully support:
- Multi-language projects (one language per project is assumed).
- Large or complex codebases where architectural decisions emerge during development / along the
project lifecycle.
- Projects requiring parallel task execution or team workflows.
- Exploratory or research-style work where requirements are inherently open-ended.

---

## What it does

sdd-conductor supports both new and pre-existing projects.

For **new projects**, the user defines goals in a structured specification file, possibly assisted
by the agent who is fully aware of what the specification file will later be used for. From that=
specification, the agent derives an ordered task list. It then and executes them one by one. This is
done following a tight execution plan, running quality checks and finally committing the result as a
separate git commit, all done autonomously by the agent and finally left to the user for
confirmation. This process is iterated over and over until the project is complete. The human
developer is always kept in the loop, allowed to introduce changes or secondary tasks where they
deem necessary. The agent pushes back if inconsistencies would thus be introudced.

For **pre-existing projects**, the agent explores the existing codebase, infers the project
specification, documents the current architecture and module boundaries, and summarizes the current
development state. The user reviews and refines the inferred spec before task derivation begins.

#### Key capabilities:
- **Lifecycle stage tracking.** The project's stage is computed at runtime from the filesystem
  state of `.sdd-conductor/` — no stored state file. Stages are `uninitialized`, `bootstrapped`,
  `plan_is_ready`, `in_progress`, `blocked`, and `complete`. Commands are gated by stage, and
  session startup loads only the context relevant to the current stage.

- **Spec change versioning.** A snapshot of the specification is taken when tasks are derived. If
  the spec is later modified, the agent detects the divergence and offers two options: revert the
  spec, or accept the change and archive the entire current development state (task history,
  architecture, modules, and old spec) into a versioned directory before re-deriving tasks.

- **Blocked state.** If a QA failure is unresolvable within the scope of a task, the agent sets
  the project to `blocked`, informs the user, and stops. Once the user resolves the underlying
  issue, running `unblock` re-runs all QA checks and clears the block if they all pass.

- **Task rollback.** Running `rollback_task` undoes the last completed task: the code commit is
  reverted and all orchestration state (`next_task.md`, `pending_tasks.md`, `current_state.md`,
  `architecture.md`, `modules.md`) is restored to the pre-task condition. Rollback is blocked
  when the spec has diverged from the snapshot, preventing operations that would cross a spec
  pivot boundary.

All technologies known by the agent are theoretically supported. Language-specific behaviour —
code style, documentation, test suite, logging — must be explicitly configured. See "Adding a new
language" below.

---

## How it works

`master.md` is the entry point. The agent reads it first and follows recursive inclusion
instructions until the full system is loaded. From that point it can accept commands.

The `.sdd-conductor/` directory is created by `bootstrap` inside the target project and is
gitignored. It holds the specification, architecture documentation, module boundaries, and all
task-state files.

---

## Quickstart

**Prerequisites:** Claude Code (or any AI agent with file-read, file-write and shell-execution
capabilities), `git`. Developed and tested with Claude Code; any agent with file-read, file-write,
and shell-execution capabilities should work.

### New project

**1. Load context**

Open Claude Code in this repository and say:

> Read `master.md` and follow all recursive inclusion instructions until the full system is loaded.

Wait for the agent to confirm it has ingested the full tree.

**2. Bootstrap**

Navigate to your target project directory and tell the agent:

> `//bootstrap`

The agent creates `.sdd-conductor/` with all required template files and adds it to `.gitignore`.

**3. Define your spec**

Open `.sdd-conductor/project.md` and fill in the `*GOALS*`, `*NON-GOALS*`, `*CONSTRAINTS*`, and
`*SUCCESS CRITERIA*` sections. Additional sections may be added as needed.

**4. Derive tasks**

> `//derive_tasks`

The agent validates the spec, then populates `.sdd-conductor/tasks/` with the task list. The first
task goes to `next_task.md`; the rest go to `pending_tasks.md`.

**5. Execute tasks**

> `//execute_next_task`

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

> `//derive_project_spec`

The agent explores the codebase (source files, manifests, git log), infers goals, constraints, and
success criteria, documents the existing architecture and module boundaries, and summarises the
current development state. It then presents the inferred `project.md`, `architecture.md`, and
`modules.md` for your review.

Edit those files as needed, then continue:

**4–6.** Same as steps 4–6 of the new project flow above.

---

## Commands

### Syntax

Commands are invoked using a double-slash prefix, namely `//<command>`. The double slash prefix
explicitly targets the sdd-conductor command set, bypassing any built-in command syntax the
agent may have (which typically starts with a single slash). Enter these commands in the prompt and
the agent should execute it.

### Command reference

| Keyword | Required stage | Description |
|---|---|---|
| `bootstrap` | `uninitialized` | Create `.sdd-conductor/` with all template files |
| `derive_project_spec` | `bootstrapped` | Infer spec and architecture from an existing codebase; pause for review |
| `derive_tasks` | `bootstrapped` | Validate spec, derive ordered task list, populate `tasks/` |
| `reset_plan` | `plan_is_ready` | Discard the current derived task plan and return to `bootstrapped` state for re-derivation |
| `show_next_task` | `plan_is_ready`, `in_progress`, or `blocked` | Print the next task: title, description, completion criteria |
| `show_pending_tasks` | `plan_is_ready`, `in_progress`, or `blocked` | Print the ordered list of pending task titles |
| `execute_next_task` | `plan_is_ready` or `in_progress` | Implement the next task, run QA checks, update state, commit |
| `inject_task` | `plan_is_ready` or `in_progress` | Classify a change request; prepend any preparatory tasks and the user-requested task to the queue |
| `unblock` | `blocked` | Re-run QA checks; clear the block if all pass, otherwise report remaining failures |
| `rollback_task` | `in_progress` or `complete` | Revert the last completed task's commit and restore all orchestration state to its pre-task condition |
| `show_stage` | any | Compute and report the current lifecycle stage |
| `help` | any | Print the full command reference table |

---

## Repository structure

```
master.md                    Entry point. The agent reads this first and proceeds recursively.
lifecycle/
  main.md                    Lifecycle stage definitions, runtime computation, and session recovery.
commands/
  main.md                    Command registry: keywords, preconditions, and descriptions.
  derive_project_spec.md     Steps for the derive_project_spec command.
  unblock.md                 Steps for the unblock command.
tasks/
  main.md                    Task artifact definitions (pending tasks, current state, completed tasks, etc.).
  derive_tasks.md            Steps for the derive_tasks command.
  task_execution.md          Step-by-step instructions for execute_next_task.
  rollback_task.md           Step-by-step instructions for rollback_task.
  inject_task.md             Steps for the inject_task command.
schemas/
  project.json               Schema that project.md must satisfy.
  current_state.json         Schema that current_state.md must satisfy.
templates/                   Template files copied into each new target project during bootstrap.
  README.md                  Starter template for the target project's public docs/README.md.
languages/                   Supported programming languages. One file per language.
tests/                       Test suite instructions. Default and language-specific.
documentation/               Documentation instructions. Default and language-specific.
logging/                     Logging configuration. Default and language-specific.
git/
  main.md                    Git versioning rules.
```

---

## Supported languages

#### Currently supported programming languages:
- **Rust**
- **Haskell**

#### How to support a new programming language:
Any language can be used without configuration files. The agent applies generic defaults for code
style, error handling, QA checks, tests, documentation, and logging — inferred from standard
practice for the detected language.

To add explicit configuration for language `A`, three levels are available:

**No files** — generic defaults apply for everything.

**`languages/A.md` only** — provides explicit code style, error handling, and QA check
instructions (specific tool names and commands) for `A`. Tests, documentation, and logging continue
to use generic defaults. Add `A.md` to the list in `languages/main.md`.

**`languages/A.md` plus any of the following** — `languages/A.md` must exist for these to be
meaningful:
- **`tests/A.md`**: language-specific test suite instructions. Add it to the list in `tests/main.md`.
- **`documentation/A.md`**: language-specific documentation instructions. Add it to the list in
  `documentation/main.md`.
- **`logging/A.md`**: language-specific logging instructions. Add it to the list in `logging/main.md`.

Language-specific files extend the generic defaults; where there is overlap they take precedence.
The agent informs the user which configuration files were found and are in effect at load time.

---

## Requirements

Any AI agent with file-read, file-write, and shell-execution capabilities, and `git`.

---

## Examples

### todo-cli

`examples/todo-cli` is a Rust CLI todo manager built end-to-end using sdd-conductor. It
supports four subcommands (`add`, `list`, `done`, `delete`), stores tasks as JSON, and is split
across two modules (`tasks.rs` for the data model and storage layer, `main.rs` for CLI dispatch).

The session was straightforward: the spec was defined upfront, tasks were derived once without
any pivots, and each task was executed in order without rollbacks or blocked states. The only
mid-session change was a user-requested task split — "done" and "delete" were originally a single
task and were separated before execution began. Beyond that, the project proceeded linearly from
`bootstrapped` to `complete` across nine committed tasks.

> **Note on `.sdd-conductor/`**: in a real project the `.sdd-conductor/` directory would be listed
> in `.gitignore` and never committed — this is the standard sdd-conductor workflow, as described
> in `templates/main.md` and `git/main.md`. It is intentionally included here, without a
> `.gitignore` entry for it, so that the full development artifact is visible as part of the
> example.

---

## Contributing

All contribution suggestions are encouraged and welcome.
