## Schema files
The following files in this project (not in the target project) contain the schemas with which
artifacts in the target project will be validated:
- `project.json`
- `current_state.json`

## Overall rationale
sdd-conductor is a project implementing an AI-assisted software project development process,
where many parts of the project (notably including code generation) is handled by the AI agent. As
such, different artifacts are necessary, and they must contain the necessary information and have
the expected structure in order for the whole development process to be well-organized and
effectively handled by the agent.

## Validation
Validations schemas for the different artifacts used by this project in the development process of
the target project:
- `.sdd-conductor/project.md` in the target repository must satisfy `schemas/project.json`.
- `.sdd-conductor/tasks/current_state.md` in the target repository must satisfy `schemas/current_state.json`.
