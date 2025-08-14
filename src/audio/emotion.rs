use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Represents the emotional state with various emotion levels.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EmotionalState {
    /// Level of happiness (0.0 to 1.0)
    pub happiness: f32,
    /// Level of anger (0.0 to 1.0)
    pub anger: f32,
    /// Level of fear (0.0 to 1.0)
    pub fear: f32,
    /// Level of trust (0.0 to 1.0)
    pub trust: f32,
    /// Level of energy (0.0 to 1.0)
    pub energy: f32,
    /// Level of curiosity (0.0 to 1.0)
    pub curiosity: f32,
}

impl EmotionalState {
    /// Create a new neutral emotional state
    pub fn neutral() -> Self {
        Self {
            happiness: 0.0,
            anger: 0.0,
            fear: 0.0,
            trust: 0.0,
            energy: 0.0,
            curiosity: 0.0,
        }
    }

    /// Create an emotional state from a map of emotion names to values
    pub fn from_map(emotions: HashMap<String, f32>) -> Self {
        Self {
            happiness: emotions.get("happiness").copied().unwrap_or(0.0).clamp(0.0, 1.0),
            anger: emotions.get("anger").copied().unwrap_or(0.0).clamp(0.0, 1.0),
            fear: emotions.get("fear").copied().unwrap_or(0.0).clamp(0.0, 1.0),
            trust: emotions.get("trust").copied().unwrap_or(0.0).clamp(0.0, 1.0),
            energy: emotions.get("energy").copied().unwrap_or(0.0).clamp(0.0, 1.0),
            curiosity: emotions.get("curiosity").copied().unwrap_or(0.0).clamp(0.0, 1.0),
        }
    }

    /// Get the dominant emotion (highest value)
    pub fn dominant_emotion(&self) -> (&'static str, f32) {
        let emotions = [
            ("happiness", self.happiness),
            ("anger", self.anger),
            ("fear", self.fear),
            ("trust", self.trust),
            ("energy", self.energy),
            ("curiosity", self.curiosity),
        ];

        emotions.iter()
            .max_by(|(_, a), (_, b)| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal))
            .copied()
            .unwrap_or(("neutral", 0.0))
    }

    /// Calculate the overall emotional intensity (0.0 to 1.0)
    pub fn intensity(&self) -> f32 {
        let sum = self.happiness + self.anger + self.fear + self.trust + self.energy + self.curiosity;
        (sum / 6.0).clamp(0.0, 1.0)
    }

    /// Blend with another emotional state
    pub fn blend_with(&self, other: &EmotionalState, weight: f32) -> Self {
        let w = weight.clamp(0.0, 1.0);
        Self {
            happiness: self.happiness * (1.0 - w) + other.happiness * w,
            anger: self.anger * (1.0 - w) + other.anger * w,
            fear: self.fear * (1.0 - w) + other.fear * w,
            trust: self.trust * (1.0 - w) + other.trust * w,
            energy: self.energy * (1.0 - w) + other.energy * w,
            curiosity: self.curiosity * (1.0 - w) + other.curiosity * w,
        }
    }

    /// Clamp all emotions to valid range [0.0, 1.0]
pub fn clamp(&mut self) {
    for val in [
        &mut self.happiness,
        &mut self.anger,
        &mut self.fear,
        &mut self.trust,
        &mut self.energy,
        &mut self.curiosity,
    ] {
        *val = val.clamp(0.0, 1.0);
    }
}

}

impl Default for EmotionalState {
    fn default() -> Self {
        Self::neutral()
    }
}