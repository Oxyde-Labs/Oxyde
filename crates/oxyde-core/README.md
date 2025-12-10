# oxyde-core

Core types, traits, and utilities for the Oxyde SDK ecosystem.

## Overview

This crate provides the foundational types and traits used across all Oxyde crates. It defines the common interfaces that enable the various components to work together seamlessly.

## Features

- **Agent Traits**: Core agent lifecycle and execution traits
- **Result Types**: Common error handling and result types
- **Event System**: Event definitions and handling
- **State Management**: Core state transition types
- **Context Types**: Shared context definitions
- **Common Utilities**: Shared helper functions

## Planned Exports

```rust
// Core traits
pub trait Agent: Send + Sync {
    async fn start(&self) -> Result<()>;
    async fn stop(&self) -> Result<()>;
    async fn process_input(&self, input: &str) -> Result<String>;
}

// Error types
pub type Result<T> = std::result::Result<T, OxydeError>;

#[derive(Debug, thiserror::Error)]
pub enum OxydeError {
    #[error("Agent error: {0}")]
    AgentError(String),
    #[error("State error: {0}")]
    StateError(String),
    #[error("Memory error: {0}")]
    MemoryError(String),
    #[error("Behavior error: {0}")]
    BehaviorError(String),
    // ...
}

// Event types
pub enum AgentEvent {
    Started,
    Stopped,
    StateChanged { from: String, to: String },
    EmotionChanged { emotion: String, value: f32 },
    MemoryStored { memory_id: String },
    BehaviorExecuted { behavior: String },
}

// Context types
pub type AgentContext = HashMap<String, serde_json::Value>;
```

## Use Cases

- Building custom Oxyde integrations
- Creating new behaviors or memory systems
- Extending the SDK with custom components
- Ensuring type compatibility across crates

## Status

**Essential** - Must be published first as all other crates depend on it.

## Dependencies

- `thiserror` (for error types)
- `serde` (for serialization)
- `uuid` (for IDs)
- `tokio` (for async traits)

## Publication Priority

**Critical** - Must be published before any other Oxyde crates as they all depend on these core types.
