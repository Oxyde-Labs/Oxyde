//! Emotion-aware behavior implementations

use async_trait::async_trait;

use crate::agent::AgentContext;
use crate::oxyde_game::behavior::{Behavior, BehaviorResult, EmotionInfluence, EmotionTrigger};
use crate::oxyde_game::emotion::EmotionalState;
use crate::oxyde_game::intent::Intent;
use crate::Result;

/// Flee behavior triggered by high fear
#[derive(Debug)]
pub struct FleeBehavior {
    /// Fear threshold to trigger flee
    fear_threshold: f32,
}

impl FleeBehavior {
    /// Create a new flee behavior
    ///
    /// # Arguments
    ///
    /// * `fear_threshold` - Minimum fear level to trigger (0.0 to 1.0)
    pub fn new(fear_threshold: f32) -> Self {
        Self {
            fear_threshold: fear_threshold.clamp(0.0, 1.0),
        }
    }
}

#[async_trait]
impl Behavior for FleeBehavior {
    async fn matches_intent(&self, intent: &Intent) -> bool {
        // Flee can trigger on any threatening intent - check raw input and keywords
        let input_lower = intent.raw_input.to_lowercase();
        let has_threat_keyword = intent.keywords.iter().any(|k| {
            let k_lower = k.to_lowercase();
            k_lower.contains("threat") || k_lower.contains("attack") || k_lower.contains("danger")
        });

        input_lower.contains("threat")
            || input_lower.contains("attack")
            || input_lower.contains("danger")
            || has_threat_keyword
    }

    async fn execute(&self, _intent: &Intent, _context: &AgentContext) -> Result<BehaviorResult> {
        Ok(BehaviorResult::Response(
            "I need to get out of here! *backs away nervously*".to_string(),
        ))
    }

    fn emotion_trigger(&self) -> Option<EmotionTrigger> {
        Some(EmotionTrigger::SpecificEmotion {
            emotion: "fear".to_string(),
            min_value: self.fear_threshold,
        })
    }

    fn emotion_influences(&self) -> Vec<EmotionInfluence> {
        // Fleeing slightly reduces fear (temporary relief) but may increase anxiety
        vec![
            EmotionInfluence::new("fear", -0.1),
            EmotionInfluence::new("anticipation", 0.2),
        ]
    }

    fn priority(&self) -> u32 {
        100 // High priority - survival response
    }

    fn emotional_priority_modifier(&self, emotional_state: &EmotionalState) -> i32 {
        // Even higher priority when fear is extreme
        let (dominant, value) = emotional_state.dominant_emotion();
        if dominant == "fear" && value > 0.8 {
            50 // Adds 50 to base priority
        } else {
            0
        }
    }
}

/// Aggressive behavior triggered by anger
#[derive(Debug)]
pub struct AggressiveBehavior {
    /// Anger threshold to trigger aggression
    anger_threshold: f32,
}

impl AggressiveBehavior {
    /// Create a new aggressive behavior
    ///
    /// # Arguments
    ///
    /// * `anger_threshold` - Minimum anger level to trigger (0.0 to 1.0)
    pub fn new(anger_threshold: f32) -> Self {
        Self {
            anger_threshold: anger_threshold.clamp(0.0, 1.0),
        }
    }
}

#[async_trait]
impl Behavior for AggressiveBehavior {
    async fn matches_intent(&self, intent: &Intent) -> bool {
        // Aggression can trigger on insults, challenges, or confrontations
        let input_lower = intent.raw_input.to_lowercase();
        let has_aggressive_keyword = intent.keywords.iter().any(|k| {
            let k_lower = k.to_lowercase();
            k_lower.contains("insult") || k_lower.contains("challenge")
                || k_lower.contains("confront") || k_lower.contains("provoke")
        });

        input_lower.contains("insult")
            || input_lower.contains("challenge")
            || input_lower.contains("confront")
            || input_lower.contains("provoke")
            || has_aggressive_keyword
    }

