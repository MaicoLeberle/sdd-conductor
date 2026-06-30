# Stages

A target project moves through the following lifecycle stages in order:

| Stage | Meaning |
|---|---|
| `uninitialized` | `.sdd-conductor/` does not yet exist |
| `bootstrapped` | `.sdd-conductor/` exists; tasks have not yet been derived |
| `plan_is_ready` | Tasks were derived; plan execution was not yet started |
| `in_progress` | At least one task has been completed; execution is not yet complete |
| `blocked` | Execution is paused due to an unresolvable QA failure |
| `complete` | All tasks have been completed |

`blocked` is a sub-state of `in_progress`. A project is `blocked` when its stage would otherwise
be `in_progress` but the STATUS section of `.sdd-conductor/tasks/current_state.md` reads `BLOCKED`.

`corrupted` is an error state, not a lifecycle stage. It can occur at any point after bootstrap if
required files are missing from `.sdd-conductor/`. It is detected during stage computation and causes
all commands to abort immediately.


# Computing the current stage

Evaluate the following conditions in order and stop at the first match.

1. **Uninitialized**: if `.sdd-conductor/` does not exist, then stage is `uninitialized`.
2. **Corrupted**: if `.sdd-conductor/` does exist but any of `project.md`, `architecture.md`,
`modules.md`, `tasks/current_state.md`, or `tasks/pending_tasks.md` is missing, then the stage is
`corrupted`.
3. **Bootstrapped**: if `.sdd-conductor/tasks/pending_tasks.md` has no task entries (i.e., contains
no `## Title` section) AND `.sdd-conductor/tasks/completed_tasks/completed_task_1.md` does not exist,
then stage is `bootstrapped`.
4. **Initialized plan**: if `.sdd-conductor/tasks/pending_tasks.md` has task entries AND
`.sdd-conductor/tasks/completed_tasks/completed_task_1.md` does not exist, then stage is
`plan_is_ready`.
5. **Executing plan**: if `.sdd-conductor/tasks/pending_tasks.md` has task entries AND
`.sdd-conductor/tasks/completed_tasks/completed_task_1.md` exists:
   - Read `.sdd-conductor/tasks/current_state.md`. If its STATUS section reads `BLOCKED`, stage is
   `blocked`. Otherwise (STATUS is `NOT BLOCKED`), stage is `in_progress`.
6. **Complete**: if `.sdd-conductor/tasks/pending_tasks.md` has no task entries (i.e., contains no
`## Title` section) AND `.sdd-conductor/tasks/completed_tasks/completed_task_1.md` exists, then stage
is `complete`.

**Note on spec changes**: a spec pivot (Option B in `task_execution.md`) re-derives tasks and
deletes all `completed_task_*.md` files from `.sdd-conductor/tasks/completed_tasks/`. After a pivot,
`pending_tasks.md` has task entries and `completed_task_1.md` does not exist, so the stage becomes
`plan_is_ready`. The lifecycle is therefore spec-change-aware by construction: the stage correctly
reflects execution progress regardless of how many spec pivots have occurred.


# Command preconditions

Each command must compute the current stage before doing anything else, then assert the expected
stage. If the assertion fails, inform the user of the current stage and the required stage, and
stop execution immediately.

If the computed stage is `corrupted`, all commands abort regardless of their individual
preconditions. Inform the user which required files are missing from `.sdd-conductor/` and stop
execution immediately.

| Command | Required stage(s) |
|---|---|
| `bootstrap` | `uninitialized` |
| `derive_project_spec` | `bootstrapped` |
| `derive_tasks` | `bootstrapped` |
| `reset_plan` | `plan_is_ready` |
| `execute_next_task` | `plan_is_ready` <br> `in_progress` |
| `inject_task` | `plan_is_ready` <br> `in_progress` |
| `unblock` | `blocked` |
| `rollback_task` | `in_progress` <br>`complete` |
| `show_next_task` | `plan_is_ready` <br> `in_progress` <br> `blocked` |
| `show_pending_tasks` | `plan_is_ready` <br> `in_progress` <br> `blocked` |
| `show_stage` | (any) |
| `help` | (any) |


# Session recovery

At the start of any new session, compute the current stage before loading any further context.
The result immediately determines which files still need to be read:

After loading the stage-specific files below, read `.sdd-conductor/meta.md` if it exists. Compare
its `## Version` field to the content of the `VERSION` file in the sdd-conductor repository.
If they differ, warn the user that the project was bootstrapped with a different version of
sdd-conductor (state the bootstrapped version and the current version), advise them to
review the changelog for any breaking changes, and stop execution until the user explicitly
confirms they wish to proceed. Once the user confirms, update the `## Version` field in
`.sdd-conductor/meta.md` to the current version so that subsequent sessions start without
repeating this warning.

- `corrupted`: identify and report which required files are missing from `.sdd-conductor/` to the
  user. Do not proceed until the user confirms the files have been restored.
- `uninitialized` or `bootstrapped`: no task-state files need loading beyond
  `.sdd-conductor/project.md`.
- `plan_is_ready`, `in_progress`, or `blocked`: load `.sdd-conductor/tasks/current_state.md` and
  `.sdd-conductor/tasks/pending_tasks.md` (the first entry is the next task to execute).
- `complete`: load `.sdd-conductor/tasks/current_state.md` for a summary of what was built.
