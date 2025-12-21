//! Dialogue behavior for topic-based conversations

use std::collections::HashMap;

use async_trait::async_trait;

use oxyde_core::AgentContext;
use oxyde_intent::{Intent, IntentType};
use oxyde_core::Result;

use super::base::{Behavior, BehaviorResult, BaseBehavior};

/// Dialogue behavior that responds to specific topics
#[derive(Debug)]
pub struct DialogueBehavior {
    /// Base behavior
    #[allow(dead_code)]
    base: BaseBehavior,

    /// Topic responses
    topics: HashMap<String, Vec<String>>,

    /// Default responses when topic not found
    default_responses: Vec<String>,
}

impl DialogueBehavior {
    /// Create a new dialogue behavior
    ///
    /// # Arguments
    ///
    /// * `topics` - Map of topics to possible responses
    /// * `default_responses` - Default responses when topic not found
    ///
    /// # Returns
    ///
    /// A new DialogueBehavior
    pub fn new(topics: HashMap<String, Vec<String>>, default_responses: Vec<String>) -> Self {
        Self {
            base: BaseBehavior::new(
                "dialogue",
                "Responds to topic-based conversations",
                5,
                vec!["question".to_string(), "chat".to_string()],
                0, // No cooldown for dialogue
            ),
            topics,
            default_responses,
        }
    }
}

#[async_trait]
impl Behavior for DialogueBehavior {
    async fn matches_intent(&self, intent: &Intent) -> bool {
        matches!(
            intent.intent_type,
            IntentType::Question | IntentType::Chat | IntentType::Command
        )
    }

    async fn execute(&self, intent: &Intent, _context: &AgentContext) -> Result<BehaviorResult> {
        // Extract topic from intent
        let topic = intent.raw_input.to_lowercase();

        // Find matching topic
        let response = self
            .topics
            .iter()
            .find(|(key, _)| topic.contains(key.as_str()))
            .and_then(|(_, responses)| {
                if responses.is_empty() {
                    None
                } else {
                    let idx = rand::random::<usize>() % responses.len();
                    Some(responses[idx].clone())
                }
            });

        // Use the found response or fall back to default
        let final_response = match response {
            Some(r) => r,
            None => {
                if self.default_responses.is_empty() {
                    return Ok(BehaviorResult::None);
                }
                let idx = rand::random::<usize>() % self.default_responses.len();
                self.default_responses[idx].clone()
            }
        };

        Ok(BehaviorResult::Response(final_response))
    }
}
