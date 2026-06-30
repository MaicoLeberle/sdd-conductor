# Definitions and artifacts
- **Tasks**. From the specification of the project in a markdown file in the target project
repository, a list of project tasks will be derived prior to anything else.
    - A task must have a title
    - A task must have a clear completion criteria.
    - A task's granularity should be coarse enough to fit precisely and naturally within one commit,
    but fine enough to affect a minimal number of files. There should be a kind of one-to-one
    mapping between tasks and commits, according to typical versioning standards on commit scope and
    concerns surface.
- **Completed tasks**. There is a list of `completed_task_*.md` markdown files
(`completed_task_1.md`, `completed_task_2.md`, ...) in `.sdd-conductor/tasks/completed_tasks/` containing the history
of completed tasks so far. Each file contains `## Title`, `## Description`, and
`## Completion criteria` sections, plus a `## Commit` section appended by the agent after
committing.
- **Current (development) state**. Each project will have its own
`.sdd-conductor/tasks/current_state.md` for humans and you alike to understand the current state of
development process. This file will contain a compressed, narrative summary of all completed tasks
so far. It should describe what has been built so far, not list completed tasks in a literal way
(think of it as a snapshot of the current state of the codebase). File format:
`templates/current_state.md`.
- **Pending tasks**. Each project will have a `.sdd-conductor/tasks/pending_tasks.md` containing all
tasks remaining for the completion of the project goals (as described in `.sdd-conductor/project.md`).
The **first entry is the task currently being executed or about to be executed**; subsequent entries
are queued in execution order. The file has no task entries in both the `bootstrapped` state (tasks
not yet derived) and the `complete` state (all tasks done); these two cases are distinguished by
whether `.sdd-conductor/tasks/completed_tasks/completed_task_1.md` exists. Each task entry contains
`## Title`, `## Description`, and `## Completion criteria` sections; consecutive entries are
separated by a `---` line.
- **Project architecture**. Each project will have its own `.sdd-conductor/architecture.md` markdown
file explaining the architecture of the project. File format: `templates/architecture.md`.
- **Module boundaries**. Each project will have its own `.sdd-conductor/modules.md` markdown file
explaining the boundaries of each module in the project. File format: `templates/modules.md`.
