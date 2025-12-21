#!/usr/bin/env python3
"""
Analysis and visualization script for behavior priority study

Generates plots comparing emotion-modulated vs baseline strategies
"""

import json
import pandas as pd
import matplotlib.pyplot as plt
import seaborn as sns
from pathlib import Path

# Set style
sns.set_style("whitegrid")
plt.rcParams['figure.figsize'] = (12, 8)

def load_data():
    """Load experimental results"""
    results_dir = Path("experiments/behavior_priority_study/results")

    # Load detailed results
    with open(results_dir / "detailed_results.json") as f:
        results = json.load(f)

    # Load statistics
    with open(results_dir / "statistics.json") as f:
        stats = json.load(f)

    # Create DataFrame
    df = pd.DataFrame(results)

    return df, stats

def plot_behavior_distribution(df, output_dir):
    """Plot behavior selection distribution by strategy"""
    fig, axes = plt.subplots(1, 3, figsize=(18, 6))

    strategies = df['strategy'].unique()

    for idx, strategy in enumerate(strategies):
        strategy_data = df[df['strategy'] == strategy]
        behavior_counts = strategy_data['selected_behavior'].value_counts()

        axes[idx].bar(range(len(behavior_counts)), behavior_counts.values)
        axes[idx].set_xticks(range(len(behavior_counts)))
        axes[idx].set_xticklabels(behavior_counts.index, rotation=45, ha='right')
        axes[idx].set_title(f'{strategy.replace("_", " ").title()}\nBehavior Distribution')
        axes[idx].set_ylabel('Count')
        axes[idx].set_xlabel('Behavior')

    plt.tight_layout()
    plt.savefig(output_dir / 'behavior_distribution.png', dpi=300, bbox_inches='tight')
    print(f"✓ Saved behavior_distribution.png")
    plt.close()

def plot_variety_comparison(stats, output_dir):
    """Plot behavior variety scores"""
    strategies = [s['strategy'] for s in stats]
    variety_scores = [s['variety_score'] for s in stats]

    plt.figure(figsize=(10, 6))
    bars = plt.bar(strategies, variety_scores, color=['#2ecc71', '#3498db', '#e74c3c'])
    plt.ylabel('Variety Score (Shannon Entropy)', fontsize=12)
    plt.xlabel('Strategy', fontsize=12)
    plt.title('Behavior Variety Comparison\n(Higher = More Diverse Behavior)', fontsize=14, fontweight='bold')
    plt.xticks([s.replace('_', ' ').title() for s in strategies])

    # Add value labels on bars
    for bar in bars:
        height = bar.get_height()
        plt.text(bar.get_x() + bar.get_width()/2., height,
                f'{height:.3f}',
                ha='center', va='bottom', fontweight='bold')

    plt.tight_layout()
    plt.savefig(output_dir / 'variety_comparison.png', dpi=300, bbox_inches='tight')
    print(f"✓ Saved variety_comparison.png")
    plt.close()

def plot_expected_match_rate(stats, output_dir):
    """Plot how well behaviors matched expected categories"""
    strategies = [s['strategy'] for s in stats]
    match_rates = [s['expected_match_rate'] for s in stats]

    plt.figure(figsize=(10, 6))
    bars = plt.bar(strategies, match_rates, color=['#2ecc71', '#3498db', '#e74c3c'])
    plt.ylabel('Match Rate (%)', fontsize=12)
    plt.xlabel('Strategy', fontsize=12)
    plt.title('Expected Behavior Match Rate\n(Higher = More Contextually Appropriate)', fontsize=14, fontweight='bold')
    plt.xticks([s.replace('_', ' ').title() for s in strategies])
    plt.ylim(0, 100)

    # Add value labels
    for bar in bars:
        height = bar.get_height()
        plt.text(bar.get_x() + bar.get_width()/2., height,
                f'{height:.1f}%',
                ha='center', va='bottom', fontweight='bold')

    plt.tight_layout()
    plt.savefig(output_dir / 'match_rate_comparison.png', dpi=300, bbox_inches='tight')
    print(f"✓ Saved match_rate_comparison.png")
    plt.close()

