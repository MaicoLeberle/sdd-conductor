<mark>THIS FILE CONTAINS THE INSTRUCTIONS FOR THE reset_plan COMMAND.</mark>

**[AGENT READ-ONLY]** THIS FILE IS PART OF THE SDD-CONDUCTOR INFRASTRUCTURE.
THE AI AGENT MUST READ IT BUT MUST NEVER MODIFY IT UNDER ANY CIRCUMSTANCES.

**WARNING: THIS FILE SHOULD BE USED IN COMBINATION WITH ALL THE OTHER IMPORTED FILES, NOT MADE TO
OVERRIDE ANY PREVIOUSLY READ FILE. ANY INCONSISTENCIES SHOULD BE REPORTED IMMEDIATELY, RESUMING
WORK AFTER USER HAS CONFIRMED THEY WERE CORRECTLY NOTIFIED.**


# Reset plan

Discards the current derived task plan and returns the project to `bootstrapped` state, ready for
`//derive_tasks` to be run again. Use this when the project spec has been revised significantly
before any task has been executed and the derived plan is no longer valid.

- Compute the current lifecycle stage per `lifecycle/main.md`. Assert it is `plan_is_ready`. If
  not, inform the user of the current stage and stop execution immediately.
- Warn the user that this will permanently discard `.project-sdd/tasks/pending_tasks.md` and
  `.project-sdd/project_snapshot.md`, and that the plan will need to be re-derived with
  `//derive_tasks`. Ask for explicit confirmation before proceeding. Do not proceed without it.
- Reset `.project-sdd/tasks/pending_tasks.md` to the template content (copy
  `templates/pending_tasks.md` into it).
- Delete `.project-sdd/project_snapshot.md`.
- Inform the user that the plan has been discarded and the project is now in `bootstrapped` state.
  They may run `//derive_tasks` to re-derive the task plan.
- Stop execution immediately.
