<mark>THIS IS THE ROOT FILE FOR INSTRUCTIONS EXPLICITLY RELATED TO THE EXECUTION OF A SINGLE
TASK.</mark>

**[AGENT READ-ONLY]** THIS FILE IS PART OF THE SDD-CONDUCTOR INFRASTRUCTURE. THE AI AGENT MUST
READ IT BUT MUST NEVER MODIFY IT UNDER ANY CIRCUMSTANCES.


###### **Preliminary notes**
- Upon request to read a specific file, you may fail to find it in the expected location. In such
case, inform the user and stop execution of the task immediately. You may only resume after the user
confirms the missing file was restituted and you have checked this is so.


#### *How to read this file*
Instructions for executing tasks. The execution process as a whole handles: development process
updates, git updates, architecture and module documentation updates, test suite extension, code
style compliance, user notifications, and (most importantly) code generation.


#### *Steps for executing a task*
1. **Setup**
    - Compute the current lifecycle stage per `lifecycle/main.md`. Assert it is `plan_is_ready` or
    `in_progress`. If not, inform the user of the current stage and stop execution immediately.
    - Run `git status` to verify the repository is not in a corrupted state. Check only tracked
    files at this point, disregarding any untracked ones. If uncommitted changes to tracked files
    are detected:
        - **Stage is `plan_is_ready`**: no task has been executed through sdd-conductor yet,
        so there is no prior sdd commit to attribute these changes to. Present two options and
        stop until the user chooses:
            - **Discard**: run `git checkout -- .` to discard all uncommitted changes to tracked
            files. Warn the user that this is irreversible.
            - **Abort**: stop execution so the user can resolve the changes manually (commit or
            stash them outside of sdd-conductor) before re-running `//execute_next_task`.
        - **Stage is `in_progress`**: the previous task may have been executed but not committed
        (e.g. the commit was rejected or the session was interrupted). Present two options and
        stop until the user chooses:
            - **Re-commit**: inspect the uncommitted changes with `git diff HEAD`. Run `git status`
            to list every modified tracked file. For each file, confirm it was changed exclusively
            to satisfy the previous task's completion criteria; if uncertain about any file, stop
            and ask the user before staging it. Stage confirmed files individually by name — do not
            use `git add .` or `git add -A`. Commit using the same commit flow described in the
            "Git" subsection of step 4. Then resume from the next Setup step.
            - **Discard**: run `git checkout -- .` to discard all uncommitted changes to tracked
            files. Warn the user that state files (`current_state.md` and `pending_tasks.md`) may
            now be ahead of the code and should be reviewed and manually corrected before proceeding.
    - Read `.project-sdd/project.md` to get the current project specification. Check it satisfies
    `schemas/project.json`.
    - Check that `.project-sdd/project_snapshot.md` exists. If it does not, inform the user that
    the spec snapshot is missing (most likely because `derive_tasks` was never completed or the
    file was deleted), instruct them to re-run `//derive_tasks` to regenerate it, and stop
    execution immediately.
    - Compare `.project-sdd/project.md` against `.project-sdd/project_snapshot.md`. If they differ,
    warn the user that the specification has changed since tasks were derived. Present two options
    and stop until the user chooses:
        - **Option A — Revert spec**: copy the content of `.project-sdd/project_snapshot.md` back
        into `.project-sdd/project.md`, restoring the specification that was in effect when the
        current task plan was derived. Resume execution normally.
        - **Option B — Accept new spec**: treat the updated `project.md` as the new specification.
        Proceed as follows:
            1. Determine the next version name by counting existing directories inside
            `.project-sdd/old_versions/` and incrementing by one (e.g. `v1`, `v2`, …).
            2. Create `.project-sdd/old_versions/<version>/`.
            3. Copy `.project-sdd/tasks/current_state.md` into
            `.project-sdd/old_versions/<version>/current_state.md`.
            4. Copy `.project-sdd/tasks/pending_tasks.md` into
            `.project-sdd/old_versions/<version>/pending_tasks.md`.
            5. Copy every `.project-sdd/tasks/completed_tasks/completed_task_*.md` file into
            `.project-sdd/old_versions/<version>/`, preserving each file's original name. Then
            delete every `.project-sdd/tasks/completed_tasks/completed_task_*.md`.
            6. Copy `.project-sdd/architecture.md` into
            `.project-sdd/old_versions/<version>/architecture.md`.
            7. Copy `.project-sdd/modules.md` into
            `.project-sdd/old_versions/<version>/modules.md`.
            8. Copy `.project-sdd/project_snapshot.md` into
            `.project-sdd/old_versions/<version>/project.md`. This preserves the exact
            specification that governed the now-archived development run.
            9. Reset the STATUS section of `.project-sdd/tasks/current_state.md` to `NOT BLOCKED`.
            10. Re-derive the DESCRIPTION section of `.project-sdd/tasks/current_state.md` to
            reflect the current state of the codebase relative to the new specification: summarise
            what has already been built that remains relevant to the new `.project-sdd/project.md`,
            treating it as the new baseline. The file must continue to satisfy
            `schemas/current_state.json`.
            11. Re-derive tasks from the updated `.project-sdd/project.md` following the same logic
            as `tasks/derive_tasks.md`, but omitting its final snapshot step — that is handled
            exclusively by step 12 below.
            12. Update `.project-sdd/project_snapshot.md` with the content of the new
            `.project-sdd/project.md`.
            13. Inform the user that the plan has been re-derived and resume from step 1 of
            "Steps for executing a task".
    - Read `.project-sdd/architecture.md` to get the current project architecture.
    - Read `.project-sdd/modules.md` to get the current project module boundaries. 
    - Read any language-specific files. These files depend on the programming language used in the
    target project. They should be located in the `languages` directory in this project, and contain
    a "QA checks" section.
    - Read `.project-sdd/tasks/current_state.md` to get the current state of the target project
    development process. Check it satisfies `schemas/current_state.json`.
    - Read `.project-sdd/tasks/pending_tasks.md` to get the full ordered task list. The first entry
    is the next task to execute. Analyze whether the first entry is coherent with the project
    specification, with the current state and with the remaining entries, and whether it is
    feasible, its scope is limited and fits within a single commit. Otherwise, present change
    suggestions to the user to adapt the next task accordingly.
    - Analyze whether `.project-sdd/tasks/current_state.md` and
    `.project-sdd/tasks/pending_tasks.md` are consistent with each other, by reading the
    specifications in `.project-sdd/project.md`.
