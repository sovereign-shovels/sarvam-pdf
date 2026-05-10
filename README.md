# sarvam-pdf

> Drag a PDF, get it in your language. 22 Indic languages. Layout preserved.

**Status:** v0.1 — planning. Not yet released.

**Sovereignty:** sovereign-by-construction. BYO endpoint, BYO key, BYO model.
A local-only configuration is documented and tested.

This is a community project, **not affiliated with Sarvam**.
Best-effort community shovel — no SLA, no roadmap commitments.

---

## What this is

Drag a PDF, get it in your language. 22 Indic languages. Layout preserved.

## What this isn't

Not an OCR tool in v0.1 (text PDFs only; OCR comes in v0.5). Not a document editor. Not a publishing tool.

## Install

> Coming with v0.1 release.

## Configure

You bring the model. By default `sarvam-pdf` tries to use a local provider:

- For LLM endpoints: Ollama at `http://localhost:11434`
- For voice endpoints: configurable, see [docs/configure.md]

To use any other provider (Claude, GPT, Hermes, OpenRouter, Sarvam, etc.):

```toml
# ~/.config/sarvam-pdf/config.toml
[provider]
endpoint = "https://api.your-provider.com/v1"
api_key_env = "YOUR_PROVIDER_KEY"
model = "your-model-name"
```

Anthropic, OpenAI, and Sarvam endpoints all work. Local Ollama, llama.cpp,
LM Studio, and vLLM all work via their OpenAI-compatible endpoints.

## Why this exists

Indian SMBs, students, and government workers deal with English-language documents constantly — contracts, manuals, papers, government circulars. DeepL doesn't do Indic. Google Translate's PDF mode is layout-mangling. Sarvam has best-in-class Indic translation but only as an API. sarvam-pdf is the desktop app: drag PDF in, get the translated PDF out.

## What's next

See [PRD-v1.md](./PRD-v1.md) for the full v0.1 → v0.5 → v1.0 plan.

## License

Apache 2.0. See [LICENSE](./LICENSE).

## Part of sovereign-shovels

This repo is part of the [sovereign-shovels](https://github.com/sovereign-shovels)
portfolio of small, focused, sovereign-by-construction AI utilities.

Other shovels: claude-vault, bulbul-studio, saaras-tray, claude-prompts,
ollama-cron, mcp-forge, sarvam-pdf, agent-console, sarvam-meet, obsidian-llm,
llm-diff, claude-bridge, claude-radio, sarvam-cast.
