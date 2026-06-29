<mark>THIS IS THE FILE CONTAINING ALL DOCUMENTATION INSTRUCTIONS FOR PROJECTS
USING HASKELL. THIS MAY OVERRIDE THE DOCUMENTATION DEFAULT CONFIGURATION IN
THE MAIN FILE IN THIS SAME DIRECTORY</mark>

**[AGENT READ-ONLY]** THIS FILE IS PART OF THE SDD-CONDUCTOR INFRASTRUCTURE.
THE AI AGENT MUST READ IT BUT MUST NEVER MODIFY IT UNDER ANY CIRCUMSTANCES.

**WARNING: THIS MAY OVERRIDE THE DOCUMENTATION DEFAULT CONFIGURATION IN THE
MAIN FILE IN THIS SAME DIRECTORY. IN SUCH CASE, YOU MUST INFORM THE USER ABOUT
THE OVERRIDDEN CONFIGURATION.**


#### *HASKELL-SPECIFIC DOCUMENTATION CONFIGURATION AND INSTRUCTIONS*

- **Haddock comments**. All exported functions, types, and constructors must have `-- |`
  Haddock comments. Module-level documentation uses `{- | ... -}` at the top of the file,
  before the `module` declaration. Comments should describe what the item does and why, not
  restate its type signature.
- **Haddock preview**. Run `cabal haddock` to render and review the generated HTML
  documentation locally before considering a task complete.
- **GHC version**. `docs/README.md` must state the minimum supported GHC version, e.g.
  "Requires GHC 9.4 or later." Update it whenever the minimum changes.
- **`docs/README.md` structure**. In addition to the default template sections, Haskell
  projects must include a "Building" section showing `cabal build` and a "Testing" section
  showing `cabal test`.
