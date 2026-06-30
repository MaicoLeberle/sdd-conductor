# Commands
## Syntax
The user may request the execution of any of the commands listed below by using the `//<command>`
syntax. Note that the use of the double slash allows the user to target directly the
sdd-conductor command set, by-passing the typical, built-in command syntax for agents.

## Global precondition
Before dispatching any command, compute the current lifecycle stage per `lifecycle/main.md` and
apply all preconditions defined there, including the `corrupted` state gate.

## Command set
The following list contains supported commands, with their respective descriptions and keywords.

#### *Bootstrap*
- **Description**: "Bootstrap" section in `templates/main.md`. If this was carried out
successfully, stop execution immediately. Else, inform what was wrong and stop execution
immediately.
- **Keyword**: `bootstrap`

#### *Derive project spec*
- **Description**: "Derive project spec" section in `commands/derive_project_spec.md`. If this was
carried out successfully, stop execution immediately. Else, inform what was wrong and stop
execution immediately.
- **Keyword**: `derive_project_spec`

#### *Derive tasks*
- **Description**: "Derive tasks" section in `tasks/derive_tasks.md`. If this was carried out
successfully, stop execution immediately. Else, inform what was wrong and stop execution
immediately.
- **Keyword**: `derive_tasks`

#### *Reset plan*
- **Description**: "Reset plan" section in `commands/reset_plan.md`. If this was carried out
successfully, stop execution immediately. Else, inform what was wrong and stop execution
immediately.
- **Keyword**: `reset_plan`

#### *Show next task*
- **Required stage**: `plan_is_ready`, `in_progress`, or `blocked`. If not, inform the user of the
current stage and stop execution immediately.
- **Description**: Return the first task entry in `.sdd-conductor/tasks/pending_tasks.md`: title,
description and completion criteria. If there are no task entries, say it. Stop execution
immediately.
- **Keyword**: `show_next_task`

#### *Execute next task*
- **Description**: "Steps for executing a task" section in `tasks/task_execution.md`.
- **Keyword**: `execute_next_task`

#### *Show pending tasks*
- **Required stage**: `plan_is_ready`, `in_progress`, or `blocked`. If not, inform the user of the
current stage and stop execution immediately.
- **Description**: Create a list of task titles from all entries in
`.sdd-conductor/tasks/pending_tasks.md`, respecting the order in that file. If there are no task
entries, say it. Stop execution immediately.
- **Keyword**: `show_pending_tasks`

#### *Inject task*
- **Description**: "Inject task" section in `tasks/inject_task.md`. If this was carried out
successfully, stop execution immediately. Else, inform what was wrong and stop execution
immediately.
- **Keyword**: `inject_task`

#### *Unblock*
- **Description**: "Unblock" section in `commands/unblock.md`. If this was carried out
successfully, stop execution immediately. Else, inform what was wrong and stop execution
immediately.
- **Keyword**: `unblock`

#### *Rollback task*
- **Description**: "Rollback task" section in `tasks/rollback_task.md`. If this was carried
out successfully, stop execution immediately. Else, inform what was wrong and stop execution
immediately.
- **Keyword**: `rollback_task`

#### *Show stage*
- **Description**: Report the current lifecycle stage to the user. Stop execution immediately.
- **Keyword**: `show_stage`

#### *Help*
- **Description**: Print the full command reference table: for each command, show its keyword,
required stage(s), and a one-line description. Stop execution immediately.
- **Keyword**: `help`
