## Default test suite
- Every project should have a test suite.
- Unless specified otherwise, the test suite for the project must follow the
following global guidelines:
    - Test suite should cover core, critical functionalities at the highest possible coverage ratio.
    - Realistic testing scenarios may not be reliably reproduced within a test
    unit run locally. If that is the case, any test case for which a proper test
    unit is not implemented must be succinctly informed to the user.
    - All relevant information on the specifics about the test suite and how to
    run it must be included in the documentation of the project.

## Language-specific test suite configuration
If a language-specific file exists for the target language (see list below), read it. Its
instructions extend the default test suite configuration above; where there is overlap, the
language-specific file takes precedence. Inform the user that language-specific test
configuration was found and is in effect. If no file exists for the target language, the
defaults above apply as-is.
- tests/Rust.md
- tests/Haskell.md
