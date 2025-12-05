//! Extended trajectory runner for 900-interaction study
//!
//! Runs 10 patterns × 3 strategies × 30 turns = 900 total interactions
//! Tracks emotional state before and after each turn for trajectory analysis

use crate::baselines::SelectionStrategy;
use crate::patterns::{create_patterns, BehaviorPattern, PatternTurn};
use oxyde::agent::AgentContext;
use oxyde::oxyde_game::behavior::{Behavior, BehaviorResult, EmotionInfluence};
use oxyde::oxyde_game::emotion::EmotionalState;
use oxyde::Result;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;

/// Result of a single turn in the trajectory study
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TrajectoryTurnResult {
    /// Pattern name
    pub pattern: String,

    /// Strategy name
    pub strategy: String,

    /// Turn number (1-30)
    pub turn: usize,

    /// Player action description
    pub player_action: String,

    /// Intent type
    pub intent_type: String,

    // BEFORE behavior selection
    /// Emotional state snapshot BEFORE this turn
    pub emotions_before: EmotionSnapshot,

    // Behavior selection
    /// Selected behavior name
    pub selected_behavior: String,

    /// Behavior type (emotional or neutral)
    pub behavior_type: String,

    /// Base priority
    pub base_priority: u32,

    /// Emotional modifier
    pub emotional_modifier: i32,

    /// Effective priority
    pub effective_priority: i32,

    /// Was this behavior selection changed by emotional modifier?
    /// (vs what would have been selected by base priority alone)
    pub priority_override_occurred: bool,

    /// NPC response text
    pub response: String,

    // AFTER behavior execution
    /// Emotional state snapshot AFTER this turn
    pub emotions_after: EmotionSnapshot,

    /// Emotion changes applied by this behavior
    pub emotion_influences: Vec<EmotionInfluenceRecord>,
}

/// Snapshot of all 8 Plutchik emotions plus derived metrics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EmotionSnapshot {
    pub joy: f32,
    pub trust: f32,
    pub fear: f32,
    pub surprise: f32,
    pub sadness: f32,
    pub disgust: f32,
    pub anger: f32,
    pub anticipation: f32,

    /// Valence: positive (-1.0) to negative (1.0)
    pub valence: f32,

    /// Arousal: calm (0.0) to excited (1.0)
    pub arousal: f32,

    /// Dominant emotion name
    pub dominant_emotion: String,

    /// Dominant emotion value
    pub dominant_value: f32,
}

impl EmotionSnapshot {
    /// Create snapshot from EmotionalState
    pub fn from_state(state: &EmotionalState) -> Self {
        let (dominant_emotion, dominant_value) = state.dominant_emotion();
        Self {
            joy: state.joy,
            trust: state.trust,
            fear: state.fear,
            surprise: state.surprise,
            sadness: state.sadness,
            disgust: state.disgust,
            anger: state.anger,
            anticipation: state.anticipation,
            valence: state.valence(),
            arousal: state.arousal(),
            dominant_emotion: dominant_emotion.to_string(),
            dominant_value,
        }
    }
}

/// Record of emotion influence applied
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EmotionInfluenceRecord {
    pub emotion: String,
    pub delta: f32,
}

/// Summary statistics for trajectory analysis
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TrajectoryStatistics {
    pub strategy: String,

    /// Total number of turns
    pub total_turns: usize,

    /// True priority override rate (% of times modifier changed selection)
    pub true_override_rate: f32,

    /// % of turns using neutral fallback behaviors
    pub neutral_fallback_rate: f32,

    /// % of turns using emotional behaviors
    pub emotional_behavior_rate: f32,

    /// Number of "stuck" emotions (max value for 10+ consecutive turns)
    pub stuck_emotions: usize,

    /// Shannon entropy of behavior selection
    pub behavior_variety: f32,

    /// Trajectory coherence score (0-1, higher = more coherent)
    pub trajectory_coherence: f32,

    /// Average emotion persistence (turns before 50% decay)
    pub avg_emotion_persistence: f32,
}

/// Run complete trajectory study
pub async fn run_trajectory_study(
    strategies: Vec<Box<dyn SelectionStrategy>>,
    behaviors: Vec<Arc<dyn Behavior>>,
) -> Result<(Vec<TrajectoryTurnResult>, Vec<TrajectoryStatistics>)> {
    let patterns = create_patterns();
    let context = HashMap::new();

    let mut all_results = Vec::new();

    println!("\n==================================================");
    println!("EXTENDED TRAJECTORY STUDY");
    println!("10 patterns × 3 strategies × 30 turns = 900 interactions");
    println!("==================================================\n");

    // Run each pattern through each strategy
    for pattern in &patterns {
        println!("Running pattern: {}", pattern.name);

        for strategy in &strategies {
            println!("  Strategy: {}", strategy.name());

            let results = run_pattern(
                pattern,
                strategy.as_ref(),
                &behaviors,
                &context,
            ).await?;

            all_results.extend(results);
        }
    }

    println!("\n✓ Completed {} total interactions\n", all_results.len());

    // Compute statistics
    let stats = compute_trajectory_statistics(&all_results);

    Ok((all_results, stats))
}

