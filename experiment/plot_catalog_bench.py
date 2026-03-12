#!/usr/bin/env python3
"""
Plot catalog benchmark results from bench_catalog_results.csv.

Usage:
    python3 plot_catalog_bench.py [path/to/bench_catalog_results.csv]

Generates:
    - catalog_build_time.png       : build time vs SF for each mode
    - catalog_peak_memory.png      : peak memory vs SF for each mode
    - catalog_thread_scaling.png   : build time vs threads for each SF (fixed mode)
    - catalog_stat_size.png        : statistic file size vs SF
"""

import sys
import pandas as pd
import matplotlib.pyplot as plt
import matplotlib
import numpy as np

matplotlib.rcParams["font.family"] = "sans-serif"
matplotlib.rcParams["figure.dpi"] = 150


def load_data(csv_path):
    df = pd.read_csv(csv_path)
    # Extract numeric SF value for sorting
    df["sf_num"] = df["sf"].str.replace("sf", "").astype(float)
    df = df.sort_values(["sf_num", "mode", "threads", "repeat"])
    return df


def plot_build_time_by_mode(df, output_dir):
    """Build time vs SF for each (mode, threads) combination."""
    fig, axes = plt.subplots(1, 2, figsize=(14, 6), sharey=True)

    for idx, mode in enumerate(df["mode"].unique()):
        ax = axes[idx]
        mode_df = df[df["mode"] == mode]
        mode_name = mode_df["mode_name"].iloc[0]

        for threads in sorted(mode_df["threads"].unique()):
            sub = mode_df[mode_df["threads"] == threads]
            grouped = sub.groupby("sf_num")["wall_time_ms"].agg(["mean", "std"]).reset_index()
            ax.errorbar(
                grouped["sf_num"],
                grouped["mean"] / 1000,
                yerr=grouped["std"] / 1000,
                marker="o",
                label=f"{threads} threads",
                capsize=3,
            )

        ax.set_xscale("log")
        ax.set_yscale("log")
        ax.set_xlabel("Scale Factor")
        ax.set_ylabel("Build Time (s)")
        ax.set_title(f"Mode: {mode_name}")
        ax.legend()
        ax.grid(True, alpha=0.3)

    fig.suptitle("Catalog Build Time vs Scale Factor", fontsize=14)
    plt.tight_layout()
    plt.savefig(f"{output_dir}/catalog_build_time.png")
    plt.close()
    print("Saved: catalog_build_time.png")


def plot_peak_memory(df, output_dir):
    """Peak memory vs SF for each mode."""
    fig, ax = plt.subplots(figsize=(10, 6))

    # Use max threads for a clean comparison
    max_threads = df["threads"].max()
    sub = df[df["threads"] == max_threads]

    for mode in sorted(sub["mode"].unique()):
        mode_df = sub[sub["mode"] == mode]
        mode_name = mode_df["mode_name"].iloc[0]
        grouped = mode_df.groupby("sf_num")["peak_mem_mb"].agg(["mean", "std"]).reset_index()
        ax.errorbar(
            grouped["sf_num"],
            grouped["mean"],
            yerr=grouped["std"],
            marker="s",
            label=f"{mode_name}",
            capsize=3,
        )

    ax.set_xscale("log")
    ax.set_xlabel("Scale Factor")
    ax.set_ylabel("Peak Memory (MB)")
    ax.set_title(f"Peak Memory vs Scale Factor (threads={max_threads})")
    ax.legend()
    ax.grid(True, alpha=0.3)

    plt.tight_layout()
    plt.savefig(f"{output_dir}/catalog_peak_memory.png")
    plt.close()
    print("Saved: catalog_peak_memory.png")


def plot_thread_scaling(df, output_dir):
    """Build time vs thread count for each SF, comparing modes."""
    sfs = sorted(df["sf_num"].unique())
    # Pick a few representative SFs
    if len(sfs) > 6:
        indices = np.linspace(0, len(sfs) - 1, 6, dtype=int)
        selected_sfs = [sfs[i] for i in indices]
    else:
        selected_sfs = sfs

    n = len(selected_sfs)
    cols = min(3, n)
    rows = (n + cols - 1) // cols
    fig, axes = plt.subplots(rows, cols, figsize=(5 * cols, 4 * rows), squeeze=False)

    for i, sf_num in enumerate(selected_sfs):
        ax = axes[i // cols][i % cols]
        sf_df = df[df["sf_num"] == sf_num]
        sf_name = sf_df["sf"].iloc[0]

        for mode in sorted(sf_df["mode"].unique()):
            mode_df = sf_df[sf_df["mode"] == mode]
            mode_name = mode_df["mode_name"].iloc[0]
            grouped = mode_df.groupby("threads")["wall_time_ms"].agg(["mean", "std"]).reset_index()
            ax.errorbar(
                grouped["threads"],
                grouped["mean"] / 1000,
                yerr=grouped["std"] / 1000,
                marker="o",
                label=mode_name,
                capsize=3,
            )

        ax.set_xlabel("Threads")
        ax.set_ylabel("Build Time (s)")
        ax.set_title(f"SF = {sf_name}")
        ax.legend(fontsize=8)
        ax.grid(True, alpha=0.3)

    # Hide unused axes
    for i in range(n, rows * cols):
        axes[i // cols][i % cols].set_visible(False)

    fig.suptitle("Thread Scaling: Build Time vs Thread Count", fontsize=14)
    plt.tight_layout()
    plt.savefig(f"{output_dir}/catalog_thread_scaling.png")
    plt.close()
    print("Saved: catalog_thread_scaling.png")


def plot_stat_size(df, output_dir):
    """Statistic file size vs SF."""
    fig, ax = plt.subplots(figsize=(8, 5))

    # stat_size is the same regardless of mode/threads, just pick one
    sub = df.groupby("sf_num").first().reset_index()
    sub = sub[sub["stat_size_bytes"] > 0]

    ax.plot(sub["sf_num"], sub["stat_size_mb"], marker="D", color="green")

    ax.set_xscale("log")
    ax.set_xlabel("Scale Factor")
    ax.set_ylabel("Statistic Size (MB)")
    ax.set_title("Catalog Statistic Serialized Size vs Scale Factor")
    ax.grid(True, alpha=0.3)

    plt.tight_layout()
    plt.savefig(f"{output_dir}/catalog_stat_size.png")
    plt.close()
    print("Saved: catalog_stat_size.png")


def main():
    csv_path = sys.argv[1] if len(sys.argv) > 1 else "experiment/bench_catalog_results.csv"
    output_dir = str(csv_path).rsplit("/", 1)[0] if "/" in csv_path else "."

    df = load_data(csv_path)

    print(f"Loaded {len(df)} rows from {csv_path}")
    print(f"Scale factors: {sorted(df['sf'].unique())}")
    print(f"Modes: {sorted(df['mode_name'].unique())}")
    print(f"Thread counts: {sorted(df['threads'].unique())}")
    print()

    plot_build_time_by_mode(df, output_dir)
    plot_peak_memory(df, output_dir)
    plot_thread_scaling(df, output_dir)
    plot_stat_size(df, output_dir)

    print("\nAll plots saved.")


if __name__ == "__main__":
    main()
