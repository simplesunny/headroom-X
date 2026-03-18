# Headroom for Zed

Context compression for Zed's AI agent. Same answers, fraction of the tokens.

## What it does

This extension gives Zed's AI agent three MCP tools:

- **headroom_compress** — Compress content on demand (files, JSON, logs, search results)
- **headroom_retrieve** — Retrieve original uncompressed content by hash
- **headroom_stats** — Session compression statistics

## Setup

### 1. Install Headroom

```bash
pip install "headroom-ai[mcp]"
```

### 2. Install this extension

In Zed: `zed: install extension` → search "Headroom"

### 3. (Optional) Enable automatic compression

For automatic compression of ALL LLM traffic, start the Headroom proxy and point Zed at it:

```bash
headroom proxy
```

Then in Zed settings (`settings.json`):

```json
{
  "language_models": {
    "openai": {
      "api_url": "http://localhost:8787/v1"
    }
  }
}
```

Now every request Zed's agent makes is automatically compressed — 50-90% fewer tokens, same quality.

## How it works

**MCP tools (this extension):** The agent can call `headroom_compress` to shrink large content before reasoning over it. Originals are stored locally and retrievable via `headroom_retrieve`.

**Proxy (optional):** All LLM API calls route through Headroom's compression pipeline. JSON arrays, code, logs, and text are automatically compressed. The agent doesn't need to do anything — compression is transparent.

Both work independently or together.

## Requirements

- [Headroom](https://github.com/chopratejas/headroom) (`pip install "headroom-ai[mcp]"`)
- Python 3.10+
- `headroom` command must be in PATH
