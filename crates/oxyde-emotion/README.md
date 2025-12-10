# oxyde-emotion

Production-ready emotional simulation system based on Plutchik's Wheel of Emotions.

## Overview

This crate provides a comprehensive emotion system for game NPCs and AI agents. It implements Plutchik's 8 primary emotions with automatic opposite emotion handling, valence/arousal calculations, and temporal decay.

## Features

- **8 Primary Emotions**: Joy, Trust, Fear, Surprise, Sadness, Disgust, Anger, Anticipation
- **Automatic Opposites**: Updates to one emotion automatically affect its opposite
- **Valence/Arousal Model**: Calculate emotional positivity and intensity
- **Temporal Decay**: Emotions naturally return to baseline over time
- **Memory Integration**: Emotional context for memories and experiences
- **Zero Dependencies**: Only requires `serde` for serialization

## Planned Exports

```rust
pub struct EmotionalState { ... }
pub struct EmotionalMemory { ... }
pub enum EmotionalContext { ... }

impl EmotionalState {
    pub fn new() -> Self;
    pub fn update_emotion(&mut self, emotion: &str, delta: f32);
    pub fn dominant_emotion(&self) -> (&str, f32);
    pub fn valence(&self) -> f32;
    pub fn arousal(&self) -> f32;
    pub fn is_positive(&self) -> bool;
    pub fn is_negative(&self) -> bool;
    pub fn decay(&mut self, rate: f32);
}
```

## Use Cases

- Game NPCs with realistic emotional responses
- Chatbots with emotional awareness
- Interactive storytelling systems
- Virtual characters with personality
- Emotion-aware dialogue systems
- Behavioral AI with emotional triggers

## Status

**Ready for Publication** - Complete implementation, tested, documented. Currently part of the Oxyde SDK, planned to be extracted as a standalone crate.

## Publication Priority

**Very High** - This is the most standalone component with the broadest appeal beyond game NPCs. It's the first production Plutchik implementation for games in Rust.
