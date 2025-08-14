use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
/// Represents a voice profile for an NPC
pub struct VoiceProfile {
    /// The name of the NPC associated with this voice profile
    pub npc_name: String,
    /// The base voice characteristics for the NPC
    pub base_voice: BaseVoice,
    /// The emotional range settings for the NPC's voice
    pub emotional_range: EmotionalVoiceRange,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
/// Represents the base voice characteristics
pub struct BaseVoice {
    /// The unique identifier for the voice
    pub voice_id: String,
    /// The base pitch of the voice
    pub base_pitch: f32,
    /// The base rate (speed) of the voice
    pub base_rate: f32,
    /// The base volume of the voice
    pub base_volume: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
/// Represents Gender options for voice profiles
pub enum Gender {
    ///Male Agent Voice
    Male,
    ///Female Agent Voice
    Female,
    ///Non-binary Agent Voice
    NonBinary,
    ///Androgynous Agent Voice
    Androgynous,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
/// Represents the emotional ranges for a voice profile
/// This struct defines the ranges for various emotions that can be expressed by the voice
pub struct EmotionalVoiceRange {
    /// Ranges for different emotions
    ///These ranges define the minimum and maximum values for each emotion
    /// *
    /// *
    /// * Happiness: 0.0 to 1.0
    pub happiness_range: (f32, f32),

    /// * Anger: 0.0 to 1.0
    pub anger_range: (f32, f32),

    /// * Fear: 0.0 to 1.0
    pub fear_range: (f32, f32),

    /// * Trust: 0.0 to 1.0
    pub trust_range: (f32, f32),

    /// * Energy: 0.0 to 1.0
    pub energy_range: (f32, f32),

    /// * Curiosity: 0.0 to 1.0
    pub curiosity_range: (f32, f32),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
/// Represents the settings for a voice profile
/// This struct is used to configure the voice settings for TTS services like ElevenLabs
pub struct VoiceSettings {
    /// The voice ID used by the TTS service
    /// This is typically a unique identifier for the voice in the TTS service
    pub voice_id: String,
    
    /// Stability of the voice output
    /// This controls how consistent the voice output is, with higher values being more stable
    pub stability: f32,
    
    /// Similarity boost for the voice output
    pub similarity_boost: f32,

    /// Style exaggeration for the voice output
    /// This controls how much the voice style is exaggerated, with higher values leading to more pronounced
    pub style_exaggeration: f32,
}

impl VoiceProfile {
    /// Create a new voice profile with default values
    pub fn default_for_npc(npc_name: &str) -> Self {
        Self {
            npc_name: npc_name.to_string(),
            base_voice: BaseVoice {
                voice_id: "JBFqnCBsd6RMkjVDRZzb".to_string(),
                base_pitch: 0.5,
                base_rate: 0.5,
                base_volume: 0.7,
            },
            emotional_range: EmotionalVoiceRange {
                happiness_range: (0.0, 0.3),
                anger_range: (0.0, 0.4),
                fear_range: (0.0, 0.3),
                trust_range: (0.0, 0.2),
                energy_range: (0.0, 0.3),
                curiosity_range: (0.0, 0.3),
            },
        }
    }

    /// Create a new voice profile for a specific NPC (merchant, guard, wizard)
    pub fn merchant() -> Self {
        Self {
            npc_name: "merchant".to_string(),
            base_voice: BaseVoice {
                voice_id: "friendly_male".to_string(),
                base_pitch: 0.4,
                base_rate: 0.6,
                base_volume: 0.8,
            },
            emotional_range: EmotionalVoiceRange {
                happiness_range: (0.1, 0.4),
                anger_range: (0.0, 0.2),
                fear_range: (0.0, 0.1),
                trust_range: (0.0, 0.1),
                energy_range: (0.2, 0.5),
                curiosity_range: (0.0, 0.5),
            },
        }
    }

    /// Create a new voice profile for a specific NPC (guard)
    pub fn guard() -> Self {
        Self {
            npc_name: "guard".to_string(),
            base_voice: BaseVoice {
                voice_id: "authoritative_male".to_string(),
                base_pitch: 0.3,
                base_rate: 0.4,
                base_volume: 0.9,
            },
            emotional_range: EmotionalVoiceRange {
                happiness_range: (0.0, 0.2),
                anger_range: (0.2, 0.6),
                fear_range: (0.0, 0.1),
                trust_range: (0.0, 0.1),
                energy_range: (0.0, 0.3),
                curiosity_range: (0.0, 0.6),
            },
        }
    }
    /// Create a new voice profile for a specific NPC (wizard)
    pub fn wizard() -> Self {
        Self {
            npc_name: "wizard".to_string(),
            base_voice: BaseVoice {
                voice_id: "wise_elder".to_string(),
                base_pitch: 0.2,
                base_rate: 0.3,
                base_volume: 0.6,
            },
            emotional_range: EmotionalVoiceRange {
                happiness_range: (0.0, 0.3),
                anger_range: (0.1, 0.4),
                fear_range: (0.0, 0.2),
                trust_range: (0.1, 0.3),
                energy_range: (0.0, 0.4),
                curiosity_range: (0.0, 0.2), // Fixed missing colon
            },
        }
    }
}

impl VoiceSettings {
    /// Create a new voice settings instance from a voice profile
    /// This method initializes the voice settings based on the provided voice profile
    pub fn from_profile(profile: &VoiceProfile) -> Self {
        Self {
            voice_id: profile.base_voice.voice_id.clone(),
            stability: 0.75,
            similarity_boost: 0.75,
            style_exaggeration: 0.3, // Default value for now
        }
    }
}

impl EmotionalVoiceRange {
    /// Create a new emotional voice range from a personality description
    /// This method generates emotional ranges based on the personality traits described
    pub fn from_personality(personality: &str) -> Self {
        let personality_lower = personality.to_lowercase();

        let mut range = Self {
            happiness_range: (0.0, 0.3),
            anger_range: (0.0, 0.3),
            fear_range: (0.0, 0.2),
            trust_range: (0.0, 0.2),
            energy_range: (0.0, 0.3),
            curiosity_range: (0.0, 0.4),
        };

        if personality_lower.contains("cheerful") || personality_lower.contains("optimistic") {
            range.happiness_range = (0.3, 0.5);
        }

        if personality_lower.contains("grumpy") || personality_lower.contains("irritable") {
            range.anger_range = (0.3, 0.5);
        }

        if personality_lower.contains("nervous") || personality_lower.contains("anxious") {
            range.fear_range = (0.2, 0.6);
        }

        if personality_lower.contains("energetic") || personality_lower.contains("enthusiastic") {
            range.happiness_range = (0.2, 0.6);
            range.energy_range = (0.3, 0.7);
        }

        range
    }
}
