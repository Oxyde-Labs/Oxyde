//! Game-specific agent utilities for the Oxyde SDK
//!
//! This module provides game-specific functionality for integrating Oxyde agents
//! into games, including behaviors, intent understanding, and engine bindings.

pub mod behavior;
pub mod emotion;
pub mod intent;
pub mod bindings;

/// Game-specific utilities and extensions
pub mod utils {
    use std::collections::HashMap;
    use std::time::Duration;

    use serde::{Deserialize, Serialize};
    use tokio::time::sleep;

    use crate::agent::Agent;
    use crate::Result;

    /// Game entity type
    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub enum EntityType {
        /// Player entity
        Player,
        /// NPC entity
        NPC,
        /// Item entity
        Item,
        /// Structure or building
        Structure,
        /// Trigger zone
        Trigger,
    }

    /// Position in 2D or 3D space
    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct Position {
        /// X coordinate
        pub x: f32,
        /// Y coordinate
        pub y: f32,
        /// Z coordinate (optional for 2D)
        pub z: Option<f32>,
    }

    /// Game entity
    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct Entity {
        /// Entity ID
        pub id: String,
        /// Entity type
        pub entity_type: EntityType,
        /// Entity name
        pub name: String,
        /// Entity position
        pub position: Position,
        /// Entity properties
        pub properties: HashMap<String, serde_json::Value>,
    }

    /// Get distance between two positions
    ///
    /// # Arguments
    ///
    /// * `a` - First position
    /// * `b` - Second position
    ///
    /// # Returns
    ///
    /// Distance between positions
    pub fn distance(a: &Position, b: &Position) -> f32 {
        let dx = a.x - b.x;
        let dy = a.y - b.y;
        
        let dz = match (a.z, b.z) {
            (Some(az), Some(bz)) => az - bz,
            _ => 0.0,
        };
        
        (dx*dx + dy*dy + dz*dz).sqrt()
    }

    /// Move an entity towards a target position
    ///
    /// # Arguments
    ///
    /// * `entity` - Entity to move
    /// * `target` - Target position
    /// * `speed` - Movement speed
    /// * `delta_time` - Time elapsed since last update
    ///
    /// # Returns
    ///
    /// Updated entity position
    pub fn move_towards(entity: &mut Entity, target: &Position, speed: f32, delta_time: f32) -> Position {
        let dist = distance(&entity.position, target);
        
        if dist < 0.001 {
            return entity.position.clone();
        }
        
        let scale = (speed * delta_time) / dist;
        let dx = (target.x - entity.position.x) * scale;
        let dy = (target.y - entity.position.y) * scale;
        
        let dz = match (entity.position.z, target.z) {
            (Some(ez), Some(tz)) => Some((tz - ez) * scale),
            _ => None,
        };
        
        let new_pos = Position {
            x: entity.position.x + dx,
            y: entity.position.y + dy,
            z: dz.map(|dz| entity.position.z.unwrap() + dz),
        };
        
        entity.position = new_pos.clone();
        new_pos
    }

    /// Run an agent in a game loop
    ///
    /// # Arguments
    ///
    /// * `agent` - Agent to run
    /// * `update_fn` - Function to update the agent context
    /// * `fps` - Frames per second
    ///
    /// # Returns
    ///
    /// Result of running the agent
    pub async fn run_agent_loop<F>(
        agent: &Agent,
        mut update_fn: F,
        fps: u32,
    ) -> Result<()>
    where
        F: FnMut(&Agent) -> Result<()>,
    {
        let frame_time = Duration::from_secs_f32(1.0 / fps as f32);
        
        // Start the agent
        agent.start().await?;
        
        // Run the game loop
        loop {
            let start = std::time::Instant::now();
            
            // Update agent context
            update_fn(agent)?;
            
            // Wait for the remainder of the frame time
            let elapsed = start.elapsed();
            if elapsed < frame_time {
                sleep(frame_time - elapsed).await;
            }
        }
    }
}
