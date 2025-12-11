# oxyde-inference

LLM inference abstraction layer for Oxyde agents.

## Overview

This crate provides a unified interface for different LLM inference backends. It abstracts away the complexity of working with various LLM providers and local models, allowing agents to switch between backends easily.

## Features

- **Multi-Backend Support**: OpenAI, Anthropic, local models, custom providers
- **Unified API**: Single interface for all backends
- **Streaming Support**: Real-time response streaming
- **Prompt Templates**: Pre-built templates for common NPC tasks
- **Token Management**: Automatic token counting and management
- **Caching**: Response caching for repeated queries
- **Fallback System**: Automatic fallback to alternative backends

## Planned Exports

```rust
#[async_trait]
pub trait InferenceBackend: Send + Sync {
    async fn generate(&self, prompt: &str, options: &GenerationOptions) -> Result<String>;
    async fn generate_stream(&self, prompt: &str, options: &GenerationOptions)
        -> Result<impl Stream<Item = Result<String>>>;
    fn max_tokens(&self) -> usize;
    fn count_tokens(&self, text: &str) -> usize;
}

pub struct GenerationOptions {
    pub temperature: f32,
    pub max_tokens: usize,
    pub stop_sequences: Vec<String>,
    pub top_p: f32,
    // ...
}

// Built-in backends
pub struct OpenAIBackend { ... }
pub struct AnthropicBackend { ... }
pub struct LocalModelBackend { ... }

// Prompt templates
pub struct PromptTemplate {
    pub template: String,
    pub variables: HashMap<String, String>,
}

impl PromptTemplate {
    pub fn npc_dialogue(character_traits: &str, context: &str) -> Self;
    pub fn intent_classification(input: &str) -> Self;
    pub fn emotion_analysis(input: &str) -> Self;
}
```

## Use Cases

- NPC dialogue generation
- Intent understanding
- Dynamic story generation
- Context-aware responses
- Multi-modal AI agents
- Edge deployment with local models

## Status

**Lower Priority** - The current llmchain integration works well. This would provide a cleaner abstraction but isn't critical.

## Dependencies

- `oxyde-core` (for core types)
- `async-trait` (for async traits)
- `tokio` (for async runtime)
- `reqwest` (for HTTP clients)
- Optional: `llmchain`, `rust-bert`, etc.

## Publication Priority

**Medium** - Useful but not critical. Many users will use their own inference systems.
