# sarvam-pdf

> Drag a PDF, get it in your language. 22 Indic languages. Layout preserved.

**Status:** v0.1 — ready to use.

**Sovereignty:** sovereign-by-construction. BYO endpoint, BYO key. Local fallback documented.

This is a community project, **not affiliated with Sarvam AI**.
Best-effort community shovel — no SLA, no roadmap commitments.

---

## What this is

Indian SMBs, students, and government workers deal with English-language documents constantly — contracts, manuals, papers, government circulars. DeepL doesn't do Indic. Google Translate's PDF mode is layout-mangling.

sarvam-pdf extracts text from PDFs and translates it using Sarvam's best-in-class Indic translation API.

## What this isn't

- Not an OCR tool in v0.1 (assume text PDFs; scanned PDFs come in v0.5)
- Not a document editor
- Not a publishing tool

See [PRD-v1.md](./PRD-v1.md) for the full anti-scope definition.

---

## Install

### From source

**Prerequisites:**
- [Rust](https://rustup.rs/) 1.75+

```bash
git clone https://github.com/sovereign-shovels/sarvam-pdf.git
cd sarvam-pdf

# Build
cargo build --release

# The binary is at target/release/sarvam-pdf
```

---

## Usage

### Extract text from PDF

```bash
sarvam-pdf extract document.pdf
```

### Translate text

```bash
sarvam-pdf translate "Hello world" --from en-IN --to hi-IN
```

### Convert PDF to translated text

```bash
sarvam-pdf convert document.pdf --from en-IN --to hi-IN --output translated.txt
```

---

## Configure

Get a free API key from [Sarvam AI Dashboard](https://dashboard.sarvam.ai/). Then:

```bash
export SARVAM_API_KEY="your-key-here"
```

Or set it in your config file:

```toml
# ~/.config/sarvam-pdf/config.toml
endpoint = "https://api.sarvam.ai/translate"
api_key_env_var = "SARVAM_API_KEY"
model = "sarvam-translate:v1"
```

**Supported languages:** 22 Indic languages including `hi-IN`, `ta-IN`, `te-IN`, `bn-IN`, `mr-IN`, `gu-IN`, `kn-IN`, `ml-IN`, `pa-IN`, `en-IN`, and more.

### Environment variables

```bash
export SARVAM_PDF_ENDPOINT="https://api.sarvam.ai/translate"
export SARVAM_PDF_API_KEY_ENV="SARVAM_API_KEY"
export SARVAM_PDF_MODEL="sarvam-translate:v1"
```

---

## Why this exists

Massive B2B and education demand in India. Sarvam isn't going to ship a desktop app. The gap is structural.

See [PRD-v1.md](./PRD-v1.md) for the full problem statement and rationale.

## What's next

- **v0.5:** Batch folder processing, glossary support, side-by-side preview
- **v1.0:** Office document support (.docx, .pptx), web service mode for SMB intranets

See [PRD-v1.md](./PRD-v1.md) for the full roadmap.

---

## License

Apache 2.0. See [LICENSE](./LICENSE).

## Part of sovereign-shovels

This repo is part of the [sovereign-shovels](https://github.com/sovereign-shovels) portfolio of small, focused, sovereign-by-construction AI utilities.

Other shovels: claude-vault, bulbul-studio, saaras-tray, claude-prompts, ollama-cron, mcp-forge, sarvam-pdf, agent-console, sarvam-meet, obsidian-llm, llm-diff, claude-bridge, claude-radio, sarvam-cast.