    async fn execute(&self, _intent: &Intent, _context: &AgentContext) -> Result<BehaviorResult> {
        Ok(BehaviorResult::Response(
            "How dare you! You'll regret that! *glares menacingly*".to_string(),
        ))
    }

    fn emotion_trigger(&self) -> Option<EmotionTrigger> {
        Some(EmotionTrigger::SpecificEmotion {
            emotion: "anger".to_string(),
            min_value: self.anger_threshold,
        })
    }

    fn emotion_influences(&self) -> Vec<EmotionInfluence> {
        // Expressing aggression may temporarily reduce anger but damages trust
        vec![
            EmotionInfluence::new("anger", -0.15),
            EmotionInfluence::new("trust", -0.2),
        ]
    }

    fn priority(&self) -> u32 {
        80 // High priority but lower than survival
    }

    fn emotional_priority_modifier(&self, emotional_state: &EmotionalState) -> i32 {
        // Higher priority when angry AND aroused
        if emotional_state.arousal() > 0.6 {
            let (dominant, _) = emotional_state.dominant_emotion();
            if dominant == "anger" {
                return 30;
            }
        }
        0
    }
}

/// Friendly behavior triggered by joy and trust
#[derive(Debug)]
pub struct FriendlyBehavior {
    /// Minimum valence to trigger friendly behavior
    min_valence: f32,
}

impl FriendlyBehavior {
    /// Create a new friendly behavior
    ///
    /// # Arguments
    ///
    /// * `min_valence` - Minimum emotional valence (-1.0 to 1.0)
    pub fn new(min_valence: f32) -> Self {
        Self {
            min_valence: min_valence.clamp(-1.0, 1.0),
        }
    }
}

#[async_trait]
impl Behavior for FriendlyBehavior {
    async fn matches_intent(&self, intent: &Intent) -> bool {
        // Friendly behavior for greetings and social interaction
        use crate::oxyde_game::intent::IntentType;

        matches!(intent.intent_type, IntentType::Greeting | IntentType::Chat)
    }

    async fn execute(&self, _intent: &Intent, _context: &AgentContext) -> Result<BehaviorResult> {
        Ok(BehaviorResult::Response(
            "It's wonderful to see you! How can I help? *smiles warmly*".to_string(),
        ))
    }

    fn emotion_trigger(&self) -> Option<EmotionTrigger> {
        Some(EmotionTrigger::ValenceRange {
            min: self.min_valence,
            max: 1.0,
        })
    }

    fn emotion_influences(&self) -> Vec<EmotionInfluence> {
        // Positive social interaction increases joy and trust
        vec![
            EmotionInfluence::new("joy", 0.1),
            EmotionInfluence::new("trust", 0.15),
        ]
    }

    fn priority(&self) -> u32 {
        60 // Medium-high priority
    }

    fn emotional_priority_modifier(&self, emotional_state: &EmotionalState) -> i32 {
        // Even more friendly when feeling very positive
        if emotional_state.valence() > 0.7 {
            20
        } else {
            0
        }
    }
}

/// Cautious behavior triggered by fear and anticipation
#[derive(Debug)]
pub struct CautiousBehavior;

impl CautiousBehavior {
    /// Create a new cautious behavior
    pub fn new() -> Self {
        Self
    }
}

impl Default for CautiousBehavior {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl Behavior for CautiousBehavior {
    async fn matches_intent(&self, intent: &Intent) -> bool {
        // Cautious behavior for requests or questions
        use crate::oxyde_game::intent::IntentType;

        matches!(intent.intent_type, IntentType::Question | IntentType::Command)
    }

    async fn execute(&self, _intent: &Intent, _context: &AgentContext) -> Result<BehaviorResult> {
        Ok(BehaviorResult::Response(
            "I'm not sure about this... Let me think carefully. *hesitates*".to_string(),
        ))
    }

    fn emotion_trigger(&self) -> Option<EmotionTrigger> {
        // Triggers when moderately fearful or anticipating
        Some(EmotionTrigger::AnyEmotion { min_intensity: 0.4 })
    }