/// Run a single pattern through a strategy
async fn run_pattern(
    pattern: &BehaviorPattern,
    strategy: &dyn SelectionStrategy,
    behaviors: &[Arc<dyn Behavior>],
    context: &AgentContext,
) -> Result<Vec<TrajectoryTurnResult>> {
    let mut results = Vec::new();
    let mut emotional_state = EmotionalState::with_decay_rate(0.1); // 10% decay per turn

    for turn in &pattern.turns {
        let result = run_turn(
            pattern,
            turn,
            strategy,
            behaviors,
            context,
            &mut emotional_state,
        ).await?;

        results.push(result);

        // Apply emotion decay after each turn
        emotional_state.decay();
    }

    Ok(results)
}

/// Run a single turn
async fn run_turn(
    pattern: &BehaviorPattern,
    turn: &PatternTurn,
    strategy: &dyn SelectionStrategy,
    behaviors: &[Arc<dyn Behavior>],
    context: &AgentContext,
    emotional_state: &mut EmotionalState,
) -> Result<TrajectoryTurnResult> {
    // Apply emotional reaction to player action BEFORE behavior selection
    // This seeds the emotional state based on what the player did
    use oxyde::oxyde_game::intent::IntentType;
    match turn.intent.intent_type {
        IntentType::Threat | IntentType::Hostile => {
            emotional_state.update_emotion("fear", 0.3);
            emotional_state.update_emotion("anger", 0.2);
        }
        IntentType::Demand => {
            emotional_state.update_emotion("anger", 0.2);
            emotional_state.update_emotion("fear", 0.15);
        }
        IntentType::Friendly | IntentType::Greeting => {
            emotional_state.update_emotion("joy", 0.15);
            emotional_state.update_emotion("trust", 0.1);
        }
        IntentType::Request => {
            emotional_state.update_emotion("trust", 0.05);
        }
        _ => {}
    }

    // Snapshot emotions AFTER player action reaction but BEFORE selection
    let emotions_before = EmotionSnapshot::from_state(emotional_state);

    // Find all behaviors that match intent (for priority override detection)
    let mut matching_behaviors: Vec<Arc<dyn Behavior>> = Vec::new();
    for behavior in behaviors {
        if behavior.matches_intent(&turn.intent).await {
            matching_behaviors.push(behavior.clone());
        }
    }

    // Select behavior using strategy
    let (selected_name, behavior_result) = strategy
        .select_behavior(behaviors, &turn.intent, emotional_state, context)
        .await?;

    // Find the selected behavior
    let selected_behavior = behaviors
        .iter()
        .find(|b| format!("{:?}", b).contains(&selected_name))
        .cloned();

    let base_priority = selected_behavior.as_ref().map(|b| b.priority()).unwrap_or(0);
    let emotional_modifier = selected_behavior
        .as_ref()
        .map(|b| b.emotional_priority_modifier(emotional_state))
        .unwrap_or(0);
    let effective_priority = base_priority as i32 + emotional_modifier;

    // Determine behavior type
    let behavior_type = if selected_name.contains("Neutral") ||
        selected_name.contains("Confused") ||
        selected_name.contains("Polite") ||
        selected_name.contains("Thoughtful") ||
        selected_name.contains("Default")
    {
        "neutral".to_string()
    } else {
        "emotional".to_string()
    };

    // Detect if priority override occurred
    // Would a different behavior have been selected by base priority alone?
    let priority_override_occurred = detect_priority_override(
        &matching_behaviors,
        &selected_name,
        emotional_state,
    );

    // Extract response text
    let response = match behavior_result {
        BehaviorResult::Response(text) => text,
        BehaviorResult::Action(action) => format!("[Action: {}]", action),
        BehaviorResult::None => "No response".to_string(),
    };

    // Apply emotion influences from selected behavior
    let mut emotion_influences = Vec::new();
    if let Some(behavior) = selected_behavior.as_ref() {
        for influence in behavior.emotion_influences() {
            emotion_influences.push(EmotionInfluenceRecord {
                emotion: influence.emotion.clone(),
                delta: influence.delta,
            });

            // Apply to state
            emotional_state.update_emotion(&influence.emotion, influence.delta);
        }
    }

    // Snapshot emotions AFTER application
    let emotions_after = EmotionSnapshot::from_state(emotional_state);

    Ok(TrajectoryTurnResult {
        pattern: pattern.name.clone(),
        strategy: strategy.name().to_string(),
        turn: turn.turn,
        player_action: turn.description.clone(),
        intent_type: format!("{:?}", turn.intent.intent_type),
        emotions_before,
        selected_behavior: selected_name,
        behavior_type,
        base_priority,
        emotional_modifier,
        effective_priority,
        priority_override_occurred,
        response,
        emotions_after,
        emotion_influences,
    })
}

