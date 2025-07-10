//! Behavior system for game NPCs
//!
//! This module provides the behavior system for NPCs, allowing them to
//! react to player actions, environmental changes, and other triggers.

use std::collections::HashMap;
use std::sync::Arc;
use std::time::{Duration, Instant};

use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use tokio::sync::RwLock;

use crate::agent::AgentContext;
use crate::oxyde_game::intent::Intent;
use crate::Result;

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
        intent.intent_type == "proximity" || intent.intent_type == "greeting"
    }
    
    async fn execute(&self, intent: &Intent, context: &AgentContext) -> Result<BehaviorResult> {
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

/// DialogueBehavior that handles conversations with players
#[derive(Debug)]
pub struct DialogueBehavior {
    /// Base behavior
    base: BaseBehavior,
    
    /// Topics the NPC can discuss
    topics: HashMap<String, Vec<String>>,
    
    /// Default response when no matching topic is found
    default_responses: Vec<String>,
}

impl DialogueBehavior {
    /// Create a new dialogue behavior
    ///
    /// # Arguments
    ///
    /// * `topics` - Map of topics to potential responses
    /// * `default_responses` - Default responses when no topic matches
    ///
    /// # Returns
    ///
    /// A new DialogueBehavior
    pub fn new(
        topics: HashMap<String, Vec<String>>,
        default_responses: Vec<String>,
    ) -> Self {
        Self {
            base: BaseBehavior::new(
                "dialogue",
                "Engages in conversation with the player",
                50,
                vec!["question".to_string(), "chat".to_string()],
                0, // No cooldown for dialogue
            ),
            topics,
            default_responses,
        }
    }
    
    /// Add a topic to the dialogue behavior
    ///
    /// # Arguments
    ///
    /// * `topic` - Topic keyword
    /// * `responses` - Potential responses for this topic
    pub fn add_topic(&mut self, topic: &str, responses: Vec<String>) {
        self.topics.insert(topic.to_string(), responses);
    }
}

#[async_trait]
impl Behavior for DialogueBehavior {
    async fn matches_intent(&self, intent: &Intent) -> bool {
        // Match dialogue-related intents
        intent.intent_type == "question" || 
        intent.intent_type == "chat" || 
        intent.intent_type == "greeting"
    }
    
    async fn execute(&self, intent: &Intent, _context: &AgentContext) -> Result<BehaviorResult> {
        // Extract keywords from intent
        let mut matched_topic = None;
        let input_lower = intent.raw_input.to_lowercase();
        
        // Find matching topic
        for (topic, _) in &self.topics {
            if input_lower.contains(&topic.to_lowercase()) {
                matched_topic = Some(topic);
                break;
            }
        }
        
        // Generate response
        let response = if let Some(topic) = matched_topic {
            let responses = self.topics.get(topic).unwrap();
            let idx = rand::random::<usize>() % responses.len();
            responses[idx].clone()
        } else {
            // No matching topic, use default response
            let idx = rand::random::<usize>() % self.default_responses.len();
            self.default_responses[idx].clone()
        };
        
        Ok(BehaviorResult::Response(response))
    }
}

/// PathfindingBehavior that handles NPC movement
#[derive(Debug)]
pub struct PathfindingBehavior {
    /// Base behavior
    base: BaseBehavior,
    
    /// Whether the NPC should follow the player
    follow_player: bool,
    
    /// Maximum follow distance
    max_follow_distance: f32,
    
    /// Movement speed
    speed: f32,
}

impl PathfindingBehavior {
    /// Create a new pathfinding behavior
    ///
    /// # Arguments
    ///
    /// * `follow_player` - Whether to follow the player
    /// * `max_follow_distance` - Maximum distance to follow
    /// * `speed` - Movement speed
    ///
    /// # Returns
    ///
    /// A new PathfindingBehavior
    pub fn new(follow_player: bool, max_follow_distance: f32, speed: f32) -> Self {
        Self {
            base: BaseBehavior::new(
                "pathfinding",
                "Controls NPC movement and pathfinding",
                5,
                vec!["movement".to_string(), "follow".to_string()],
                0, // No cooldown for movement
            ),
            follow_player,
            max_follow_distance,
            speed,
        }
    }
    
    /// Create a behavior for following the player
    ///
    /// # Returns
    ///
    /// A new PathfindingBehavior configured to follow the player
    pub fn new_follow_player() -> Self {
        Self::new(true, 10.0, 1.5)
    }
    
    /// Create a behavior for staying in place
    ///
    /// # Returns
    ///
    /// A new PathfindingBehavior configured to stay in place
    pub fn new_stationary() -> Self {
        Self::new(false, 0.0, 0.0)
    }
}

#[async_trait]
impl Behavior for PathfindingBehavior {
    async fn matches_intent(&self, intent: &Intent) -> bool {
        // Only respond to movement/follow intents if configured to follow player
        if !self.follow_player {
            return false;
        }
        
        intent.intent_type == "movement" || 
        intent.intent_type == "follow" ||
        (intent.intent_type == "command" && intent.keywords.contains(&"follow".to_string()))
    }
    
    async fn execute(&self, intent: &Intent, context: &AgentContext) -> Result<BehaviorResult> {
        if !self.follow_player {
            return Ok(BehaviorResult::None);
        }
        
        // Extract player position from context
        let player_x = context.get("player_x").and_then(|v| v.as_f64()).unwrap_or(0.0) as f32;
        let player_y = context.get("player_y").and_then(|v| v.as_f64()).unwrap_or(0.0) as f32;
        
        // Check if we should start following
        if intent.intent_type == "command" && intent.keywords.contains(&"follow".to_string()) {
            // Send action to start following
            return Ok(BehaviorResult::Action(format!(
                "follow|{:.2}|{:.2}|{:.2}",
                player_x, player_y, self.speed
            )));
        }
        
        // Check distance to player
        let npc_x = context.get("npc_x").and_then(|v| v.as_f64()).unwrap_or(0.0) as f32;
        let npc_y = context.get("npc_y").and_then(|v| v.as_f64()).unwrap_or(0.0) as f32;
        
        let dx = player_x - npc_x;
        let dy = player_y - npc_y;
        let distance = (dx * dx + dy * dy).sqrt();
        
        if distance > self.max_follow_distance {
            // Too far, stop following
            return Ok(BehaviorResult::Action("stop_follow".to_string()));
        }
        
        // Move towards player
        Ok(BehaviorResult::Action(format!(
            "move_to|{:.2}|{:.2}|{:.2}",
            player_x, player_y, self.speed
        )))
    }
}

/// Factory function to create common behaviors
pub mod factory {
    use super::*;
    
    /// Create a standard greeting behavior
    pub fn create_greeting() -> GreetingBehavior {
        GreetingBehavior::new_default()
    }
    
    /// Create a dialogue behavior with standard topics
    pub fn create_dialogue() -> DialogueBehavior {
        let mut topics = HashMap::new();
        
        topics.insert(
            "quest".to_string(),
            vec![
                "I might have a task for you if you're interested.".to_string(),
                "Are you looking for work? I could use some help.".to_string(),
            ],
        );
        
        topics.insert(
            "shop".to_string(),
            vec![
                "I have various goods for sale. Take a look!".to_string(),
                "Everything is fairly priced, I assure you.".to_string(),
            ],
        );
        
        topics.insert(
            "weather".to_string(),
            vec![
                "Fine day today, isn't it?".to_string(),
                "I hope the weather holds up. It's been unpredictable lately.".to_string(),
            ],
        );
        
        let default_responses = vec![
            "Hmm, interesting.".to_string(),
            "I'm not sure I follow.".to_string(),
            "Let's talk about something else.".to_string(),
            "I don't know much about that.".to_string(),
        ];
        
        DialogueBehavior::new(topics, default_responses)
    }
    
    /// Create a pathfinding behavior for following the player
    pub fn create_follow() -> PathfindingBehavior {
        PathfindingBehavior::new_follow_player()
    }
    
    /// Create a stationary pathfinding behavior
    pub fn create_stationary() -> PathfindingBehavior {
        PathfindingBehavior::new_stationary()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[tokio::test]
    async fn test_greeting_behavior() {
        let intent = Intent {
            intent_type: "proximity".to_string(),
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
