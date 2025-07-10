//! Standalone RPG Demo
//! 
//! This demo showcases a text-based RPG with NPCs that simulate the behavior
//! we'd expect from the full Oxyde SDK integration. It demonstrates:
//! 
//! 1. NPC agents with different roles and dialogue options
//! 2. Memory system for NPCs to recall interactions with the player
//! 3. Proximity-based greetings when player approaches NPCs
//! 4. Contextual dialogue responses based on player input
//! 5. Simple movement and interaction system
//!
//! This standalone version doesn't depend on the full Oxyde SDK, making it
//! easier to run and test the core concept without complex dependencies.

use std::collections::HashMap;
use std::io::{self, Write};
use std::thread;
use std::time::{Duration, Instant, SystemTime, UNIX_EPOCH};

/// Position in the 2D world
#[derive(Clone, Debug)]
struct Position {
    x: f32,
    y: f32,
}

impl Position {
    fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }
    
    fn distance_to(&self, other: &Position) -> f32 {
        let dx = self.x - other.x;
        let dy = self.y - other.y;
        (dx * dx + dy * dy).sqrt()
    }
}

/// Entity types in the game
#[derive(Debug, PartialEq)]
enum EntityType {
    Player,
    NPC,
}

/// Base entity implementation
#[derive(Debug)]
struct Entity {
    id: String,
    name: String,
    entity_type: EntityType,
    position: Position,
    properties: HashMap<String, String>,
}

impl Entity {
    fn new(id: &str, name: &str, entity_type: EntityType, x: f32, y: f32) -> Self {
        Self {
            id: id.to_string(),
            name: name.to_string(),
            entity_type,
            position: Position::new(x, y),
            properties: HashMap::new(),
        }
    }
    
    fn set_property(&mut self, key: &str, value: &str) {
        self.properties.insert(key.to_string(), value.to_string());
    }
    
    fn get_property(&self, key: &str) -> Option<&String> {
        self.properties.get(key)
    }
}

/// NPC Memory system for tracking interactions
#[derive(Debug)]
struct Memory {
    short_term: Vec<String>,
    long_term: HashMap<String, String>,
    last_interaction: Option<Instant>,
}

impl Memory {
    fn new() -> Self {
        Self {
            short_term: Vec::new(),
            long_term: HashMap::new(),
            last_interaction: None,
        }
    }
    
    fn add_short_term(&mut self, memory: &str) {
        self.short_term.push(memory.to_string());
        
        // Keep short-term memory limited
        if self.short_term.len() > 10 {
            self.short_term.remove(0);
        }
        
        self.last_interaction = Some(Instant::now());
    }
    
    fn set_long_term(&mut self, key: &str, value: &str) {
        self.long_term.insert(key.to_string(), value.to_string());
    }
    
    fn get_long_term(&self, key: &str) -> Option<&String> {
        self.long_term.get(key)
    }
}

/// NPC Agent with behaviors, memory, and interaction capability
#[derive(Debug)]
struct NPCAgent {
    entity: Entity,
    role: String,
    memory: Memory,
    dialogue_responses: Vec<String>,
    greeting_responses: Vec<String>,
    last_greeting: Option<Instant>,
}

impl NPCAgent {
    fn new(entity: Entity, role: &str) -> Self {
        Self {
            entity,
            role: role.to_string(),
            memory: Memory::new(),
            dialogue_responses: Vec::new(),
            greeting_responses: Vec::new(),
            last_greeting: None,
        }
    }
    
    fn add_dialogue_response(&mut self, response: &str) {
        self.dialogue_responses.push(response.to_string());
    }
    
    fn add_greeting(&mut self, greeting: &str) {
        self.greeting_responses.push(greeting.to_string());
    }
    
    fn process_player_proximity(&mut self, player_pos: &Position) -> Option<String> {
        let distance = self.entity.position.distance_to(player_pos);
        
        // Only greet if player is close and we haven't greeted recently
        if distance < 3.0 {
            let greeting_cooldown = Duration::from_secs(60);
            
            if self.last_greeting.is_none() || 
               self.last_greeting.unwrap().elapsed() > greeting_cooldown {
                
                // Select random greeting
                if !self.greeting_responses.is_empty() {
                    // Use current time to select a greeting
                    let now = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
                    let idx = (now.as_millis() % self.greeting_responses.len() as u128) as usize;
                    let greeting = &self.greeting_responses[idx];
                    
                    // Record in memory
                    self.memory.add_short_term(&format!("I greeted player at distance {:.1}", distance));
                    self.last_greeting = Some(Instant::now());
                    
                    return Some(greeting.clone());
                }
            }
        }
        
        None
    }
    
