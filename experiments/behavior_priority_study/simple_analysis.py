#!/usr/bin/env python3
"""
Simple analysis script without external dependencies
Analyzes the behavior priority study results
"""

import json
from pathlib import Path
from collections import Counter

def load_data():
    """Load experimental results"""
    results_dir = Path("experiments/behavior_priority_study/results")

    with open(results_dir / "detailed_results.json") as f:
        results = json.load(f)

    with open(results_dir / "statistics.json") as f:
        stats = json.load(f)

    return results, stats

def analyze_by_strategy(results):
    """Group results by strategy"""
    strategies = {}
    for result in results:
        strategy = result['strategy']
        if strategy not in strategies:
            strategies[strategy] = []
        strategies[strategy].append(result)
    return strategies

def print_comparison_table(strategies_data):
    """Print comparison table"""
    print("\n" + "="*80)
    print("BEHAVIOR SELECTION COMPARISON")
    print("="*80)

    for strategy, results in strategies_data.items():
        print(f"\n{strategy.upper().replace('_', ' ')}")
        print("-" * 80)

        # Count behaviors
        behaviors = Counter(r['selected_behavior'] for r in results)
        total = len(results)

        print(f"Total Interactions: {total}")
        print("\nBehavior Distribution:")
        for behavior, count in behaviors.most_common():
            pct = (count / total) * 100
            print(f"  {behavior:40} {count:3} ({pct:5.1f}%)")

        # Average emotional modifier
        avg_modifier = sum(r['emotional_modifier'] for r in results) / len(results)
        print(f"\nAvg Emotional Modifier: {avg_modifier:+.2f}")

        # Match rate
        matches = sum(1 for r in results if r['matches_expected'])
        match_rate = (matches / total) * 100
        print(f"Expected Match Rate: {match_rate:.1f}%")

def analyze_scenarios(results):
    """Analyze by scenario"""
    print("\n" + "="*80)
    print("SCENARIO ANALYSIS")
    print("="*80)

    scenarios = {}
    for result in results:
        scenario = result['scenario']
        if scenario not in scenarios:
            scenarios[scenario] = {}

        strategy = result['strategy']
        if strategy not in scenarios[scenario]:
            scenarios[scenario][strategy] = []
        scenarios[scenario][strategy].append(result)

    for scenario, strategies_data in scenarios.items():
        print(f"\n{scenario.upper().replace('_', ' ')}")
        print("-" * 80)

        for strategy, results_list in strategies_data.items():
            behaviors = [r['selected_behavior'] for r in results_list]
            print(f"  {strategy:20} => {', '.join(behaviors[:5])}")

def analyze_emotional_modifiers(results):
    """Analyze emotional priority modifiers"""
    print("\n" + "="*80)
    print("EMOTIONAL PRIORITY MODIFIERS")
    print("="*80)

    emotion_mod_results = [r for r in results if r['strategy'] == 'emotion_modulated']

    if not emotion_mod_results:
        print("No emotion-modulated results found")
        return

    print("\nLargest Priority Boosts (emotion_modulated):")
    sorted_results = sorted(emotion_mod_results, key=lambda x: x['emotional_modifier'], reverse=True)

    for result in sorted_results[:10]:
        if result['emotional_modifier'] > 0:
            print(f"  {result['scenario']:25} Step {result['step']:2}: "
                  f"{result['selected_behavior']:30} "
                  f"Modifier: +{result['emotional_modifier']:2} "
                  f"(Emotion: {result['dominant_emotion']}/{result['dominant_value']:.2f})")

    print("\nLargest Priority Reductions (emotion_modulated):")
    for result in sorted(emotion_mod_results, key=lambda x: x['emotional_modifier'])[:10]:
        if result['emotional_modifier'] < 0:
            print(f"  {result['scenario']:25} Step {result['step']:2}: "
                  f"{result['selected_behavior']:30} "
                  f"Modifier: {result['emotional_modifier']:2} "
                  f"(Emotion: {result['dominant_emotion']}/{result['dominant_value']:.2f})")

def generate_text_report(results, stats):
    """Generate text report"""
    report_path = Path("experiments/behavior_priority_study/results/REPORT.txt")

    with open(report_path, 'w') as f:
        f.write("BEHAVIOR PRIORITY STUDY - RESULTS\n")
        f.write("=" * 80 + "\n\n")

        f.write("Overview\n")
        f.write("-" * 80 + "\n")
        f.write("This experiment compares emotion-modulated behavior selection against\n")
        f.write("traditional fixed-priority and random selection approaches.\n\n")

        f.write("Key Findings\n")
        f.write("-" * 80 + "\n\n")

        for stat in stats:
            f.write(f"{stat['strategy'].upper().replace('_', ' ')}\n")
            f.write(f"  Variety Score: {stat['variety_score']:.3f}\n")
            f.write(f"  Match Rate: {stat['expected_match_rate']:.1f}%\n")
            f.write(f"  Avg Priority Override: {stat['avg_priority_override']:.2f}\n\n")

        # Conclusions
        emotion_mod = next(s for s in stats if s['strategy'] == 'emotion_modulated')
        fixed = next(s for s in stats if s['strategy'] == 'fixed_priority')

        variety_diff = ((emotion_mod['variety_score'] - fixed['variety_score'])
                       / fixed['variety_score'] * 100)
        match_diff = emotion_mod['expected_match_rate'] - fixed['expected_match_rate']

        f.write("Conclusions\n")
        f.write("-" * 80 + "\n")
        f.write(f"1. Variety Difference: {variety_diff:+.1f}%\n")
        f.write(f"2. Match Rate Difference: {match_diff:+.1f}%\n")
        f.write(f"3. Emotional Influence: {emotion_mod['avg_priority_override']:.2f} avg modifier\n")

    print(f"\n✓ Saved text report to {report_path}")

def main():
    print("="*80)
    print("BEHAVIOR PRIORITY STUDY - SIMPLE ANALYSIS")
    print("="*80)

    # Load data
    print("\nLoading data...")
    results, stats = load_data()
    print(f"✓ Loaded {len(results)} interactions")

    # Group by strategy
    strategies_data = analyze_by_strategy(results)

    # Print comparisons
    print_comparison_table(strategies_data)
    analyze_scenarios(results)
    analyze_emotional_modifiers(results)

    # Generate report
    print("\nGenerating text report...")
    generate_text_report(results, stats)

    print("\n" + "="*80)
    print("✓ Analysis complete!")
    print("="*80)

if __name__ == "__main__":
    main()