def plot_priority_modulation(df, output_dir):
    """Plot priority modulation over scenarios"""
    emotion_mod = df[df['strategy'] == 'emotion_modulated']

    if emotion_mod.empty:
        print("! No emotion-modulated data found")
        return

    fig, axes = plt.subplots(2, 1, figsize=(14, 10))

    # Plot 1: Base vs Final Priority
    scenarios = emotion_mod['scenario'].unique()
    for scenario in scenarios:
        scenario_data = emotion_mod[emotion_mod['scenario'] == scenario]
        axes[0].plot(scenario_data['step'], scenario_data['base_priority'],
                    'o--', alpha=0.6, label=f'{scenario} (base)')
        axes[0].plot(scenario_data['step'], scenario_data['final_priority'],
                    's-', alpha=0.8, label=f'{scenario} (final)')

    axes[0].set_xlabel('Interaction Step', fontsize=11)
    axes[0].set_ylabel('Priority', fontsize=11)
    axes[0].set_title('Emotional Priority Modulation\nBase vs Final Priority', fontsize=13, fontweight='bold')
    axes[0].legend(bbox_to_anchor=(1.05, 1), loc='upper left', fontsize=8)
    axes[0].grid(True, alpha=0.3)

    # Plot 2: Emotional Modifier Impact
    for scenario in scenarios:
        scenario_data = emotion_mod[emotion_mod['scenario'] == scenario]
        axes[1].plot(scenario_data['step'], scenario_data['emotional_modifier'],
                    'o-', alpha=0.7, label=scenario)

    axes[1].set_xlabel('Interaction Step', fontsize=11)
    axes[1].set_ylabel('Emotional Modifier', fontsize=11)
    axes[1].set_title('Emotional Modifier Over Time\n(Positive = Priority Boost, Negative = Priority Reduction)',
                     fontsize=13, fontweight='bold')
    axes[1].legend(bbox_to_anchor=(1.05, 1), loc='upper left', fontsize=9)
    axes[1].axhline(y=0, color='k', linestyle='--', alpha=0.3)
    axes[1].grid(True, alpha=0.3)

    plt.tight_layout()
    plt.savefig(output_dir / 'priority_modulation.png', dpi=300, bbox_inches='tight')
    print(f"✓ Saved priority_modulation.png")
    plt.close()

def plot_emotion_behavior_heatmap(df, output_dir):
    """Heatmap showing which emotions trigger which behaviors"""
    emotion_mod = df[df['strategy'] == 'emotion_modulated']

    if emotion_mod.empty:
        return

    # Create pivot table
    pivot = emotion_mod.groupby(['dominant_emotion', 'selected_behavior']).size().unstack(fill_value=0)

    plt.figure(figsize=(12, 8))
    sns.heatmap(pivot, annot=True, fmt='d', cmap='YlOrRd', cbar_kws={'label': 'Count'})
    plt.title('Emotion-Behavior Association\n(Emotion-Modulated Strategy)', fontsize=14, fontweight='bold')
    plt.xlabel('Selected Behavior', fontsize=12)
    plt.ylabel('Dominant Emotion', fontsize=12)
    plt.tight_layout()
    plt.savefig(output_dir / 'emotion_behavior_heatmap.png', dpi=300, bbox_inches='tight')
    print(f"✓ Saved emotion_behavior_heatmap.png")
    plt.close()

