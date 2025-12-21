//! Test scenarios for behavior priority experiments
//!
//! Each scenario represents a sequence of interactions with specific emotional contexts
//! designed to test different aspects of behavior selection.

use oxyde::oxyde_game::emotion::EmotionalState;
use oxyde::oxyde_game::intent::{Intent, IntentType};
use serde::{Deserialize, Serialize};

/// A single interaction step in a scenario
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InteractionStep {
    /// Description of what's happening
    pub description: String,

    /// The intent from the player/environment
    pub intent: Intent,

    /// Pre-configured emotional state for this step
    pub emotional_state: EmotionalState,

    /// Expected behavior type (for validation)
    pub expected_behavior_category: String,
}

/// A complete test scenario
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Scenario {
    /// Scenario name
    pub name: String,

    /// Scenario description
    pub description: String,

    /// Sequence of interaction steps
    pub steps: Vec<InteractionStep>,
}

/// Create all test scenarios
pub fn create_scenarios() -> Vec<Scenario> {
    vec![
        create_peaceful_scenario(),
        create_threatening_scenario(),
        create_provocative_scenario(),
        create_mixed_emotions_scenario(),
        create_escalation_scenario(),
    ]
}

/// Peaceful village interaction
fn create_peaceful_scenario() -> Scenario {
    let mut steps = Vec::new();

    // Step 1: Greeting in positive mood
    let mut emotional_state = EmotionalState::new();
    emotional_state.joy = 0.7;
    emotional_state.trust = 0.6;
    steps.push(InteractionStep {
        description: "Player approaches and waves hello".to_string(),
        intent: Intent {
            intent_type: IntentType::Greeting,
            confidence: 1.0,
            raw_input: "Hello there!".to_string(),
            keywords: vec!["hello".to_string(), "greeting".to_string()],
        },
        emotional_state,
        expected_behavior_category: "friendly".to_string(),
    });

    // Step 2: Question while calm
    let mut emotional_state = EmotionalState::new();
    emotional_state.trust = 0.5;
    emotional_state.anticipation = 0.4;
    steps.push(InteractionStep {
        description: "Player asks about the village".to_string(),
        intent: Intent {
            intent_type: IntentType::Question,
            confidence: 0.9,
            raw_input: "Can you tell me about this place?".to_string(),
            keywords: vec!["question".to_string(), "place".to_string()],
        },
        emotional_state,
        expected_behavior_category: "cautious".to_string(),
    });

    // Step 3: Casual chat while happy
    let mut emotional_state = EmotionalState::new();
    emotional_state.joy = 0.8;
    emotional_state.trust = 0.7;
    steps.push(InteractionStep {
        description: "Player makes friendly conversation".to_string(),
        intent: Intent {
            intent_type: IntentType::Chat,
            confidence: 0.95,
            raw_input: "Nice weather today, isn't it?".to_string(),
            keywords: vec!["chat".to_string(), "friendly".to_string()],
        },
        emotional_state,
        expected_behavior_category: "friendly_or_joyful".to_string(),
    });

    Scenario {
        name: "peaceful_village".to_string(),
        description: "Normal friendly interactions in a peaceful setting".to_string(),
        steps,
    }
}