    fn process_dialogue(&mut self, text: &str) -> String {
        // Record in memory
        self.memory.add_short_term(&format!("Player said: {}", text));
        
        // Process certain keywords if found
        let text_lower = text.to_lowercase();
        
        if text_lower.contains("name") {
            return format!("My name is {}. I am a {}.", self.entity.name, self.role);
        }
        
        if text_lower.contains("help") {
            return "You can talk to me about various topics. Try asking about my role or the village.".to_string();
        }
        
        // Default to random response using timestamp
        if !self.dialogue_responses.is_empty() {
            let now = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
            let idx = (now.as_millis() % self.dialogue_responses.len() as u128) as usize;
            return self.dialogue_responses[idx].clone();
        }
        
        "Hmm, interesting.".to_string()
    }
}

/// Game world that manages entities and interactions
struct GameWorld {
    player: Entity,
    npcs: Vec<NPCAgent>,
    messages: Vec<String>,
    game_time: f32,
    last_update: Instant,
}

impl GameWorld {
    fn new() -> Self {
        println!("Initializing RPG Demo world...");
        
        // Create player
        let player = Entity::new("player", "Player", EntityType::Player, 0.0, 0.0);
        
        // Create NPCs
        let mut npcs = Vec::new();
        
        // Merchant NPC
        let mut merchant = Entity::new("merchant", "Marcus", EntityType::NPC, 5.0, 3.0);
        merchant.set_property("occupation", "Shopkeeper");
        
        let mut merchant_agent = NPCAgent::new(merchant, "Merchant");
        merchant_agent.add_greeting("Welcome to my shop, traveler!");
        merchant_agent.add_greeting("Ah, a customer! Looking to buy or sell?");
        merchant_agent.add_dialogue_response("I have many fine wares. Take a look around.");
        merchant_agent.add_dialogue_response("That'll be 50 gold coins. A fair price, I assure you.");
        merchant_agent.add_dialogue_response("I import goods from all over the realm.");
        merchant_agent.add_dialogue_response("Business has been good lately, despite the troubles to the north.");
        merchant_agent.add_dialogue_response("I might be able to offer a discount for a fellow traveler.");
        npcs.push(merchant_agent);
        
        // Guard NPC
        let mut guard = Entity::new("guard", "Gareth", EntityType::NPC, -5.0, -2.0);
        guard.set_property("occupation", "Village Guard");
        
        let mut guard_agent = NPCAgent::new(guard, "Guard");
        guard_agent.add_greeting("Halt! State your business, traveler.");
        guard_agent.add_greeting("Keep your weapons sheathed while in town.");
        guard_agent.add_dialogue_response("I'm watching you. Don't cause any trouble in my village.");
        guard_agent.add_dialogue_response("There have been reports of bandits on the north road.");
        guard_agent.add_dialogue_response("I've been a guard here for fifteen years.");
        guard_agent.add_dialogue_response("Our laws are simple: no stealing, no fighting, respect others.");
        guard_agent.add_dialogue_response("The captain increased our patrols after the recent dragon sighting.");
        npcs.push(guard_agent);
        
        // Villager NPC
        let mut villager = Entity::new("villager", "Velma", EntityType::NPC, 2.0, -4.0);
        villager.set_property("occupation", "Farmhand");
        
        let mut villager_agent = NPCAgent::new(villager, "Villager");
        villager_agent.add_greeting("Hello there! Nice day, isn't it?");
        villager_agent.add_greeting("Oh! I didn't see you there.");
        villager_agent.add_dialogue_response("Did you hear about the old ruins to the east? They say it's haunted.");
        villager_agent.add_dialogue_response("The harvest festival is next week. You should stay for it!");
        villager_agent.add_dialogue_response("I've lived in this village my whole life.");
        villager_agent.add_dialogue_response("The blacksmith's daughter is sweet on the baker's son. Quite the scandal!");
        villager_agent.add_dialogue_response("We've had strange weather lately. Some say it's magic from the mountains.");
        npcs.push(villager_agent);
        
        println!("Created {} NPCs", npcs.len());
        
        Self {
            player,
            npcs,
            messages: Vec::new(),
            game_time: 0.0,
            last_update: Instant::now(),
        }
    }
    
