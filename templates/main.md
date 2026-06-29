<mark>THIS FILE CONTAINS INSTRUCTIONS FOR BOOTSTRAPPING A PROJECT FROM A SET OF
TEMPLATES.</mark>

**[AGENT READ-ONLY]** THIS FILE IS PART OF THE SDD-CONDUCTOR INFRASTRUCTURE.
THE AI AGENT MUST READ IT BUT MUST NEVER MODIFY IT UNDER ANY CIRCUMSTANCES.

**WARNING: THIS FILE SHOULD BE USED IN COMBINATION WITH ALL THE OTHER IMPORTED FILES, NOT MADE TO
OVERRIDE ANY PREVIOUSLY READ FILE. ANY INCONSISTENCIES SHOULD BE REPORTED IMMEDIATELY, RESUMING
WORK AFTER USER HAS CONFIRMED THEY WERE CORRECTLY NOTIFIED.**

###### NOTE
`.project-sdd` is a placeholder name that should be replaced throughout with the string created by
replacing prefix "project" with the actual target project name, all in small letters and replacing
`_` (underscores) with `-` (dashes). For example, if the target project is called "my_project", then
directory `.project-sdd` is actually referring to `.my-project-sdd`. 

# Project template
The following are the instructions for bootstrapping the development infrastructure for a new
project. These should only be done if the project was not yet bootstrapped in a previous iteration.

# Commands
#### *Bootstrap*
- Compute the current lifecycle stage per `lifecycle/main.md`. Assert it is `uninitialized`. If
  not, inform the user of the current stage and stop execution immediately.
- Create a `.project-sdd` directory at the root of the target project. All content related to the
AI-assisted development of the target project will live inside this directory.
- If git versioning has not yet been done in the target project repository, then run `git init`.
- Add an entry for this directory to the `.gitignore` file at the root of the target project:
append the line `.project-sdd/` (with trailing slash) to it, substituting the resolved directory
name per the NOTE above. If `.gitignore` does not exist, create it first and stage it for commit
(which will effectively take part in a commit when the first commit is issued). Only `.gitignore` is
supported as an ignore mechanism; do not use `.git/info/exclude` or any other alternative.
- **`docs/README.md`**. The project's public documentation file. Create a `docs` directory at the
root of the target project and create `docs/README.md` inside it as an UNMODIFIED copy of the
`README.md` template file in this directory. It will be filled by the agent as tasks are
completed.
- **`project.md`**. This is the specification of the target project, filled by the user and
containing relevant design / architecture decisions as well as allowed / forbidden dependencies,
constraints, etc. Create it inside `.project-sdd` simply as an UNMODIFIED copy of the `project.md`
template file in this directory (its sections, including the mandatory ones, are intentionally left
empty).
- **`architecture.md`**. This is where the explanations regarding the architecture of the project
will be kept. Create it inside `.project-sdd` simply as an UNMODIFIED copy of the `architecture.md`
template file in this directory (intentionally left empty).
- **`modules.md`**. This is where the explanations regarding module boundaries of the project will
be kept. Create it inside `.project-sdd` simply as an UNMODIFIED copy of the `modules.md` template
file in this directory (intentionally left empty).
- **`pending_tasks.md`**. The list of tasks remaining to reach the project goal. Create it inside
`.project-sdd/tasks/` as an UNMODIFIED copy of the `pending_tasks.md` template file in this
directory. It will be filled by the agent during task derivation.
- **`current_state.md`**. A narrative snapshot of what has been built so far. Create it inside
`.project-sdd/tasks/` as an UNMODIFIED copy of the `current_state.md` template file in this
directory. It will be updated by the agent after each completed task.
- **`tasks/completed_tasks/`**. An empty directory inside `.project-sdd/tasks/`. It will hold the
history of completed tasks as the project progresses.
- **`.old_versions/`**. An empty directory inside `.project-sdd/`. It will hold versioned snapshots
of project state at each point the specification is updated.
- **`.meta.md`**. Metadata about this sdd-conductor instance. Create it inside `.project-sdd/`
as `.meta.md` (note the dot prefix), from the `meta.md` template file in this directory, filling in:
  - `## Version`: the content of the `VERSION` file in the sdd-conductor repository.
  - `## Bootstrapped`: today's date in `YYYY-MM-DDTHH:mm:ss.SSSZ` format.