/// Threatening situation
fn create_threatening_scenario() -> Scenario {
    let mut steps = Vec::new();

    // Step 1: Mild fear
    let mut emotional_state = EmotionalState::new();
    emotional_state.fear = 0.5;
    emotional_state.anticipation = 0.4;
    steps.push(InteractionStep {
        description: "Distant sounds of danger".to_string(),
        intent: Intent {
            intent_type: IntentType::Custom,
            confidence: 0.7,
            raw_input: "There's a threat nearby".to_string(),
            keywords: vec!["threat".to_string(), "danger".to_string()],
        },
        emotional_state,
        expected_behavior_category: "cautious_or_flee".to_string(),
    });

    // Step 2: High fear - should trigger flee
    let mut emotional_state = EmotionalState::new();
    emotional_state.fear = 0.85;
    emotional_state.surprise = 0.3;
    steps.push(InteractionStep {
        description: "Direct threat appears".to_string(),
        intent: Intent {
            intent_type: IntentType::Custom,
            confidence: 1.0,
            raw_input: "A monster attacks!".to_string(),
            keywords: vec!["attack".to_string(), "danger".to_string(), "threat".to_string()],
        },
        emotional_state,
        expected_behavior_category: "flee".to_string(),
    });

    // Step 3: Extreme fear with high arousal
    let mut emotional_state = EmotionalState::new();
    emotional_state.fear = 0.95;
    emotional_state.surprise = 0.7;
    steps.push(InteractionStep {
        description: "Overwhelming danger".to_string(),
        intent: Intent {
            intent_type: IntentType::Custom,
            confidence: 1.0,
            raw_input: "The threat is coming closer!".to_string(),
            keywords: vec!["threat".to_string(), "danger".to_string()],
        },
        emotional_state,
        expected_behavior_category: "flee".to_string(),
    });

    Scenario {
        name: "threatening_situation".to_string(),
        description: "Escalating threat triggers fear-based behaviors".to_string(),
        steps,
    }
}

/// Provocative interaction
fn create_provocative_scenario() -> Scenario {
    let mut steps = Vec::new();

    // Step 1: Mild provocation, low anger
    let mut emotional_state = EmotionalState::new();
    emotional_state.anger = 0.3;
    emotional_state.disgust = 0.2;
    steps.push(InteractionStep {
        description: "Player makes rude comment".to_string(),
        intent: Intent {
            intent_type: IntentType::Custom,
            confidence: 0.8,
            raw_input: "You're not very helpful, are you?".to_string(),
            keywords: vec!["rude".to_string()],
        },
        emotional_state,
        expected_behavior_category: "cautious_or_friendly".to_string(),
    });

    // Step 2: Direct insult, rising anger
    let mut emotional_state = EmotionalState::new();
    emotional_state.anger = 0.6;
    emotional_state.disgust = 0.4;
    steps.push(InteractionStep {
        description: "Player directly insults NPC".to_string(),
        intent: Intent {
            intent_type: IntentType::Custom,
            confidence: 0.95,
            raw_input: "You're pathetic!".to_string(),
            keywords: vec!["insult".to_string()],
        },
        emotional_state,
        expected_behavior_category: "aggressive_or_cautious".to_string(),
    });

    // Step 3: Challenge with high anger and arousal
    let mut emotional_state = EmotionalState::new();
    emotional_state.anger = 0.85;
    emotional_state.disgust = 0.5;
    steps.push(InteractionStep {
        description: "Player issues direct challenge".to_string(),
        intent: Intent {
            intent_type: IntentType::Custom,
            confidence: 1.0,
            raw_input: "I challenge you to a fight!".to_string(),
            keywords: vec!["challenge".to_string(), "provoke".to_string()],
        },
        emotional_state,
        expected_behavior_category: "aggressive".to_string(),
    });

    Scenario {
        name: "provocative_interaction".to_string(),
        description: "Escalating provocation triggers anger-based behaviors".to_string(),
        steps,
    }
}

