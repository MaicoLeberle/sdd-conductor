<mark>THIS FILE CONTAINS DEFAULT GIT VERSIONING RULES.</mark>

**[AGENT READ-ONLY]** THIS FILE IS PART OF THE SDD-CONDUCTOR INFRASTRUCTURE.
THE AI AGENT MUST READ IT BUT MUST NEVER MODIFY IT UNDER ANY CIRCUMSTANCES.

**WARNING: THIS FILE SHOULD BE USED IN COMBINATION WITH ALL THE OTHER IMPORTED FILES, NOT MADE TO
OVERRIDE ANY PREVIOUSLY READY FILE. ANY INCONSISTENCIES SHOULD BE REPORTED IMMEDIATELY, RESUMING
WORK AFTER USER HAS CONFIRMED THEY WERE CORRECTLY NOTIFIED.**


# git versioning
* Start every task from a clean state, which must be checked by running `git status` and confirming
the working tree is clean. Disregard untracked files; only considered tracked files when assessing
the cleanness of the working tree. 
* After completion, each task should be made into a commit. Only the changes related to the task may
be added into the commit.
* Do not EVER push changes, force push changes, rebase or modify git history.
* Do not commit any part or the totality of directory `.sdd-conductor`.
* COMMIT MESSAGES. These should be done using infinitive verbs and kept as simple as possible. No
big explanations, just mere, short titles to the task handled in the commit. The first letter of the
commit message is lower case. Example of a commit message "implement database storage feature".
