//! Guard NPC for the RPG demo
//!
//! Implements a guard character that patrols the village and responds
//! to security concerns.

use std::path::Path;

use oxyde::agent::{Agent, AgentBuilder};
use oxyde::config::AgentConfig;
use oxyde::oxyde_game::behavior::{DialogueBehavior, GreetingBehavior, PathfindingBehavior};
use oxyde::Result;

/// Create a guard agent
pub fn create_agent() -> Result<Agent> {
    println!("Creating guard agent...");
    
    // Load configuration
    let config_path = Path::new("examples/rpg_demo/assets/config.json");
    let full_config: serde_json::Value = serde_json::from_reader(
        std::fs::File::open(config_path)?
    )?;
    
    // Extract guard config
    let config_value = &full_config["guard"];
    let config: AgentConfig = serde_json::from_value(config_value.clone())?;
    
    // Create behaviors
    
    // Greeting behavior - more stern and authoritative than other NPCs
    let greeting_behavior = GreetingBehavior::new(
        4.0, // Slightly larger detection range
        vec![
            "Halt! State your business in our village.".to_string(),
            "Keep your weapons sheathed, traveler. We don't want trouble.".to_string(),
            "I'm watching you, stranger. Behave yourself here.".to_string(),
            "Welcome to our village. Follow our rules and we'll get along fine.".to_string(),
            "Guard duty again... Oh! Didn't see you there. Keep to the roads.".to_string(),
        ]
    );
    
    // Security dialogue behavior
    let mut guard_topics = std::collections::HashMap::new();
    
    guard_topics.insert(
        "danger".to_string(),
        vec![
            "There have been reports of bandits on the north road. Travel in groups.".to_string(),
            "The forest to the east is not safe after dark. Strange creatures roam there.".to_string(),
            "Our patrols keep the village safe, but venture outside at your own risk.".to_string(),
        ]
    );
    
    guard_topics.insert(
        "law".to_string(),
        vec![
            "Our laws are simple: no stealing, no fighting, respect others. Follow them.".to_string(),
            "The village elder makes the laws, I just enforce them.".to_string(),
            "Breaking our laws means a night in the stockade, or worse.".to_string(),
        ]
    );
    
    guard_topics.insert(
        "weapon".to_string(),
        vec![
            "That's a decent blade, but you'd better not use it here.".to_string(),
            "I've seen better armor on a training dummy. No offense.".to_string(),
            "Keep that weapon sheathed in the village, or we'll have problems.".to_string(),
        ]
    );
    
    let default_responses = vec![
        "I'm on duty. Make it quick.".to_string(),
        "Keep moving. I have to patrol the entire village.".to_string(),
        "I'm not paid enough to answer random questions.".to_string(),
        "Is this relevant to village security? No? Then I'm busy.".to_string(),
    ];
    
    let dialogue_behavior = DialogueBehavior::new(guard_topics, default_responses);
    
    // Patrol behavior
    let patrol_behavior = PathfindingBehavior::new(
        false, // Don't follow player
        0.0,   // No follow distance
        1.2    // Movement speed
    );
    
    // Create agent with behaviors
    let agent = AgentBuilder::new()
        .with_config(config)
        .with_behavior(greeting_behavior)
        .with_behavior(dialogue_behavior)
        .with_behavior(patrol_behavior)
        .build()?;
    
    Ok(agent)
}

/// Guard-specific patrol behavior
pub struct PatrolBehavior {
    // Implementation details would go here
    // This is left as a stub for the demo
}