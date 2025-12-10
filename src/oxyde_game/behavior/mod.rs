//! NPC behavior system for game agents
//!
//! This module provides a flexible behavior system for NPCs, including:
//! - Base behavior trait and implementation
//! - Greeting behavior for proximity detection
//! - Dialogue behavior for topic-based conversations
//! - Pathfinding behavior for navigation
//! - Emotion-aware behaviors that trigger based on emotional state
//! - Behavior selection strategies (emotion-modulated, fixed-priority)

mod base;
mod dialogue;
mod emotional;
mod greeting;
mod pathfinding;
mod strategy;

pub mod factory;

// Re-export all public types
pub use base::{Behavior, BehaviorResult, BaseBehavior, EmotionInfluence, EmotionTrigger};
pub use dialogue::DialogueBehavior;
pub use emotional::{
    AggressiveBehavior, CautiousBehavior, FleeBehavior, FriendlyBehavior, JoyfulBehavior,
    // Neutral fallback behaviors
    NeutralGreetingBehavior, ConfusedBehavior, PoliteDeclineBehavior,
    ThoughtfulPauseBehavior, DefaultAcknowledgeBehavior,
};
pub use greeting::GreetingBehavior;
pub use pathfinding::PathfindingBehavior;
pub use strategy::{SelectionStrategy, EmotionModulatedStrategy, FixedPriorityStrategy};

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    #[tokio::test]
    async fn test_greeting_behavior() {
        use crate::oxyde_game::intent::{Intent, IntentType};

        let intent = Intent {
            intent_type: IntentType::Proximity,
            confidence: 1.0,
            raw_input: "".to_string(),
            keywords: vec![],
        };

        let mut context = HashMap::new();
        context.insert("player_distance".to_string(), serde_json::json!(2.0));

        let behavior = GreetingBehavior::new_default();

        assert!(behavior.matches_intent(&intent).await);

        let result = behavior.execute(&intent, &context).await.unwrap();
        match result {
            BehaviorResult::Response(text) => {
                assert!(!text.is_empty());
            },
            _ => panic!("Expected Response result"),
        }
    }
}
