# oxyde-behavior

Emotion-aware behavior system for game NPCs and AI agents.

## Overview

This crate provides a flexible behavior system that integrates with emotional states. Behaviors can trigger based on emotions, modify emotions when executed, and have dynamic priorities influenced by emotional context.

## Features

- **Emotion Triggers**: Behaviors activate based on emotional conditions
- **Emotion Influences**: Behaviors modify emotional state when executed
- **Dynamic Priority**: Priority adjusts based on emotional context
- **Built-in Behaviors**: Flee, Aggressive, Friendly, Cautious, Joyful, and more
- **Intent Matching**: Behaviors respond to different player intents
- **Cooldown System**: Prevent behavior spam with cooldown tracking
- **Async Execution**: Full async/await support for behavior execution

## Planned Exports

```rust
#[async_trait]
pub trait Behavior: Send + Sync {
    async fn matches_intent(&self, intent: &Intent) -> bool;
    async fn execute(&self, intent: &Intent, context: &AgentContext) -> Result<BehaviorResult>;
    fn emotion_trigger(&self) -> Option<EmotionTrigger>;
    fn emotion_influences(&self) -> Vec<EmotionInfluence>;
    fn priority(&self) -> u32;
    fn emotional_priority_modifier(&self, state: &EmotionalState) -> i32;
}

pub enum EmotionTrigger {
    AnyEmotion { min_intensity: f32 },
    SpecificEmotion { emotion: String, min_value: f32 },
    ValenceRange { min: f32, max: f32 },
    HighArousal { min_arousal: f32 },
    Positive,
    Negative,
    None,
}

pub struct EmotionInfluence {
    pub emotion: String,
    pub delta: f32,
}

pub enum BehaviorResult {
    Response(String),
    Action(String),
    None,
}

// Built-in behaviors
pub struct FleeBehavior { ... }
pub struct AggressiveBehavior { ... }
pub struct FriendlyBehavior { ... }
pub struct CautiousBehavior { ... }
pub struct JoyfulBehavior { ... }
pub struct GreetingBehavior { ... }
pub struct DialogueBehavior { ... }
pub struct PathfindingBehavior { ... }
```

## Use Cases

- NPCs with emotion-driven actions
- Dynamic behavior selection based on mood
- Realistic fear/aggression/joy responses
- Context-aware NPC reactions
- Personality-driven dialogue choices
- Survival instinct modeling

## Status

**Ready for Publication** - Complete implementation with 8 built-in behaviors, comprehensive tests, and full emotion integration.

## Dependencies

- `oxyde-emotion` (for emotional state and triggers)
- `async-trait` (for async trait methods)
- `tokio` (for async runtime)
- `serde` (for serialization)

## Publication Priority

**Very High** - Unique emotion-behavior integration. First Rust crate to offer this level of emotional behavior control for game NPCs.
