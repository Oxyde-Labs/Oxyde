//! Base behavior functionality with cooldown tracking

use std::collections::HashMap;
use std::time::{Duration, Instant};

use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use tokio::sync::RwLock;

use oxyde_core::{AgentContext, Result};
use oxyde_emotion::EmotionalState;
use oxyde_intent::Intent;

/// Emotional trigger condition for behaviors
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum EmotionTrigger {
    /// Trigger when any emotion exceeds threshold
    AnyEmotion { min_intensity: f32 },

    /// Trigger when specific emotion exceeds threshold
    SpecificEmotion { emotion: String, min_value: f32 },

    /// Trigger when valence is in range
    ValenceRange { min: f32, max: f32 },

    /// Trigger when arousal exceeds threshold
    HighArousal { min_arousal: f32 },

    /// Trigger when in positive emotional state
    Positive,

    /// Trigger when in negative emotional state
    Negative,

    /// No emotional trigger (always passes)
    None,
}

impl EmotionTrigger {
    /// Check if the emotional state satisfies this trigger
    pub fn matches(&self, state: &EmotionalState) -> bool {
        match self {
            EmotionTrigger::AnyEmotion { min_intensity } => {
                state.arousal() >= *min_intensity
            }
            EmotionTrigger::SpecificEmotion { emotion, min_value } => {
                let (dominant, value) = state.dominant_emotion();
                dominant == emotion && value >= *min_value
            }
            EmotionTrigger::ValenceRange { min, max } => {
                let valence = state.valence();
                valence >= *min && valence <= *max
            }
            EmotionTrigger::HighArousal { min_arousal } => {
                state.arousal() >= *min_arousal
            }
            EmotionTrigger::Positive => state.is_positive(),
            EmotionTrigger::Negative => state.is_negative(),
            EmotionTrigger::None => true,
        }
    }
}

/// Emotional influence that a behavior has when executed
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EmotionInfluence {
    /// Emotion to modify
    pub emotion: String,

    /// Delta to apply (-1.0 to 1.0)
    pub delta: f32,
}

impl EmotionInfluence {
    /// Create a new emotion influence
    pub fn new(emotion: &str, delta: f32) -> Self {
        Self {
            emotion: emotion.to_string(),
            delta: delta.clamp(-1.0, 1.0),
        }
    }
}

/// Result of executing a behavior
#[derive(Debug, Clone)]
pub enum BehaviorResult {
    /// Behavior produced a text response
    Response(String),

    /// Behavior triggered an action
    Action(String),

    /// Behavior did not produce a result
    None,
}

/// Trait for NPC behaviors
#[async_trait]
pub trait Behavior: Send + Sync + std::fmt::Debug {
    /// Check if this behavior matches the given intent
    ///
    /// # Arguments
    ///
    /// * `intent` - Player intent to check against
    ///
    /// # Returns
    ///
    /// Whether this behavior should respond to the intent
    async fn matches_intent(&self, intent: &Intent) -> bool;

    /// Execute the behavior
    ///
    /// # Arguments
    ///
    /// * `intent` - Player intent to respond to
    /// * `context` - Current context data
    ///
    /// # Returns
    ///
    /// Result of executing the behavior
    async fn execute(&self, intent: &Intent, context: &AgentContext) -> Result<BehaviorResult>;

    /// Get the emotional trigger for this behavior (optional)
    ///
    /// Behaviors can override this to specify when they should trigger
    /// based on the agent's emotional state.
    ///
    /// # Returns
    ///
    /// Emotion trigger condition, or None to always trigger
    fn emotion_trigger(&self) -> Option<EmotionTrigger> {
        Some(EmotionTrigger::None)
    }

    /// Get the emotional influences this behavior produces (optional)
    ///
    /// Behaviors can override this to specify how executing them
    /// affects the agent's emotional state.
    ///
    /// # Returns
    ///
    /// Vector of emotion influences to apply when behavior executes
    fn emotion_influences(&self) -> Vec<EmotionInfluence> {
        Vec::new()
    }

