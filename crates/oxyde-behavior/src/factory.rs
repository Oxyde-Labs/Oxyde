//! Factory functions to create common behaviors

use std::collections::HashMap;

use super::{DialogueBehavior, GreetingBehavior, PathfindingBehavior};

/// Create a standard greeting behavior
///
/// # Returns
///
/// A new GreetingBehavior with standard greetings
pub fn create_greeting() -> GreetingBehavior {
    GreetingBehavior::new_default()
}

/// Create a simple dialogue behavior
///
/// # Arguments
///
/// * `topics` - Map of topics to responses
///
/// # Returns
///
/// A new DialogueBehavior
pub fn create_dialogue(topics: HashMap<String, Vec<String>>) -> DialogueBehavior {
    let default_responses = vec![
        "I'm not sure what you mean.".to_string(),
        "Could you rephrase that?".to_string(),
        "I don't understand.".to_string(),
    ];

    DialogueBehavior::new(topics, default_responses)
}

/// Create a pathfinding behavior for following the player
///
/// # Returns
///
/// A new PathfindingBehavior configured to follow the player
pub fn create_follow() -> PathfindingBehavior {
    PathfindingBehavior::new_follow_player()
}

/// Create a stationary pathfinding behavior
///
/// # Returns
///
/// A new PathfindingBehavior configured to stay in place
pub fn create_stationary() -> PathfindingBehavior {
    PathfindingBehavior::new_stationary()
}
