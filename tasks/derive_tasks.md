<mark>THIS FILE CONTAINS THE INSTRUCTIONS FOR THE derive_tasks COMMAND.</mark>

**[AGENT READ-ONLY]** THIS FILE IS PART OF THE SDD-CONDUCTOR INFRASTRUCTURE.
THE AI AGENT MUST READ IT BUT MUST NEVER MODIFY IT UNDER ANY CIRCUMSTANCES.

**WARNING: THIS FILE SHOULD BE USED IN COMBINATION WITH ALL THE OTHER IMPORTED FILES, NOT MADE TO
OVERRIDE ANY PREVIOUSLY READ FILE. ANY INCONSISTENCIES SHOULD BE REPORTED IMMEDIATELY, RESUMING
WORK AFTER USER HAS CONFIRMED THEY WERE CORRECTLY NOTIFIED.**


# Derive tasks

- Compute the current lifecycle stage per `lifecycle/main.md`. Assert it is `bootstrapped`. If
  not, inform the user of the current stage and stop execution immediately.
- Then:
        - Derive an ordered list of tasks from the project specification in
        `.project-sdd/project.md`. Said order should have the first task at the top of the list,
        followed below by the second one, and so on. The tasks should satisfy the properties
        defined in `tasks/main.md`.
        - Present the derived plan to the user: show each task's title and completion criteria in
        order. Ask whether they approve the plan or wish to request changes (splits, merges,
        reordering, additions, or removals).
        - If the user requests changes: apply them to the in-memory task list, then re-present the
        updated plan and ask again. If a requested change would violate the task properties in
        `tasks/main.md` (e.g. a merge producing a task too large for a single commit, or a split
        producing a task too trivial to stand alone) or would contradict the project specification
        in `.project-sdd/project.md`, push back with a clear explanation and propose an alternative.
        Repeat until the user approves the plan.
        - Once approved, write the finalised list into `.project-sdd/tasks/pending_tasks.md`. The
        first entry is the task to execute next; each subsequent entry is queued in execution order.
        Each task entry contains `## Title`, `## Description`, and `## Completion criteria`
        sections; consecutive entries are separated by a `---` line.
        - Copy `.project-sdd/project.md` into `.project-sdd/.project_snapshot.md`. This snapshot
        records the exact specification at the time tasks were derived, and will be used to detect
        subsequent changes to the spec.
        - Create a `.project-sdd/tasks/completed_tasks` empty dir, where completed tasks will be
        stored.
