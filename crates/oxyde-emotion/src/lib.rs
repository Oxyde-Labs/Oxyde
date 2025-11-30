//! Emotion system for Oxyde agents
//!
//! This module implements Plutchik's wheel of emotions with 8 primary emotions
//! and derived dimensions (valence and arousal). Emotions decay over time and
//! influence agent behavior and memory consolidation.

use serde::{Deserialize, Serialize};

/// Emotional state based on Plutchik's wheel of emotions
///
/// Each emotion is represented as a value between -1.0 and 1.0, where:
/// - Positive values indicate presence of the emotion
/// - Negative values indicate presence of the opposite emotion
/// - 0.0 indicates neutral state
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct EmotionalState {
    /// Joy (opposite: sadness)
    /// Positive: happiness, elation
    /// Negative: sorrow, grief
    pub joy: f32,

    /// Trust (opposite: disgust)
    /// Positive: acceptance, admiration
    /// Negative: loathing, aversion
    pub trust: f32,

    /// Fear (opposite: anger)
    /// Positive: apprehension, terror
    /// Negative: rage, annoyance
    pub fear: f32,

    /// Surprise (opposite: anticipation)
    /// Positive: amazement, distraction
    /// Negative: vigilance, interest
    pub surprise: f32,

    /// Sadness (derived from negative joy)
    /// Positive: pensiveness, grief
    /// Negative: serenity, joy
    pub sadness: f32,

    /// Disgust (derived from negative trust)
    /// Positive: boredom, loathing
    /// Negative: acceptance, trust
    pub disgust: f32,

    /// Anger (derived from negative fear)
    /// Positive: annoyance, rage
    /// Negative: apprehension, fear
    pub anger: f32,

    /// Anticipation (derived from negative surprise)
    /// Positive: interest, vigilance
    /// Negative: distraction, amazement
    pub anticipation: f32,

    /// Decay rate for emotions (0.0 - 1.0)
    /// Higher values mean emotions fade faster
    decay_rate: f32,
}

impl EmotionalState {
    /// Create a new emotional state with neutral emotions
    pub fn new() -> Self {
        Self {
            joy: 0.0,
            trust: 0.0,
            fear: 0.0,
            surprise: 0.0,
            sadness: 0.0,
            disgust: 0.0,
            anger: 0.0,
            anticipation: 0.0,
            decay_rate: 0.1, // 10% decay per update
        }
    }

    /// Create an emotional state with custom decay rate
    ///
    /// # Arguments
    ///
    /// * `decay_rate` - Rate at which emotions decay (0.0 - 1.0)
    pub fn with_decay_rate(decay_rate: f32) -> Self {
        let mut state = Self::new();
        state.decay_rate = decay_rate.clamp(0.0, 1.0);
        state
    }

    /// Calculate overall emotional valence (positive/negative)
    ///
    /// Returns a value between -1.0 (very negative) and 1.0 (very positive)
    pub fn valence(&self) -> f32 {
        let positive = self.joy + self.trust + self.anticipation;
        let negative = self.sadness + self.disgust + self.anger + self.fear;
        ((positive - negative) / 7.0).clamp(-1.0, 1.0)
    }

    /// Calculate emotional arousal (intensity/activation level)
    ///
    /// Returns a value between 0.0 (calm) and 1.0 (highly aroused)
    pub fn arousal(&self) -> f32 {
        let total = self.joy.abs()
            + self.trust.abs()
            + self.fear.abs()
            + self.surprise.abs()
            + self.sadness.abs()
            + self.disgust.abs()
            + self.anger.abs()
            + self.anticipation.abs();
        (total / 8.0).clamp(0.0, 1.0)
    }

    /// Get the dominant emotion
    ///
    /// Returns the name of the strongest emotion and its value
    pub fn dominant_emotion(&self) -> (&'static str, f32) {
        let emotions = [
            ("joy", self.joy),
            ("trust", self.trust),
            ("fear", self.fear),
            ("surprise", self.surprise),
            ("sadness", self.sadness),
            ("disgust", self.disgust),
            ("anger", self.anger),
            ("anticipation", self.anticipation),
        ];

        emotions
            .iter()
            .max_by(|(_, a), (_, b)| a.abs().partial_cmp(&b.abs()).unwrap())
            .map(|(name, value)| (*name, *value))
            .unwrap_or(("neutral", 0.0))
    }

    /// Apply time-based decay to all emotions
    ///
    /// Emotions gradually return to neutral state over time
    pub fn decay(&mut self) {
        self.joy *= 1.0 - self.decay_rate;
        self.trust *= 1.0 - self.decay_rate;
        self.fear *= 1.0 - self.decay_rate;
        self.surprise *= 1.0 - self.decay_rate;
        self.sadness *= 1.0 - self.decay_rate;
        self.disgust *= 1.0 - self.decay_rate;
        self.anger *= 1.0 - self.decay_rate;
        self.anticipation *= 1.0 - self.decay_rate;
    }