def plot_scenario_comparison(df, output_dir):
    """Compare behavior selections across scenarios"""
    fig, axes = plt.subplots(5, 1, figsize=(14, 18))

    scenarios = sorted(df['scenario'].unique())
    strategies = df['strategy'].unique()

    for idx, scenario in enumerate(scenarios):
        scenario_data = df[df['scenario'] == scenario]

        for strategy in strategies:
            strategy_data = scenario_data[scenario_data['strategy'] == strategy]
            steps = strategy_data['step'].values
            # Encode behaviors as numbers for plotting
            behavior_map = {b: i for i, b in enumerate(df['selected_behavior'].unique())}
            behaviors = [behavior_map[b] for b in strategy_data['selected_behavior'].values]

            axes[idx].plot(steps, behaviors, 'o-', label=strategy, alpha=0.7, markersize=8)

        axes[idx].set_title(f'Scenario: {scenario.replace("_", " ").title()}', fontsize=12, fontweight='bold')
        axes[idx].set_xlabel('Step')
        axes[idx].set_ylabel('Behavior')
        axes[idx].legend()
        axes[idx].grid(True, alpha=0.3)

    plt.tight_layout()
    plt.savefig(output_dir / 'scenario_comparison.png', dpi=300, bbox_inches='tight')
    print(f"✓ Saved scenario_comparison.png")
    plt.close()

def generate_summary_report(df, stats, output_dir):
    """Generate markdown summary report"""
    report = """# Behavior Priority Study - Results

## Overview
This experiment compares emotion-modulated behavior selection against traditional approaches.

## Key Findings

"""

    # Add statistics comparison
    for stat in stats:
        report += f"### {stat['strategy'].replace('_', ' ').title()}\n\n"
        report += f"- **Variety Score**: {stat['variety_score']:.3f}\n"
        report += f"- **Expected Match Rate**: {stat['expected_match_rate']:.1f}%\n"
        report += f"- **Avg Priority Override**: {stat['avg_priority_override']:.2f}\n"
        report += f"- **Total Interactions**: {stat['total_interactions']}\n\n"

        report += "**Behavior Distribution**:\n"
        for behavior, count in sorted(stat['behavior_counts'].items(), key=lambda x: x[1], reverse=True):
            pct = (count / stat['total_interactions']) * 100
            report += f"- {behavior}: {count} ({pct:.1f}%)\n"
        report += "\n"

    # Conclusions
    emotion_mod_stats = next(s for s in stats if s['strategy'] == 'emotion_modulated')
    fixed_stats = next(s for s in stats if s['strategy'] == 'fixed_priority')

    variety_improvement = ((emotion_mod_stats['variety_score'] - fixed_stats['variety_score'])
                          / fixed_stats['variety_score'] * 100)

    report += f"""## Conclusions

1. **Behavior Variety**: Emotion-modulated approach shows {variety_improvement:+.1f}% difference in variety compared to fixed priority
2. **Context Appropriateness**: {emotion_mod_stats['expected_match_rate']:.1f}% of behaviors matched expected categories
3. **Emotional Influence**: Average priority modification of {emotion_mod_stats['avg_priority_override']:.2f}

## Visualizations

See generated PNG files for detailed visualizations.
"""

    with open(output_dir / 'REPORT.md', 'w') as f:
        f.write(report)

    print(f"✓ Saved REPORT.md")

def main():
    print("==================================================")
    print("Analyzing Behavior Priority Study Results")
    print("==================================================\n")

    # Load data
    print("Loading data...")
    df, stats = load_data()
    print(f"✓ Loaded {len(df)} interactions\n")

    # Create output directory
    output_dir = Path("experiments/behavior_priority_study/results")
    output_dir.mkdir(parents=True, exist_ok=True)

    # Generate plots
    print("Generating visualizations...")
    plot_behavior_distribution(df, output_dir)
    plot_variety_comparison(stats, output_dir)
    plot_expected_match_rate(stats, output_dir)
    plot_priority_modulation(df, output_dir)
    plot_emotion_behavior_heatmap(df, output_dir)
    plot_scenario_comparison(df, output_dir)

    # Generate report
    print("\nGenerating summary report...")
    generate_summary_report(df, stats, output_dir)

    print("\n==================================================")
    print("✓ Analysis complete!")
    print(f"✓ Results saved to {output_dir}")
    print("==================================================")

if __name__ == "__main__":
    main()