2. **Request permission to execute**
    - Prepare brief summary of next task, adding current state as context only when absolutely
    necessary.
    - Present summary to the user.
    - Request permission from user to proceed.
3. **Execution of the task**. This requires going through a cycle until task can be confirmed to be
completed and correct. Follow these instructions:
    - **Refresh context**. Before proceeding any further, make sure the first entry of
    `.project-sdd/tasks/pending_tasks.md` and `.project-sdd/project.md` are in context.
    - **Code generation in target project**
        - If this is the first iteration of the cycle, implement the changes read from the first
        entry of `.project-sdd/tasks/pending_tasks.md`. Do not forget to add unit tests to the test
        suite when it makes sense.
        - Else, take the conclusions about the unsuitability of the changes drawn at the end of the
        previous iteration, and adjust them according to the first entry of
        `.project-sdd/tasks/pending_tasks.md`.
    - **Checks**
        - **Task scope was not exceeded**. Run `git diff HEAD` on target project root directory to
        assess whether all the changes in the target project repository are absolutely and strictly
        necessary for the completion of the task described in the first entry of
        `.project-sdd/tasks/pending_tasks.md` (instead of any
        of them being secondary, distantly-related, not strictly necessary, or not tightly-tied to
        the scope of the task). If anything needs to be adjusted, do it now.
        - **Overall suitability of the solution**.
            - With the output from `git diff HEAD` run above, assess the suitability of the solution
            at a conceptual level.
                - If the solution is conceptually suitable for the task:
                    - Follow the instructions in the "QA checks" of the language-specific files.
                        - If they succeed, inform the user a correct solution was implemented and
                        quit this "Execution of the task" cycle altogether and proceed with the
                        "Post-task updates" instructions (see below).
                        - Else, adjust generated code in a minimalistic way if possible. If the
                        failure is determined to be unresolvable within the scope of this task,
                        transition to lifecycle stage `blocked` (i.e., set the STATUS section in
                        `.project-sdd/tasks/current_state.md` to `BLOCKED`), inform the user about
                        the blocker, and stop execution immediately. Otherwise, start this
                        subsection (namely, "Overall suitability of the solution") from scratch.
                - Else, compute gap between a real solution to this task and the current solution.
                Inform the user. Start the next cycle iteration.
