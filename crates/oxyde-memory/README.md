# oxyde-memory

Advanced memory system for AI agents with emotional weighting and retrieval.

## Overview

This crate provides a sophisticated memory system that integrates with emotional states. Memories are tagged with emotional context, enabling emotionally-weighted retrieval and realistic memory formation/recall patterns.

## Features

- **Emotional Memory**: Memories tagged with emotional context
- **Weighted Retrieval**: Retrieve memories based on emotional similarity
- **Importance Scoring**: Automatic memory importance calculation
- **Decay System**: Memories fade over time unless reinforced
- **Vector Memory Support**: Optional feature for semantic similarity search
- **Working Memory**: Short-term vs long-term memory distinction
- **Memory Consolidation**: Transfer important memories to long-term storage

## Planned Exports

```rust
pub struct Memory { ... }
pub struct MemoryStore { ... }
pub struct MemoryQuery { ... }
pub enum MemoryType { Episodic, Semantic, Procedural }

impl MemoryStore {
    pub fn new() -> Self;
    pub fn store(&mut self, memory: Memory) -> Result<MemoryId>;
    pub fn recall(&self, query: &MemoryQuery) -> Vec<&Memory>;
    pub fn recall_emotional(&self, emotional_state: &EmotionalState) -> Vec<&Memory>;
    pub fn consolidate(&mut self, importance_threshold: f32);
    pub fn decay(&mut self, rate: f32);
}
```

## Use Cases

- NPCs that remember past interactions
- Emotionally-aware conversation history
- Dynamic relationship systems
- Trauma and positive reinforcement modeling
- Character development over time
- Context-aware dialogue generation

## Status

**Requires Refactoring** - Currently integrated into the main SDK. Needs to be extracted with clean separation from agent core.

## Dependencies

- `oxyde-emotion` (for emotional context)
- `serde` (for serialization)
- Optional: `ndarray` (for vector memory feature)

## Publication Priority

**Very High** - Unique emotional memory integration. No other Rust crate offers this combination.
