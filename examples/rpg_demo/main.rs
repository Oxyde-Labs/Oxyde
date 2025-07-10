//! RPG Demo for Oxyde SDK
//! 
//! A simple demo showing basic navigation and interaction with NPCs

use std::collections::HashMap;
use std::io::{self, Write};
use std::thread;
use std::time::{Duration, Instant};

/// Position in a 2D world
struct Position {
    x: f32,
    y: f32,
}

/// Entity types
enum EntityType {
    Player,
    NPC,
}

/// Entity in the game world
struct Entity {
    id: String,
    name: String,
    entity_type: EntityType,
    position: Position,
}

/// NPC character
struct NPC {
    entity: Entity,
    dialogue: Vec<String>,
}

/// Game state
struct GameState {
    player: Entity,
    npcs: Vec<NPC>,
    messages: Vec<String>,
    time: f32,
    last_update: Instant,
}

impl GameState {
    /// Create a new game state
    fn new() -> Self {
        println!("Initializing Oxyde RPG Demo...");
        
        // Create player
        let player = Entity {
            id: "player".to_string(),
            name: "Player".to_string(),
            entity_type: EntityType::Player,
            position: Position { x: 0.0, y: 0.0 },
        };
        
        // Create NPCs
        let mut npcs = Vec::new();
        
        // Shopkeeper
        npcs.push(NPC {
            entity: Entity {
                id: "shopkeeper".to_string(),
                name: "Merchant".to_string(),
                entity_type: EntityType::NPC,
                position: Position { x: 5.0, y: 3.0 },
            },
            dialogue: vec![
                "Welcome to my shop, traveler!".to_string(),
                "I have many fine goods for sale. What interests you?".to_string(),
                "Let me see what you've got. I pay fair prices for quality items.".to_string(),
                "For you? 30 gold. And that's my best offer.".to_string(),
                "I'm a merchant, not a conversationalist. Let's talk business.".to_string(),
            ],
        });
        
        // Guard
        npcs.push(NPC {
            entity: Entity {
                id: "guard".to_string(),
                name: "Guard".to_string(),
                entity_type: EntityType::NPC,
                position: Position { x: -5.0, y: -2.0 },
            },
            dialogue: vec![
                "Halt! State your business in our village.".to_string(),
                "Keep your weapons sheathed, traveler. We don't want trouble.".to_string(),
                "The forest to the east is not safe after dark. Strange creatures roam there.".to_string(),
                "Our laws are simple: no stealing, no fighting, respect others. Follow them.".to_string(),
                "I'm on duty. Make it quick.".to_string(),
            ],
        });
        
        // Villager
        npcs.push(NPC {
            entity: Entity {
                id: "villager".to_string(),
                name: "Villager".to_string(),
                entity_type: EntityType::NPC,
                position: Position { x: 2.0, y: -4.0 },
            },
            dialogue: vec![
                "Hello there! Nice day, isn't it?".to_string(),
                "Our village has stood for over 200 years. Started as a trading post, it did.".to_string(),
                "Did you hear? The blacksmith's daughter is sweet on the baker's son. Quite the scandal!".to_string(),
                "The great flood of '42 nearly wiped us out. We rebuilt everything from scratch.".to_string(),
                "I've lived here all my life. Never felt the need to leave.".to_string(),
            ],
        });
        
        println!("Created {} NPCs", npcs.len());
        
        GameState {
            player,
            npcs,
            messages: Vec::new(),
            time: 0.0,
            last_update: Instant::now(),
        }
    }
    
    /// Move the player
    fn move_player(&mut self, dx: f32, dy: f32) {
        self.player.position.x += dx;
        self.player.position.y += dy;
        
        println!("Player moved to ({:.1}, {:.1})", self.player.position.x, self.player.position.y);
        
        // Check proximity to NPCs - collect messages first to avoid borrowing conflict
        let mut proximity_messages = Vec::new();
        for npc in &self.npcs {
            let distance = self.distance(&self.player.position, &npc.entity.position);
            
            if distance < 3.0 {
                proximity_messages.push(format!("You are near {}", npc.entity.name));
            }
        }
        
        // Now add all messages after the loop is done
        for message in proximity_messages {
            self.add_message(&message);
        }
    }
    
    /// Calculate distance between positions
    fn distance(&self, a: &Position, b: &Position) -> f32 {
        let dx = a.x - b.x;
        let dy = a.y - b.y;
        (dx * dx + dy * dy).sqrt()
    }
    
    /// Process player chat
    fn process_chat(&mut self, text: &str) {
        // Add player message to chat
        self.add_message(&format!("Player: {}", text));
        
        // Find closest NPC
        let mut closest_npc_index = 0;
        let mut closest_distance = f32::MAX;
        
        for (i, npc) in self.npcs.iter().enumerate() {
            let distance = self.distance(&self.player.position, &npc.entity.position);
            
            if distance < closest_distance {
                closest_distance = distance;
                closest_npc_index = i;
            }
        }
        
        // Only process chat if an NPC is nearby
        if closest_distance <= 5.0 {
            // Prepare response message - avoid borrowing conflict
            let response_message = {
                let npc = &self.npcs[closest_npc_index];
                
                // Select a random response from this NPC's dialogue
                use rand::Rng;
                let mut rng = rand::thread_rng();
                let response_idx = rng.gen_range(0..npc.dialogue.len());
                let response = &npc.dialogue[response_idx];
                
                format!("{}: {}", npc.entity.name, response)
            };
            
            self.add_message(&response_message);
        } else {
            self.add_message("No one is close enough to hear you...");
        }
    }
    
    /// Add a message to the chat log
    fn add_message(&mut self, message: &str) {
        println!("{}", message);
        self.messages.push(message.to_string());
        
        // Limit message history
        if self.messages.len() > 100 {
            self.messages.remove(0);
        }
    }
    
    /// Display the game state
    fn render(&self) {
        // Clear screen
        print!("\x1B[2J\x1B[1;1H");
        
        // Print game title
        println!("=== Oxyde RPG Demo ===");
        println!("Game time: {:.1}s", self.time);
        println!();
        
        // Print player position
        println!("Player position: ({:.1}, {:.1})", self.player.position.x, self.player.position.y);
        
        // Print NPCs
        println!("\nNPCs:");
        for npc in &self.npcs {
            let distance = self.distance(&self.player.position, &npc.entity.position);
            
            println!("- {} at ({:.1}, {:.1}) - {:.1} units away", 
                npc.entity.name, npc.entity.position.x, npc.entity.position.y, distance);
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
    
    /// Update the game state
    fn update(&mut self) {
        // Calculate delta time
        let now = Instant::now();
        let delta_time = now.duration_since(self.last_update).as_secs_f32();
        self.last_update = now;
        
        // Update game time
        self.time += delta_time;
    }
}

fn main() {
    // Create game state
    let mut game = GameState::new();
    
    // Game loop
    loop {
        // Update game state
        game.update();
        
        // Render game state
        game.render();
        
        // Handle input
        let mut input = String::new();
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut input).unwrap_or(0);
        let input = input.trim().to_lowercase();
        
        if !input.is_empty() {
            match input.as_str() {
                "w" => game.move_player(0.0, 1.0),
                "a" => game.move_player(-1.0, 0.0),
                "s" => game.move_player(0.0, -1.0),
                "d" => game.move_player(1.0, 0.0),
                "q" => break,
                "t" => {
                    // Talk to nearest NPC
                    println!("Enter your message:");
                    let mut chat = String::new();
                    io::stdout().flush().unwrap();
                    io::stdin().read_line(&mut chat).unwrap_or(0);
                    game.process_chat(chat.trim());
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
