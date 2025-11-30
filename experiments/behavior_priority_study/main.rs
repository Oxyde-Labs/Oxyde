//! Extended Trajectory Study - Main Entry Point
//!
//! Research experiment: "Emotional Memory and Character Development in Game NPCs"
//!
//! This study compares three behavior selection strategies across 10 player
//! behavior patterns (30 turns each) = 900 total interactions.
//!
//! Focus: Do emotion-modulated NPCs show coherent emotional arcs and character
//! development compared to fixed-priority or random selection?

mod baselines;
mod patterns;
mod runner;
mod scenarios;
mod trajectory_runner;

use baselines::{EmotionModulatedStrategy, FixedPriorityStrategy, RandomSelectionStrategy};
use oxyde::oxyde_game::behavior::{
    AggressiveBehavior, CautiousBehavior, FleeBehavior, FriendlyBehavior, JoyfulBehavior,
    NeutralGreetingBehavior, ConfusedBehavior, PoliteDeclineBehavior,
    ThoughtfulPauseBehavior, DefaultAcknowledgeBehavior,
};
use std::fs;
use std::path::Path;
use std::sync::Arc;

#[tokio::main]
async fn main() -> oxyde::Result<()> {
    // Initialize logging
    env_logger::init();

    println!("==================================================");
    println!("EXTENDED TRAJECTORY STUDY");
    println!("Emotional Memory and Character Development");
    println!("==================================================\n");

    // Create strategies
    let strategies = vec![
        Box::new(EmotionModulatedStrategy) as Box<dyn baselines::SelectionStrategy>,
        Box::new(FixedPriorityStrategy),
        Box::new(RandomSelectionStrategy),
    ];

    // Create behaviors (10 total: 5 emotional + 5 neutral)
    let behaviors = vec![
        Arc::new(FleeBehavior::new(0.7)) as Arc<dyn oxyde::oxyde_game::behavior::Behavior>,
        Arc::new(AggressiveBehavior::new(0.6)),
        Arc::new(JoyfulBehavior::new()),
        Arc::new(FriendlyBehavior::new(0.3)),
        Arc::new(CautiousBehavior::new()),
        Arc::new(NeutralGreetingBehavior::new()),
        Arc::new(ConfusedBehavior::new()),
        Arc::new(PoliteDeclineBehavior::new()),
        Arc::new(ThoughtfulPauseBehavior::new()),
        Arc::new(DefaultAcknowledgeBehavior::new()),
    ];

    // Run trajectory study
    println!("Running 900 interactions (10 patterns × 3 strategies × 30 turns)...\n");
    let (results, stats) = trajectory_runner::run_trajectory_study(strategies, behaviors).await?;

    println!("✓ Completed {} total interactions\n", results.len());

    // Print statistics
    println!("==================================================");
    println!("TRAJECTORY STATISTICS");
    println!("==================================================\n");

    for stat in &stats {
        print_trajectory_stats(stat);
    }

    // Save results
    let results_dir = Path::new("experiments/behavior_priority_study/results");
    fs::create_dir_all(results_dir)?;

    // Save detailed trajectory data
    let results_json = serde_json::to_string_pretty(&results)?;
    fs::write(results_dir.join("trajectory_results.json"), results_json)?;
    println!("\n✓ Saved detailed trajectory data to results/trajectory_results.json");

    // Save statistics
    let stats_json = serde_json::to_string_pretty(&stats)?;
    fs::write(results_dir.join("trajectory_statistics.json"), stats_json)?;
    println!("✓ Saved statistics to results/trajectory_statistics.json");

    // Save CSV for analysis and plotting
    save_trajectory_csv(&results, results_dir)?;
    println!("✓ Saved CSV data to results/trajectory_data.csv\n");

    println!("==================================================");
    println!("Next Steps:");
    println!("1. Run: python3 experiments/behavior_priority_study/plot_trajectories.py");
    println!("2. View 30 trajectory plots in results/plots/");
    println!("3. Use trajectory_data.csv + PAPER_GENERATION_BRIEF.md for paper");
    println!("==================================================");

    Ok(())
}

fn print_trajectory_stats(stats: &trajectory_runner::TrajectoryStatistics) {
    println!("Strategy: {}", stats.strategy.to_uppercase());
    println!("  Total Turns: {}", stats.total_turns);
    println!("  True Override Rate: {:.1}%", stats.true_override_rate);
    println!("  Neutral Fallback Rate: {:.1}%", stats.neutral_fallback_rate);
    println!("  Emotional Behavior Rate: {:.1}%", stats.emotional_behavior_rate);
    println!("  Stuck Emotions: {}", stats.stuck_emotions);
    println!("  Behavior Variety: {:.3}", stats.behavior_variety);
    println!("  Trajectory Coherence: {:.3}", stats.trajectory_coherence);
    println!("  Avg Emotion Persistence: {:.1} turns", stats.avg_emotion_persistence);
    println!();
}

fn save_trajectory_csv(results: &[trajectory_runner::TrajectoryTurnResult], results_dir: &Path) -> oxyde::Result<()> {
    let mut csv = String::new();

    // CSV header
    csv.push_str("pattern,strategy,turn,player_action,intent_type,");
    csv.push_str("joy_before,trust_before,fear_before,surprise_before,sadness_before,disgust_before,anger_before,anticipation_before,");
    csv.push_str("valence_before,arousal_before,dominant_emotion_before,");
    csv.push_str("selected_behavior,behavior_type,base_priority,emotional_modifier,effective_priority,priority_override,");
    csv.push_str("joy_after,trust_after,fear_after,surprise_after,sadness_after,disgust_after,anger_after,anticipation_after,");
    csv.push_str("valence_after,arousal_after,dominant_emotion_after\n");

    for result in results {
        csv.push_str(&format!(
            "{},{},{},{},{},",
            result.pattern,
            result.strategy,
            result.turn,
            result.player_action.replace(',', ";"), // Escape commas
            result.intent_type,
        ));

        // Emotions before
        csv.push_str(&format!(
            "{:.3},{:.3},{:.3},{:.3},{:.3},{:.3},{:.3},{:.3},",
            result.emotions_before.joy,
            result.emotions_before.trust,
            result.emotions_before.fear,
            result.emotions_before.surprise,
            result.emotions_before.sadness,
            result.emotions_before.disgust,
            result.emotions_before.anger,
            result.emotions_before.anticipation,
        ));

        csv.push_str(&format!(
            "{:.3},{:.3},{},",
            result.emotions_before.valence,
            result.emotions_before.arousal,
            result.emotions_before.dominant_emotion,
        ));

        // Behavior selection
        csv.push_str(&format!(
            "{},{},{},{},{},{},",
            result.selected_behavior,
            result.behavior_type,
            result.base_priority,
            result.emotional_modifier,
            result.effective_priority,
            result.priority_override_occurred,
        ));

        // Emotions after
        csv.push_str(&format!(
            "{:.3},{:.3},{:.3},{:.3},{:.3},{:.3},{:.3},{:.3},",
            result.emotions_after.joy,
            result.emotions_after.trust,
            result.emotions_after.fear,
            result.emotions_after.surprise,
            result.emotions_after.sadness,
            result.emotions_after.disgust,
            result.emotions_after.anger,
            result.emotions_after.anticipation,
        ));

        csv.push_str(&format!(
            "{:.3},{:.3},{}\n",
            result.emotions_after.valence,
            result.emotions_after.arousal,
            result.emotions_after.dominant_emotion,
        ));
    }

    fs::write(results_dir.join("trajectory_data.csv"), csv)?;
    Ok(())
}
