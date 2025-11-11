//! Greeting behavior that responds when a player gets close

use async_trait::async_trait;

use crate::agent::AgentContext;
use crate::oxyde_game::intent::{Intent, IntentType};
use crate::Result;

use super::base::{Behavior, BehaviorResult, BaseBehavior};

/// Greeting behavior that responds when a player gets close
#[derive(Debug)]
pub struct GreetingBehavior {
    /// Base behavior
    base: BaseBehavior,

    /// Distance threshold for greeting
    distance_threshold: f32,

    /// Greeting phrases
    greetings: Vec<String>,
}

impl GreetingBehavior {
    /// Create a new greeting behavior
    ///
    /// # Arguments
    ///
    /// * `distance_threshold` - Distance at which to trigger the greeting
    /// * `greetings` - List of possible greeting phrases
    ///
    /// # Returns
    ///
    /// A new GreetingBehavior
    pub fn new_with_options(distance_threshold: f32, greetings: Vec<String>) -> Self {
        Self {
            base: BaseBehavior::new(
                "greeting",
                "Greets the player when they get close",
                10,
                vec!["proximity".to_string()],
                60, // 1 minute cooldown
            ),
            distance_threshold,
            greetings,
        }
    }

    /// Create a new greeting behavior with default phrases
    ///
    /// # Returns
    ///
    /// A new GreetingBehavior with default greetings
    pub fn new_default() -> Self {
        Self::new_with_options(
            3.0,
            vec![
                "Hello there!".to_string(),
                "Greetings, traveler!".to_string(),
                "Welcome!".to_string(),
                "Good day to you!".to_string(),
                "Well met!".to_string(),
            ],
        )
    }

    /// Create a new greeting behavior with a single greeting phrase
    ///
    /// # Arguments
    ///
    /// * `greeting` - The greeting phrase to use
    ///
    /// # Returns
    ///
    /// A new GreetingBehavior with a single greeting
    pub fn new(greeting: &str) -> Self {
        Self::new_with_options(
            3.0,
            vec![greeting.to_string()],
        )
    }
}

#[async_trait]
impl Behavior for GreetingBehavior {
    async fn matches_intent(&self, intent: &Intent) -> bool {
        // Check if on cooldown
        if self.base.is_on_cooldown().await {
            return false;
        }

        // Check if player is close enough
        intent.intent_type == IntentType::Proximity || intent.intent_type == IntentType::Greeting
    }

    async fn execute(&self, _intent: &Intent, context: &AgentContext) -> Result<BehaviorResult> {
        // Check player distance in context
        let player_distance = context.get("player_distance")
            .and_then(|v| v.as_f64())
            .unwrap_or(f64::INFINITY) as f32;

        if player_distance <= self.distance_threshold {
            // Mark as executed to start cooldown
            self.base.mark_executed().await;

            // Select a random greeting
            let greeting_idx = rand::random::<usize>() % self.greetings.len();
            let greeting = &self.greetings[greeting_idx];

            Ok(BehaviorResult::Response(greeting.clone()))
        } else {
            // Player not close enough
            Ok(BehaviorResult::None)
        }
    }
}