/// Mixed emotions scenario
fn create_mixed_emotions_scenario() -> Scenario {
    let mut steps = Vec::new();

    // Step 1: Conflicted - both fear and anger
    let mut emotional_state = EmotionalState::new();
    emotional_state.fear = 0.6;
    emotional_state.anger = 0.5;
    steps.push(InteractionStep {
        description: "Threatening but also insulting situation".to_string(),
        intent: Intent {
            intent_type: IntentType::Custom,
            confidence: 0.8,
            raw_input: "You're weak, and there's danger here!".to_string(),
            keywords: vec!["insult".to_string(), "threat".to_string()],
        },
        emotional_state,
        expected_behavior_category: "flee_or_aggressive".to_string(),
    });

    // Step 2: High joy overrides other emotions
    let mut emotional_state = EmotionalState::new();
    emotional_state.joy = 0.9;
    emotional_state.fear = 0.3;
    steps.push(InteractionStep {
        description: "Celebration despite minor concerns".to_string(),
        intent: Intent {
            intent_type: IntentType::Chat,
            confidence: 0.9,
            raw_input: "We won the festival!".to_string(),
            keywords: vec!["celebration".to_string(), "happy".to_string()],
        },
        emotional_state,
        expected_behavior_category: "joyful".to_string(),
    });

    // Step 3: Surprise with anticipation
    let mut emotional_state = EmotionalState::new();
    emotional_state.surprise = 0.7;
    emotional_state.anticipation = 0.6;
    steps.push(InteractionStep {
        description: "Unexpected but intriguing situation".to_string(),
        intent: Intent {
            intent_type: IntentType::Question,
            confidence: 0.85,
            raw_input: "What's happening over there?".to_string(),
            keywords: vec!["question".to_string(), "curious".to_string()],
        },
        emotional_state,
        expected_behavior_category: "cautious".to_string(),
    });

    Scenario {
        name: "mixed_emotions".to_string(),
        description: "Complex emotional states test priority resolution".to_string(),
        steps,
    }
}

/// Escalation and de-escalation scenario
fn create_escalation_scenario() -> Scenario {
    let mut steps = Vec::new();

    // Step 1: Calm start
    let mut emotional_state = EmotionalState::new();
    emotional_state.trust = 0.6;
    steps.push(InteractionStep {
        description: "Normal greeting".to_string(),
        intent: Intent {
            intent_type: IntentType::Greeting,
            confidence: 1.0,
            raw_input: "Hello".to_string(),
            keywords: vec!["hello".to_string()],
        },
        emotional_state,
        expected_behavior_category: "friendly".to_string(),
    });

    // Step 2: Rising tension
    let mut emotional_state = EmotionalState::new();
    emotional_state.anger = 0.7;
    emotional_state.anticipation = 0.5;
    steps.push(InteractionStep {
        description: "Tension builds".to_string(),
        intent: Intent {
            intent_type: IntentType::Custom,
            confidence: 0.9,
            raw_input: "I don't like your attitude".to_string(),
            keywords: vec!["confront".to_string()],
        },
        emotional_state,
        expected_behavior_category: "aggressive_or_cautious".to_string(),
    });

    // Step 3: Peak anger
    let mut emotional_state = EmotionalState::new();
    emotional_state.anger = 0.9;
    emotional_state.disgust = 0.6;
    steps.push(InteractionStep {
        description: "Peak of conflict".to_string(),
        intent: Intent {
            intent_type: IntentType::Custom,
            confidence: 1.0,
            raw_input: "How dare you insult me!".to_string(),
            keywords: vec!["insult".to_string(), "provoke".to_string()],
        },
        emotional_state,
        expected_behavior_category: "aggressive".to_string(),
    });

    // Step 4: De-escalation attempt
    let mut emotional_state = EmotionalState::new();
    emotional_state.anger = 0.4;
    emotional_state.sadness = 0.3;
    emotional_state.trust = 0.2;
    steps.push(InteractionStep {
        description: "Cooling down".to_string(),
        intent: Intent {
            intent_type: IntentType::Chat,
            confidence: 0.7,
            raw_input: "Wait, I'm sorry".to_string(),
            keywords: vec!["apology".to_string()],
        },
        emotional_state,
        expected_behavior_category: "cautious_or_friendly".to_string(),
    });

    // Step 5: Return to calm
    let mut emotional_state = EmotionalState::new();
    emotional_state.trust = 0.5;
    emotional_state.joy = 0.3;
    steps.push(InteractionStep {
        description: "Resolution".to_string(),
        intent: Intent {
            intent_type: IntentType::Chat,
            confidence: 0.9,
            raw_input: "Let's start over".to_string(),
            keywords: vec!["peace".to_string(), "friendly".to_string()],
        },
        emotional_state,
        expected_behavior_category: "friendly".to_string(),
    });

    Scenario {
        name: "escalation_and_resolution".to_string(),
        description: "Tests behavior adaptation through emotional escalation and de-escalation".to_string(),
        steps,
    }
}
