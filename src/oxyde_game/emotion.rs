use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EmotionalState {
    // Basic emotions on a -1.0 to 1.0 scale
    pub happiness: f32,
    pub anger: f32,
    pub fear: f32,
}

impl EmotionalState {
    pub fn new() -> Self {
        Self {
            happiness: 0.0,
            anger: 0.0,
            fear: 0.0,
        }
    }
}