# Inject task

This command allows the user to inject a new change request into the development process at any
point after the task plan has been derived. The agent classifies the request, determines whether
it is compatible with the existing project specification, identifies any preparatory work needed
to reconcile the request with already-completed tasks, and prepends the resulting task(s) to the
front of the pending task queue for immediate execution.

## Steps

1. **Assert stage**. Compute the current lifecycle stage per `lifecycle/main.md`. Assert it is
   `plan_is_ready` or `in_progress`. If not, inform the user of the current stage and stop
   immediately.

2. **Read context**. Read the following files:
   - `.sdd-conductor/project.md` — current project specification.
   - `.sdd-conductor/architecture.md` — current architecture.
   - `.sdd-conductor/modules.md` — current module boundaries.
   - `.sdd-conductor/tasks/current_state.md` — current development state.
   - `.sdd-conductor/tasks/pending_tasks.md` — the ordered list of pending tasks.
   - All `.sdd-conductor/tasks/completed_tasks/completed_task_*.md` files, if any exist — to
     understand what has already been built.

3. **Classify the request**. Analyze the change request against the full context just loaded.
   Assign it to exactly one of three categories:

   - **Category A — In-scope**: The change fits within the existing project specification as
     written. No spec update is required.
   - **Category B — Spec extension**: The change is consistent with the project's goals but
     introduces something not yet stated in the specification. A minimal update to the spec is
     required.
   - **Category C — Contradiction**: The change conflicts with an existing constraint, design
     decision, or goal stated in the specification.

4. **Category C — Stop**. Explain clearly why the change contradicts the existing specification
   (cite the specific section or constraint). Inform the user that the request has been discarded
   and that they may submit a revised request. Stop execution immediately.

5. **Category B — Propose spec update**. Formulate the minimal set of edits to
   `.sdd-conductor/project.md` required to accommodate the change. Add or refine only the
   statements directly needed; do not restructure or expand the spec beyond what the change
   strictly requires. Propose edits to `.sdd-conductor/architecture.md` and
   `.sdd-conductor/modules.md` only if the change has structural implications that make them
   strictly necessary.

   Present the proposed spec edits to the user and ask for confirmation. Incorporate any
   requested adjustments and re-present until confirmed, or stop if the user wishes to abandon
   the request.

6. **Assess impact on completed work**. Consider the full set of completed tasks alongside the
   change request. Determine whether the change requires modifying code, structure, or behaviour
   that has already been implemented. If so, design the minimal set of agent-designed preparatory
   tasks needed to bring the codebase to a consistent state before the user-requested task
   executes (for example, a refactoring task or a module-move task). Each preparatory task must
   satisfy the task properties defined in `tasks/main.md`.

   If no completed work is affected, the preparatory task list is empty.

7. **Formulate the user's task**. Convert the change request into a single task with `## Title`,
   `## Description`, and `## Completion criteria` sections, satisfying the task properties
   defined in `tasks/main.md`.

8. **Present the full plan**. Show the user:
   - If Category B: the confirmed spec edits.
   - The list of tasks that will be prepended to the queue, in order:
     1. Any preparatory tasks (possibly none), followed by
     2. The user-requested task.
   - A note that these tasks will execute before all currently pending tasks.

   Ask for explicit confirmation before proceeding. If the user does not confirm, stop
   immediately without modifying any files.

9. **Apply changes**.
   - If Category B:
     - Apply the confirmed edits to `.sdd-conductor/project.md`.
     - Apply any confirmed edits to `.sdd-conductor/architecture.md` and
       `.sdd-conductor/modules.md`, if applicable.
     - Rewrite `.sdd-conductor/project_snapshot.md` as the
       `[AGENT-MANAGED FILE — DO NOT MODIFY MANUALLY]` header on the first line, followed by a
       blank line, followed by the exact content of `.sdd-conductor/project.md`, and nothing else. This ensures the
       spec-divergence check in `execute_next_task` does not trigger for this authorized change.
   - Prepend the new tasks (preparatory tasks, if any, followed by the user-requested task) to
     `.sdd-conductor/tasks/pending_tasks.md`. If `pending_tasks.md` already has task entries,
     insert a `---` separator line between the last prepended task and the existing content.

10. **Report**. Inform the user which tasks were prepended and that the project is ready for
    `//execute_next_task`. Stop execution immediately.
