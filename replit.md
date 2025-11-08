# Oxyde: AI Agent SDK for Game NPCs

## Overview
Oxyde is a Rust-based SDK for creating autonomous, goal-driven NPCs with advanced AI and emotional intelligence. This project enables NPCs to pursue their own objectives, adapt to player interactions, and generate emergent storylines in real-time.

## Project Architecture

### Technology Stack
- **Language**: Rust (stable)
- **Framework**: Custom TCP server for web demo
- **AI Integration**: Multi-LLM support (OpenAI, Anthropic, Groq, xAI, Perplexity)
- **Frontend**: Vanilla JavaScript (embedded in Rust server)
- **Port**: 5000 (web server)

### Project Structure
```
oxyde/
├── src/                    # Core SDK library
│   ├── agent.rs           # Goal-driven agent implementation
│   ├── inference.rs       # Multi-LLM provider abstraction
│   ├── memory.rs          # Vector embeddings & memory system
│   └── oxyde_game/        # Game integration modules
│       ├── behavior.rs    # Autonomous NPC behaviors
│       ├── intent.rs      # Player intent detection
│       └── bindings/      # Engine integration (Unity, Unreal, WASM)
├── examples/
│   └── rpg_demo/          # Interactive web demo
│       ├── src/
│       │   ├── web_server.rs       # Main web server (port 5000)
│       │   ├── llm_service.rs      # LLM provider management
│       │   ├── emotion_engine.rs   # 6D emotional tracking
│       │   └── goal_system.rs      # Goal management engine
│       └── Cargo.toml
└── Cargo.toml             # Workspace configuration
```

## Recent Changes
- **2025-11-08**: Initial Replit setup
  - Installed Rust stable and Node.js 20
  - Installed OpenSSL and pkg-config system dependencies
  - Successfully compiled web_server binary
  - Configured for port 5000 (already set in source)

## User Preferences
None specified yet.

## Key Features
1. **Goal-Driven AI**: NPCs pursue personal objectives autonomously
2. **Emotional Intelligence**: 6-dimensional emotional tracking (happiness, anger, fear, trust, energy, curiosity)
3. **Multi-LLM Architecture**: Smart provider selection across multiple AI providers
4. **Emergent Storytelling**: Dynamic story generation from NPC interactions
5. **Real-Time Adaptation**: NPCs learn and evolve from player interactions

## Running the Project

### Web Demo
The main application is an interactive RPG demo with AI-powered NPCs:
```bash
cd examples/rpg_demo
cargo run --release --bin web_server
```
Access at: http://localhost:5000

### Required API Keys
At least one of the following LLM provider API keys is required:
- `OPENAI_API_KEY` - For OpenAI GPT models (recommended)
- `ANTHROPIC_API_KEY` - For Anthropic Claude models
- `GROQ_API_KEY` - For fast inference
- `XAI_API_KEY` - For creative dialogue
- `PERPLEXITY_API_KEY` - For real-time knowledge

## NPCs in Demo
1. **Marcus the Merchant** - Goal: "earn 1000 gold coins"
2. **Gareth the Guard** - Goal: "uncover the local smuggling operation"
3. **Velma the Villager** - Goal: "organize the harvest festival"

## Dependencies

### System
- OpenSSL (for HTTPS connections)
- pkg-config (for build system)

### Rust Crates
- tokio (async runtime)
- reqwest (HTTP client)
- serde/serde_json (serialization)
- oxyde (main SDK library)

### Node.js
- @anthropic-ai/sdk
- openai

## Development Notes
- The web server binds to `0.0.0.0:5000` (already configured for Replit)
- Compilation warnings are present but don't affect functionality
- The main.rs binary has issues, but web_server binary works correctly
- Profile settings in rpg_demo/Cargo.toml are ignored (workspace-level profiles apply)