    fn update(&mut self) {
        // Calculate delta time
        let now = Instant::now();
        let delta_time = now.duration_since(self.last_update).as_secs_f32();
        self.last_update = now;
        
        // Update game time
        self.game_time += delta_time;
        
        // Check for NPC proximity greetings
        // First collect any greetings
        let mut greetings = Vec::new();
        
        for npc in &mut self.npcs {
            if let Some(greeting) = npc.process_player_proximity(&self.player.position) {
                greetings.push((npc.entity.name.clone(), greeting));
            }
        }
        
        // Then add messages outside the loop
        for (name, greeting) in greetings {
            self.add_message(&format!("{}: {}", name, greeting));
        }
    }
    
    fn move_player(&mut self, dx: f32, dy: f32) {
        self.player.position.x += dx;
        self.player.position.y += dy;
        
        println!("Player moved to ({:.1}, {:.1})", self.player.position.x, self.player.position.y);
    }
    
    fn process_chat(&mut self, text: &str) {
        // Add player message
        self.add_message(&format!("Player: {}", text));
        
        // Find closest NPC
        let mut closest_npc_index = 0;
        let mut closest_distance = f32::MAX;
        
        for (i, npc) in self.npcs.iter().enumerate() {
            let distance = npc.entity.position.distance_to(&self.player.position);
            
            if distance < closest_distance {
                closest_distance = distance;
                closest_npc_index = i;
            }
        }
        
        // Only process chat if an NPC is nearby
        if closest_distance <= 5.0 {
            // First get the response and name
            let response;
            let name;
            
            {
                let npc = &mut self.npcs[closest_npc_index];
                response = npc.process_dialogue(text);
                name = npc.entity.name.clone();
            }
            
            // Then add the message
            self.add_message(&format!("{}: {}", name, response));
        } else {
            self.add_message("No one is close enough to hear you...");
        }
    }
    
    fn add_message(&mut self, message: &str) {
        println!("{}", message);
        self.messages.push(message.to_string());
        
        // Limit message history
        if self.messages.len() > 100 {
            self.messages.remove(0);
        }
    }
    
    fn render(&self) {
        // Clear screen
        print!("\x1B[2J\x1B[1;1H");
        
        // Print game title
        println!("=== Oxyde RPG Demo ===");
        println!("Game time: {:.1}s", self.game_time);
        println!();
        
        // Print player position
        println!("Player position: ({:.1}, {:.1})", self.player.position.x, self.player.position.y);
        
        // Print NPCs
        println!("\nNPCs:");
        for npc in &self.npcs {
            let distance = npc.entity.position.distance_to(&self.player.position);
            
            println!("- {} ({}) at ({:.1}, {:.1}) - {:.1} units away", 
                npc.entity.name, npc.role, npc.entity.position.x, npc.entity.position.y, distance);
        }
        
        // Print recent messages
        println!("\nRecent messages:");
        let start_idx = if self.messages.len() > 5 { self.messages.len() - 5 } else { 0 };
        for i in start_idx..self.messages.len() {
            println!("  {}", self.messages[i]);
        }
        
        // Print controls
        println!("\nControls:");
        println!("WASD: Move player");
        println!("T: Talk to nearest NPC");
        println!("Q: Quit");
        println!("\nPress Enter after typing a command.");
    }
}

fn main() {
    // Create game world
    let mut world = GameWorld::new();
    
    // Game loop
    loop {
        // Update game state
        world.update();
        
        // Render game state
        world.render();
        
        // Handle input
        let mut input = String::new();
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut input).unwrap_or(0);
        let input = input.trim().to_lowercase();
        
        if !input.is_empty() {
            match input.as_str() {
                "w" => world.move_player(0.0, 1.0),
                "a" => world.move_player(-1.0, 0.0),
                "s" => world.move_player(0.0, -1.0),
                "d" => world.move_player(1.0, 0.0),
                "q" => break,
                "t" => {
                    // Talk to nearest NPC
                    println!("Enter your message:");
                    let mut chat = String::new();
                    io::stdout().flush().unwrap();
                    io::stdin().read_line(&mut chat).unwrap_or(0);
                    world.process_chat(chat.trim());
                },
                _ => {
                    println!("Unknown command: {}", input);
                }
            }
        }
        
        // Short delay to prevent CPU hogging
        thread::sleep(Duration::from_millis(50));
    }
    
    println!("Thanks for playing the Oxyde RPG Demo!");
}