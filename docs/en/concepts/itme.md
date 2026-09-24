# Itme (Benchmarking Tool)
**Itme** is a high-precision benchmarking utility designed for LightVM, providing developers with actionable insights into code performance. By leveraging statistical analysis and adaptive iteration cycles, **Itme** ensures reliable and reproducible measurements for your functions.

## Rust workload suites

The two Cargo auto-discovered drivers each register 36 paired Itme cases. Cargo discovers them as:

```bash
cargo bench --bench normal
cargo bench --bench optimize
```

To actually run the Itme cases with the default stable-Rust test harness, use:

```bash
cargo bench --bench normal -- --test
cargo bench --bench optimize -- --test
```

Auto-discovered benchmarks use Cargo's default test harness, so the extra `-- --test` is necessary; without it, the targets compile but the harness reports zero benchmarks. Both modes use the same raw workloads. Optimization happens once before Itme setup; VM creation and loading are excluded from the measured `time per op`. Each execution starts with fresh VM runtime state.

## How Itme Works
Itme utilizes an adaptive execution strategy to balance benchmarking duration with precision. It automatically calibrates the number of iterations required to reach a target execution window, followed by a rigorous statistical analysis of the samples to filter out noise and report accurate performance metrics.

 * **Adaptive Iteration**: Rather than using a fixed loop count, Itme dynamically adjusts the number of iterations until the measured time meets the `target_time`. This ensures both short-running and long-running functions receive a statistically significant sample size.
 * **Warm-up Cycles**: Executes multiple pre-measurement runs to ensure the CPU cache and branch predictors are primed, reducing "cold start" noise in the final data.
 * **Outlier Filtering (IQR Method)**: Uses the Interquartile Range (IQR) method to identify and prune performance outliers. By excluding anomalous data points, Itme provides a more accurate representation of the median execution time.
 * **Statistical Analysis**: Calculates mean, standard deviation, and stability percentages, allowing you to gauge the consistency of your code’s performance across multiple runs.
 * **Throughput Calculation**: When byte sizes are provided, Itme automatically calculates throughput in MiB/s, helping you measure the data-processing efficiency of your algorithms.
 * **Noise Detection**: Automatically flags benchmarks as `[NOISY]` if high variance is detected (stability > 15%), alerting you to potential performance instability or external interference.
 * **Precise Reporting**: Generates a formatted, color-coded CLI output that clearly displays the time per operation, the performance range, stability metrics, and throughput.
