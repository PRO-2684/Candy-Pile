# Domain Docs

How the engineering skills should consume this repo's domain documentation when exploring the codebase.

This repo is a multi-context workspace containing small independent projects. Treat each project as its own context when reading or adding domain documentation.

## Before exploring, read these

- **`CONTEXT-MAP.md`** at the repo root if it exists -- it points at one `CONTEXT.md` per context. Read each one relevant to the topic.
- **Project-level `CONTEXT.md` files** for the crate or project being changed.
- **`docs/adr/`** at the repo root for workspace-wide decisions.
- **Project-level `docs/adr/` directories** for context-specific decisions.

If any of these files don't exist, **proceed silently**. Don't flag their absence; don't suggest creating them upfront. The producer skill (`/grill-with-docs`) creates them lazily when terms or decisions actually get resolved.

## File structure

Multi-context workspace:

```text
/
|-- CONTEXT-MAP.md
|-- docs/adr/                         # workspace-wide decisions
|-- macro_rules_attr/
|   |-- CONTEXT.md
|   `-- docs/adr/                     # project-specific decisions
|-- optfield-lite/
|   |-- CONTEXT.md
|   `-- docs/adr/
`-- should_match/
    |-- CONTEXT.md
    `-- docs/adr/
```

## Use the glossary's vocabulary

When your output names a domain concept (in an issue title, a refactor proposal, a hypothesis, a test name), use the term as defined in the relevant `CONTEXT.md`. Don't drift to synonyms the glossary explicitly avoids.

If the concept you need isn't in the glossary yet, that's a signal -- either you're inventing language the project doesn't use (reconsider) or there's a real gap (note it for `/grill-with-docs`).

## Flag ADR conflicts

If your output contradicts an existing ADR, surface it explicitly rather than silently overriding:

> _Contradicts ADR-0007 (event-sourced orders) -- but worth reopening because..._
