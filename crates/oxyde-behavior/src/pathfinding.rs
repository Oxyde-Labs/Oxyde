//! Pathfinding behavior for NPC navigation

use async_trait::async_trait;

use oxyde_core::AgentContext;
use oxyde_intent::{Intent, IntentType};
use oxyde_core::Result;

use super::base::{Behavior, BehaviorResult, BaseBehavior};

/// Pathfinding behavior that controls NPC movement
#[derive(Debug)]
pub struct PathfindingBehavior {
    /// Base behavior
    #[allow(dead_code)]
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

        intent.intent_type == IntentType::Custom || // movement/follow are custom types
        (intent.intent_type == IntentType::Command && intent.keywords.contains(&"follow".to_string()))
    }

    async fn execute(&self, _intent: &Intent, context: &AgentContext) -> Result<BehaviorResult> {
        if !self.follow_player {
            return Ok(BehaviorResult::None);
        }

        // Extract player position from context
        let player_x = context.get("player_x").and_then(|v| v.as_f64()).unwrap_or(0.0) as f32;
        let player_y = context.get("player_y").and_then(|v| v.as_f64()).unwrap_or(0.0) as f32;

        // Check if we should start following
        if _intent.intent_type == IntentType::Command && _intent.keywords.contains(&"follow".to_string()) {
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
