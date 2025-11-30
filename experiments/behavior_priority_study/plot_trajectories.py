#!/usr/bin/env python3
"""
Trajectory Plotting Script

Generates 30 emotional trajectory plots (10 patterns × 3 strategies)
showing how emotions evolve over 30 turns for each combination.

Usage:
    python3 plot_trajectories.py
"""

import pandas as pd
import matplotlib.pyplot as plt
import os
from pathlib import Path

# Configuration
RESULTS_DIR = Path("results")
PLOTS_DIR = RESULTS_DIR / "plots"
DATA_FILE = RESULTS_DIR / "trajectory_data.csv"

# Plutchik's 8 emotions
EMOTIONS = ["joy", "trust", "fear", "surprise", "sadness", "disgust", "anger", "anticipation"]

# Color mapping for emotions
EMOTION_COLORS = {
    "joy": "#FFD700",  # Gold
    "trust": "#32CD32",  # LimeGreen
    "fear": "#8B008B",  # DarkMagenta
    "surprise": "#FF69B4",  # HotPink
    "sadness": "#4169E1",  # RoyalBlue
    "disgust": "#8B4513",  # SaddleBrown
    "anger": "#FF4500",  # OrangeRed
    "anticipation": "#00CED1",  # DarkTurquoise
}

def load_data():
    """Load trajectory data from CSV"""
    print(f"Loading data from {DATA_FILE}...")
    df = pd.read_csv(DATA_FILE)
    print(f"Loaded {len(df)} total interactions")
    return df

def create_trajectory_plot(df, pattern, strategy, output_path):
    """
    Create a single trajectory plot showing all 8 emotions over 30 turns

    Args:
        df: DataFrame with trajectory data
        pattern: Pattern name
        strategy: Strategy name
        output_path: Where to save the plot
    """
    # Filter data for this pattern + strategy
    data = df[(df["pattern"] == pattern) & (df["strategy"] == strategy)].copy()
    data = data.sort_values("turn")

    if len(data) == 0:
        print(f"WARNING: No data for {pattern} + {strategy}")
        return

    # Create figure
    fig, ax = plt.subplots(figsize=(12, 7))

    # Plot each emotion
    for emotion in EMOTIONS:
        col_name = f"{emotion}_after"
        if col_name in data.columns:
            ax.plot(
                data["turn"],
                data[col_name],
                label=emotion.capitalize(),
                color=EMOTION_COLORS.get(emotion, "#000000"),
                linewidth=2,
                marker='o',
                markersize=4,
                alpha=0.8
            )

    # Formatting
    ax.set_xlabel("Turn", fontsize=12, fontweight='bold')
    ax.set_ylabel("Emotion Intensity", fontsize=12, fontweight='bold')
    ax.set_title(f"{pattern}\n{strategy.upper()}", fontsize=14, fontweight='bold', pad=20)
    ax.set_xlim(1, 30)
    ax.set_ylim(-0.05, 1.05)
    ax.grid(True, alpha=0.3, linestyle='--')
    ax.legend(loc='upper left', bbox_to_anchor=(1.02, 1), fontsize=10)

    # Add horizontal line at 0.5 for reference
    ax.axhline(y=0.5, color='gray', linestyle=':', linewidth=1, alpha=0.5)

    plt.tight_layout()
    plt.savefig(output_path, dpi=150, bbox_inches='tight')
    plt.close()

    print(f"  ✓ Saved {output_path.name}")

def generate_all_plots():
    """Generate all 30 trajectory plots"""
    print("\n" + "="*60)
    print("TRAJECTORY PLOT GENERATION")
    print("="*60 + "\n")

    # Load data
    df = load_data()

    # Create output directory
    PLOTS_DIR.mkdir(parents=True, exist_ok=True)
    print(f"Output directory: {PLOTS_DIR}\n")

    # Get unique patterns and strategies
    patterns = sorted(df["pattern"].unique())
    strategies = sorted(df["strategy"].unique())

    print(f"Found {len(patterns)} patterns and {len(strategies)} strategies")
    print(f"Generating {len(patterns) * len(strategies)} plots...\n")

    # Generate plots
    plot_count = 0
    for pattern in patterns:
        print(f"Pattern: {pattern}")
        for strategy in strategies:
            # Clean filename
            safe_pattern = pattern.replace(" ", "_").replace("/", "_")
            safe_strategy = strategy.replace("_", "-")
            filename = f"{safe_pattern}_{safe_strategy}.png"
            output_path = PLOTS_DIR / filename

            create_trajectory_plot(df, pattern, strategy, output_path)
            plot_count += 1
        print()

    print("="*60)
    print(f"✓ Generated {plot_count} trajectory plots")
    print(f"✓ Plots saved to: {PLOTS_DIR}")
    print("="*60)

def generate_comparison_grid():
    """
    Generate a comparison grid showing all patterns side-by-side
    for each strategy (bonus visualization)
    """
    print("\nGenerating comparison grids...")

    df = load_data()
    patterns = sorted(df["pattern"].unique())
    strategies = sorted(df["strategy"].unique())

    for strategy in strategies:
        fig, axes = plt.subplots(2, 5, figsize=(24, 10))
        fig.suptitle(f"All Patterns - {strategy.upper()}", fontsize=16, fontweight='bold')

        for idx, pattern in enumerate(patterns):
            row = idx // 5
            col = idx % 5
            ax = axes[row, col]

            # Filter data
            data = df[(df["pattern"] == pattern) & (df["strategy"] == strategy)].copy()
            data = data.sort_values("turn")

            # Plot emotions
            for emotion in EMOTIONS:
                col_name = f"{emotion}_after"
                if col_name in data.columns:
                    ax.plot(
                        data["turn"],
                        data[col_name],
                        label=emotion,
                        color=EMOTION_COLORS.get(emotion, "#000000"),
                        linewidth=1.5,
                        alpha=0.7
                    )

            ax.set_title(pattern, fontsize=10, fontweight='bold')
            ax.set_xlim(1, 30)
            ax.set_ylim(0, 1)
            ax.grid(True, alpha=0.2)

            if col == 0:
                ax.set_ylabel("Intensity", fontsize=9)
            if row == 1:
                ax.set_xlabel("Turn", fontsize=9)

        plt.tight_layout()
        output_path = PLOTS_DIR / f"comparison_grid_{strategy}.png"
        plt.savefig(output_path, dpi=150, bbox_inches='tight')
        plt.close()

        print(f"  ✓ Saved {output_path.name}")

if __name__ == "__main__":
    generate_all_plots()
    generate_comparison_grid()

    print("\n" + "="*60)
    print("NEXT STEPS:")
    print("1. Review trajectory plots in results/plots/")
    print("2. Look for coherent emotional arcs vs flat/chaotic patterns")
    print("3. Use trajectory_data.csv + PAPER_GENERATION_BRIEF.md for paper")
    print("="*60)
