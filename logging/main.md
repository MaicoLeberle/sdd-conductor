## Required logging features
Here is a list of logging features required by the project, which should be directly added to the
source code along the way. This is the default logging configuration, which must be complied with in
all projects unless specified otherwise / refined in any way.
- Debugging logging messages should be added at a minimal level, only if it indeed assists the
debugging process.
- Debugging logging messages should be disabled when compilation targets a releasable executable.
- Some log messages may be useful for debugging purposes when the program crashes unexpectedly, even
in a production setting. You are allowed to include them also in releasable executables, but if you
do they should not expose any internal logic that may cause a vulnerability of the app in any way.


## Language-specific logging configuration
If a language-specific file exists for the target language (see list below), read it. Its
instructions extend the default logging configuration above; where there is overlap, the
language-specific file takes precedence. Inform the user that language-specific logging
configuration was found and is in effect. If no file exists for the target language, the
defaults above apply as-is.
- logging/Rust.md
- logging/Haskell.md
