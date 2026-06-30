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
