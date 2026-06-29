<mark>THIS IS THE ***MASTER*** FILE OF AN AI-DRIVEN INFRASTRUCTURE PROJECT FOR THE ASSISTED
DEVELOPMENT OF OTHER SOFTWARE PROJECTS</mark>

**[AGENT READ-ONLY]** THIS FILE IS PART OF THE SDD-CONDUCTOR INFRASTRUCTURE.
THE AI AGENT MUST READ IT BUT MUST NEVER MODIFY IT UNDER ANY CIRCUMSTANCES.


# Instructions
1. **Load context**. Read the content of this file and follow derived instructions.
2. **Session recovery**. Compute the current lifecycle stage per `lifecycle/main.md`. Load any
   additional context files required by that stage, as described in the "Session recovery" section
   of `lifecycle/main.md`.
3. **Report to the user whether the context loading was successful or not**.
4. **Await for any further instructions, questions or commands from the user**.


## Definitions and typical usage of sdd-conductor
- sdd-conductor is a software project intended for assisting in the
implementation of other software projects in a sensible, controlled and iteratively
checked way.
- A key functionality is that code, tests, documentation and versioning are handled directly by the
agent. The human is only required to review and accept the agent's generation. The user may desire
to make changes on the agent generation.
- Core terminology:
    - sdd-conductor will be called "this project".
    - The project that is being developed will be called the "target project".
    - The developer leveraging this project to implement the target project will be called "the
    user".
- The typical workflow from the perspective of the user of this project would
look like this:
    - **Creating initial AI context**. Making the AI agent ingest this project
    into its context, starting from this file and proceeding recursively as
    instructed in the files.
    - **Initialize AI-infrastructure**. Bootstrapping the AI-assisted
    development process by copying the files in the `templates` directory of this
    project into a dedicated, gitignored directory called `.project-sdd`.
        - NOTE: `.project-sdd` is a placeholder name that should be replaced throughout with the
        string created by replacing prefix "project" with the actual target project name, all in
        small letters and replacing `_` (underscores) with `-` (dashes). For example, if the target
        project is called "my_project", then directory `.project-sdd` is actually referring to
        `.my-project-sdd`. 
    with the actual name of the target project in its root directory.
    - **Defining the goals of the target project**. Manually filling the goals
    section in `.project-sdd/project.md` (along with any of the other sections if
    a greater level of details is required in any way). For pre-existing projects
    with code already in place, run `derive_project_spec` instead: the agent will
    infer the spec, architecture, and module boundaries from the codebase and
    pause for review before task derivation.
    - **Deriving the tasks**. Requesting the AI agent to derive the list of
    tasks from `.project-sdd/project.md`, as defined in the `tasks` directory.
    - **Executing the tasks**. Executing one by one the tasks, using the
    commands for such goal and manually reviewing the execution results before
    proceeding with the following task.
    - **Final checks after development completion**. Once all tasks have been
    executed, the user may want to run the code and manually check that the
    target project has been correctly implemented.

## Mandatory pre-readings
Before reading anything else in this file, you MUST read the following files and add them to your
context. Some express configuration values, others state constraints that must be satisfied, and
some others express execution instructions that will be followed eventually, when necessary.
Required readings may recursively introduce additional required readings. Such dependencies should
be followed as specified by the originating document, and the contents of all dependent readings
should be considered part of the overall context, too. This recursive task must be completed before
proceeding any further. Proceed in the order listed below. Do not re-read any previously-read file,
only read required files as you traverse the reading tree, only those files, and only once each.
- **languages/main.md**: programming languages configuration.
- **tests/main.md**: instructions on how to create the test suite.
- **templates/main.md**: templates for boostrapping new projects and their
software development process.
- **git/main.md**: default configuration for git versioning.
- **documentation/main.md**: instructions on how to produce the documentation.
- **tasks/main.md**: task artifact definitions (next task, pending tasks, current state, etc.).
- **commands/main.md**: commands let subactions be composed into higher-level
actions.
- **logging/main.md**: instructions on how to add logging features.
- **schemas/main.md**: some of the artifacts in sdd-conductor are required
to satisfy certain properties and constraints in order for the agent to execute
as expected.
- **lifecycle/main.md**: lifecycle stages of the target project and how to
compute the current stage at runtime.


# Global guidelines
* Always plan things and explain the plan first, then request permission to
proceed with changes.
* The development process will consist of a series of steps, each of which
covering a single, well-defined task (see definition of task below).
* During execution of task, any REALLY important derived realizations should be
explained back to me but never automatically implemented. Stick to the current
task and refrain from going beyond the limited scope of action assigned to each
task.
* Do not overengineer anything. Any unnecessary abstractions should be avoided
unless explicitly requested.
