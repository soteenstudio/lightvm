# Itme (Benchmarking Tool)
**Itme** is a benchmarking utility for **LightVM** that measures function performance using adaptive iteration counts and statistical analysis.

## How Itme Works
Itme calibrates iterations per sample, performs warm-up runs, and measures the configured number of samples. It filters outliers and reports timing variation; measurements can still be affected by the host environment.

  * **Adaptive Iteration**: Adjusts iterations per sample until calibration reaches `target_time` or the limit of one billion iterations. Calibration changes iteration count, not sample count; `samples()` configures the sample count, which defaults to 15.
  * **Warm-up Cycles**: Performs 25 unmeasured runs using the calibrated iteration count to reduce cold-start effects.
  * **Outlier Filtering (IQR Method)**: Filters samples outside the quartile boundaries expanded by 1.5 times the IQR. If fewer than three samples remain, it uses all original samples.
  * **Statistical Analysis**: Calculates mean and standard deviation from the effective samples. The stability metric is standard deviation / mean × 100; lower values indicate less variation.
  * **Throughput Calculation**: When byte sizes are provided, Itme automatically calculates throughput in MiB/s, helping you measure the data-processing efficiency of your algorithms.
  * **Noise Detection**: Automatically flags benchmarks as `[NOISY]` if high variance is detected (stability > 15%), alerting you to potential performance instability or external interference.
  * **Formatted Reporting**: Displays median time per operation, minimum and maximum times per operation, stability, and optional throughput in color-coded CLI output.
