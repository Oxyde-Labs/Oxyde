//! Baseline behavior selection systems for comparison
//!
//! This module implements the RandomSelectionStrategy baseline for research purposes.
//! The EmotionModulatedStrategy and FixedPriorityStrategy have been moved to the main
//! SDK at `oxyde::oxyde_game::behavior::strategy`.

use oxyde::agent::AgentContext;
use oxyde::oxyde_game::behavior::{Behavior, BehaviorResult};
use oxyde::oxyde_game::emotion::EmotionalState;
use oxyde::oxyde_game::intent::Intent;
use oxyde::Result;
use std::sync::Arc;

// Re-export SDK strategies for convenience
pub use oxyde::oxyde_game::behavior::{
    EmotionModulatedStrategy, FixedPriorityStrategy, SelectionStrategy,
};

/// Random selection from matching behaviors (research baseline only)
///
/// This strategy randomly selects from behaviors that match the intent,
/// ignoring both emotional state and priority. It's useful for ablation
/// studies to demonstrate the value of priority-based selection.
///
/// **Note**: This is kept in the experiment folder because it's only useful
/// for research comparisons, not production use cases.
pub struct RandomSelectionStrategy;

#[async_trait::async_trait]
impl SelectionStrategy for RandomSelectionStrategy {
    async fn select_behavior(
        &self,
        behaviors: &[Arc<dyn Behavior>],
        intent: &Intent,
        _emotional_state: &EmotionalState,
        context: &AgentContext,
    ) -> Result<(String, BehaviorResult)> {
        // Filter matching behaviors (IGNORE emotional state for baseline)
        let mut candidates = Vec::new();
        for behavior in behaviors {
            // NO emotion trigger check - this is the baseline!
            if behavior.matches_intent(intent).await {
                candidates.push(behavior);
            }
        }

        if candidates.is_empty() {
            return Ok(("none".to_string(), BehaviorResult::None));
        }

        // Select randomly - use index to avoid holding RNG across await
        let selected_idx = {
            use rand::Rng;
            let mut rng = rand::thread_rng();
            rng.gen_range(0..candidates.len())
        };

        let selected = candidates[selected_idx];
        let result = selected.execute(intent, context).await?;
        let name = format!("{:?}", selected)
            .split('(')
            .next()
            .unwrap_or("unknown")
            .to_string();

        Ok((name, result))
    }

    fn name(&self) -> &str {
        "random_selection"
    }
}
