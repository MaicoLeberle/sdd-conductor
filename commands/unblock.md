<mark>THIS FILE CONTAINS THE INSTRUCTIONS FOR THE unblock COMMAND.</mark>

**[AGENT READ-ONLY]** THIS FILE IS PART OF THE SDD-CONDUCTOR INFRASTRUCTURE.
THE AI AGENT MUST READ IT BUT MUST NEVER MODIFY IT UNDER ANY CIRCUMSTANCES.

**WARNING: THIS FILE SHOULD BE USED IN COMBINATION WITH ALL THE OTHER IMPORTED FILES, NOT MADE TO
OVERRIDE ANY PREVIOUSLY READ FILE. ANY INCONSISTENCIES SHOULD BE REPORTED IMMEDIATELY, RESUMING
WORK AFTER USER HAS CONFIRMED THEY WERE CORRECTLY NOTIFIED.**


# Unblock

This command is intended for use after the project has entered the `blocked` stage due to an
unresolvable QA failure during `execute_next_task`. It re-runs all QA checks against the current
state of the codebase. If they all pass, the block is cleared and execution can resume. If any
fail, the project remains blocked and the user is informed of the remaining failures.

## Steps

1. **Assert stage**. Compute the current lifecycle stage per `lifecycle/main.md`. Assert it is
   `blocked`. If not, inform the user of the current stage and stop execution immediately.

2. **Read context**. Read the following files to restore the necessary context:
   - `.project-sdd/project.md` — current project specification.
   - `.project-sdd/architecture.md` — current architecture.
   - `.project-sdd/modules.md` — current module boundaries.
   - `.project-sdd/tasks/current_state.md` — current development state (STATUS is `BLOCKED`).
   - `.project-sdd/tasks/pending_tasks.md` — the first entry is the task whose QA failure caused
     the block.
   - Any language-specific files relevant to the target project (located in the `languages/`
     directory of this project).

3. **Run QA checks**. Execute all QA checks defined in the language-specific file for the target
   project (the "QA checks" section). Do not skip any check.

4. **Evaluate results**.
   - **If all checks pass**: set the STATUS section of `.project-sdd/tasks/current_state.md` to
     `NOT BLOCKED`. Inform the user that the block has been resolved and that they may now proceed
     with `execute_next_task`. Stop execution immediately.
   - **If any check fails**: do not modify `.project-sdd/tasks/current_state.md`. Report to the
     user exactly which checks failed and what the failures were. Inform the user that the project
     remains blocked and that they must address the failures before running `unblock` again. Stop
     execution immediately.
