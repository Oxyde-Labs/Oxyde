//! Experiment runner for behavior priority study
//!
//! Runs all scenarios through all selection strategies and collects data

use crate::baselines::{
    EmotionModulatedStrategy, FixedPriorityStrategy, RandomSelectionStrategy, SelectionStrategy,
};
use crate::scenarios::{create_scenarios, InteractionStep, Scenario};
use oxyde::agent::AgentContext;
use oxyde::oxyde_game::behavior::{
    AggressiveBehavior, Behavior, CautiousBehavior, FleeBehavior, FriendlyBehavior, JoyfulBehavior,
    NeutralGreetingBehavior, ConfusedBehavior, PoliteDeclineBehavior,
    ThoughtfulPauseBehavior, DefaultAcknowledgeBehavior,
};
use oxyde::Result;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;

/// Result of a single interaction
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InteractionResult {
    /// Strategy used
    pub strategy: String,

    /// Scenario name
    pub scenario: String,

    /// Step number
    pub step: usize,

    /// Step description
    pub description: String,

    /// Selected behavior
    pub selected_behavior: String,

    /// Base priority of selected behavior
    pub base_priority: u32,

    /// Emotional priority modifier (only for emotion-modulated)
    pub emotional_modifier: i32,

    /// Final priority
    pub final_priority: i32,

    /// Dominant emotion
    pub dominant_emotion: String,

    /// Dominant emotion value
    pub dominant_value: f32,

    /// Valence
    pub valence: f32,

    /// Arousal
    pub arousal: f32,

    /// Response text
    pub response: String,

    /// Whether this matched expected category
    pub matches_expected: bool,
}

/// Summary statistics for a strategy
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StrategyStats {
    /// Strategy name
    pub strategy: String,

    /// Total interactions
    pub total_interactions: usize,

    /// Behavior selection counts
    pub behavior_counts: HashMap<String, usize>,

    /// Average emotional override impact
    pub avg_priority_override: f32,

    /// Behavior variety score (entropy)
    pub variety_score: f32,

    /// Percentage matching expected behaviors
    pub expected_match_rate: f32,
}

/// Run all experiments
pub async fn run_experiments() -> Result<(Vec<InteractionResult>, Vec<StrategyStats>)> {
    let scenarios = create_scenarios();
    let strategies = create_strategies();
    let behaviors = create_behaviors();
    let context = HashMap::new();

    let mut all_results = Vec::new();

    // Run each scenario through each strategy
    for scenario in &scenarios {
        for strategy in &strategies {
            let results = run_scenario(scenario, strategy.as_ref(), &behaviors, &context).await?;
            all_results.extend(results);
        }
    }

    // Compute statistics
    let stats = compute_statistics(&all_results);

    Ok((all_results, stats))
}

/// Create all selection strategies
fn create_strategies() -> Vec<Box<dyn SelectionStrategy>> {
    vec![
        Box::new(EmotionModulatedStrategy),
        Box::new(FixedPriorityStrategy),
        Box::new(RandomSelectionStrategy),
    ]
}

/// Create behavior instances
fn create_behaviors() -> Vec<Arc<dyn Behavior>> {
    vec![
        // Emotional behaviors
        Arc::new(FleeBehavior::new(0.7)),
        Arc::new(AggressiveBehavior::new(0.6)),
        Arc::new(FriendlyBehavior::new(0.3)),
        Arc::new(CautiousBehavior::new()),
        Arc::new(JoyfulBehavior::new()),
        // Neutral fallback behaviors (low priority, always available)
        Arc::new(NeutralGreetingBehavior::new()),
        Arc::new(ConfusedBehavior::new()),
        Arc::new(PoliteDeclineBehavior::new()),
        Arc::new(ThoughtfulPauseBehavior::new()),
        Arc::new(DefaultAcknowledgeBehavior::new()),
    ]
}

/// Run a scenario through a strategy
async fn run_scenario(
    scenario: &Scenario,
    strategy: &dyn SelectionStrategy,
    behaviors: &[Arc<dyn Behavior>],
    context: &AgentContext,
) -> Result<Vec<InteractionResult>> {
    let mut results = Vec::new();

    for (step_idx, step) in scenario.steps.iter().enumerate() {
        let result = run_step(step, step_idx, scenario, strategy, behaviors, context).await?;
        results.push(result);
    }

    Ok(results)
}