/// Detect if emotional modifier changed which behavior was selected
fn detect_priority_override(
    matching_behaviors: &[Arc<dyn Behavior>],
    selected_name: &str,
    emotional_state: &EmotionalState,
) -> bool {
    if matching_behaviors.is_empty() {
        return false;
    }

    // Sort by base priority only
    let mut base_sorted = matching_behaviors.to_vec();
    base_sorted.sort_by_key(|b| -(b.priority() as i32));
    let base_winner = format!("{:?}", base_sorted[0]);

    // Sort by effective priority (base + modifier)
    let mut effective_sorted = matching_behaviors.to_vec();
    effective_sorted.sort_by_key(|b| {
        -(b.priority() as i32 + b.emotional_priority_modifier(emotional_state))
    });
    let effective_winner = format!("{:?}", effective_sorted[0]);

    // Override occurred if winners differ
    !base_winner.contains(selected_name) && effective_winner.contains(selected_name)
}

/// Compute trajectory statistics for all strategies
fn compute_trajectory_statistics(results: &[TrajectoryTurnResult]) -> Vec<TrajectoryStatistics> {
    let strategies: Vec<String> = results
        .iter()
        .map(|r| r.strategy.clone())
        .collect::<std::collections::HashSet<_>>()
        .into_iter()
        .collect();

    strategies
        .iter()
        .map(|strategy| {
            let strategy_results: Vec<_> = results
                .iter()
                .filter(|r| &r.strategy == strategy)
                .collect();

            compute_strategy_statistics(strategy, &strategy_results)
        })
        .collect()
}

/// Compute statistics for a single strategy
fn compute_strategy_statistics(
    strategy: &str,
    results: &[&TrajectoryTurnResult],
) -> TrajectoryStatistics {
    let total_turns = results.len();

    // True override rate
    let override_count = results.iter().filter(|r| r.priority_override_occurred).count();
    let true_override_rate = if total_turns > 0 {
        (override_count as f32 / total_turns as f32) * 100.0
    } else {
        0.0
    };

    // Neutral fallback rate
    let neutral_count = results.iter().filter(|r| r.behavior_type == "neutral").count();
    let neutral_fallback_rate = if total_turns > 0 {
        (neutral_count as f32 / total_turns as f32) * 100.0
    } else {
        0.0
    };

    let emotional_behavior_rate = 100.0 - neutral_fallback_rate;

    // Stuck emotions (emotion at max for 10+ consecutive turns)
    let stuck_emotions = detect_stuck_emotions(results);

    // Behavior variety (Shannon entropy)
    let behavior_variety = calculate_behavior_entropy(results);

    // Trajectory coherence (pattern-specific analysis)
    let trajectory_coherence = calculate_trajectory_coherence(results);

    // Emotion persistence
    let avg_emotion_persistence = calculate_emotion_persistence(results);

    TrajectoryStatistics {
        strategy: strategy.to_string(),
        total_turns,
        true_override_rate,
        neutral_fallback_rate,
        emotional_behavior_rate,
        stuck_emotions,
        behavior_variety,
        trajectory_coherence,
        avg_emotion_persistence,
    }
}

/// Detect stuck emotions (maxed out for 10+ turns)
fn detect_stuck_emotions(results: &[&TrajectoryTurnResult]) -> usize {
    let mut stuck_count = 0;
    let threshold = 0.95; // Consider "maxed" if >= 0.95

    // Check each emotion type
    for emotion in &["joy", "trust", "fear", "surprise", "sadness", "disgust", "anger", "anticipation"] {
        let mut consecutive_max = 0;

        for result in results {
            let value = match *emotion {
                "joy" => result.emotions_after.joy,
                "trust" => result.emotions_after.trust,
                "fear" => result.emotions_after.fear,
                "surprise" => result.emotions_after.surprise,
                "sadness" => result.emotions_after.sadness,
                "disgust" => result.emotions_after.disgust,
                "anger" => result.emotions_after.anger,
                "anticipation" => result.emotions_after.anticipation,
                _ => 0.0,
            };

            if value >= threshold {
                consecutive_max += 1;
            } else {
                if consecutive_max >= 10 {
                    stuck_count += 1;
                }
                consecutive_max = 0;
            }
        }

        if consecutive_max >= 10 {
            stuck_count += 1;
        }
    }

    stuck_count
}

