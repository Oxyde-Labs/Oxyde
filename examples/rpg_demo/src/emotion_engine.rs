use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EmotionalState {
    pub happiness: f32,    // 0.0 to 1.0
    pub anger: f32,        // 0.0 to 1.0
    pub fear: f32,         // 0.0 to 1.0
    pub trust: f32,        // 0.0 to 1.0
    pub energy: f32,       // 0.0 to 1.0
    pub curiosity: f32,    // 0.0 to 1.0
}

impl Default for EmotionalState {
    fn default() -> Self {
        Self {
            happiness: 0.5,
            anger: 0.0,
            fear: 0.2,
            trust: 0.3,
            energy: 0.6,
            curiosity: 0.4,
        }
    }
}

impl EmotionalState {
    pub fn new_for_role(role: &str) -> Self {
        match role.to_lowercase().as_str() {
            "merchant" => Self {
                happiness: 0.7,
                anger: 0.1,
                fear: 0.1,
                trust: 0.6,
                energy: 0.8,
                curiosity: 0.5,
            },
            "guard" => Self {
                happiness: 0.4,
                anger: 0.2,
                fear: 0.1,
                trust: 0.3,
                energy: 0.7,
                curiosity: 0.2,
            },
            "villager" => Self {
                happiness: 0.6,
                anger: 0.1,
                fear: 0.3,
                trust: 0.5,
                energy: 0.5,
                curiosity: 0.8,
            },
            _ => Self::default(),
        }
    }

    pub fn update_from_interaction(&mut self, player_message: &str, interaction_type: InteractionType) {
        let message_lower = player_message.to_lowercase();
        
        // Analyze message sentiment
        let is_positive = message_lower.contains("hello") || message_lower.contains("thank") || 
                         message_lower.contains("please") || message_lower.contains("good") ||
                         message_lower.contains("nice") || message_lower.contains("help");
        
        let is_negative = message_lower.contains("angry") || message_lower.contains("hate") ||
                         message_lower.contains("stupid") || message_lower.contains("bad") ||
                         message_lower.contains("annoying") || message_lower.contains("shut up");
        
        let is_threatening = message_lower.contains("kill") || message_lower.contains("hurt") ||
                           message_lower.contains("attack") || message_lower.contains("fight");
        
        let is_curious = message_lower.contains("what") || message_lower.contains("how") ||
                        message_lower.contains("why") || message_lower.contains("where") ||
                        message_lower.contains("tell me");

        // Update emotions based on message content
        if is_threatening {
            self.fear = (self.fear + 0.3).min(1.0);
            self.anger = (self.anger + 0.2).min(1.0);
            self.trust = (self.trust - 0.4).max(0.0);
            self.happiness = (self.happiness - 0.3).max(0.0);
        } else if is_negative {
            self.anger = (self.anger + 0.2).min(1.0);
            self.happiness = (self.happiness - 0.2).max(0.0);
            self.trust = (self.trust - 0.1).max(0.0);
        } else if is_positive {
            self.happiness = (self.happiness + 0.2).min(1.0);
            self.trust = (self.trust + 0.1).min(1.0);
            self.anger = (self.anger - 0.1).max(0.0);
            self.fear = (self.fear - 0.1).max(0.0);
        }

        if is_curious {
            self.curiosity = (self.curiosity + 0.1).min(1.0);
        }

        // Apply interaction type modifiers
        match interaction_type {
            InteractionType::FirstMeeting => {
                self.curiosity = (self.curiosity + 0.2).min(1.0);
                self.energy = (self.energy + 0.1).min(1.0);
            },
            InteractionType::Repeated => {
                self.trust = (self.trust + 0.1).min(1.0);
                self.happiness = (self.happiness + 0.1).min(1.0);
            },
            InteractionType::LongConversation => {
                self.energy = (self.energy - 0.1).max(0.0);
                self.trust = (self.trust + 0.2).min(1.0);
            }
        }

        // Natural emotion decay over time
        self.decay_emotions();
    }

    fn decay_emotions(&mut self) {
        // Emotions naturally return to baseline over time
        self.anger = (self.anger - 0.05).max(0.0);
        self.fear = (self.fear - 0.03).max(0.0);
        
        // Positive emotions decay slower
        if self.happiness > 0.5 {
            self.happiness = (self.happiness - 0.02).max(0.5);
        } else {
            self.happiness = (self.happiness + 0.02).min(0.5);
        }
        
        if self.trust > 0.3 {
            self.trust = (self.trust - 0.01).max(0.3);
        }
    }