/// Run a single interaction step
async fn run_step(
    step: &InteractionStep,
    step_idx: usize,
    scenario: &Scenario,
    strategy: &dyn SelectionStrategy,
    behaviors: &[Arc<dyn Behavior>],
    context: &AgentContext,
) -> Result<InteractionResult> {
    let (selected_name, behavior_result) = strategy
        .select_behavior(behaviors, &step.intent, &step.emotional_state, context)
        .await?;

    // Get selected behavior details
    let selected_behavior = behaviors
        .iter()
        .find(|b| format!("{:?}", b).contains(&selected_name))
        .cloned();

    let base_priority = selected_behavior.as_ref().map(|b| b.priority()).unwrap_or(0);
    let emotional_modifier = selected_behavior
        .as_ref()
        .map(|b| b.emotional_priority_modifier(&step.emotional_state))
        .unwrap_or(0);
    let final_priority = base_priority as i32 + emotional_modifier;

    let (dominant_emotion, dominant_value) = step.emotional_state.dominant_emotion();

    let response = match behavior_result {
        oxyde::oxyde_game::behavior::BehaviorResult::Response(text) => text,
        oxyde::oxyde_game::behavior::BehaviorResult::Action(action) => {
            format!("[Action: {}]", action)
        }
        oxyde::oxyde_game::behavior::BehaviorResult::None => "No response".to_string(),
    };

    // Check if behavior matches expected category
    let matches_expected = check_expected_match(&selected_name, &step.expected_behavior_category);

    Ok(InteractionResult {
        strategy: strategy.name().to_string(),
        scenario: scenario.name.clone(),
        step: step_idx,
        description: step.description.clone(),
        selected_behavior: selected_name,
        base_priority,
        emotional_modifier,
        final_priority,
        dominant_emotion: dominant_emotion.to_string(),
        dominant_value,
        valence: step.emotional_state.valence(),
        arousal: step.emotional_state.arousal(),
        response,
        matches_expected,
    })
}

/// Check if selected behavior matches expected category
fn check_expected_match(selected: &str, expected: &str) -> bool {
    let selected_lower = selected.to_lowercase();
    let expected_lower = expected.to_lowercase();

    // Handle "or" cases
    if expected_lower.contains("_or_") {
        expected_lower
            .split("_or_")
            .any(|e| selected_lower.contains(e))
    } else {
        selected_lower.contains(&expected_lower)
    }
}

/// Compute statistics for all strategies
fn compute_statistics(results: &[InteractionResult]) -> Vec<StrategyStats> {
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

            let total_interactions = strategy_results.len();

            // Count behavior selections
            let mut behavior_counts = HashMap::new();
            for result in &strategy_results {
                *behavior_counts
                    .entry(result.selected_behavior.clone())
                    .or_insert(0) += 1;
            }

            // Average priority override
            let avg_priority_override = if !strategy_results.is_empty() {
                strategy_results
                    .iter()
                    .map(|r| r.emotional_modifier as f32)
                    .sum::<f32>()
                    / strategy_results.len() as f32
            } else {
                0.0
            };

            // Variety score (Shannon entropy)
            let variety_score = calculate_entropy(&behavior_counts, total_interactions);

            // Expected match rate
            let expected_matches = strategy_results.iter().filter(|r| r.matches_expected).count();
            let expected_match_rate = if !strategy_results.is_empty() {
                (expected_matches as f32 / strategy_results.len() as f32) * 100.0
            } else {
                0.0
            };

            StrategyStats {
                strategy: strategy.clone(),
                total_interactions,
                behavior_counts,
                avg_priority_override,
                variety_score,
                expected_match_rate,
            }
        })
        .collect()
}

/// Calculate Shannon entropy for behavior variety
fn calculate_entropy(counts: &HashMap<String, usize>, total: usize) -> f32 {
    if total == 0 {
        return 0.0;
    }

    counts
        .values()
        .map(|&count| {
            let p = count as f32 / total as f32;
            if p > 0.0 {
                -p * p.log2()
            } else {
                0.0
            }
        })
        .sum()
}
