use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use crate::emotion_engine::EmotionalState;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Goal {
    pub id: String,
    pub description: String,
    pub goal_type: GoalType,
    pub priority: f32,           // 0.0 to 1.0
    pub progress: f32,           // 0.0 to 1.0
    pub target_conditions: Vec<String>,
    pub emotional_drivers: Vec<String>,
    pub time_pressure: Option<u64>, // seconds until goal expires
    pub created_at: u64,
    pub last_updated: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum GoalType {
    Survival,       // Basic needs: food, safety, health
    Economic,       // Making money, trading, acquiring items
    Social,         // Relationships, reputation, alliances
    Knowledge,      // Learning information, discovering secrets
    Creative,       // Building, crafting, artistic pursuits
    Adventure,      // Exploration, quests, challenges
    Revenge,        // Settling scores, getting justice
    Romance,        // Finding love, maintaining relationships
    Protection,     // Safeguarding others or things
    Personal,       // Self-improvement, skill development
}

impl Goal {
    pub fn new(id: String, description: String, goal_type: GoalType, priority: f32) -> Self {
        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs();
            
        Self {
            id,
            description,
            goal_type,
            priority,
            progress: 0.0,
            target_conditions: Vec::new(),
            emotional_drivers: Vec::new(),
            time_pressure: None,
            created_at: now,
            last_updated: now,
        }
    }

    pub fn update_progress(&mut self, delta: f32, reason: &str) {
        self.progress = (self.progress + delta).clamp(0.0, 1.0);
        self.last_updated = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs();
        
        println!("Goal '{}' progress updated by {}: {} -> {:.2}% ({})", 
            self.description, delta, self.progress - delta, self.progress * 100.0, reason);
    }

    pub fn is_completed(&self) -> bool {
        self.progress >= 1.0
    }

    pub fn is_urgent(&self) -> bool {
        if let Some(deadline) = self.time_pressure {
            let now = std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs();
            deadline.saturating_sub(now) < 300 // Less than 5 minutes
        } else {
            false
        }
    }

    pub fn calculate_motivation(&self, emotional_state: &EmotionalState) -> f32 {
        let base_motivation = self.priority;
        let urgency_boost = if self.is_urgent() { 0.3 } else { 0.0 };
        
        let emotional_modifier = match self.goal_type {
            GoalType::Economic => emotional_state.energy * 0.2,
            GoalType::Social => emotional_state.happiness * 0.2 + emotional_state.trust * 0.1,
            GoalType::Survival => emotional_state.fear * 0.3,
            GoalType::Revenge => emotional_state.anger * 0.4,
            GoalType::Protection => emotional_state.trust * 0.2 + (1.0 - emotional_state.fear) * 0.1,
            _ => emotional_state.curiosity * 0.1,
        };

        (base_motivation + urgency_boost + emotional_modifier).clamp(0.0, 1.0)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StoryEvent {
    pub id: String,
    pub event_type: EventType,
    pub description: String,
    pub participants: Vec<String>, // NPC IDs involved
    pub consequences: Vec<String>,
    pub emotional_impact: HashMap<String, f32>, // NPC ID -> emotional change
    pub timestamp: u64,
    pub location: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum EventType {
    Trade,
    Conflict,
    Alliance,
    Discovery,
    Betrayal,
    Romance,
    Achievement,
    Disaster,
    Celebration,
    Secret,
}

pub struct GoalEngine {
    npc_goals: HashMap<String, Vec<Goal>>,
    world_events: Vec<StoryEvent>,
    npc_relationships: HashMap<String, HashMap<String, f32>>, // NPC -> NPC -> relationship score
    event_counter: u64,
}

impl GoalEngine {
    pub fn new() -> Self {
        Self {
            npc_goals: HashMap::new(),
            world_events: Vec::new(),
            npc_relationships: HashMap::new(),
            event_counter: 0,
        }
    }

    pub fn initialize_npc_goals(&mut self, npc_id: &str, npc_role: &str) {
        let goals = self.generate_initial_goals(npc_id, npc_role);
        self.npc_goals.insert(npc_id.to_string(), goals);
        self.npc_relationships.insert(npc_id.to_string(), HashMap::new());
    }

    fn generate_initial_goals(&self, npc_id: &str, npc_role: &str) -> Vec<Goal> {
        let mut goals = Vec::new();
        
        match npc_role.to_lowercase().as_str() {
            "merchant" => {
                goals.push(Goal::new(
                    format!("{}_profit", npc_id),
                    "Earn 1000 gold coins this month".to_string(),
                    GoalType::Economic,
                    0.8,
                ));
                
                goals.push(Goal::new(
                    format!("{}_reputation", npc_id),
                    "Build trust with local customers".to_string(),
                    GoalType::Social,
                    0.6,
                ));
                
                goals.push(Goal::new(
                    format!("{}_rare_item", npc_id),
                    "Acquire a legendary artifact to sell".to_string(),
                    GoalType::Adventure,
                    0.4,
                ));
            },
            "guard" => {
                goals.push(Goal::new(
                    format!("{}_protect_town", npc_id),
                    "Keep the town safe from threats".to_string(),
                    GoalType::Protection,
                    0.9,
                ));
                
                goals.push(Goal::new(
                    format!("{}_criminal_network", npc_id),
                    "Uncover the local smuggling operation".to_string(),
                    GoalType::Knowledge,
                    0.7,
                ));
                
                goals.push(Goal::new(
                    format!("{}_training", npc_id),
                    "Improve combat skills through practice".to_string(),
                    GoalType::Personal,
                    0.5,
                ));
            },
            "villager" => {
                goals.push(Goal::new(
                    format!("{}_gossip", npc_id),
                    "Learn all the latest town news and secrets".to_string(),
                    GoalType::Knowledge,
                    0.7,
                ));
                
                goals.push(Goal::new(
                    format!("{}_friendship", npc_id),
                    "Make friends with the new travelers".to_string(),
                    GoalType::Social,
                    0.6,
                ));
                
                goals.push(Goal::new(
                    format!("{}_festival", npc_id),
                    "Organize the upcoming harvest festival".to_string(),
                    GoalType::Creative,
                    0.5,
                ));
            },
            _ => {
                goals.push(Goal::new(
                    format!("{}_survival", npc_id),
                    "Maintain basic needs and safety".to_string(),
                    GoalType::Survival,
                    0.8,
                ));
            }
        }
        
        goals
    }

    pub fn get_npc_goals(&self, npc_id: &str) -> Vec<&Goal> {
        self.npc_goals
            .get(npc_id)
            .map(|goals| goals.iter().collect())
            .unwrap_or_default()
    }

    pub fn get_primary_goal(&self, npc_id: &str, emotional_state: &EmotionalState) -> Option<&Goal> {
        let goals = self.npc_goals.get(npc_id)?;
        
        goals.iter()
            .filter(|g| !g.is_completed())
            .max_by(|a, b| {
                let a_motivation = a.calculate_motivation(emotional_state);
                let b_motivation = b.calculate_motivation(emotional_state);
                a_motivation.partial_cmp(&b_motivation).unwrap()
            })
    }

    pub fn update_goal_from_interaction(&mut self, npc_id: &str, player_message: &str, ai_response: &str) {
        if let Some(goals) = self.npc_goals.get_mut(npc_id) {
            let message_lower = player_message.to_lowercase();
            let response_lower = ai_response.to_lowercase();
            
            for goal in goals.iter_mut() {
                match goal.goal_type {
                    GoalType::Economic => {
                        if message_lower.contains("buy") || message_lower.contains("purchase") || 
                           message_lower.contains("trade") || response_lower.contains("gold") {
                            goal.update_progress(0.1, "potential trade interaction");
                        }
                    },
                    GoalType::Social => {
                        if message_lower.contains("hello") || message_lower.contains("friend") ||
                           message_lower.contains("thank") || response_lower.contains("friend") {
                            goal.update_progress(0.05, "positive social interaction");
                        }
                    },
                    GoalType::Knowledge => {
                        if message_lower.contains("what") || message_lower.contains("tell me") ||
                           message_lower.contains("news") || message_lower.contains("know") {
                            goal.update_progress(0.08, "information exchange");
                        }
                    },
                    GoalType::Protection => {
                        if message_lower.contains("danger") || message_lower.contains("threat") ||
                           message_lower.contains("safe") || response_lower.contains("protect") {
                            goal.update_progress(0.12, "security-related conversation");
                        }
                    },
                    _ => {}
                }
            }
        }
    }

    pub fn generate_story_event(&mut self, npc_id: &str, _context: &str) -> Option<StoryEvent> {
        let goals = self.npc_goals.get(npc_id)?;
        if goals.is_empty() {
            return None;
        }

        // Find goals that are near completion or highly motivated
        let active_goals: Vec<_> = goals.iter()
            .filter(|g| g.progress > 0.3 && !g.is_completed())
            .collect();

        if active_goals.is_empty() {
            return None;
        }

        let goal = active_goals[0]; // Take the first active goal for simplicity
        
        let event_counter = self.event_counter;
        self.event_counter += 1;
        let event_id = format!("event_{}", event_counter);

        let (event_type, description) = match goal.goal_type {
            GoalType::Economic => {
                (EventType::Trade, format!("{} discovered a new trade opportunity", npc_id))
            },
            GoalType::Social => {
                (EventType::Alliance, format!("{} formed a new friendship", npc_id))
            },
            GoalType::Knowledge => {
                (EventType::Discovery, format!("{} uncovered important information", npc_id))
            },
            GoalType::Protection => {
                (EventType::Conflict, format!("{} confronted a potential threat", npc_id))
            },
            _ => {
                (EventType::Achievement, format!("{} made progress toward their goal", npc_id))
            }
        };

        let event = StoryEvent {
            id: event_id,
            event_type,
            description,
            participants: vec![npc_id.to_string()],
            consequences: vec![goal.description.clone()],
            emotional_impact: HashMap::new(),
            timestamp: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs(),
            location: None,
        };

        self.world_events.push(event.clone());
        println!("Generated story event: {}", event.description);
        
        Some(event)
    }

    pub fn get_contextual_motivation(&self, npc_id: &str, emotional_state: &EmotionalState) -> String {
        if let Some(primary_goal) = self.get_primary_goal(npc_id, emotional_state) {
            let motivation = primary_goal.calculate_motivation(emotional_state);
            let urgency = if primary_goal.is_urgent() { " (URGENT)" } else { "" };
            
            format!(
                "Current focus: {} (motivation: {:.1}/10, progress: {:.0}%{})",
                primary_goal.description,
                motivation * 10.0,
                primary_goal.progress * 100.0,
                urgency
            )
        } else {
            "No active goals at the moment".to_string()
        }
    }

    pub fn get_recent_events(&self, limit: usize) -> Vec<&StoryEvent> {
        self.world_events
            .iter()
            .rev()
            .take(limit)
            .collect()
    }

    pub fn complete_goal(&mut self, npc_id: &str, goal_id: &str) -> bool {
        let goal_type = {
            if let Some(goals) = self.npc_goals.get_mut(npc_id) {
                if let Some(goal) = goals.iter_mut().find(|g| g.id == goal_id) {
                    goal.progress = 1.0;
                    goal.last_updated = std::time::SystemTime::now()
                        .duration_since(std::time::UNIX_EPOCH)
                        .unwrap()
                        .as_secs();
                    
                    println!("Goal completed: {} - {}", npc_id, goal.description);
                    Some(goal.goal_type.clone())
                } else {
                    None
                }
            } else {
                None
            }
        };
        
        if let Some(completed_goal_type) = goal_type {
            // Generate a new goal to replace the completed one
            self.generate_new_goal(npc_id, &completed_goal_type);
            true
        } else {
            false
        }
    }

    fn generate_new_goal(&mut self, npc_id: &str, completed_goal_type: &GoalType) {
        let new_goal = match completed_goal_type {
            GoalType::Economic => Goal::new(
                format!("{}_new_venture_{}", npc_id, self.event_counter),
                "Expand business to new markets".to_string(),
                GoalType::Economic,
                0.7,
            ),
            GoalType::Social => Goal::new(
                format!("{}_new_relationship_{}", npc_id, self.event_counter),
                "Strengthen community bonds".to_string(),
                GoalType::Social,
                0.6,
            ),
            GoalType::Knowledge => Goal::new(
                format!("{}_new_mystery_{}", npc_id, self.event_counter),
                "Investigate strange occurrences".to_string(),
                GoalType::Knowledge,
                0.8,
            ),
            _ => Goal::new(
                format!("{}_new_challenge_{}", npc_id, self.event_counter),
                "Take on a new personal challenge".to_string(),
                GoalType::Personal,
                0.5,
            ),
        };

        if let Some(goals) = self.npc_goals.get_mut(npc_id) {
            goals.push(new_goal);
        }
        
        self.event_counter += 1;
    }
}