    /// Update a specific emotion
    ///
    /// # Arguments
    ///
    /// * `emotion` - Name of the emotion to update
    /// * `delta` - Amount to change the emotion by
    pub fn update_emotion(&mut self, emotion: &str, delta: f32) {
        let value = match emotion {
            "joy" => &mut self.joy,
            "trust" => &mut self.trust,
            "fear" => &mut self.fear,
            "surprise" => &mut self.surprise,
            "sadness" => &mut self.sadness,
            "disgust" => &mut self.disgust,
            "anger" => &mut self.anger,
            "anticipation" => &mut self.anticipation,
            _ => return,
        };

        *value = (*value + delta).clamp(-1.0, 1.0);

        // Update opposite emotions (Plutchik's wheel opposites)
        match emotion {
            "joy" => self.sadness = -self.joy,
            "sadness" => self.joy = -self.sadness,
            "trust" => self.disgust = -self.trust,
            "disgust" => self.trust = -self.disgust,
            "fear" => self.anger = -self.fear,
            "anger" => self.fear = -self.anger,
            "surprise" => self.anticipation = -self.surprise,
            "anticipation" => self.surprise = -self.anticipation,
            _ => {}
        }
    }

    /// Set multiple emotions at once
    ///
    /// # Arguments
    ///
    /// * `emotions` - Vector of (emotion_name, value) tuples
    pub fn set_emotions(&mut self, emotions: Vec<(&str, f32)>) {
        for (emotion, value) in emotions {
            self.update_emotion(emotion, value);
        }
    }

    /// Check if the agent is in a generally positive emotional state
    pub fn is_positive(&self) -> bool {
        self.valence() > 0.2
    }

    /// Check if the agent is in a generally negative emotional state
    pub fn is_negative(&self) -> bool {
        self.valence() < -0.2
    }

    /// Check if the agent is emotionally aroused (experiencing strong emotions)
    pub fn is_aroused(&self) -> bool {
        self.arousal() > 0.5
    }

    /// Reset all emotions to neutral
    pub fn reset(&mut self) {
        self.joy = 0.0;
        self.trust = 0.0;
        self.fear = 0.0;
        self.surprise = 0.0;
        self.sadness = 0.0;
        self.disgust = 0.0;
        self.anger = 0.0;
        self.anticipation = 0.0;
    }
}

impl Default for EmotionalState {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_emotional_state() {
        let state = EmotionalState::new();
        assert_eq!(state.joy, 0.0);
        assert_eq!(state.trust, 0.0);
        assert_eq!(state.fear, 0.0);
        assert_eq!(state.valence(), 0.0);
        assert_eq!(state.arousal(), 0.0);
    }

    #[test]
    fn test_valence_calculation() {
        let mut state = EmotionalState::new();
        // Directly set emotions for testing (bypassing update logic)
        state.joy = 0.8;
        state.trust = 0.6;
        state.sadness = 0.0;
        state.disgust = 0.0;
        state.anger = 0.0;
        state.fear = 0.0;
        assert!(state.valence() > 0.0);
        assert!(state.is_positive());

        // Set to negative emotions
        state.joy = 0.0;
        state.trust = 0.0;
        state.sadness = 0.9;
        state.anger = 0.7;
        assert!(state.valence() < 0.0);
        assert!(state.is_negative());
    }

    #[test]
    fn test_arousal_calculation() {
        let mut state = EmotionalState::new();
        // Directly set emotions for testing - need high values for is_aroused() which needs > 0.5
        // arousal = sum / 8, so we need sum > 4.0
        state.joy = 0.9;
        state.fear = 0.8;
        state.anger = 0.7;
        state.surprise = 0.8;
        state.trust = 0.9;
        assert!(state.arousal() > 0.0);
        assert!(state.is_aroused());
    }

    #[test]
    fn test_dominant_emotion() {
        let mut state = EmotionalState::new();
        state.joy = 0.9;
        state.fear = 0.3;

        let (emotion, value) = state.dominant_emotion();
        assert_eq!(emotion, "joy");
        assert_eq!(value, 0.9);
    }

    #[test]
    fn test_emotion_decay() {
        let mut state = EmotionalState::with_decay_rate(0.5);
        state.joy = 1.0;

        state.decay();
        assert_eq!(state.joy, 0.5);

        state.decay();
        assert_eq!(state.joy, 0.25);
    }

    #[test]
    fn test_update_emotion() {
        let mut state = EmotionalState::new();
        state.update_emotion("joy", 0.5);

        assert_eq!(state.joy, 0.5);
        assert_eq!(state.sadness, -0.5); // Opposite emotion

        state.update_emotion("joy", 0.8);
        assert_eq!(state.joy, 1.0); // Clamped to 1.0
    }

    #[test]
    fn test_set_emotions() {
        let mut state = EmotionalState::new();
        state.set_emotions(vec![("joy", 0.7), ("trust", 0.5), ("fear", 0.3)]);

        assert_eq!(state.joy, 0.7);
        assert_eq!(state.trust, 0.5);
        assert_eq!(state.fear, 0.3);
    }

    #[test]
    fn test_reset() {
        let mut state = EmotionalState::new();
        state.set_emotions(vec![("joy", 0.7), ("anger", 0.5)]);
        state.reset();

        assert_eq!(state.joy, 0.0);
        assert_eq!(state.anger, 0.0);
        assert_eq!(state.valence(), 0.0);
    }
}
