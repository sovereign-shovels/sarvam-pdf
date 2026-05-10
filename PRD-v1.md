---
repo: sarvam-pdf
rank: 7
score: 0.67
sprint: 3
substrate_anchor: Sarvam
build_estimate: "3–4 weeks for v0.1"
status: planned
---

# PRD v1.0 — sarvam-pdf

> **One-liner:** Drag a PDF, get it in your language. 22 Indic languages. Layout preserved.
>
> **Substrate:** Indian SMBs, students, government workers translating English documents to/from Indic languages
> **Launch channels:** LinkedIn India, r/India, Indian SMB communities, AIKosh, education networks
> **Build estimate (v0.1):** 3–4 weeks for v0.1

---

## What problem does this solve

Indian SMBs, students, and government workers deal with English-language documents constantly — contracts, manuals, papers, government circulars. DeepL doesn't do Indic. Google Translate's PDF mode is layout-mangling. Sarvam has best-in-class Indic translation but only as an API. sarvam-pdf is the desktop app: drag PDF in, get the translated PDF out.

## Why this is a shovel and not a product

Massive B2B and education demand in India. Sovereign by construction (local fallback supported). Sarvam isn't going to ship a desktop app. Scope-evolves into office-document support, web-page translation, batch processing.

---

## v0.1 — what ships

Desktop app (Tauri). Drag-drop PDF in. Pick target language. Get translated PDF out, layout preserved where possible. Sarvam PDF Translation API as primary; LibreOffice + sarvam-translate as local fallback.

### Acceptance criteria for v0.1

A v0.1 release is publishable to GitHub when ALL of these are true:

- [ ] Core functionality described above works on the primary developer machine.
- [ ] At least one local-only configuration is documented and tested (no cloud required).
- [ ] BYO endpoint / BYO key configuration is documented.
- [ ] README explains: what it is, who it's for, how to install, how to configure, what it doesn't do.
- [ ] LICENSE present (Apache 2.0 unless overridden).
- [ ] No hardcoded keys or vendor URLs anywhere.
- [ ] No telemetry / phone-home.
- [ ] At least one passing test for the main code path.
- [ ] CI green.
- [ ] AGENTS.md compliance reviewed.

## v0.5 — first major evolution

Batch folder processing. Glossary support (preserve technical terms). Side-by-side preview.

## v1.0 — fuller scope

Office document support (.docx, .pptx). Web service mode for SMB intranets.

---

## Architecture sketch

### Stack

Tauri shell. PDF.js for rendering. Sarvam API for translation. Local fallback uses pdftotext + sarvam-translate + reportlab/wkhtmltopdf.

### Provider abstraction

The shovel MUST expose a provider abstraction even if v0.1 only uses one
provider. Suggested shape:

```
interface Provider {
  name: string;
  endpoint: URL;
  apiKeyEnvVar: string;
  call(input: ProviderInput): Promise<ProviderOutput>;
}
```

The default config in v0.1 must point to a free, local provider where
applicable, and document how to swap in any other.

### Configuration

Configuration order of precedence (highest to lowest):

1. Command-line flags
2. Environment variables (prefix: `SARVAM_PDF_*`)
3. User config file (`~/.config/sarvam-pdf/config.toml` on Linux/Mac, equivalent on Windows)
4. Default config (shipped, but never with secrets)

---

## Anti-scope (do NOT build)

Not an OCR tool in v0.1 (assume text PDFs; scanned PDFs come in v0.5). Not a document editor. Not a publishing tool.

---

## Tombstone risk and mitigation

**Risk:** Sarvam shipping their own desktop app. Probability low — they're an API company. Medium: a global tool (DeepL, Google) suddenly becoming as good at Indic.

**Mitigation:** Ship fast (v0.1 in 3–4 weeks for v0.1). Build community early
(launch on LinkedIn India, r/India, Indian SMB communities, AIKosh, education networks). Even if upstream absorbs the feature, accumulated
stars and the community are the audience-build payoff.

**Kill signal:** Adobe Acrobat or Microsoft 365 shipping inline Indic translation. Plausible in 12–18 months.

If the kill signal triggers, the maintainer must announce within one week and
either (a) refocus on a remaining gap, (b) merge gracefully into upstream if
they're receptive, or (c) mark the repo as archived with a clear pointer to the
replacement.

---

## Launch plan

### Pre-launch checklist

- [ ] Repo on GitHub at `github.com/sovereign-shovels/sarvam-pdf`
- [ ] README polished (see template in `_templates/`)
- [ ] At least 3 issues / discussions seeded (real ones, not placeholder)
- [ ] LICENSE, CODE_OF_CONDUCT, CONTRIBUTING present
- [ ] Demo asset (gif, screenshot, or short video — depending on category)
- [ ] First-launch post drafted for primary launch channel

### Day-1 launch

Post to: LinkedIn India, r/India, Indian SMB communities, AIKosh, education networks

Subject template (adjust per channel):
- Show HN: `Show HN: sarvam-pdf – Drag a PDF, get it in your language. 22 Indic languages. Layout preserved.`
- Reddit: `[OSS] Drag a PDF, get it in your language. 22 Indic languages. Layout preserved.` with full post explaining the gap and the build
- Twitter/X: thread leading with the demo gif

### Week-1 follow-up

- Respond to every issue and comment within 24h.
- Ship at least one bugfix release based on launch feedback.
- Cross-post to secondary channels.

### Month-1 review

- Assess star velocity and community formation.
- If kill signal triggered, follow tombstone protocol above.
- If trajectory is healthy, plan v0.5.

---

## Cross-references

- Constitution: [[AGENTS]]
- Public README: [[README]]
- Progress frontmatter: [[progress]]
- Internal knowledge graph: [[knowledge-graph]]
