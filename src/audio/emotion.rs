// // use serde::{Deserialize, Serialize};
// use std::collections::HashMap;
// use crate::oxyde_game::emotion::EmotionalState;

// /// Represents the emotional state with various emotion levels.
// // #[derive(Debug, Clone, Serialize, Deserialize)]
// // pub struct EmotionalState {
// //     /// Level of joy (0.0 to 1.0)
// //     pub joy: f32,
// //     /// Level of anger (0.0 to 1.0)
// //     pub anger: f32,
// //     /// Level of fear (0.0 to 1.0)
// //     pub fear: f32,
// //     /// Level of trust (0.0 to 1.0)
// //     pub trust: f32,
// //     /// Level of surprise (0.0 to 1.0)
// //     pub surprise: f32,
// //     /// Level of sadness (0.0 to 1.0)
// //     pub sadness: f32,
// // }

// impl EmotionalState {

//     /// Create an emotional state from a map of emotion names to values
//     pub fn from_map(emotions: HashMap<String, f32>) -> Self {
//         Self {
//             joy: emotions.get("joy").copied().unwrap_or(0.0).clamp(0.0, 1.0),
//             anger: emotions.get("anger").copied().unwrap_or(0.0).clamp(0.0, 1.0),
//             fear: emotions.get("fear").copied().unwrap_or(0.0).clamp(0.0, 1.0),
//             trust: emotions.get("trust").copied().unwrap_or(0.0).clamp(0.0, 1.0),
//             surprise: emotions.get("surprise").copied().unwrap_or(0.0).clamp(0.0, 1.0),
//             sadness: emotions.get("sadness").copied().unwrap_or(0.0).clamp(0.0, 1.0),
//             disgust: emotions.get("disgust").copied().unwrap_or(0.0).clamp(0.0, 1.0),
//             anticipation: emotions.get("anticipation").copied().unwrap_or(0.0).clamp(0.0, 1.0),
//         }
//     }


//     /// Calculate the overall emotional intensity (0.0 to 1.0)
//     pub fn intensity(&self) -> f32 {
//         let sum = self.joy + self.anger + self.fear + self.trust + self.surprise + self.sadness;
//         (sum / 6.0).clamp(0.0, 1.0)
//     }

//     /// Blend with another emotional state
//     pub fn blend_with(&self, other: &EmotionalState, weight: f32) -> Self {
//         let w = weight.clamp(0.0, 1.0);
//         Self {
//             joy: self.joy * (1.0 - w) + other.joy * w,
//             anger: self.anger * (1.0 - w) + other.anger * w,
//             fear: self.fear * (1.0 - w) + other.fear * w,
//             trust: self.trust * (1.0 - w) + other.trust * w,
//             surprise: self.surprise * (1.0 - w) + other.surprise * w,
//             sadness: self.sadness * (1.0 - w) + other.sadness * w,
//         }
//     }

//     /// Clamp all emotions to valid range [0.0, 1.0]
// pub fn clamp(&mut self) {
//     for val in [
//         &mut self.joy,
//         &mut self.anger,
//         &mut self.fear,
//         &mut self.trust,
//         &mut self.surprise,
//         &mut self.sadness,
//     ] {
//         *val = val.clamp(0.0, 1.0);
//     }
// }

// }
