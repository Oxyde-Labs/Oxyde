//! Utility types and functions for game integration
//!
//! This module provides utility types and functions for integrating Oxyde with games.

use std::collections::HashMap;
use serde::{Deserialize, Serialize};

/// Position in 2D or 3D space
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Position {
    /// X coordinate
    pub x: f32,
    
    /// Y coordinate
    pub y: f32,
    
    /// Optional Z coordinate
    pub z: Option<f32>,
}

impl Position {
    /// Create a new 2D position
    ///
    /// # Arguments
    ///
    /// * `x` - X coordinate
    /// * `y` - Y coordinate
    ///
    /// # Returns
    ///
    /// A new Position instance
    pub fn new_2d(x: f32, y: f32) -> Self {
        Self { x, y, z: None }
    }
    
    /// Create a new 3D position
    ///
    /// # Arguments
    ///
    /// * `x` - X coordinate
    /// * `y` - Y coordinate
    /// * `z` - Z coordinate
    ///
    /// # Returns
    ///
    /// A new Position instance
    pub fn new_3d(x: f32, y: f32, z: f32) -> Self {
        Self { x, y, z: Some(z) }
    }
    
    /// Calculate distance to another position (2D only)
    ///
    /// # Arguments
    ///
    /// * `other` - Other position
    ///
    /// # Returns
    ///
    /// Euclidean distance
    pub fn distance_2d(&self, other: &Position) -> f32 {
        let dx = self.x - other.x;
        let dy = self.y - other.y;
        (dx * dx + dy * dy).sqrt()
    }
    
    /// Calculate distance to another position (3D if available)
    ///
    /// # Arguments
    ///
    /// * `other` - Other position
    ///
    /// # Returns
    ///
    /// Euclidean distance (2D if z coordinates are not available)
    pub fn distance(&self, other: &Position) -> f32 {
        let dx = self.x - other.x;
        let dy = self.y - other.y;
        
        if let (Some(z1), Some(z2)) = (self.z, other.z) {
            let dz = z1 - z2;
            (dx * dx + dy * dy + dz * dz).sqrt()
        } else {
            (dx * dx + dy * dy).sqrt()
        }
    }
}

/// Entity types
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum EntityType {
    /// Player entity
    Player,
    
    /// NPC entity
    NPC,
    
    /// Object entity
    Object,
    
    /// Environment entity
    Environment,
}

/// Game entity
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Entity {
    /// Unique entity ID
    pub id: String,
    
    /// Entity type
    pub entity_type: EntityType,
    
    /// Entity name
    pub name: String,
    
    /// Entity position
    pub position: Position,
    
    /// Additional properties
    pub properties: HashMap<String, serde_json::Value>,
}

impl Entity {
    /// Create a new entity
    ///
    /// # Arguments
    ///
    /// * `id` - Entity ID
    /// * `entity_type` - Entity type
    /// * `name` - Entity name
    /// * `position` - Entity position
    ///
    /// # Returns
    ///
    /// A new Entity instance
    pub fn new(
        id: &str,
        entity_type: EntityType,
        name: &str,
        position: Position,
    ) -> Self {
        Self {
            id: id.to_string(),
            entity_type,
            name: name.to_string(),
            position,
            properties: HashMap::new(),
        }
    }
    
    /// Set a property value
    ///
    /// # Arguments
    ///
    /// * `key` - Property key
    /// * `value` - Property value
    pub fn set_property<T: Serialize>(&mut self, key: &str, value: T) -> Result<(), serde_json::Error> {
        let json = serde_json::to_value(value)?;
        self.properties.insert(key.to_string(), json);
        Ok(())
    }
    
    /// Get a property value
    ///
    /// # Arguments
    ///
    /// * `key` - Property key
    ///
    /// # Returns
    ///
    /// Property value or None if not found
    pub fn get_property<T: for<'de> Deserialize<'de>>(&self, key: &str) -> Result<Option<T>, serde_json::Error> {
        if let Some(value) = self.properties.get(key) {
            let typed_value = serde_json::from_value(value.clone())?;
            Ok(Some(typed_value))
        } else {
            Ok(None)
        }
    }
    
    /// Calculate distance to another entity
    ///
    /// # Arguments
    ///
    /// * `other` - Other entity
    ///
    /// # Returns
    ///
    /// Euclidean distance between entities
    pub fn distance_to(&self, other: &Entity) -> f32 {
        self.position.distance(&other.position)
    }
}

/// Helper function to create a player entity
///
/// # Arguments
///
/// * `id` - Player ID
/// * `name` - Player name
/// * `position` - Player position
///
/// # Returns
///
/// A new player Entity
pub fn create_player(id: &str, name: &str, position: Position) -> Entity {
    Entity::new(id, EntityType::Player, name, position)
}

/// Helper function to create an NPC entity
///
/// # Arguments
///
/// * `id` - NPC ID
/// * `name` - NPC name
/// * `position` - NPC position
///
/// # Returns
///
/// A new NPC Entity
pub fn create_npc(id: &str, name: &str, position: Position) -> Entity {
    Entity::new(id, EntityType::NPC, name, position)
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_position_distance() {
        let pos1 = Position::new_2d(0.0, 0.0);
        let pos2 = Position::new_2d(3.0, 4.0);
        
        assert_eq!(pos1.distance_2d(&pos2), 5.0);
    }
    
    #[test]
    fn test_entity_properties() {
        let mut entity = Entity::new("test", EntityType::NPC, "Test Entity", Position::new_2d(0.0, 0.0));
        
        entity.set_property("health", 100).unwrap();
        entity.set_property("name", "Test").unwrap();
        
        let health: Option<i32> = entity.get_property("health").unwrap();
        let name: Option<String> = entity.get_property("name").unwrap();
        
        assert_eq!(health, Some(100));
        assert_eq!(name, Some("Test".to_string()));
    }
}