<mark>THIS FILE CONTAINS SCHEMAS THAT CERTAIN ARTIFACTS MUST SATISFY IN ORDER TO
BE USED IN THE DEVELOPMENT PROCESS.</mark>

**[AGENT READ-ONLY]** THIS FILE IS PART OF THE SDD-CONDUCTOR INFRASTRUCTURE.
THE AI AGENT MUST READ IT BUT MUST NEVER MODIFY IT UNDER ANY CIRCUMSTANCES.

**WARNING: THIS FILE SHOULD BE USED IN COMBINATION WITH ALL THE OTHER IMPORTED
FILES, NOT MADE TO OVERRIDE ANY PREVIOUSLY READ FILE. ANY INCONSISTENCIES
SHOULD BE REPORTED IMMEDIATELY, RESUMING WORK AFTER USER HAS CONFIRMED THEY WERE
CORRECTLY NOTIFIED.**

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
- `.project-sdd/project.md` in the target repository must satisfy `schemas/project.json`.
- `.project-sdd/tasks/current_state.md` in the target repository must satisfy `schemas/current_state.json`.
