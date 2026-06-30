# Rollback task

This command undoes exactly the last completed task. It reverts the corresponding code commit and
restores all `.sdd-conductor/` artifacts to the state they were in immediately before that task was
executed. The project returns to `in_progress` (or `plan_is_ready` if the first-ever task is being
rolled back).

Use this command whenever the last completed task needs to be undone: the implementation was
incorrect, the result was unsatisfactory after review, the scope turned out to be wrong, or any
other reason. The task is restored as the first entry of `pending_tasks.md` and the project is ready for a fresh
`//execute_next_task`.


## Steps

1. **Assert stage**. Compute the current lifecycle stage per `lifecycle/main.md`. Assert it is
   `in_progress` or `complete`. If not, inform the user of the current stage and stop
   immediately.

2. **Assert clean working tree**. Run `git status` in the target project root. Examine only
   tracked files; disregard untracked files. If any uncommitted changes to tracked files exist,
   inform the user and stop immediately. A clean working tree is required before a rollback.

3. **Assert no spec divergence**. Read `.sdd-conductor/project.md` and
   `.sdd-conductor/project_snapshot.md`. Strip the `[AGENT-MANAGED FILE — DO NOT MODIFY MANUALLY]`
   header and the blank line that follows it from `project_snapshot.md` before comparing — only
   the spec content is compared.
   If their contents differ, the specification has been
   modified since the current task plan was derived. Rolling back a task while the spec is
   diverged is unsafe because the pending task list may no longer be coherent with the spec.
   Inform the user of this and instruct them to resolve the spec divergence first by running
   `//execute_next_task`, which will offer the option to revert or accept the spec change.
   Stop immediately.

   Note: this check also prevents rollback across spec pivot boundaries. After a spec pivot,
   all completed tasks from the previous spec era are archived to `.sdd-conductor/old_versions/`
   and deleted from `.sdd-conductor/tasks/`. The only `completed_task_N.md` files present are from
   the current spec era, so a rollback always stays within that era. No additional check is
   needed for this.

4. **Identify last completed task**. Find the highest-numbered `completed_task_N.md` file in
   `.sdd-conductor/tasks/completed_tasks/`. Read it. Extract the commit hash from its `## Commit` section. If the
   `## Commit` section is missing, inform the user that the file predates the commit-hash
   recording requirement and that the rollback cannot proceed automatically; the user must
   identify and revert the correct commit manually. Stop immediately.

5. **Present and confirm**. Show the user the title, description, completion criteria, and
   commit hash of the task about to be rolled back. Ask for explicit confirmation before
   proceeding. Do not proceed without it.

6. **Revert code commit**. Run `git revert <hash> --no-edit` in the target project root. This
   automatically restores all tracked files — source code, tests, `docs/README.md` — to their
   pre-task state by creating a new revert commit. Do not use `git reset`; history must not be
   rewritten.

7. **Restore `.sdd-conductor/tasks/` state**. Prepend the content of `completed_task_N.md`, minus
   the `## Commit` section, as the new first entry of `.sdd-conductor/tasks/pending_tasks.md`. If
   `pending_tasks.md` already has task entries, insert a `---` separator line between the restored
   task and the existing content. Delete `completed_task_N.md` from
   `.sdd-conductor/tasks/completed_tasks/` afterwards.

8. **Re-derive `current_state.md`**. Read all remaining `completed_task_1.md` through
   `completed_task_{N-1}.md` files in `.sdd-conductor/tasks/completed_tasks/` together with the now-reverted code. Synthesise a new
   compressed narrative description of the current state of the codebase and write it to the
   DESCRIPTION section of `.sdd-conductor/tasks/current_state.md`. Set the STATUS section to
   `NOT BLOCKED`. The file must continue to satisfy `schemas/current_state.json`.

   If no completed tasks remain (i.e. N was 1), reset the DESCRIPTION section to
   `(EMPTY — NO TASKS HAVE BEEN COMPLETED YET)` and set STATUS to `NOT BLOCKED`.

9. **Re-derive `architecture.md` and `modules.md`**. Inspect the now-reverted code. If
   `.sdd-conductor/architecture.md` or `.sdd-conductor/modules.md` no longer accurately reflects the
   reverted codebase — because the rolled-back task had updated either file — rewrite the
   affected file as a compressed, accurate description of the current (reverted) state. Leave a
   file unchanged if it still accurately reflects the code. Warn the user that any rewritten
   files were re-derived and should be reviewed before execution continues.

10. **Report**. Inform the user that the rollback is complete: which task was rolled back, which
    commit was reverted, and that the project is now ready for `//execute_next_task`.
    Stop immediately.