/// Calculate Shannon entropy of behavior selection
fn calculate_behavior_entropy(results: &[&TrajectoryTurnResult]) -> f32 {
    let mut behavior_counts: HashMap<String, usize> = HashMap::new();

    for result in results {
        *behavior_counts.entry(result.selected_behavior.clone()).or_insert(0) += 1;
    }

    let total = results.len() as f32;
    behavior_counts
        .values()
        .map(|&count| {
            let p = count as f32 / total;
            if p > 0.0 {
                -p * p.log2()
            } else {
                0.0
            }
        })
        .sum()
}

/// Calculate trajectory coherence score
/// Higher score = emotional arcs make narrative sense
fn calculate_trajectory_coherence(results: &[&TrajectoryTurnResult]) -> f32 {
    // Group by pattern
    let mut pattern_results: HashMap<String, Vec<&TrajectoryTurnResult>> = HashMap::new();
    for result in results {
        pattern_results.entry(result.pattern.clone()).or_insert_with(Vec::new).push(result);
    }

    let mut coherence_scores = Vec::new();

    for (_pattern, pattern_results) in pattern_results {
        // Check for smooth transitions (no sudden jumps > 0.5)
        let mut smooth_transitions = 0;
        let mut total_transitions = 0;

        for window in pattern_results.windows(2) {
            if let [prev, curr] = window {
                total_transitions += 1;

                // Check all emotions for sudden jumps
                let emotions = [
                    (prev.emotions_after.joy, curr.emotions_before.joy),
                    (prev.emotions_after.trust, curr.emotions_before.trust),
                    (prev.emotions_after.fear, curr.emotions_before.fear),
                    (prev.emotions_after.surprise, curr.emotions_before.surprise),
                    (prev.emotions_after.sadness, curr.emotions_before.sadness),
                    (prev.emotions_after.disgust, curr.emotions_before.disgust),
                    (prev.emotions_after.anger, curr.emotions_before.anger),
                    (prev.emotions_after.anticipation, curr.emotions_before.anticipation),
                ];

                let max_jump = emotions.iter()
                    .map(|(prev, curr)| (curr - prev).abs())
                    .fold(0.0f32, f32::max);

                if max_jump < 0.5 {
                    smooth_transitions += 1;
                }
            }
        }

        if total_transitions > 0 {
            coherence_scores.push(smooth_transitions as f32 / total_transitions as f32);
        }
    }

    // Average coherence across patterns
    if coherence_scores.is_empty() {
        0.0
    } else {
        coherence_scores.iter().sum::<f32>() / coherence_scores.len() as f32
    }
}

/// Calculate average emotion persistence (turns before 50% decay)
fn calculate_emotion_persistence(results: &[&TrajectoryTurnResult]) -> f32 {
    let mut persistence_measurements = Vec::new();

    // For each significant emotional event (emotion > 0.7), track how long it persists
    for (i, result) in results.iter().enumerate() {
        let emotions = [
            ("joy", result.emotions_after.joy),
            ("trust", result.emotions_after.trust),
            ("fear", result.emotions_after.fear),
            ("surprise", result.emotions_after.surprise),
            ("sadness", result.emotions_after.sadness),
            ("disgust", result.emotions_after.disgust),
            ("anger", result.emotions_after.anger),
            ("anticipation", result.emotions_after.anticipation),
        ];

        for (emotion_name, initial_value) in emotions {
            if initial_value > 0.7 {
                // Track how many turns until this emotion drops below 50% of initial
                let threshold = initial_value * 0.5;
                let mut turns_to_decay = 0;

                for j in (i + 1)..results.len() {
                    if results[j].pattern != result.pattern {
                        break; // Different pattern
                    }

                    let current_value = match emotion_name {
                        "joy" => results[j].emotions_before.joy,
                        "trust" => results[j].emotions_before.trust,
                        "fear" => results[j].emotions_before.fear,
                        "surprise" => results[j].emotions_before.surprise,
                        "sadness" => results[j].emotions_before.sadness,
                        "disgust" => results[j].emotions_before.disgust,
                        "anger" => results[j].emotions_before.anger,
                        "anticipation" => results[j].emotions_before.anticipation,
                        _ => 0.0,
                    };

                    turns_to_decay += 1;

                    if current_value < threshold {
                        break;
                    }
                }

                persistence_measurements.push(turns_to_decay as f32);
            }
        }
    }

    if persistence_measurements.is_empty() {
        0.0
    } else {
        persistence_measurements.iter().sum::<f32>() / persistence_measurements.len() as f32
    }
}
