# AGENTS.md — sovereign-shovels/sarvam-pdf

> If you are Claude Code (or any other agent) operating in this repo,
> this file is your **constitution**. Read it fully before any other action.
> If anything you are about to do conflicts with this file, STOP and ask the human.

---

## Identity

This repo is **sarvam-pdf** — part of the [sovereign-shovels](../README.md) portfolio.

- **Substrate anchor:** Sarvam
- **Utility category:** pdf
- **Tagline:** Drag a PDF, get it in your language. 22 Indic languages. Layout preserved.
- **Sprint:** 3
- **Estimated build for v0.1:** 3–4 weeks for v0.1

The substrate this shovel rides is: Indian SMBs, students, government workers translating English documents to/from Indic languages

The launch channels are: LinkedIn India, r/India, Indian SMB communities, AIKosh, education networks

---

## Philosophy this repo lives by

Read the universal philosophy: [[../PHILOSOPHY|Philosophy]].

In short, every shovel must satisfy all five rules:

1. **Substrate-anchored name** ✓ (this is `sarvam-pdf`)
2. **Sovereign by construction** — user owns model choice, BYO endpoint, must work with local-only
3. **Real demand evidence** — gap is documented, not assumed
4. **Buildable in 1–3 weeks** — v0.1 estimate is 3–4 weeks for v0.1
5. **Scope-evolution headroom** — see PRD-v1 for v0.1 → v0.5 → v1.0

---

## STRICT NO-NOS — do not violate any of these

### Universal (inherited from [[../NO-NOS|NO-NOS]])

1. NO hardcoded API keys, vendor URLs, or model names in code outside config files.
2. NO default that requires a sign-up upstream. Tool must work with a local model.
3. NO telemetry, phone-home, or analytics by default.
4. NO closed-source runtime dependencies that compromise sovereignty.
5. NO scope creep into "agent platform" or "general assistant" territory.
6. NO fake claims of being official upstream tooling.
7. NO breaking config changes in v0.x without printed migration path.
8. NO maintenance promises in README. "Best-effort community shovel."
9. NO PRs merged without working tests.
10. NO publishing v1.0 without PRD-v1 acceptance criteria met.

### Specific to sarvam-pdf

1. Never upload PDFs to any service the user didn't explicitly authorize.
2. Default mode for sensitive documents must be local-only fallback.
3. Glossary defaults must respect Indic technical-term conventions, not anglicize them.

### Anti-scope (what NOT to build, ever, in this repo)

Not an OCR tool in v0.1 (assume text PDFs; scanned PDFs come in v0.5). Not a document editor. Not a publishing tool.

---

## How to operate here

When you (the agent) start work in this repo:

1. **Read** [[PRD-v1]] before writing any code.
2. **Read** [[../PHILOSOPHY|Philosophy]] and [[../NO-NOS|NO-NOS]] if you haven't this session.
3. **Update** [[progress]] after each significant change. Update the YAML
   frontmatter — the root [[../PORTFOLIO]] view aggregates from it.
4. **If a decision conflicts with this AGENTS.md**, STOP and ask the human.
5. **If a user asks for something that violates a no-no**, push back. Don't comply.
6. **Commits** follow conventional commits (`feat:`, `fix:`, `docs:`, etc.).
7. **Branches**: `main` is protected. Work on `feat/<thing>` branches and PR.
8. **Tests** are required for any code path that ships in v0.1.

---

## Tombstone watch

What could kill this shovel:

Sarvam shipping their own desktop app. Probability low — they're an API company. Medium: a global tool (DeepL, Google) suddenly becoming as good at Indic.

What we're watching for:

Adobe Acrobat or Microsoft 365 shipping inline Indic translation. Plausible in 12–18 months.

If the kill signal triggers, notify the human before taking further action.

---

## Cross-references inside the vault

- Product spec: [[PRD-v1]]
- Public README: [[README]]
- Progress + status frontmatter: [[progress]]
- Internal knowledge graph: [[knowledge-graph]]
- Vault philosophy: [[../PHILOSOPHY]]
- Universal no-nos: [[../NO-NOS]]
- Naming convention: [[../NAMING-CONVENTION]]
- Portfolio view: [[../PORTFOLIO]]
- Launch plan: [[../LAUNCH-PLAN]]
