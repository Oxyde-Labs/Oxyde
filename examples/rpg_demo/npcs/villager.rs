//! Villager NPC for the RPG demo
//!
//! Implements a friendly local resident who knows about the village
//! history and shares gossip with the player.

use std::path::Path;

use oxyde::agent::{Agent, AgentBuilder};
use oxyde::config::AgentConfig;
use oxyde::oxyde_game::behavior::{DialogueBehavior, GreetingBehavior, PathfindingBehavior};
use oxyde::Result;

/// Create a villager agent
pub fn create_agent() -> Result<Agent> {
    println!("Creating villager agent...");
    
    // Load configuration
    let config_path = Path::new("examples/rpg_demo/assets/config.json");
    let full_config: serde_json::Value = serde_json::from_reader(
        std::fs::File::open(config_path)?
    )?;
    
    // Extract villager config
    let config_value = &full_config["villager"];
    let config: AgentConfig = serde_json::from_value(config_value.clone())?;
    
    // Create behaviors
    
    // Friendly greeting behavior
    let greeting_behavior = GreetingBehavior::new(
        2.5, // Distance threshold
        vec![
            "Hello there! Nice day, isn't it?".to_string(),
            "Oh! A visitor! We don't get many strangers here.".to_string(),
            "Good day to you! Welcome to our little village.".to_string(),
            "Well met, traveler! How are you finding our humble village?".to_string(),
            "Hello! I haven't seen you around before. Just passing through?".to_string(),
        ]
    );
    
    // Village gossip and information dialogue behavior
    let mut villager_topics = std::collections::HashMap::new();
    
    villager_topics.insert(
        "village".to_string(),
        vec![
            "Our village has stood for over 200 years. Started as a trading post, it did.".to_string(),
            "Not much happens here, but that's how we like it. Peaceful and quiet.".to_string(),
            "We're known for our wheat fields and the old mill by the river. Best flour in the region!".to_string(),
        ]
    );
    
    villager_topics.insert(
        "gossip".to_string(),
        vec![
            "Did you hear? The blacksmith's daughter is sweet on the baker's son. Quite the scandal!".to_string(),
            "They say the old ruins to the north are haunted. Lights seen at night, strange noises...".to_string(),
            "The merchant raised his prices again. Says it's due to bandit attacks on the trade routes.".to_string(),
        ]
    );
    
    villager_topics.insert(
        "history".to_string(),
        vec![
            "The great flood of '42 nearly wiped us out. We rebuilt everything from scratch.".to_string(),
            "This land used to belong to the old kingdom. There's a ruined castle about a day's journey east.".to_string(),
            "My grandfather says a dragon used to live in the mountains. Nonsense, if you ask me!".to_string(),
        ]
    );
    
    let default_responses = vec![
        "I've lived here all my life. Never felt the need to leave.".to_string(),
        "My crops need tending, but I always have time for a chat.".to_string(),
        "Have you tried the tavern's ale? Best in three counties!".to_string(),
        "Weather's been good for the crops this season. We're expecting a bountiful harvest.".to_string(),
    ];
    
    let dialogue_behavior = DialogueBehavior::new(villager_topics, default_responses);
    
    // Daily routine movement behavior
    let routine_behavior = PathfindingBehavior::new(
        false, // Don't follow player
        0.0,   // No follow distance
        0.8    // Slower movement speed - just going about daily business
    );
    
    // Create agent with behaviors
    let agent = AgentBuilder::new()
        .with_config(config)
        .with_behavior(greeting_behavior)
        .with_behavior(dialogue_behavior)
        .with_behavior(routine_behavior)
        .build()?;
    
    Ok(agent)
}

/// Villager-specific daily routine behavior
pub struct DailyRoutineBehavior {
    // Implementation details would go here
    // This is left as a stub for the demo
}