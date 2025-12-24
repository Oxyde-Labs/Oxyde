# Configuration Demo

This example demonstrates creating Oxyde agents from configuration files.

## Setup

1. Copy `.env.example` to `.env`:
```bash
   cp .env.example .env
```

2. Add your OpenAI API key to `.env`:
```
   OPENAI_API_KEY=sk-...
```

3. Run the demo:
```bash
   cargo run --example config_demo
```

## Configuration Files

- `merchant_agent.json` - Merchant NPC (JSON format)
- `guard_agent.yaml` - Guard NPC (YAML format)
- `villager_agent.toml` - Villager NPC (TOML format)
- `prompts/custom_prompts.toml` - Custom medieval-style prompts

## Available Demos

1. Load agent from JSON
2. Load agent from YAML
3. Load agent from TOML
4. Use custom prompt templates
5. Use bundled default prompts
6. Interactive chat with agent
7. Compare different personalities