    fn emotion_influences(&self) -> Vec<EmotionInfluence> {
        // Being cautious slightly reduces fear through deliberation
        vec![
            EmotionInfluence::new("fear", -0.05),
            EmotionInfluence::new("anticipation", 0.1),
        ]
    }

    fn priority(&self) -> u32 {
        40 // Lower priority - more of a modulation behavior
    }
}

/// Joyful behavior that triggers when happy
#[derive(Debug)]
pub struct JoyfulBehavior;

impl JoyfulBehavior {
    /// Create a new joyful behavior
    pub fn new() -> Self {
        Self
    }
}

impl Default for JoyfulBehavior {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl Behavior for JoyfulBehavior {
    async fn matches_intent(&self, intent: &Intent) -> bool {
        // Any positive interaction when joyful - avoid threats
        let input_lower = intent.raw_input.to_lowercase();
        !input_lower.contains("threat") && !input_lower.contains("attack")
    }

    async fn execute(&self, _intent: &Intent, _context: &AgentContext) -> Result<BehaviorResult> {
        Ok(BehaviorResult::Response(
            "This is amazing! I'm so happy right now! *beams with joy*".to_string(),
        ))
    }

    fn emotion_trigger(&self) -> Option<EmotionTrigger> {
        Some(EmotionTrigger::SpecificEmotion {
            emotion: "joy".to_string(),
            min_value: 0.7,
        })
    }

    fn emotion_influences(&self) -> Vec<EmotionInfluence> {
        // Expressing joy reinforces positive emotions
        vec![
            EmotionInfluence::new("joy", 0.05),
            EmotionInfluence::new("trust", 0.1),
        ]
    }

    fn priority(&self) -> u32 {
        50 // Medium priority
    }

    fn emotional_priority_modifier(&self, emotional_state: &EmotionalState) -> i32 {
        // Very high priority when extremely joyful
        let (dominant, value) = emotional_state.dominant_emotion();
        if dominant == "joy" && value > 0.85 {
            40
        } else {
            0
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_emotion_trigger_specific_emotion() {
        let mut state = EmotionalState::new();
        // Directly set fear for testing
        state.fear = 0.8;

        let trigger = EmotionTrigger::SpecificEmotion {
            emotion: "fear".to_string(),
            min_value: 0.7,
        };

        assert!(trigger.matches(&state));
    }

    #[test]
    fn test_emotion_trigger_valence_range() {
        let mut state = EmotionalState::new();
        // Set multiple positive emotions for higher valence
        state.joy = 0.8;
        state.trust = 0.7;
        state.anticipation = 0.6;

        let trigger = EmotionTrigger::ValenceRange {
            min: 0.2,
            max: 1.0,
        };

        assert!(trigger.matches(&state));
    }

    #[test]
    fn test_emotion_trigger_high_arousal() {
        let mut state = EmotionalState::new();
        // Set multiple emotions to high values for high arousal
        state.joy = 0.9;
        state.fear = 0.8;
        state.anger = 0.7;
        state.surprise = 0.8;
        state.trust = 0.9;

        let trigger = EmotionTrigger::HighArousal { min_arousal: 0.5 };

        assert!(trigger.matches(&state));
    }

    #[test]
    fn test_emotion_influence_creation() {
        let influence = EmotionInfluence::new("joy", 0.5);

        assert_eq!(influence.emotion, "joy");
        assert_eq!(influence.delta, 0.5);
    }

    #[test]
    fn test_emotion_influence_clamping() {
        let influence = EmotionInfluence::new("fear", 2.0);

        assert_eq!(influence.delta, 1.0); // Should be clamped to 1.0
    }

    #[tokio::test]
    async fn test_flee_behavior() {
        let behavior = FleeBehavior::new(0.7);

        assert_eq!(behavior.priority(), 100);
        assert!(behavior.emotion_influences().len() == 2);
    }

    #[tokio::test]
    async fn test_friendly_behavior() {
        let behavior = FriendlyBehavior::new(0.5);

        assert_eq!(behavior.priority(), 60);
        assert!(behavior.emotion_influences().len() == 2);
    }
}
