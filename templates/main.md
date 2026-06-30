# Project template
The following are the instructions for bootstrapping the development infrastructure for a new
project. These should only be done if the project was not yet bootstrapped in a previous iteration.

# Commands
#### *Bootstrap*
- Compute the current lifecycle stage per `lifecycle/main.md`. Assert it is `uninitialized`. If
  not, inform the user of the current stage and stop execution immediately.
- Create a `.sdd-conductor` directory at the root of the target project. All content related to the
AI-assisted development of the target project will live inside this directory.
- If git versioning has not yet been done in the target project repository, then run `git init`.
- Add an entry for this directory to the `.gitignore` file at the root of the target project:
append the line `.sdd-conductor/` (with trailing slash) to it. If `.gitignore` does not exist, create
it first and stage it for commit (which will effectively take part in a commit when the first
commit is issued). Only `.gitignore` is supported as an ignore mechanism; do not use
`.git/info/exclude` or any other alternative.
- **`docs/README.md`**. The project's public documentation file. Create a `docs` directory at the
root of the target project and create `docs/README.md` inside it as an UNMODIFIED copy of the
`README.md` template file in this directory. It will be filled by the agent as tasks are
completed.
- **`project.md`**. This is the specification of the target project, filled by the user and
containing relevant design / architecture decisions as well as allowed / forbidden dependencies,
constraints, etc. Create it inside `.sdd-conductor` simply as an UNMODIFIED copy of the `project.md`
template file in this directory (its sections, including the mandatory ones, are intentionally left
empty).
- **`architecture.md`**. This is where the explanations regarding the architecture of the project
will be kept. Create it inside `.sdd-conductor` simply as an UNMODIFIED copy of the `architecture.md`
template file in this directory (intentionally left empty).
- **`modules.md`**. This is where the explanations regarding module boundaries of the project will
be kept. Create it inside `.sdd-conductor` simply as an UNMODIFIED copy of the `modules.md` template
file in this directory (intentionally left empty).
- **`pending_tasks.md`**. The list of tasks remaining to reach the project goal. Create it inside
`.sdd-conductor/tasks/` as an UNMODIFIED copy of the `pending_tasks.md` template file in this
directory. It will be filled by the agent during task derivation.
- **`current_state.md`**. A narrative snapshot of what has been built so far. Create it inside
`.sdd-conductor/tasks/` as an UNMODIFIED copy of the `current_state.md` template file in this
directory. It will be updated by the agent after each completed task.
- **`tasks/completed_tasks/`**. An empty directory inside `.sdd-conductor/tasks/`. It will hold the
history of completed tasks as the project progresses.
- **`old_versions/`**. An empty directory inside `.sdd-conductor/`. It will hold versioned snapshots
of project state at each point the specification is updated.
- **`meta.md`**. Metadata about this sdd-conductor instance. Create it inside `.sdd-conductor/`
as `meta.md`, from the `meta.md` template file in this directory, filling in:
  - `## Version`: the content of the `VERSION` file in the sdd-conductor repository.
  - `## Bootstrapped`: today's date in `YYYY-MM-DDTHH:mm:ss.SSSZ` format.