    pub fn get_dominant_emotion(&self) -> String {
        let mut emotions = vec![
            ("happy", self.happiness),
            ("angry", self.anger),
            ("fearful", self.fear),
            ("trusting", self.trust),
            ("energetic", self.energy),
            ("curious", self.curiosity),
        ];
        
        emotions.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());
        emotions[0].0.to_string()
    }

    pub fn get_emotional_modifier(&self) -> String {
        let dominant = self.get_dominant_emotion();
        
        match dominant.as_str() {
            "happy" => "You're in a cheerful mood and speak warmly.",
            "angry" => "You're irritated and speak curtly.",
            "fearful" => "You're nervous and speak cautiously.",
            "trusting" => "You're comfortable and speak openly.",
            "energetic" => "You're enthusiastic and speak with excitement.",
            "curious" => "You're intrigued and ask questions.",
            _ => "You speak normally.",
        }.to_string()
    }

    pub fn should_end_conversation(&self) -> bool {
        self.anger > 0.8 || self.fear > 0.8 || self.energy < 0.2
    }

    pub fn get_response_style(&self) -> ResponseStyle {
        if self.anger > 0.6 {
            ResponseStyle::Hostile
        } else if self.fear > 0.6 {
            ResponseStyle::Nervous
        } else if self.happiness > 0.7 {
            ResponseStyle::Friendly
        } else if self.trust > 0.7 {
            ResponseStyle::Open
        } else if self.curiosity > 0.7 {
            ResponseStyle::Inquisitive
        } else {
            ResponseStyle::Neutral
        }
    }
}

#[derive(Debug, Clone)]
pub enum InteractionType {
    FirstMeeting,
    Repeated,
    LongConversation,
}

#[derive(Debug, Clone)]
pub enum ResponseStyle {
    Friendly,
    Hostile,
    Nervous,
    Open,
    Inquisitive,
    Neutral,
}

impl ResponseStyle {
    pub fn get_style_prompt(&self) -> &str {
        match self {
            ResponseStyle::Friendly => "Respond in a warm, friendly manner with enthusiasm.",
            ResponseStyle::Hostile => "Respond curtly and show irritation. Keep answers brief.",
            ResponseStyle::Nervous => "Respond cautiously and show hesitation. Be brief and careful.",
            ResponseStyle::Open => "Respond openly and share more details willingly.",
            ResponseStyle::Inquisitive => "Respond with interest and ask follow-up questions.",
            ResponseStyle::Neutral => "Respond normally and professionally.",
        }
    }
}

pub struct EmotionEngine {
    pub npc_emotions: HashMap<String, EmotionalState>,
    interaction_counts: HashMap<String, u32>,
}

impl EmotionEngine {
    pub fn new() -> Self {
        Self {
            npc_emotions: HashMap::new(),
            interaction_counts: HashMap::new(),
        }
    }

    pub fn get_or_create_emotional_state(&mut self, npc_id: &str, npc_role: &str) -> &mut EmotionalState {
        self.npc_emotions
            .entry(npc_id.to_string())
            .or_insert_with(|| EmotionalState::new_for_role(npc_role))
    }

    pub fn update_npc_emotion(&mut self, npc_id: &str, npc_role: &str, player_message: &str) -> (String, ResponseStyle) {
        let interaction_count = self.interaction_counts
            .entry(npc_id.to_string())
            .and_modify(|count| *count += 1)
            .or_insert(1);

        let interaction_type = match *interaction_count {
            1 => InteractionType::FirstMeeting,
            2..=5 => InteractionType::Repeated,
            _ => InteractionType::LongConversation,
        };

        let emotional_state = self.get_or_create_emotional_state(npc_id, npc_role);
        emotional_state.update_from_interaction(player_message, interaction_type);

        let emotional_modifier = emotional_state.get_emotional_modifier();
        let response_style = emotional_state.get_response_style();
        
        println!("NPC {} emotional state - Dominant: {}, Happiness: {:.2}, Anger: {:.2}, Trust: {:.2}", 
            npc_id, emotional_state.get_dominant_emotion(), 
            emotional_state.happiness, emotional_state.anger, emotional_state.trust);

        (emotional_modifier, response_style)
    }

    pub fn should_npc_end_conversation(&self, npc_id: &str) -> bool {
        self.npc_emotions
            .get(npc_id)
            .map(|state| state.should_end_conversation())
            .unwrap_or(false)
    }
}