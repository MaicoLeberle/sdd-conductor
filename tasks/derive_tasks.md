# Derive tasks

- Compute the current lifecycle stage per `lifecycle/main.md`. Assert it is `bootstrapped`. If
  not, inform the user of the current stage and stop execution immediately.
- Then:
        - Derive an ordered list of tasks from the project specification in
        `.sdd-conductor/project.md`. Said order should have the first task at the top of the list,
        followed below by the second one, and so on. The tasks should satisfy the properties
        defined in `tasks/main.md`.
        - Present the derived plan to the user: show each task's title and completion criteria in
        order. Ask whether they approve the plan or wish to request changes (splits, merges,
        reordering, additions, or removals).
        - If the user requests changes: apply them to the in-memory task list, then re-present the
        updated plan and ask again. If a requested change would violate the task properties in
        `tasks/main.md` (e.g. a merge producing a task too large for a single commit, or a split
        producing a task too trivial to stand alone) or would contradict the project specification
        in `.sdd-conductor/project.md`, push back with a clear explanation and propose an alternative.
        Repeat until the user approves the plan.
        - Once approved, write the finalised list into `.sdd-conductor/tasks/pending_tasks.md`. The
        first entry is the task to execute next; each subsequent entry is queued in execution order.
        Each task entry contains `## Title`, `## Description`, and `## Completion criteria`
        sections; consecutive entries are separated by a `---` line.
        - Write `.sdd-conductor/project_snapshot.md` as the `[AGENT-MANAGED FILE — DO NOT MODIFY MANUALLY]`
        header on the first line, followed by a blank line, followed by the exact content of
        `.sdd-conductor/project.md`, and nothing else. This snapshot records the exact specification at the time tasks were derived,
        and will be used to detect subsequent changes to the spec.
        - Create a `.sdd-conductor/tasks/completed_tasks` empty dir, where completed tasks will be
        stored.