    /// Get the base priority of this behavior
    ///
    /// Can be overridden by behaviors that need dynamic priority
    ///
    /// # Returns
    ///
    /// Priority value (higher = more important)
    fn priority(&self) -> u32 {
        50 // Default medium priority
    }

    /// Calculate dynamic priority based on emotional state
    ///
    /// Behaviors can override this to adjust priority based on emotions.
    /// The final priority will be base_priority + emotional_priority_modifier.
    ///
    /// # Arguments
    ///
    /// * `emotional_state` - Current emotional state
    ///
    /// # Returns
    ///
    /// Priority modifier to add to base priority
    fn emotional_priority_modifier(&self, _emotional_state: &EmotionalState) -> i32 {
        0
    }
}

/// Base behavior with cooldown tracking
#[derive(Debug)]
pub struct BaseBehavior {
    /// Name of the behavior
    name: String,

    /// Description of the behavior
    description: String,

    /// Priority of the behavior (higher means more important)
    priority: u32,

    /// Intent types this behavior responds to
    #[allow(dead_code)]
    intent_types: Vec<String>,

    /// Cooldown period in seconds
    cooldown_seconds: u64,

    /// Last execution time
    last_execution: RwLock<Option<Instant>>,

    /// Custom parameters
    parameters: HashMap<String, serde_json::Value>,
}

impl BaseBehavior {
    /// Create a new base behavior
    ///
    /// # Arguments
    ///
    /// * `name` - Name of the behavior
    /// * `description` - Description of the behavior
    /// * `priority` - Priority of the behavior
    /// * `intent_types` - Intent types this behavior responds to
    /// * `cooldown_seconds` - Cooldown period in seconds
    ///
    /// # Returns
    ///
    /// A new BaseBehavior
    pub fn new(
        name: &str,
        description: &str,
        priority: u32,
        intent_types: Vec<String>,
        cooldown_seconds: u64,
    ) -> Self {
        Self {
            name: name.to_string(),
            description: description.to_string(),
            priority,
            intent_types,
            cooldown_seconds,
            last_execution: RwLock::new(None),
            parameters: HashMap::new(),
        }
    }

    /// Get the behavior name
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Get the behavior description
    pub fn description(&self) -> &str {
        &self.description
    }

    /// Get the behavior priority
    pub fn priority(&self) -> u32 {
        self.priority
    }

    /// Check if the behavior is on cooldown
    ///
    /// # Returns
    ///
    /// Whether the behavior is currently on cooldown
    pub async fn is_on_cooldown(&self) -> bool {
        let last_execution = self.last_execution.read().await;

        if let Some(time) = *last_execution {
            let elapsed = time.elapsed();
            elapsed < Duration::from_secs(self.cooldown_seconds)
        } else {
            false
        }
    }

    /// Update the last execution time
    pub async fn mark_executed(&self) {
        let mut last_execution = self.last_execution.write().await;
        *last_execution = Some(Instant::now());
    }

    /// Set a parameter value
    ///
    /// # Arguments
    ///
    /// * `key` - Parameter key
    /// * `value` - Parameter value
    pub fn set_parameter<T: Serialize>(&mut self, key: &str, value: T) -> Result<()> {
        let json_value = serde_json::to_value(value)?;
        self.parameters.insert(key.to_string(), json_value);
        Ok(())
    }

    /// Get a parameter value
    ///
    /// # Arguments
    ///
    /// * `key` - Parameter key
    ///
    /// # Returns
    ///
    /// Parameter value or None if not found
    pub fn get_parameter<T: for<'de> Deserialize<'de>>(&self, key: &str) -> Result<Option<T>> {
        if let Some(value) = self.parameters.get(key) {
            let typed_value = serde_json::from_value(value.clone())?;
            Ok(Some(typed_value))
        } else {
            Ok(None)
        }
    }
}
