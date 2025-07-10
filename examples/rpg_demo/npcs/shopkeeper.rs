//! Shopkeeper NPC for the RPG demo
//!
//! Implements a merchant character that buys and sells items, 
//! and provides information about goods.

use std::path::Path;

use oxyde::agent::{Agent, AgentBuilder};
use oxyde::config::AgentConfig;
use oxyde::oxyde_game::behavior::{DialogueBehavior, GreetingBehavior};
use oxyde::Result;

/// Create a shopkeeper agent
pub fn create_agent() -> Result<Agent> {
    println!("Creating shopkeeper agent...");
    
    // Load configuration
    let config_path = Path::new("examples/rpg_demo/assets/config.json");
    let full_config: serde_json::Value = serde_json::from_reader(
        std::fs::File::open(config_path)?
    )?;
    
    // Extract shopkeeper config
    let config_value = &full_config["shopkeeper"];
    let config: AgentConfig = serde_json::from_value(config_value.clone())?;
    
    // Create behaviors
    
    // Greeting behavior
    let greeting_behavior = GreetingBehavior::new(
        3.0, // Distance threshold
        vec![
            "Welcome to my shop, traveler!".to_string(),
            "Looking to trade? I've got the finest goods.".to_string(),
            "Ah, a customer! What can I interest you in today?".to_string(),
            "Need supplies for your journey? You've come to the right place.".to_string(),
            "Greetings! Care to see my wares?".to_string(),
        ]
    );
    
    // Trading dialogue behavior
    let mut trading_topics = std::collections::HashMap::new();
    
    trading_topics.insert(
        "buy".to_string(),
        vec![
            "I have many fine goods for sale. What interests you?".to_string(),
            "Here's what I have in stock today. Quality items, fair prices.".to_string(),
            "I can offer you weapons, potions, or general supplies. What do you need?".to_string(),
        ]
    );
    
    trading_topics.insert(
        "sell".to_string(),
        vec![
            "Let me see what you've got. I pay fair prices for quality items.".to_string(),
            "I'm always looking to expand my inventory. Show me what you're offering.".to_string(),
            "I might be interested, depending on the condition. Let's have a look.".to_string(),
        ]
    );
    
    trading_topics.insert(
        "price".to_string(),
        vec![
            "That would cost you 45 gold coins. A fair price, I assure you.".to_string(),
            "For you? 30 gold. And that's my best offer.".to_string(),
            "These don't come cheap. 75 gold, and worth every coin.".to_string(),
        ]
    );
    
    trading_topics.insert(
        "discount".to_string(),
        vec![
            "Hmm, I might be able to lower the price a bit. How about 10% off?".to_string(),
            "I run a business here, not a charity. But for you, 5% off.".to_string(),
            "These prices are already quite reasonable, but I could part with it for a few coins less.".to_string(),
        ]
    );
    
    let default_responses = vec![
        "I'm a merchant, not a conversationalist. Let's talk business.".to_string(),
        "Interesting, but I'm here to trade. Need any supplies?".to_string(),
        "I've traveled far and wide selling my wares. Now, what can I help you with?".to_string(),
        "Time is money, friend. Did you want to buy something?".to_string(),
    ];
    
    let dialogue_behavior = DialogueBehavior::new(trading_topics, default_responses);
    
    // Create agent with behaviors
    let agent = AgentBuilder::new()
        .with_config(config)
        .with_behavior(greeting_behavior)
        .with_behavior(dialogue_behavior)
        .build()?;
    
    Ok(agent)
}

/// Shopkeeper-specific bargaining behavior
pub struct BargainingBehavior {
    // Implementation details would go here
    // This is left as a stub for the demo
}