4. **Post-task updates**
    - The architecture of the project may have changed due to the completion of this task. If
    absolutely necessary, rewrite `.project-sdd/architecture.md` as a compressed, accurate
    description of the current architecture — do not append. Leave it unchanged if no update is
    necessary.
    - The module boundaries of the project may have changed due to the completion of this task. If
    absolutely necessary, rewrite `.project-sdd/modules.md` as a compressed, accurate description of
    the current module boundaries — do not append. Leave it unchanged if no update is necessary.
    - The public documentation of the project may have changed due to the completion of this task.
    If absolutely necessary, rewrite `docs/README.md` as a compressed, accurate description of the
    current state of the project — do not append. Leave it unchanged if no update is necessary.
    - Take `.project-sdd/tasks/current_state.md` and the first entry of
    `.project-sdd/tasks/pending_tasks.md`, and compute the new current state of the project. This
    new state must be a compressed description of the current state (as opposed to appending the
    latest task at the end). Write it in the form of a list, making each idea an item in that list.
    Replace the content in the DESCRIPTION section in
    `.project-sdd/tasks/current_state.md` with the newly-computed state, and set its STATUS section
    to `NOT BLOCKED`.
    - Create a new `.project-sdd/tasks/completed_tasks/completed_task_n.md`, where `n` is defined as
    the next sequential number (starting from 1). Copy the content of the first entry of
    `.project-sdd/tasks/pending_tasks.md` (all content up to but not including the first `---`
    separator, or the entire file content if there is only one entry) into the newly created file.
    - Remove the first entry (and its trailing `---` separator, if present) from
    `.project-sdd/tasks/pending_tasks.md`. If the file still has task entries, the new first entry
    is the next task. If the file has no task entries remaining, the project is complete: inform the
    user, change the STATUS section in `.project-sdd/tasks/current_state.md` to `COMPLETE` and
    continue with the steps listed below.
    - Prepare a summary report on changes done for the completion of this task, and present it to
    the user.
    - **Git**
        - A change is task-related if and only if it was made exclusively to satisfy the
        completion criteria of the task just executed (the entry that was first in
        `.project-sdd/tasks/pending_tasks.md` before the post-task update above). Changes made for
        any other reason — incidental cleanup, opportunistic refactoring, unrelated bug fixes — must not
        be staged. Run `git status` to list every modified tracked file. For each file, confirm
        it is directly required by the current task; if uncertain about any file, stop and ask
        the user before staging it. Stage confirmed files individually by name — do not use
        `git add .` or `git add -A`.
        - Create commit message and show it to the user, prompting them for confirmation to proceed.
        Accept changes from the user.
        - Submit git commit.
        - Run `git rev-parse HEAD` to obtain the hash of the commit just created. Append the
        following block to `.project-sdd/tasks/completed_tasks/completed_task_N.md` (where N is the
        number used when that file was created earlier in this step):

          ```
          ## Commit
          <hash>
          ```

          This hash is required by `rollback_task` to identify the exact commit to revert.
        - Inform the user the whole task has been completed and that now you may proceed with next
        task. Stop execution immediately after.
