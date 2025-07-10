//! Standalone RPG demo
//! 
//! This is a simple demonstration of the Oxyde SDK concepts
//! without requiring all the complex dependencies.

use std::collections::HashMap;
use std::io::{self, Write};
use std::thread;
use std::time::{Duration, Instant};

use rand::Rng;

// Core concepts from the Oxyde SDK
struct Agent {
    id: String,
    name: String,
    role: String,
    memory: Vec<String>,
    context: HashMap<String, String>,
}

impl Agent {
    fn new(id: &str, name: &str, role: &str) -> Self {
        Self {
            id: id.to_string(),
            name: name.to_string(),
            role: role.to_string(),
            memory: Vec::new(),
            context: HashMap::new(),
        }
    }
    
    fn process_input(&mut self, input: &str, responses: &[String]) -> String {
        // Record in memory
        self.memory.push(format!("Player said: {}", input));
        if self.memory.len() > 10 {
            self.memory.remove(0);
        }
        
        // For demo, just select a random response
        let mut rng = rand::thread_rng();
        let idx = rng.gen_range(0..responses.len());
        responses[idx].clone()
    }
}

fn main() {
    println!("=== Oxyde SDK Demo ===");
    println!("Welcome to the simple Oxyde RPG demo");
    println!("This demonstrates the core concepts of the Oxyde SDK");
    println!();
    
    // Create some agents
    let mut merchant = Agent::new("merchant", "Marcus", "Shopkeeper");
    let mut guard = Agent::new("guard", "Gareth", "Village Guard");
    let mut villager = Agent::new("villager", "Velma", "Local Resident");
    
    // Create response templates
    let merchant_responses = vec![
        "Welcome to my shop! I have the finest goods in the village.".to_string(),
        "That will cost you 50 gold coins. A fair price, I assure you.".to_string(),
        "I might be able to lower the price a bit. How about 45 gold?".to_string(),
        "I'm afraid I don't have that item in stock currently.".to_string(),
        "I've traveled to many lands in search of rare goods to sell here.".to_string(),
    ];
    
    let guard_responses = vec![
        "Keep to the roads, traveler. The woods aren't safe.".to_string(),
        "I'm watching you. Don't cause any trouble in my village.".to_string(),
        "There have been reports of bandits on the north road.".to_string(),
        "I've been a guard here for fifteen years.".to_string(),
        "Our laws are simple: no stealing, no fighting, respect others.".to_string(),
    ];
    
    let villager_responses = vec![
        "Nice day today, isn't it?".to_string(),
        "Did you hear about the old ruins to the east? They say it's haunted.".to_string(),
        "The harvest festival is next week. You should stay for it!".to_string(),
        "I've lived in this village my whole life.".to_string(),
        "The blacksmith's daughter is sweet on the baker's son. Quite the scandal!".to_string(),
    ];
    
    // Main interaction loop
    loop {
        println!("\nWho would you like to talk to?");
        println!("1. Marcus (Merchant)");
        println!("2. Gareth (Guard)");
        println!("3. Velma (Villager)");
        println!("4. Exit demo");
        
        print!("\nYour choice: ");
        io::stdout().flush().unwrap();
        
        let mut choice = String::new();
        io::stdin().read_line(&mut choice).unwrap();
        let choice = choice.trim();
        
        match choice {
            "1" => interact_with_agent(&mut merchant, &merchant_responses),
            "2" => interact_with_agent(&mut guard, &guard_responses),
            "3" => interact_with_agent(&mut villager, &villager_responses),
            "4" | "exit" | "quit" => break,
            _ => println!("Invalid choice. Please select 1-4."),
        }
    }
    
    println!("\nThank you for trying the Oxyde SDK demo!");
}

fn interact_with_agent(agent: &mut Agent, responses: &[String]) {
    println!("\nYou approach {}. What would you like to say?", agent.name);
    println!("(Type 'leave' to end the conversation)");
    
    loop {
        print!("> ");
        io::stdout().flush().unwrap();
        
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let input = input.trim();
        
        if input.to_lowercase() == "leave" {
            println!("You walk away from {}.", agent.name);
            break;
        }
        
        // Process input through the agent
        let response = agent.process_input(input, responses);
        println!("{}: {}", agent.name, response);
    }
}